#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod account;
mod error;
/// Data codecs emitted from the exact pinned Codama input (Dart account padding layout).
pub mod generated;
mod instruction;
pub use account::*;
pub use error::*;
pub use instruction::*;

use solana_kit_core::Address;

/// Canonical Address Lookup Table program identity.
pub const PROGRAM: Address = Address::from_bytes([
    2, 119, 166, 175, 151, 51, 155, 122, 200, 141, 24, 146, 201, 4, 70, 245, 0, 2, 48, 146, 102,
    246, 46, 83, 193, 24, 36, 73, 130, 0, 0, 0,
]);

/// Derives the canonical highest-bump table PDA from authority and little-endian slot.
///
/// # Errors
/// Returns the core PDA error if no viable bump exists.
#[cfg(feature = "pda")]
pub fn find_lookup_table(
    authority: Address,
    recent_slot: u64,
    program: Address,
) -> Result<solana_kit_core::crypto::ProgramDerivedAddress, solana_kit_core::crypto::PdaError> {
    solana_kit_core::crypto::find_program_address(
        &[authority.as_bytes(), &recent_slot.to_le_bytes()],
        &program,
    )
}
