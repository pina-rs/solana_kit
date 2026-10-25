//! Source-defined account creation and delegation plans.
use crate::{
    Authorized, DelegateStake, DelegateStakeAccounts, Epoch, Initialize, InitializeAccounts,
    Lockup, PROGRAM, UnixTimestamp,
};
use solana_kit_core::{
    Address,
    codecs::{Error, Limits},
    plans::{InstructionPlan, Plan},
    sysvars::{CLOCK_ADDRESS, RENT_ADDRESS, STAKE_HISTORY_ADDRESS},
};

/// Canonical allocation for a Stake state account.
pub const STAKE_STATE_ACCOUNT_SIZE: u64 = 200;
/// Canonical Stake configuration account.
pub const STAKE_CONFIG: Address = Address::from_bytes([
    6, 161, 216, 23, 165, 2, 5, 11, 104, 7, 145, 230, 206, 109, 184, 142, 30, 91, 113, 80, 246, 31,
    198, 121, 10, 78, 180, 209, 0, 0, 0, 0,
]);

/// Creates and initializes an account as one indivisible instruction plan.
///
/// Lamports is an exact unsigned wire amount; Rust rejects negative values and
/// values above u64 before construction. An absent lockup uses the source's
/// zero timestamp/epoch and System Program custodian.
/// # Errors
/// Returns the System builder or Stake codec's original validation failure.
pub fn create_stake_account_plan(
    payer: Address,
    stake: Address,
    lamports: u64,
    authorized: Authorized,
    lockup: Option<Lockup>,
    limits: Limits,
) -> Result<InstructionPlan, solana_kit_system::ClientError> {
    let create = solana_kit_system::CreateAccount {
        lamports,
        space: STAKE_STATE_ACCOUNT_SIZE,
        program_address: PROGRAM,
    }
    .instruction(
        solana_kit_system::CreateAccountAccounts {
            payer,
            new_account: stake,
        },
        limits,
    )?;
    let initialize = InitializeAccounts {
        stake,
        rent_sysvar: RENT_ADDRESS,
    }
    .instruction(
        PROGRAM,
        &Initialize {
            arg0: authorized,
            arg1: lockup.unwrap_or(Lockup {
                unix_timestamp: UnixTimestamp { value: 0 },
                epoch: Epoch { value: 0 },
                custodian: solana_kit_system::SYSTEM_PROGRAM,
            }),
        },
        &[],
        limits,
    )?;

    Ok(Plan::indivisible(alloc::vec![
        create.into(),
        initialize.into()
    ]))
}

/// Builds delegation with the source's canonical sysvar/config defaults.
///
/// Caller-selected sysvars remain available through [`DelegateStakeAccounts`].
/// # Errors
/// Rejects caller byte/account limits or allocation failure.
pub fn delegate_stake_plan(
    stake: Address,
    vote: Address,
    stake_authority: Address,
    limits: Limits,
) -> Result<InstructionPlan, Error> {
    let instruction = DelegateStakeAccounts {
        stake,
        vote,
        clock_sysvar: CLOCK_ADDRESS,
        stake_history: STAKE_HISTORY_ADDRESS,
        unused: STAKE_CONFIG,
        stake_authority,
    }
    .instruction(PROGRAM, &DelegateStake {}, &[], limits)?;
    Ok(instruction.into())
}
