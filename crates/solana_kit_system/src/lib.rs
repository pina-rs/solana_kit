#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod generated;
pub use generated::*;
