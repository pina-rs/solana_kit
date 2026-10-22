#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod generated;
mod helpers;

pub use generated::*;
pub use helpers::*;
