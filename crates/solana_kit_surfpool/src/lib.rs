#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

#[macro_use]
mod wire;

/// Account mutation builders and exact authority clearing semantics.
pub mod builders;
/// All 28 pinned Surfpool cheatcode request types.
pub mod cheatcodes;
/// Runtime-neutral configuration and explicit CLI argument preparation.
pub mod config;
/// Sequential program deployment from caller-provided bytes.
pub mod deploy;
/// Method-aware errors retaining the shared client's original failure.
pub mod errors;
/// Canonical lowercase program and account byte encoding.
pub mod hex;
/// Exact response schemas and confidential-transfer updates.
pub mod types;

mod client;
mod response;

#[cfg(feature = "crypto")]
pub use client::funding::get_ata;
pub use client::{Surfnet, SurfpoolClient};
pub use response::Response;

/// Provider extension objects whose shape is intentionally unspecified upstream.
pub type Object = serde_json::Map<String, serde_json::Value>;

/// Default endpoint used by the Surfpool CLI.
pub const DEFAULT_SURFNET_ENDPOINT: &str = "http://127.0.0.1:8899";
