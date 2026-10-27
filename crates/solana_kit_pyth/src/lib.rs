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
mod legacy;
mod messages;
mod receiver;
mod vaa;

pub use account_types::*;
pub use accumulator::*;
pub use legacy::*;
pub use messages::*;
pub use receiver::*;
pub use vaa::*;
