#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

/// Owned account snapshots and decoding.
pub mod accounts;
/// Typed program-error matching and log extraction.
pub mod program_error;

/// Portable sysvar data layouts.
pub mod sysvars;

mod encoding;
mod instruction;
/// Immutable off-chain application messages and signing envelopes.
pub mod offchain;
/// Pure instruction trees and exact transaction planning.
pub mod plans;
pub mod transaction;

mod values;

/// Bounded deterministic binary codecs.
pub mod codecs;

#[cfg(feature = "crypto")]
pub mod crypto;

pub use encoding::ParseError;
pub use instruction::{AccountMeta, AccountRole, Instruction};
pub use values::{Address, Hash, Signature};

/// Exact decimal and binary fixed-point arithmetic.
#[cfg(feature = "fixed-points")]
pub mod fixed_points;
