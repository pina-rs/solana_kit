//! Pinned instruction-role fixtures and exact positional account behavior.

use alloc::vec;
use solana_kit_core::{AccountMeta, AccountRole, Address, Instruction};

use crate::require;

const ROLES: [AccountRole; 4] = [
    AccountRole::Readonly,
    AccountRole::Writable,
    AccountRole::ReadonlySigner,
    AccountRole::WritableSigner,
];
const MERGED: [[AccountRole; 4]; 4] = [
    [ROLES[0], ROLES[1], ROLES[2], ROLES[3]],
    [ROLES[1], ROLES[1], ROLES[3], ROLES[3]],
    [ROLES[2], ROLES[3], ROLES[2], ROLES[3]],
    [ROLES[3], ROLES[3], ROLES[3], ROLES[3]],
];

/// Executes all 16 explicit merge results from the reference roles tests.
pub(crate) fn role_matrix() -> Result<(), &'static str> {
    for (left, expected) in ROLES.iter().zip(MERGED) {
        check_row(*left, expected)?;
    }

    Ok(())
}

/// Checks one row without computing the expected union with the tested algorithm.
fn check_row(left: AccountRole, expected: [AccountRole; 4]) -> Result<(), &'static str> {
    for (right, merged) in ROLES.iter().zip(expected) {
        require(left.merge(*right) == merged, "account role merge mismatch")?;
    }

    Ok(())
}

/// A concrete fixture includes duplicate account positions with distinct roles.
fn instruction() -> Instruction {
    let first = Address::from_bytes([1; 32]);
    let second = Address::from_bytes([2; 32]);

    Instruction {
        program_address: Address::from_bytes([0; 32]),
        accounts: vec![
            AccountMeta {
                address: second,
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: first,
                role: AccountRole::WritableSigner,
            },
            AccountMeta {
                address: second,
                role: AccountRole::ReadonlySigner,
            },
        ],
        data: vec![1, 2, 3, 4],
    }
}

/// Construction preserves account positions and duplicates without merging them.
pub(crate) fn ordered_accounts() -> Result<(), &'static str> {
    let actual = instruction();
    let expected = [
        Address::from_bytes([2; 32]),
        Address::from_bytes([1; 32]),
        Address::from_bytes([2; 32]),
    ];
    let addresses: alloc::vec::Vec<_> = actual.accounts.iter().map(|meta| meta.address).collect();

    require(
        addresses == expected,
        "instruction reordered or deduplicated accounts",
    )?;
    require(
        actual.accounts[0].role == AccountRole::Readonly,
        "instruction changed first duplicate privileges",
    )?;
    require(
        actual.accounts[2].role == AccountRole::ReadonlySigner,
        "instruction changed last duplicate privileges",
    )
}

/// Pinned instruction data [1, 2, 3, 4] survives owned construction and cloning.
pub(crate) fn preserved_data() -> Result<(), &'static str> {
    let original = instruction();
    let mut independent = original.clone();
    independent.data[0] = 99;

    require(
        original.data == [1, 2, 3, 4],
        "instruction data mismatch or aliased clone",
    )?;
    require(
        original.program_address == Address::from_bytes([0; 32]),
        "instruction program address changed",
    )
}
