#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod accounts;
#[cfg(feature = "das")]
/// Typed provider RPC composition without ambient endpoints or credentials.
pub mod das;
mod events;
/// Reproducibly generated instruction, state and error contracts.
pub mod generated;
mod leaf_instructions;
mod metadata;
mod proof;

#[cfg(feature = "hashing")]
mod hashing;
#[cfg(feature = "hashing")]
mod merkle;
#[cfg(feature = "pda")]
mod mint;
#[cfg(feature = "pda")]
mod mint_collection;

#[cfg(feature = "pda")]
mod pda;
#[cfg(feature = "tree")]
mod tree_instructions;

pub use accounts::*;
pub use events::*;
pub use generated::*;
#[cfg(feature = "hashing")]
pub use hashing::*;
pub use leaf_instructions::*;
#[cfg(feature = "hashing")]
pub use merkle::*;
pub use metadata::*;
#[cfg(feature = "pda")]
pub use mint::*;
#[cfg(feature = "pda")]
pub use mint_collection::*;
#[cfg(feature = "pda")]
pub use pda::*;
pub use proof::*;
#[cfg(feature = "tree")]
pub use tree_instructions::*;

/// Checks transfer eligibility without asserting authorization or proof validity.
#[must_use]
pub const fn can_transfer(frozen: bool, non_transferable: bool) -> bool {
    !frozen && !non_transferable
}
