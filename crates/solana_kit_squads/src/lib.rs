#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod accounts;
/// Complete generated instruction, account, named value and program-error contracts.
pub mod generated;
mod message;
#[cfg(feature = "pda")]
mod pda;

pub use accounts::*;
pub use generated::*;
pub use message::*;
#[cfg(feature = "pda")]
pub use pda::*;

/// Canonical Squads V4 program identity in the pinned MIT Dart client.
pub const PROGRAM_ADDRESS: &str = "SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf";
