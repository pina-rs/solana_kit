//! Explicit optional RPC fetching; the application owns transport and executor.
use crate::{
    PROGRAM,
    accounts::{Token2022Account, decode_account_with_program},
};
use alloc::vec::Vec;
use solana_kit_client::{
    Client, Transport,
    methods::{
        Contextual, ReadConfig,
        accounts::{
            AccountConfig, AccountData, AccountEncoding, AccountInfoWithConfig, MultipleAccounts,
        },
    },
};
use solana_kit_core::{
    Address,
    accounts::{Account, MaybeAccount},
    codecs::Limits,
};

/// RPC, metadata, owner or deterministic state decoding failure.
#[derive(Debug)]
pub enum FetchError {
    /// The RPC request or response failed.
    Rpc(solana_kit_client::Error),
    /// The data representation, allocation or initialization was invalid.
    InvalidAccount,
    /// The deterministic account codec or owner check failed.
    Decode(solana_kit_core::codecs::Error),
    /// A required account was absent, retaining its address.
    Missing(Address),
}

/// Fetches one optional account with context, exact metadata and owner validation.
pub async fn fetch_maybe_account<T: Transport>(
    client: &Client<T>,
    address: Address,
    program: Option<Address>,
    read: ReadConfig,
    limits: Limits,
) -> Result<Contextual<MaybeAccount<Token2022Account>>, FetchError> {
    let result = client
        .call(AccountInfoWithConfig(address, Some(config(read))))
        .await
        .map_err(FetchError::Rpc)?;
    Ok(Contextual {
        context: result.context,
        value: decode(result.value, program.unwrap_or(PROGRAM), limits)?,
    })
}

/// Fetches a required initialized account, preserving a missing requested identity.
pub async fn fetch_account<T: Transport>(
    client: &Client<T>,
    address: Address,
    program: Option<Address>,
    read: ReadConfig,
    limits: Limits,
) -> Result<Contextual<Account<Token2022Account>>, FetchError> {
    let result = fetch_maybe_account(client, address, program, read, limits).await?;

    let value = match result.value {
        MaybeAccount::Present(account) => account,
        MaybeAccount::Missing(address) => return Err(FetchError::Missing(address)),
    };

    Ok(Contextual {
        context: result.context,
        value,
    })
}

/// Fetches an ordered batch, retaining duplicate requests and missing positions.
pub async fn fetch_maybe_accounts<T: Transport>(
    client: &Client<T>,
    addresses: Vec<Address>,
    program: Option<Address>,
    read: ReadConfig,
    limits: Limits,
) -> Result<Contextual<Vec<MaybeAccount<Token2022Account>>>, FetchError> {
    let result = client
        .call(MultipleAccounts(addresses, Some(config(read))))
        .await
        .map_err(FetchError::Rpc)?;
    let value = result
        .value
        .into_iter()
        .map(|value| decode(value, program.unwrap_or(PROGRAM), limits))
        .collect::<Result<_, _>>()?;
    Ok(Contextual {
        context: result.context,
        value,
    })
}

fn config(read: ReadConfig) -> AccountConfig {
    AccountConfig {
        read,
        encoding: Some(AccountEncoding::Base64),
        data_slice: None,
    }
}

fn decode(
    value: MaybeAccount<AccountData>,
    program: Address,
    limits: Limits,
) -> Result<MaybeAccount<Token2022Account>, FetchError> {
    let account = match value {
        MaybeAccount::Missing(address) => return Ok(MaybeAccount::Missing(address)),
        MaybeAccount::Present(account) => account,
    };

    let AccountData::Bytes { bytes, slice: None } = account.data else {
        return Err(FetchError::InvalidAccount);
    };

    if u64::try_from(bytes.len()).ok() != Some(account.space) || account.executable {
        return Err(FetchError::InvalidAccount);
    }

    let data = decode_account_with_program(program, account.owner, &bytes, limits)
        .map_err(FetchError::Decode)?;
    Ok(MaybeAccount::Present(Account {
        address: account.address,
        owner: account.owner,
        lamports: account.lamports,
        executable: account.executable,
        space: account.space,
        rent_epoch: account.rent_epoch,
        data,
    }))
}
