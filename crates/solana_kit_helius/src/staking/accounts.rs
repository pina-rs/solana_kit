//! Stake discovery decodes owned account bytes and checks the requested withdrawal authority.

use super::HELIUS_VALIDATOR;
use solana_kit_client::{
    Client, Transport,
    methods::{
        Contextual, ReadConfig,
        accounts::{
            AccountConfig, AccountData, AccountEncoding, Filter, FilterEncoding, Memcmp,
            ProgramAccounts, ProgramAccountsConfig, ProgramAccountsResponse,
        },
    },
};
use solana_kit_core::{
    Address,
    accounts::{Account, AccountError, EncodedAccount},
    codecs::Limits,
};
use solana_kit_stake::{StakeStateAccount, StakeStateV2};

const WITHDRAWER_OFFSET: u64 = 44;

/// Caller-owned evaluation constraints and result/codec budgets for one discovery scan.
#[derive(Debug, Clone, Copy)]
pub struct StakeAccountQuery {
    /// Commitment and minimum evaluation slot sent to the node.
    pub read: ReadConfig,
    /// Per-account decoding limits; the shared RPC client bounds the complete response.
    pub decode_limits: Limits,
    /// Maximum returned account count before filtering by validator.
    pub max_accounts: usize,
}

/// An RPC, account-decoding or requested-snapshot validation failure.
#[derive(Debug)]
pub enum StakeAccountError {
    /// Original shared-client failure.
    Rpc(solana_kit_client::Error),
    /// Original program owner, allocation or codec failure.
    Decode(AccountError),
    /// The node returned more accounts than the caller permitted.
    AccountLimit,
    /// The node supplied an unexpected response or account representation.
    Representation,
    /// A returned executable account cannot be used as stake data.
    Executable(Address),
    /// Returned account metadata disagrees with the requested withdrawal authority.
    AuthorityMismatch(Address),
    /// The response context precedes the requested minimum slot.
    StaleContext,
}

impl core::fmt::Display for StakeAccountError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "stake-account discovery failed: {self:?}")
    }
}
impl std::error::Error for StakeAccountError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Rpc(error) => Some(error),
            Self::Decode(error) => Some(error),
            _ => None,
        }
    }
}

/// Reads one contextual scan for the withdrawal authority and retains Helius delegations.
///
/// The source filter is byte offset 44, the withdrawal-authority field.
/// Other validator delegations and valid undelegated states are excluded. Returned
/// accounts retain their RPC metadata; decoding does not authenticate node state.
pub async fn get_helius_stake_accounts<T: Transport>(
    client: &Client<T>,
    withdrawer: Address,
    query: StakeAccountQuery,
) -> Result<Contextual<Vec<Account<StakeStateAccount>>>, StakeAccountError> {
    let response = client
        .call(discovery_request(withdrawer, query.read)?)
        .await
        .map_err(StakeAccountError::Rpc)?;
    let ProgramAccountsResponse::Contextual(response) = response else {
        return Err(StakeAccountError::Representation);
    };

    if query
        .read
        .min_context_slot
        .is_some_and(|minimum| response.context.slot < minimum)
    {
        return Err(StakeAccountError::StaleContext);
    }

    if response.value.len() > query.max_accounts {
        return Err(StakeAccountError::AccountLimit);
    }

    let mut accounts = Vec::new();

    for account in response.value {
        let account = decode(account, query.decode_limits)?;

        if matches_delegation(&account, withdrawer)? {
            accounts.push(account);
        }
    }

    Ok(Contextual {
        context: response.context,
        value: accounts,
    })
}

fn discovery_request(
    withdrawer: Address,
    read: ReadConfig,
) -> Result<ProgramAccounts, StakeAccountError> {
    let filter = Memcmp::new(
        WITHDRAWER_OFFSET,
        withdrawer.as_bytes().to_vec(),
        FilterEncoding::Base58,
    )
    .map_err(StakeAccountError::Rpc)?;
    Ok(ProgramAccounts(
        solana_kit_stake::PROGRAM,
        Some(ProgramAccountsConfig {
            account: AccountConfig {
                read,
                encoding: Some(AccountEncoding::Base64),
                data_slice: None,
            },
            filters: Some(vec![Filter::Memcmp(filter)]),
            with_context: Some(true),
        }),
    ))
}

fn decode(
    account: Account<AccountData>,
    limits: Limits,
) -> Result<Account<StakeStateAccount>, StakeAccountError> {
    let Account {
        address,
        owner,
        lamports,
        executable,
        space,
        rent_epoch,
        data,
    } = account;

    if executable {
        return Err(StakeAccountError::Executable(address));
    }

    let AccountData::Bytes { bytes, slice: None } = data else {
        return Err(StakeAccountError::Representation);
    };
    let encoded = EncodedAccount {
        address,
        owner,
        lamports,
        executable,
        space,
        rent_epoch,
        data: bytes,
    };
    solana_kit_stake::decode_account(encoded, limits).map_err(StakeAccountError::Decode)
}

fn matches_delegation(
    account: &Account<StakeStateAccount>,
    withdrawer: Address,
) -> Result<bool, StakeAccountError> {
    let (meta, voter) = match &account.data.state {
        StakeStateV2::Stake(value) => (&value.item0, Some(value.item1.delegation.voter_pubkey)),
        StakeStateV2::Initialized(meta) => (meta, None),
        _ => return Ok(false),
    };

    if meta.authorized.withdrawer != withdrawer {
        return Err(StakeAccountError::AuthorityMismatch(account.address));
    }

    Ok(voter == Some(HELIUS_VALIDATOR))
}
