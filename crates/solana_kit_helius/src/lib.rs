#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

#[macro_use]
mod models;
mod field;

/// Project credit and service usage administration.
pub mod admin;
/// Authentication project and API-key services.
pub mod auth;
/// Digital asset requests, proofs and metadata.
pub mod das;
/// Enhanced transaction history services.
pub mod enhanced;
/// Exact closed string enums from the pinned provider contract.
pub mod enums;
/// Binary preconfirmation frames. A decoded frame is an untrusted early signal.
pub mod preconf;
/// Priority fee estimation requests and decimal results.
pub mod priority_fee;
/// Enhanced RPC V2 queries and bounded cursor collection.
pub mod rpc_v2;
/// Single-exchange Helius Sender submission using the existing Kit wire encoder.
pub mod sender;
/// Caller-owned REST exchange contracts.
pub mod service;
/// Staking transaction preparation through the provider service.
pub mod staking;
/// Typed JSON subscription bridge through the shared bounded connection owner.
pub mod subscriptions;
/// Wallet identity, balance and history services.
pub mod wallet;
/// Webhook administration through explicit typed REST requests.
pub mod webhooks;
/// ZK compression query models and proofs.
pub mod zk;

mod bounded;
mod config;
mod secret;

pub use bounded::Bounded;
pub use config::{Config, MissingApiKey};
pub use field::{Decimal, Field};
pub use secret::SensitiveString;

/// Composes independently configured RPC and REST owners without hidden adapters.
pub struct Helius<R, S> {
    /// Typed JSON-RPC methods use the existing Kit envelope/correlation owner.
    pub rpc: solana_kit_client::Client<R>,
    /// Typed REST methods use caller-approved service destinations.
    pub services: service::ServiceClient<S>,
}

impl<R, S> Helius<R, S> {
    /// Composes already configured owners; no credentials or tasks are created.
    pub const fn new(
        rpc: solana_kit_client::Client<R>,
        services: service::ServiceClient<S>,
    ) -> Self {
        Self { rpc, services }
    }
}
