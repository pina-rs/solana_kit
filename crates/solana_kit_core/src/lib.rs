#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

/// Insertion-ordered values for portable client codecs and application data.
pub mod collections;

/// Stable code, domain, public context and retained diagnostic causes.
pub mod errors;

/// Owned account snapshots and decoding.
pub mod accounts;
/// Canonical account addresses without program-client dependencies.
pub mod address_constants;
/// Typed program-error matching and log extraction.
pub mod program_error;

/// Portable sysvar data layouts.
pub mod sysvars;

mod address_order;
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

pub use address_order::compare_addresses;
pub use encoding::{ParseError, ValueKind};
pub use instruction::{
    AccountLookupMeta, AccountMeta, AccountRole, Instruction, InstructionAccount,
    InstructionWithAccounts, LookupRole,
};
pub use values::{Address, Hash, Signature};

/// Optional Keccak-256 hashing without host APIs.
#[cfg(feature = "keccak")]
pub mod keccak;

/// Exact decimal and binary fixed-point arithmetic.
#[cfg(feature = "fixed-points")]
pub mod fixed_points;
