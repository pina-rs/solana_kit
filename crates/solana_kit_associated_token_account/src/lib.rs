#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod accounts;
mod program;

#[cfg(feature = "crypto")]
mod pda;

pub use accounts::*;
pub use program::*;

#[cfg(feature = "crypto")]
pub use pda::*;

use alloc::vec;
use solana_kit_core::{
    Instruction,
    codecs::{Error, Limits},
};

/// All three canonical Associated Token Account invocations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AssociatedTokenInstruction {
    /// Creates an account and fails if it exists.
    Create(CreateAccounts),
    /// Creates an account or succeeds when the matching account already exists.
    CreateIdempotent(CreateAccounts),
    /// Recovers tokens and lamports from a nested associated account.
    RecoverNested(RecoverNestedAccounts),
}

impl AssociatedTokenInstruction {
    /// Builds exact ordered metadata and the canonical single-byte discriminator.
    ///
    /// # Errors
    /// Rejects byte or account-count budgets that cannot contain this invocation.
    pub fn instruction(self, limits: Limits) -> Result<Instruction, Error> {
        let count = match self {
            Self::Create(_) | Self::CreateIdempotent(_) => 6,
            Self::RecoverNested(_) => 7,
        };

        if limits.max_bytes < 1 || count > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        let (tag, accounts) = match self {
            Self::Create(accounts) => (0, accounts.metadata().to_vec()),
            Self::CreateIdempotent(accounts) => (1, accounts.metadata().to_vec()),
            Self::RecoverNested(accounts) => (2, accounts.metadata().to_vec()),
        };

        Ok(Instruction {
            program_address: PROGRAM,
            accounts,
            data: vec![tag],
        })
    }
}

/// Fully parses a canonical instruction with exact roles and account counts.
///
/// # Errors
/// Rejects foreign programs, wrong counts or roles, unknown tags, truncated or
/// trailing data, and exceeded caller budgets. Account identities remain caller-owned.
pub fn parse_instruction(
    instruction: &Instruction,
    limits: Limits,
) -> Result<AssociatedTokenInstruction, Error> {
    if instruction.program_address != PROGRAM {
        return Err(Error::InvalidValue);
    }

    if instruction.data.len() > limits.max_bytes || instruction.accounts.len() > limits.max_items {
        return Err(Error::LimitExceeded);
    }

    let tag = *instruction.data.first().ok_or(Error::Truncated)?;

    if instruction.data.len() != 1 {
        return Err(Error::TrailingBytes);
    }

    match tag {
        0 => Ok(AssociatedTokenInstruction::Create(CreateAccounts::parse(
            &instruction.accounts,
        )?)),
        1 => Ok(AssociatedTokenInstruction::CreateIdempotent(
            CreateAccounts::parse(&instruction.accounts)?,
        )),
        2 => Ok(AssociatedTokenInstruction::RecoverNested(
            RecoverNestedAccounts::parse(&instruction.accounts)?,
        )),
        _ => Err(Error::InvalidDiscriminator),
    }
}

/// Source error code for an owner that disagrees with address derivation.
pub const ERROR_INVALID_OWNER: u32 = 0;

/// Returns the pinned error message, or None for an unknown program error.
#[must_use]
pub const fn error_message(code: u32) -> Option<&'static str> {
    match code {
        ERROR_INVALID_OWNER => {
            Some("Associated token account owner does not match address derivation")
        }
        _ => None,
    }
}
