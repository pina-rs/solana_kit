#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod accounts;
/// Source-defined instructions, accounts, Borsh variants and program errors.
pub mod generated;
mod helpers;
#[cfg(feature = "pda")]
mod pda;

pub use accounts::*;
pub use generated::*;
pub use helpers::*;
#[cfg(feature = "pda")]
pub use pda::*;
