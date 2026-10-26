#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod accounts;
/// Complete pinned instruction, account, event, PDA/default and error contracts.
pub mod generated;
mod schedule;
mod tails;

pub use accounts::*;
pub use generated::*;
pub use schedule::*;
pub use tails::*;
