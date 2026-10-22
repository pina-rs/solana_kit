#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod encoding;
mod instruction;
mod values;

#[cfg(feature = "crypto")]
pub mod crypto;

pub use encoding::ParseError;
pub use instruction::{AccountMeta, AccountRole, Instruction};
pub use values::{Address, Hash, Signature};
