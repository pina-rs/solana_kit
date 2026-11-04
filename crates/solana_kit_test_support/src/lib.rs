#![doc(
    html_logo_url = "https://raw.githubusercontent.com/pina-rs/solana_kit/main/assets/brand/solana-kit-mark.png"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/pina-rs/solana_kit/main/assets/brand/solana-kit-favicon-32.png"
)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

/// Raw success results for common public RPC reads.
pub mod canned;
mod lock;
mod pending;
mod script;
/// Optional deterministic keypair fixture.
#[cfg(feature = "crypto")]
pub mod signer;

pub use pending::{PendingExchange, PendingStatus};
pub use script::{CapturedRequest, Exchange, ScriptStep, ScriptedTransport, UnexpectedRequest};
