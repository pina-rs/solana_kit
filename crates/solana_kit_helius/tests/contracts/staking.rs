//! Canonical local staking construction and RPC/signing preparation replace unsupported REST routes.

use solana_kit_core::{AccountRole, Address, codecs::Limits, sysvars};
use solana_kit_helius::staking::*;

mod accounts;
mod runtime;

pub(super) const LIMITS: Limits = Limits::new(4096, 4096);

/// Public helper wires retain the source-defined roles, exact funding and signer custody.
pub fn preparation() {
    instruction_contracts();
    accounts::discovery();
    runtime::transactions();
    runtime::failures();
    runtime::cancellation();
}

fn instruction_contracts() {
    let owner = Address::from_bytes([1; 32]);
    let stake = Address::from_bytes([2; 32]);
    assert_eq!(
        HELIUS_VALIDATOR.to_string(),
        "he1iusunGwqrNtafDtLdhsUQDFvo13z9sUa36PauBtk"
    );
    let request = StakeRequest {
        owner,
        stake_account: stake,
        lamports: 9_007_199_254_740_993,
    };
    let result = build_stake_instructions(request, 2, LIMITS).expect("exact stake funding");
    assert_eq!(result.stake_account, stake);
    let [create, initialize, delegate] = &result.instructions;
    assert_eq!(create.program_address, solana_kit_system::SYSTEM_PROGRAM);
    create_contract(owner, stake, create);
    initialization_and_delegation(owner, stake, initialize, delegate);
    deactivation_and_withdrawal(owner, stake);
    invalid_funding(request);
}

fn create_contract(owner: Address, stake: Address, create: &solana_kit_core::Instruction) {
    let expected = [
        &[0, 0, 0, 0][..],
        &9_007_199_254_740_995_u64.to_le_bytes(),
        &200_u64.to_le_bytes(),
        solana_kit_stake::PROGRAM.as_bytes(),
    ]
    .concat();
    assert_eq!(create.data, expected);
    assert_eq!(
        create
            .accounts
            .iter()
            .map(|a| (a.address, a.role))
            .collect::<Vec<_>>(),
        [
            (owner, AccountRole::WritableSigner),
            (stake, AccountRole::WritableSigner),
        ]
    );
}

fn initialization_and_delegation(
    owner: Address,
    stake: Address,
    initialize: &solana_kit_core::Instruction,
    delegate: &solana_kit_core::Instruction,
) {
    let expected_initialize = [
        &[0, 0, 0, 0][..],
        owner.as_bytes(),
        owner.as_bytes(),
        &[0; 16],
        owner.as_bytes(),
    ]
    .concat();
    assert_eq!(initialize.data, expected_initialize);
    assert_eq!(
        initialize
            .accounts
            .iter()
            .map(|a| a.address)
            .collect::<Vec<_>>(),
        [stake, sysvars::RENT_ADDRESS]
    );
    assert_eq!(delegate.data, [2, 0, 0, 0]);
    assert_eq!(
        delegate
            .accounts
            .iter()
            .map(|a| (a.address, a.role))
            .collect::<Vec<_>>(),
        [
            (stake, AccountRole::Writable),
            (HELIUS_VALIDATOR, AccountRole::Readonly),
            (sysvars::CLOCK_ADDRESS, AccountRole::Readonly),
            (sysvars::STAKE_HISTORY_ADDRESS, AccountRole::Readonly),
            (solana_kit_stake::STAKE_CONFIG, AccountRole::Readonly),
            (owner, AccountRole::ReadonlySigner),
        ]
    );
}

fn deactivation_and_withdrawal(owner: Address, stake: Address) {
    let deactivate = get_unstake_instruction(
        UnstakeRequest {
            owner,
            stake_account: stake,
        },
        LIMITS,
    )
    .expect("deactivation instruction");
    assert_eq!(deactivate.program_address, solana_kit_stake::PROGRAM);
    assert_eq!(deactivate.data, [5, 0, 0, 0]);
    let destination = Address::from_bytes([3; 32]);
    let withdrawal = get_withdraw_instruction(
        WithdrawalRequest {
            authority: owner,
            stake_account: stake,
            destination,
            lamports: u64::MAX,
        },
        LIMITS,
    )
    .expect("exact withdrawal instruction");
    assert_eq!(
        withdrawal.data,
        [&[4, 0, 0, 0][..], &u64::MAX.to_le_bytes()].concat()
    );
    assert_eq!(withdrawal.accounts.len(), 5);
    assert_eq!(withdrawal.accounts[1].address, destination);
    assert_eq!(withdrawal.accounts[4].role, AccountRole::ReadonlySigner);
}

fn invalid_funding(request: StakeRequest) {
    assert!(matches!(
        build_stake_instructions(
            StakeRequest {
                lamports: u64::MAX,
                ..request
            },
            1,
            LIMITS
        ),
        Err(StakingBuildError::LamportOverflow)
    ));
    assert!(build_stake_instructions(request, 0, Limits::new(0, 0)).is_err());
}
