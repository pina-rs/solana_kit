//! Literal amount sets preserve the program's partial/full withdrawal distinction.

use solana_kit_core::{
    Address,
    accounts::Account,
    codecs::Error,
    sysvars::{Clock, StakeHistoryData, StakeHistoryEntry},
};
use solana_kit_stake::{
    Authorized, Delegation, Epoch, Lockup, Meta, Stake, StakeFlags, StakeStateAccount,
    StakeStateV2, StakeStateV2StakePayload, UnixTimestamp, WithdrawalContext, WithdrawalError,
    WithdrawalLimits, WithdrawalSigners, assess_withdrawal,
};

const BALANCE: u64 = 100;
const RENT: u64 = 10;
const UNLOCK_EPOCH: u64 = 5;
const UNLOCK_TIME: i64 = 50;
const ACTIVE_PRINCIPAL: u64 = 80;
const TOPPED_UP_BALANCE: u64 = 150;
const WITHDRAWER: Address = Address::from_bytes([1; 32]);
const CUSTODIAN: Address = Address::from_bytes([2; 32]);
const ACCOUNT: Address = Address::from_bytes([3; 32]);

/// Full closure is valid even though requests between the partial cap and balance fail.
#[cfg_attr(test, test)]
pub fn inactive_closure_does_not_make_the_rent_gap_withdrawable() {
    let value = account(StakeStateV2::Initialized(meta()));
    let limits = assess(&value, &clock(UNLOCK_EPOCH, UNLOCK_TIME), &[], None).unwrap();
    assert_eq!(limits.max_partial_lamports, Some(BALANCE - RENT));
    assert_eq!(limits.close_lamports, Some(BALANCE));

    for (amount, permitted) in [
        (0, true),
        (BALANCE - RENT, true),
        (BALANCE - RENT + 1, false),
        (BALANCE - 1, false),
        (BALANCE, true),
        (BALANCE + 1, false),
    ] {
        assert_eq!(limits.permits(amount), permitted, "request={amount}");
    }

    let mut underfunded = value;
    underfunded.lamports = RENT - 1;
    let limits = assess(&underfunded, &clock(UNLOCK_EPOCH, UNLOCK_TIME), &[], None).unwrap();
    assert_eq!(limits.max_partial_lamports, None);
    assert_eq!(limits.close_lamports, Some(RENT - 1));
    assert!(!limits.permits(0));
}

/// An uninitialized account requires its own signer and has no rent reservation.
#[cfg_attr(test, test)]
pub fn uninitialized_and_empty_accounts_require_their_account_signer() {
    let mut value = account(StakeStateV2::Uninitialized);
    let observation = clock(0, 0);
    let context = context(&observation, &[]);
    let signers = WithdrawalSigners {
        withdrawer: ACCOUNT,
        custodian: None,
    };

    assert_eq!(
        assess_withdrawal(&value, signers, context).unwrap(),
        WithdrawalLimits {
            max_partial_lamports: Some(BALANCE - 1),
            close_lamports: Some(BALANCE)
        }
    );
    assert_eq!(
        assess(&value, &observation, &[], None),
        Err(WithdrawalError::AuthorityMismatch)
    );
    value.space = 0;
    value.lamports = 0;

    let limits = assess_withdrawal(&value, signers, context).unwrap();
    assert_eq!(limits.max_partial_lamports, None);
    assert_eq!(limits.close_lamports, Some(0));
    assert!(limits.permits(0));
}

/// Warmup cannot expose delegated principal as an available withdrawal.
#[cfg_attr(test, test)]
pub fn warming_stake_reserves_the_entire_delegation() {
    let value = delegated(ACTIVE_PRINCIPAL, 4, u64::MAX);
    let limits = assess(&value, &clock(4, UNLOCK_TIME), &[], None).unwrap();
    assert_eq!(
        limits.max_partial_lamports,
        Some(BALANCE - ACTIVE_PRINCIPAL - RENT)
    );
    assert_eq!(limits.close_lamports, None);
    assert!(limits.permits(10));
    assert!(!limits.permits(11));
    assert!(!limits.permits(BALANCE));
}

/// Cooldown reserves only effective stake, using the supplied integer rate epoch.
#[cfg_attr(test, test)]
pub fn cooldown_history_and_rate_select_the_actual_reserved_principal() {
    let mut value = delegated(BALANCE, 0, 1);
    value.lamports = TOPPED_UP_BALANCE;
    let history = [history(1), history(0)];
    let old = assess(&value, &clock(2, UNLOCK_TIME), &history, None).unwrap();
    assert_eq!(old.max_partial_lamports, Some(115));
    assert_eq!(old.close_lamports, None);

    let current = assess(&value, &clock(2, UNLOCK_TIME), &history, Some(0)).unwrap();
    assert_eq!(current.max_partial_lamports, Some(125));
    assert_eq!(current.close_lamports, None);
    let expired = assess(&value, &clock(2, UNLOCK_TIME), &[], Some(0)).unwrap();
    assert_eq!(expired.max_partial_lamports, Some(140));
    assert_eq!(expired.close_lamports, Some(TOPPED_UP_BALANCE));
}

/// Neither the epoch nor signed timestamp boundary alone releases a lockup.
#[cfg_attr(test, test)]
pub fn lockup_requires_both_boundaries_or_the_matching_custodian() {
    let mut metadata = meta();
    metadata.lockup.epoch.value = UNLOCK_EPOCH;
    metadata.lockup.unix_timestamp.value = UNLOCK_TIME;
    let value = account(StakeStateV2::Initialized(metadata));

    for observation in [
        clock(UNLOCK_EPOCH - 1, UNLOCK_TIME),
        clock(UNLOCK_EPOCH, UNLOCK_TIME - 1),
        clock(0, -1),
    ] {
        assert_eq!(
            assess(&value, &observation, &[], None),
            Err(WithdrawalError::LockupInForce)
        );
        let context = context(&observation, &[]);

        for (custodian, allowed) in [
            (Some(CUSTODIAN), true),
            (Some(ACCOUNT), false),
            (None, false),
        ] {
            let result = assess_withdrawal(
                &value,
                WithdrawalSigners {
                    withdrawer: WITHDRAWER,
                    custodian,
                },
                context,
            );
            assert_eq!(result.is_ok(), allowed);
        }
    }

    assert!(assess(&value, &clock(UNLOCK_EPOCH, UNLOCK_TIME), &[], None).is_ok());
    let signers = WithdrawalSigners {
        withdrawer: ACCOUNT,
        custodian: Some(CUSTODIAN),
    };
    assert_eq!(
        assess_withdrawal(&value, signers, context(&clock(0, 0), &[])),
        Err(WithdrawalError::AuthorityMismatch)
    );
}

/// Bad metadata, ambiguous history and reserve overflow remain errors.
#[cfg_attr(test, test)]
pub fn invalid_observations_never_become_successful_zero_amounts() {
    let good = account(StakeStateV2::Initialized(meta()));
    let mut foreign = good.clone();
    foreign.owner = Address::from_bytes([9; 32]);
    let mut executable = good.clone();
    executable.executable = true;
    let mut short = good;
    short.space = 1;

    for invalid in [
        foreign,
        executable,
        short,
        account(StakeStateV2::RewardsPool),
    ] {
        assert_eq!(
            assess(&invalid, &clock(UNLOCK_EPOCH, UNLOCK_TIME), &[], None),
            Err(WithdrawalError::InvalidAccount)
        );
    }

    let active = delegated(ACTIVE_PRINCIPAL, 0, u64::MAX);
    assert_eq!(
        assess(
            &active,
            &clock(UNLOCK_EPOCH, UNLOCK_TIME),
            &[history(0), history(0)],
            None
        ),
        Err(WithdrawalError::History(Error::InvalidValue))
    );
    let huge = delegated(u64::MAX, u64::MAX, u64::MAX);
    assert_eq!(
        assess(&huge, &clock(UNLOCK_EPOCH, UNLOCK_TIME), &[], None),
        Err(WithdrawalError::ReserveOverflow)
    );
}

/// The context's selected reserve supersedes stale metadata in current program rules.
#[cfg_attr(test, test)]
pub fn selected_rent_reserve_does_not_read_the_stored_metadata_reserve() {
    let mut metadata = meta();
    metadata.rent_exempt_reserve = u64::MAX;
    let value = account(StakeStateV2::Initialized(metadata));
    assert_eq!(
        assess(&value, &clock(UNLOCK_EPOCH, UNLOCK_TIME), &[], None)
            .unwrap()
            .max_partial_lamports,
        Some(BALANCE - RENT)
    );
}

/// A small coherent clock keeps signed-time boundaries explicit.
fn clock(epoch: u64, unix_timestamp: i64) -> Clock {
    Clock {
        slot: 100,
        epoch_start_timestamp: 0,
        epoch,
        leader_schedule_epoch: epoch,
        unix_timestamp,
    }
}

/// The caller supplies deployed-program rules rather than relying on metadata defaults.
fn context<'a>(clock: &'a Clock, history: &'a [StakeHistoryEntry]) -> WithdrawalContext<'a> {
    WithdrawalContext {
        clock,
        history,
        rent_exempt_reserve: RENT,
        new_rate_activation_epoch: None,
    }
}

/// Most cases use the ordinary withdrawal authority and no custodian.
fn assess(
    value: &Account<StakeStateAccount>,
    clock: &Clock,
    history: &[StakeHistoryEntry],
    rate: Option<u64>,
) -> Result<WithdrawalLimits, WithdrawalError> {
    assess_withdrawal(
        value,
        WithdrawalSigners {
            withdrawer: WITHDRAWER,
            custodian: None,
        },
        WithdrawalContext {
            new_rate_activation_epoch: rate,
            ..context(clock, history)
        },
    )
}

/// Explicit lockup identities distinguish withdrawal and custodian authority.
fn meta() -> Meta {
    Meta {
        rent_exempt_reserve: RENT,
        authorized: Authorized {
            staker: ACCOUNT,
            withdrawer: WITHDRAWER,
        },
        lockup: Lockup {
            unix_timestamp: UnixTimestamp { value: 0 },
            epoch: Epoch { value: 0 },
            custodian: CUSTODIAN,
        },
    }
}

/// Metadata includes the actual reserved allocation and retains the observed balance.
fn account(state: StakeStateV2) -> Account<StakeStateAccount> {
    Account {
        address: ACCOUNT,
        owner: solana_kit_stake::PROGRAM,
        lamports: BALANCE,
        executable: false,
        space: 200,
        rent_epoch: None,
        data: StakeStateAccount { state },
    }
}

/// Literal delegation values avoid relying on another withdrawal implementation.
fn delegated(stake: u64, activation: u64, deactivation: u64) -> Account<StakeStateAccount> {
    account(StakeStateV2::Stake(StakeStateV2StakePayload {
        item0: meta(),
        item1: Stake {
            delegation: Delegation {
                voter_pubkey: ACCOUNT,
                stake,
                activation_epoch: Epoch { value: activation },
                deactivation_epoch: Epoch {
                    value: deactivation,
                },
                reserved: vec![0; 8],
            },
            credits_observed: 0,
        },
        item2: StakeFlags { bits: 0 },
    }))
}

/// A constant cluster gives independently calculable quarter and nine-percent steps.
fn history(epoch: u64) -> StakeHistoryEntry {
    StakeHistoryEntry {
        epoch,
        stake_history: StakeHistoryData {
            effective: 1_000,
            activating: 500,
            deactivating: 500,
        },
    }
}
