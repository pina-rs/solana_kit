//! System identity and precise client-side construction failures.

use core::fmt;
use solana_kit_core::{Address, codecs};

/// Canonical System Program identity (11111111111111111111111111111111).
pub const SYSTEM_PROGRAM: Address = Address::from_bytes([0; 32]);
/// Fixed RecentBlockhashes sysvar used by nonce instructions.
pub const RECENT_BLOCKHASHES: Address = solana_kit_core::sysvars::RECENT_BLOCKHASHES_ADDRESS;
/// Fixed Rent sysvar used by nonce instructions.
pub const RENT: Address = solana_kit_core::sysvars::RENT_ADDRESS;

/// A construction or canonical instruction parsing failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClientError {
    /// Bounded instruction data encoding or decoding failed.
    Codec(codecs::Error),
    /// The instruction targets another program.
    ProgramMismatch,
    /// The number of positional accounts is invalid.
    AccountCount,
    /// A positional account has different privileges.
    AccountRole(usize),
    /// A fixed sysvar account has the wrong address.
    AccountAddress(usize),
    /// Seed UTF-8 bytes exceed the protocol's32-byte maximum.
    SeedTooLong,
    /// The encoded base differs from the supplied signing base.
    BaseMismatch,
    /// Funding requires an explicit payer account.
    MissingPayer,
}

impl From<codecs::Error> for ClientError {
    fn from(error: codecs::Error) -> Self {
        Self::Codec(error)
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "system client error: {self:?}")
    }
}

impl core::error::Error for ClientError {}

/// Checks canonical positional contracts in one place; generated parsers supply metadata only.
pub(crate) fn validate_accounts(
    instruction: &solana_kit_core::Instruction,
    contracts: &[(solana_kit_core::AccountRole, Option<Address>)],
    minimum: usize,
) -> Result<(), ClientError> {
    if instruction.program_address != SYSTEM_PROGRAM {
        return Err(ClientError::ProgramMismatch);
    }

    if !(minimum..=contracts.len()).contains(&instruction.accounts.len()) {
        return Err(ClientError::AccountCount);
    }

    for (index, (meta, (role, fixed))) in instruction.accounts.iter().zip(contracts).enumerate() {
        if meta.role != *role {
            return Err(ClientError::AccountRole(index));
        }

        if fixed.is_some_and(|address| address != meta.address) {
            return Err(ClientError::AccountAddress(index));
        }
    }

    Ok(())
}
