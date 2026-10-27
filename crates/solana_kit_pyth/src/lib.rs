#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;
#[cfg(feature = "hermes")]
extern crate std;

/// Bounded Hermes response models with exact integer components.
#[cfg(feature = "hermes")]
pub mod hermes;

mod account_types;
mod accumulator;
mod instructions;
mod legacy;
mod messages;
#[cfg(feature = "crypto")]
mod pda;
mod program;
mod receiver;
mod vaa;

pub use account_types::*;
pub use accumulator::*;
pub use instructions::*;
pub use legacy::*;
pub use messages::*;
#[cfg(feature = "crypto")]
pub use pda::*;
pub use program::*;
pub use receiver::*;
pub use vaa::*;
