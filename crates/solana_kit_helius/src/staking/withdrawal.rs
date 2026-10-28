//! Bundled standard RPC observations replace an unverified provider REST route.

use super::{StakeAccountError, accounts::full_bytes};
use solana_kit_client::{
    Client, Transport,
    methods::{
        Contextual, ReadConfig,
        accounts::{AccountConfig, AccountData, AccountEncoding, MultipleAccounts},
    },
};
use solana_kit_core::{
    Address,
    accounts::{Account, MaybeAccount},
    codecs::{Decoder, Limits},
    sysvars::{self, Clock, Rent, StakeHistory, SysvarCodec},
};
use solana_kit_stake::{
    StakeStateAccount, StakeStateV2, WithdrawalContext, WithdrawalLimits, WithdrawalSigners,
    assess_withdrawal,
};

const MAX_HISTORY_ENTRIES: u64 = 512;
const MAX_ACCOUNT_BYTES: u64 = 10 * 1024 * 1024;
const STORAGE_OVERHEAD: u64 = 128;

/// Explicit cluster read constraints, decode budgets and deployed-program rate epoch.
#[derive(Clone, Copy, Debug)]
pub struct StakeWithdrawalQuery {
    /// Commitment and lower bound for the single bank evaluation.
    pub read: ReadConfig,
    /// Per-account decoding limits, in addition to the shared client's response budget.
    pub decode_limits: Limits,
    /// First reduced-rate epoch; the pinned current integer Stake program uses `Some(0)`.
    pub new_rate_activation_epoch: Option<u64>,
}

/// Reads the account, Clock, Rent and Stake History together and assesses amount ranges.
///
/// This follows the pinned current Stake program's live-rent rules. Rent thresholds
/// 1 and 2 use exact integer arithmetic, including balances above JavaScript's safe
/// integer range. Other thresholds are rejected rather than approximated. The rate
/// epoch must match the deployed program; this function does not identify its version.
///
/// A complete history and matching Clock/context slot are required. These checks detect
/// inconsistent responses; they do not authenticate a node. Proposed signers must still
/// provide actual transaction signatures. Destination validity, submission-time changes,
/// transport cancellation and execution remain caller-owned.
pub async fn get_withdrawable_amount<T: Transport>(
    client: &Client<T>,
    stake_account: Address,
    signers: WithdrawalSigners,
    query: StakeWithdrawalQuery,
) -> Result<Contextual<WithdrawalLimits>, StakeAccountError> {
    let response = client
        .call(MultipleAccounts(
            vec![
                stake_account,
                sysvars::CLOCK_ADDRESS,
                sysvars::RENT_ADDRESS,
                sysvars::STAKE_HISTORY_ADDRESS,
            ],
            Some(AccountConfig {
                read: query.read,
                encoding: Some(AccountEncoding::Base64),
                data_slice: None,
            }),
        ))
        .await
        .map_err(StakeAccountError::Rpc)?;

    if query
        .read
        .min_context_slot
        .is_some_and(|minimum| response.context.slot < minimum)
    {
        return Err(StakeAccountError::StaleContext);
    }

    let accounts = response
        .value
        .try_into()
        .map_err(|_| StakeAccountError::Representation)?;
    let value = evaluate_snapshot(accounts, response.context.slot, signers, query)?;

    Ok(Contextual {
        context: response.context,
        value,
    })
}

/// Validates and interprets the single bank observation before applying program rules.
fn evaluate_snapshot(
    accounts: [MaybeAccount<AccountData>; 4],
    slot: u64,
    signers: WithdrawalSigners,
    query: StakeWithdrawalQuery,
) -> Result<WithdrawalLimits, StakeAccountError> {
    let [stake, clock, rent, history] = accounts;
    let clock: Clock = decode_sysvar(clock, query.decode_limits)?;
    let rent: Rent = decode_sysvar(rent, query.decode_limits)?;
    let history: StakeHistory = decode_sysvar(history, query.decode_limits)?;
    validate_snapshot(&clock, slot, &history)?;
    let account = decode_stake(stake, query.decode_limits)?;
    let reserve = rent_reserve(&rent, account.space)?;
    assess_withdrawal(
        &account,
        signers,
        WithdrawalContext {
            clock: &clock,
            history: &history,
            rent_exempt_reserve: reserve,
            new_rate_activation_epoch: query.new_rate_activation_epoch,
        },
    )
    .map_err(StakeAccountError::Withdrawal)
}

/// All sysvars require the expected owner, complete allocation and exact wire layout.
fn decode_sysvar<T>(
    value: MaybeAccount<AccountData>,
    limits: Limits,
) -> Result<T, StakeAccountError>
where
    SysvarCodec<T>: Decoder<T>,
{
    let account = value.require().map_err(StakeAccountError::Decode)?;
    let account = full_bytes(account)?;
    account
        .decode(&SysvarCodec::<T>::NEW, sysvars::OWNER_ADDRESS, limits)
        .map(|account| account.data)
        .map_err(StakeAccountError::Decode)
}

/// The current program also treats a present zero-data Stake account as uninitialized.
fn decode_stake(
    value: MaybeAccount<AccountData>,
    limits: Limits,
) -> Result<Account<StakeStateAccount>, StakeAccountError> {
    let account = value.require().map_err(StakeAccountError::Decode)?;
    let account = full_bytes(account)?;

    if account.space != 0 || !account.data.is_empty() {
        return solana_kit_stake::decode_account(account, limits)
            .map_err(StakeAccountError::Decode);
    }

    Ok(Account {
        address: account.address,
        owner: account.owner,
        lamports: account.lamports,
        executable: account.executable,
        space: account.space,
        rent_epoch: account.rent_epoch,
        data: StakeStateAccount {
            state: StakeStateV2::Uninitialized,
        },
    })
}

/// A complete sysvar contains the immediately preceding epochs, up to its fixed capacity.
fn validate_snapshot(
    clock: &Clock,
    slot: u64,
    history: &StakeHistory,
) -> Result<(), StakeAccountError> {
    let count = clock.epoch.min(MAX_HISTORY_ENTRIES) as usize;

    if clock.slot != slot || history.len() != count {
        return Err(StakeAccountError::IncoherentSnapshot);
    }

    if history
        .iter()
        .enumerate()
        .any(|(index, entry)| entry.epoch != clock.epoch - 1 - index as u64)
    {
        return Err(StakeAccountError::IncoherentSnapshot);
    }

    Ok(())
}

/// Both supported wire thresholds use the deployed program's checked integer rent rules.
fn rent_reserve(rent: &Rent, space: u64) -> Result<u64, StakeAccountError> {
    let factor = match rent.exemption_threshold {
        1.0 => 1,
        2.0 => 2,
        _ => return Err(StakeAccountError::InvalidRent),
    };
    let maximum_rate = u64::MAX / (MAX_ACCOUNT_BYTES + STORAGE_OVERHEAD) / factor;

    if space > MAX_ACCOUNT_BYTES || rent.lamports_per_byte_year > maximum_rate {
        return Err(StakeAccountError::InvalidRent);
    }

    (space + STORAGE_OVERHEAD)
        .checked_mul(rent.lamports_per_byte_year)
        .and_then(|reserve| reserve.checked_mul(factor))
        .ok_or(StakeAccountError::InvalidRent)
}
