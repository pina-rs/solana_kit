#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

/// Owned account snapshots and decoding.
pub mod accounts;

mod encoding;
mod instruction;
pub mod transaction;
mod values;

/// Bounded deterministic binary codecs.
pub mod codecs;

#[cfg(feature = "crypto")]
pub mod crypto;

pub use encoding::ParseError;
pub use instruction::{AccountMeta, AccountRole, Instruction};
pub use values::{Address, Hash, Signature};
