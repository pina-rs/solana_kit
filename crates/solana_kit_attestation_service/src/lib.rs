#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod accounts;
/// Complete pinned data layouts and strict instruction builders/parsers.
pub mod generated;
#[cfg(feature = "pda")]
mod pdas;
mod schema;

pub use accounts::*;
pub use generated::*;
#[cfg(feature = "pda")]
pub use pdas::*;
pub use schema::*;

/// Matches a custom error at its actual transaction instruction index.
#[must_use]
pub fn is_attestation_service_error(
    failure: &solana_kit_core::program_error::InstructionFailure,
    programs: &[solana_kit_core::Address],
    code: AttestationServiceError,
) -> bool {
    failure.is_program_error(programs, PROGRAM, Some(code.code()))
}
