#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod addresses;
mod codecs;

mod domain;
mod records;
mod registry;

pub use addresses::*;
pub use codecs::*;

pub use domain::*;
pub use records::*;
pub use registry::*;
