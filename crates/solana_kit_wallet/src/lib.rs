//! Portable off-chain Wallet Standard application contracts.
//!
//! Backends own authorization, signing, and cancellation. No executor or platform
//! discovery is provided. Raw signatures are not cryptographic proof. On-chain
//! programs should use Pina or official Solana program libraries.
#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

/// Host-independent wallet view snapshots and user actions.
pub mod ui;

/// Portable Mobile Wallet Adapter cryptography and application protocol.
#[cfg(feature = "mwa")]
pub mod mwa;

mod capabilities;
mod logo_assets;
mod logos;
mod metadata;
mod registry;
mod solana;

pub use capabilities::*;
pub use logos::*;
pub use metadata::*;
pub use registry::*;
pub use solana::*;
