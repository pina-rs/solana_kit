//! Deterministic Token-2022 client for off-chain applications.
//! On-chain programs should use Pina or official Solana program libraries.
#![no_std]
extern crate alloc;

mod generated {
    include!("generated.rs");
    include!("extension_sequence.rs");
}
/// Mint extension pre-initialization operations.
pub mod extensions;
mod instructions;
mod invocation;

pub mod accounts;
pub mod errors;
/// Optional executor-neutral typed account fetching.
#[cfg(feature = "client")]
pub mod fetch;

pub use generated::*;
pub use instructions::*;
pub use invocation::{Authority, ParsedInstruction};
