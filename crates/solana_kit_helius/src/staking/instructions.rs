//! Source-defined staking roles and sysvars use the maintained Kit program builders.

use super::{
    StakeInstructions, StakeRequest, StakingBuildError, UnstakeRequest, WithdrawalRequest,
};
use solana_kit_core::{Address, Instruction, codecs::Limits, sysvars};
use solana_kit_stake::{
    Authorized, Deactivate, DeactivateAccounts, DelegateStake, DelegateStakeAccounts, Epoch,
    Initialize, InitializeAccounts, Lockup, PROGRAM, STAKE_CONFIG, STAKE_STATE_ACCOUNT_SIZE,
    UnixTimestamp, Withdraw, WithdrawAccounts,
};

/// The provider's validator vote address, verified against its pinned SDK source.
pub const HELIUS_VALIDATOR: Address = Address::from_bytes([
    10, 105, 21, 29, 166, 140, 221, 153, 24, 26, 127, 219, 178, 195, 219, 82, 115, 203, 170, 252,
    228, 13, 38, 144, 40, 49, 8, 227, 182, 97, 21, 81,
]);

/// Builds create, initialize and Helius-delegation instructions using exact lamport arithmetic.
///
/// Limits apply to each reused instruction builder. The caller must supply the new
/// account's signature and combine all three instructions into one transaction.
pub fn build_stake_instructions(
    request: StakeRequest,
    rent_exempt_reserve: u64,
    limits: Limits,
) -> Result<StakeInstructions, StakingBuildError> {
    let lamports = request
        .lamports
        .checked_add(rent_exempt_reserve)
        .ok_or(StakingBuildError::LamportOverflow)?;
    let create = create_account(request.owner, request.stake_account, lamports, limits)?;
    let initialize = initialize_account(request.owner, request.stake_account, limits)?;
    let delegate = delegate_account(request.owner, request.stake_account, limits)?;

    Ok(StakeInstructions {
        stake_account: request.stake_account,
        instructions: [create, initialize, delegate],
    })
}

fn create_account(
    owner: Address,
    stake: Address,
    lamports: u64,
    limits: Limits,
) -> Result<Instruction, StakingBuildError> {
    let data = solana_kit_system::CreateAccount {
        lamports,
        space: STAKE_STATE_ACCOUNT_SIZE,
        program_address: PROGRAM,
    };
    let accounts = solana_kit_system::CreateAccountAccounts {
        payer: owner,
        new_account: stake,
    };
    Ok(data.instruction(accounts, limits)?)
}

fn initialize_account(
    owner: Address,
    stake: Address,
    limits: Limits,
) -> Result<Instruction, StakingBuildError> {
    let authorized = Authorized {
        staker: owner,
        withdrawer: owner,
    };
    let lockup = Lockup {
        unix_timestamp: UnixTimestamp { value: 0 },
        epoch: Epoch { value: 0 },
        custodian: owner,
    };
    let accounts = InitializeAccounts {
        stake,
        rent_sysvar: sysvars::RENT_ADDRESS,
    };
    Ok(accounts.instruction(
        PROGRAM,
        &Initialize {
            arg0: authorized,
            arg1: lockup,
        },
        &[],
        limits,
    )?)
}

fn delegate_account(
    owner: Address,
    stake: Address,
    limits: Limits,
) -> Result<Instruction, StakingBuildError> {
    let accounts = DelegateStakeAccounts {
        stake,
        vote: HELIUS_VALIDATOR,
        clock_sysvar: sysvars::CLOCK_ADDRESS,
        stake_history: sysvars::STAKE_HISTORY_ADDRESS,
        unused: STAKE_CONFIG,
        stake_authority: owner,
    };
    Ok(accounts.instruction(PROGRAM, &DelegateStake {}, &[], limits)?)
}

/// Builds a deactivation instruction; cooldown and subsequent withdrawal are separate operations.
pub fn get_unstake_instruction(
    request: UnstakeRequest,
    limits: Limits,
) -> Result<Instruction, StakingBuildError> {
    let accounts = DeactivateAccounts {
        stake: request.stake_account,
        clock_sysvar: sysvars::CLOCK_ADDRESS,
        stake_authority: request.owner,
    };
    Ok(accounts.instruction(PROGRAM, &Deactivate {}, &[], limits)?)
}

/// Builds an exact withdrawal request. The Stake program checks authority, lockup and available stake.
///
/// This convenience method supplies no lockup custodian. Use the underlying Kit
/// WithdrawAccounts builder when a separate custodian signature is required.
pub fn get_withdraw_instruction(
    request: WithdrawalRequest,
    limits: Limits,
) -> Result<Instruction, StakingBuildError> {
    let accounts = WithdrawAccounts {
        stake: request.stake_account,
        recipient: request.destination,
        clock_sysvar: sysvars::CLOCK_ADDRESS,
        stake_history: sysvars::STAKE_HISTORY_ADDRESS,
        withdraw_authority: request.authority,
        lockup_authority: None,
    };
    Ok(accounts.instruction(
        PROGRAM,
        &Withdraw {
            args: request.lamports,
        },
        &[],
        limits,
    )?)
}
