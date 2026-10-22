//! Private off-chain RPC controls over the public client. Never use on chain.
#![forbid(unsafe_code)]

pub mod alternatives;
pub mod conformance;
pub use solana_kit_client::{
    Client, Error, Limits, Presence, Reply, Request, Transport, TransportError,
    TransportFailureKind, methods,
};
