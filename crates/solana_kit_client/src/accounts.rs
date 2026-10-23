//! Off-chain account helpers preserve the typed RPC boundary and node context.

use crate::{
    Client, Error, Transport,
    methods::{
        AccountInfo, Contextual, ReadConfig,
        accounts::{
            AccountConfig, AccountData, AccountEncoding, AccountInfoWithConfig, MultipleAccounts,
            data,
        },
    },
};

/// Requires the parsed branch of an RPC snapshot. Encoded and compressed
/// representations remain distinct and cannot be asserted into parsed values.
pub fn assert_account_decoded(
    account: &Account<AccountData>,
) -> Result<&crate::methods::accounts::ParsedAccount, Error> {
    match &account.data {
        AccountData::Parsed(parsed) => Ok(parsed),
        _ => Err(Error::AccountNotDecoded(account.address)),
    }
}

/// Standalone account metadata, without a requested address or data payload.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseAccount {
    /// Whether the account contains an executable program.
    pub executable: bool,
    /// Balance checked to fit an unsigned 64-bit integer.
    #[serde(deserialize_with = "data::metadata_number")]
    pub lamports: u64,
    /// Program that owns the account.
    #[serde(deserialize_with = "data::address")]
    pub owner: Address,
    /// Complete allocation, independent of a returned data payload.
    #[serde(deserialize_with = "data::metadata_number")]
    pub space: u64,
    /// Rent epoch when supplied. Explicit null is invalid.
    #[serde(default)]
    pub rent_epoch: crate::Presence<u64>,
}

/// Parses an external standalone header without requiring data or an address.
/// Numeric strings follow Dart integer syntax, bounded to Rust's u64 range.
pub fn parse_base_account(raw: &str) -> Result<BaseAccount, Error> {
    serde_json::from_str(raw).map_err(Error::Schema)
}

/// Requires parsed data only when the requested account exists.
pub fn assert_maybe_account_decoded(
    account: &MaybeAccount<AccountData>,
) -> Result<Option<&crate::methods::accounts::ParsedAccount>, Error> {
    match account {
        MaybeAccount::Missing(_) => Ok(None),
        MaybeAccount::Present(account) => assert_account_decoded(account).map(Some),
    }
}

/// Reports every encoded or compressed account in input order.
pub fn assert_accounts_decoded(accounts: &[Account<AccountData>]) -> Result<(), Vec<Address>> {
    decoded_addresses(accounts.iter())
}

/// Reports encoded existing accounts; absent addresses impose no decoded-data
/// requirement. Custom binary decoders instead produce a statically typed T.
pub fn assert_maybe_accounts_decoded(
    accounts: &[MaybeAccount<AccountData>],
) -> Result<(), Vec<Address>> {
    decoded_addresses(accounts.iter().filter_map(|account| match account {
        MaybeAccount::Present(account) => Some(account),
        MaybeAccount::Missing(_) => None,
    }))
}

fn decoded_addresses<'a>(
    accounts: impl Iterator<Item = &'a Account<AccountData>>,
) -> Result<(), Vec<Address>> {
    let addresses: Vec<_> = accounts
        .filter(|account| assert_account_decoded(account).is_err())
        .map(|account| account.address)
        .collect();

    if addresses.is_empty() {
        return Ok(());
    }

    Err(addresses)
}
use solana_kit_core::{
    Address,
    accounts::{Account, MaybeAccount, MaybeEncodedAccount},
};
use solana_kit_core::{
    accounts::{AccountError, EncodedAccount},
    codecs::{Decoder, Limits},
};

/// Decodes borrowed bytes while retaining the original snapshot on failure.
/// The declared allocation is metadata, so sliced or padded source bytes are
/// passed unchanged to the caller's exact decoder. Owner validation is the
/// caller's policy; use core's stricter `EncodedAccount::decode` when required.
pub fn decode_account<T>(
    account: &EncodedAccount,
    decoder: &impl Decoder<T>,
    limits: Limits,
) -> Result<Account<T>, AccountError> {
    let data = decoder
        .decode(&account.data, limits)
        .map_err(|source| AccountError::Decode {
            address: account.address,
            source,
        })?;

    Ok(Account {
        address: account.address,
        owner: account.owner,
        lamports: account.lamports,
        executable: account.executable,
        space: account.space,
        rent_epoch: account.rent_epoch,
        data,
    })
}

/// Decodes existing accounts and preserves missing identities without invoking
/// the decoder. The borrowed input retains metadata when decoding fails.
pub fn decode_maybe_account<T>(
    account: &MaybeEncodedAccount,
    decoder: &impl Decoder<T>,
    limits: Limits,
) -> Result<MaybeAccount<T>, AccountError> {
    match account {
        MaybeAccount::Missing(address) => Ok(MaybeAccount::Missing(*address)),
        MaybeAccount::Present(account) => {
            decode_account(account, decoder, limits).map(MaybeAccount::Present)
        }
    }
}

/// Requires all accounts to exist, returning every missing address in input
/// order, including duplicates. Empty collections satisfy the requirement.
pub fn assert_accounts_exist<T>(accounts: &[MaybeAccount<T>]) -> Result<(), Vec<Address>> {
    let missing: Vec<_> = accounts
        .iter()
        .filter(|account| !account.exists())
        .map(MaybeAccount::address)
        .collect();

    if missing.is_empty() {
        return Ok(());
    }

    Err(missing)
}

impl<T: Transport> Client<T> {
    /// Fetches a complete base64 snapshot, retaining the node evaluation context.
    pub async fn fetch_encoded_account(
        &self,
        address: Address,
        config: ReadConfig,
    ) -> Result<Contextual<MaybeEncodedAccount>, Error> {
        self.call(AccountInfo(address, config)).await
    }

    /// Fetches snapshots in requested order, including duplicates and missing entries.
    /// A response with a different number of entries is rejected.
    pub async fn fetch_encoded_accounts(
        &self,
        addresses: Vec<Address>,
        config: ReadConfig,
    ) -> Result<Contextual<Vec<MaybeEncodedAccount>>, Error> {
        let result = self
            .call(MultipleAccounts(
                addresses,
                Some(account_config(config, AccountEncoding::Base64)),
            ))
            .await?;
        let value = result
            .value
            .into_iter()
            .map(encoded)
            .collect::<Result<_, _>>()?;

        Ok(Contextual {
            context: result.context,
            value,
        })
    }

    /// Fetches opaque parsed JSON or a binary fallback, retaining source metadata.
    pub async fn fetch_json_parsed_account(
        &self,
        address: Address,
        config: ReadConfig,
    ) -> Result<Contextual<MaybeAccount<AccountData>>, Error> {
        self.call(AccountInfoWithConfig(
            address,
            Some(account_config(config, AccountEncoding::JsonParsed)),
        ))
        .await
    }

    /// Fetches positional parsed JSON, binary fallbacks and missing accounts.
    pub async fn fetch_json_parsed_accounts(
        &self,
        addresses: Vec<Address>,
        config: ReadConfig,
    ) -> Result<Contextual<Vec<MaybeAccount<AccountData>>>, Error> {
        self.call(MultipleAccounts(
            addresses,
            Some(account_config(config, AccountEncoding::JsonParsed)),
        ))
        .await
    }
}

/// Parses a nullable standalone RPC account with tagged or plain base64 data.
/// Allocation metadata is independent of the returned byte length.
pub fn parse_base64_rpc_account(address: Address, raw: &str) -> Result<MaybeEncodedAccount, Error> {
    encoded(data::parse_account(address, raw, AccountEncoding::Base64)?)
}

/// Parses a nullable standalone RPC account with tagged or legacy base58 data.
/// Unlike Dart's dynamic coercion, non-string encoded values are rejected.
pub fn parse_base58_rpc_account(address: Address, raw: &str) -> Result<MaybeEncodedAccount, Error> {
    encoded(data::parse_account(address, raw, AccountEncoding::Base58)?)
}

/// Parses nullable program JSON without discarding unknown parsed fields.
/// A base64 fallback retains its distinct binary representation.
pub fn parse_json_rpc_account(
    address: Address,
    raw: &str,
) -> Result<MaybeAccount<AccountData>, Error> {
    data::parse_account(address, raw, AccountEncoding::JsonParsed)
}

fn account_config(read: ReadConfig, encoding: AccountEncoding) -> AccountConfig {
    AccountConfig {
        read,
        encoding: Some(encoding),
        data_slice: None,
    }
}

fn encoded(account: MaybeAccount<AccountData>) -> Result<MaybeEncodedAccount, Error> {
    match account {
        MaybeAccount::Missing(address) => Ok(MaybeAccount::Missing(address)),
        MaybeAccount::Present(account) => {
            let AccountData::Bytes { bytes, slice: None } = account.data else {
                return Err(Error::AccountData);
            };

            Ok(MaybeAccount::Present(Account {
                address: account.address,
                owner: account.owner,
                lamports: account.lamports,
                executable: account.executable,
                space: account.space,
                rent_epoch: account.rent_epoch,
                data: bytes,
            }))
        }
    }
}
