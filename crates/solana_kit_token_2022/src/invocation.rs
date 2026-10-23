//! Shared positional account validation for generated instruction wrappers.
use crate::Token2022InstructionData;
use alloc::vec::Vec;
use solana_kit_core::{
    AccountMeta, AccountRole, Address,
    codecs::{Error, Limits},
};

/// An explicit signing or multisignature authority; no ambient identity is used.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Authority {
    /// Authority address, including an on-chain multisig account when applicable.
    pub address: Address,
    /// Whether this account itself signs the instruction.
    pub is_signer: bool,
}

/// Decoded instruction data with every original positional account meta retained.
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedInstruction {
    /// Exact source instruction variant and full-width values.
    pub data: Token2022InstructionData,
    /// Ordered accounts including duplicate addresses and remaining segments.
    pub accounts: Vec<AccountMeta>,
}

pub(crate) enum Signer {
    Never,
    Required,
    Either,
}
pub(crate) struct Role {
    pub writable: bool,
    pub signer: Signer,
    pub optional: bool,
}

pub(crate) const fn meta(address: Address, role: AccountRole) -> AccountMeta {
    AccountMeta { address, role }
}

impl Role {
    fn matches(&self, meta: &AccountMeta, program: Address, placeholders: bool) -> bool {
        if self.optional && placeholders && meta.address == program {
            return meta.role == AccountRole::Readonly;
        }

        meta.role.is_writable() == self.writable
            && match self.signer {
                Signer::Never => !meta.role.is_signer(),
                Signer::Required => meta.role.is_signer(),
                Signer::Either => true,
            }
    }
}

pub(crate) fn validate_accounts(
    program: Address,
    accounts: &[AccountMeta],
    fixed: &[Role],
    remaining: &[AccountRole],
    tail_count: Option<usize>,
    placeholders: bool,
    limits: Limits,
) -> Result<(), Error> {
    if accounts.len() > limits.max_items {
        return Err(Error::LimitExceeded);
    }

    let mut positions = new_positions(accounts.len().checked_add(1).ok_or(Error::Overflow)?)?;
    positions[0] = true;

    // Optional omitted roles can share privileges. Preserve all possible positions;
    // assigning semantic names to indistinguishable readonly metas would invent facts.
    for role in fixed {
        positions = advance(role, &positions, accounts, program, placeholders)?;
    }

    if positions.iter().enumerate().any(|(start, possible)| {
        *possible && remaining_matches(&accounts[start..], remaining, tail_count)
    }) {
        return Ok(());
    }

    Err(Error::InvalidValue)
}

/// Each step retains every viable optional-role alignment without naming one.
fn advance(
    role: &Role,
    positions: &[bool],
    accounts: &[AccountMeta],
    program: Address,
    placeholders: bool,
) -> Result<Vec<bool>, Error> {
    let mut next = new_positions(positions.len())?;

    for (position, possible) in positions.iter().copied().enumerate() {
        if !possible {
            continue;
        }

        if role.optional && !placeholders {
            next[position] = true;
        }

        if accounts
            .get(position)
            .is_some_and(|meta| role.matches(meta, program, placeholders))
        {
            next[position + 1] = true;
        }
    }

    Ok(next)
}

fn new_positions(length: usize) -> Result<Vec<bool>, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| Error::AllocationFailed)?;
    values.resize(length, false);
    Ok(values)
}

fn remaining_matches(accounts: &[AccountMeta], roles: &[AccountRole], tail: Option<usize>) -> bool {
    match roles {
        [] => accounts.is_empty(),
        [role] => accounts.iter().all(|meta| meta.role == *role),
        [first, second] => tail
            .and_then(|count| accounts.len().checked_sub(count))
            .is_some_and(|split| {
                accounts[..split].iter().all(|meta| meta.role == *first)
                    && accounts[split..].iter().all(|meta| meta.role == *second)
            }),
        _ => false,
    }
}

pub(crate) fn append_optional(
    accounts: &mut Vec<AccountMeta>,
    address: Option<Address>,
    program: Address,
    role: AccountRole,
    omitted: bool,
) {
    if let Some(address) = address {
        accounts.push(AccountMeta { address, role });
    } else if !omitted {
        accounts.push(AccountMeta {
            address: program,
            role: AccountRole::Readonly,
        });
    }
}
