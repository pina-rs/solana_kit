#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod accounts;
/// Complete pinned instruction, account, plugin and error wire contracts.
pub mod generated;
#[cfg(feature = "pda")]
mod pda;
mod registry;

pub use accounts::*;
pub use generated::*;
#[cfg(feature = "pda")]
pub use pda::*;
pub use registry::*;
