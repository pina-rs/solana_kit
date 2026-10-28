//! Surfpool startup settings can be prepared without launching a host process.

mod ports;
mod startup;

pub use ports::{ConfigError, Ports};

use solana_kit_core::Address;
use std::num::{NonZeroU16, NonZeroU64};

/// Surfpool block production policy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BlockProductionMode {
    /// The caller manually advances blocks.
    Manual,
    /// A fixed clock interval advances blocks.
    Clock,
    /// Each transaction advances block production.
    #[default]
    Transaction,
}

impl BlockProductionMode {
    /// Exact `--block-production-mode` value.
    pub const fn cli_value(self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::Clock => "clock",
            Self::Transaction => "transaction",
        }
    }
}

/// A caller-supplied CLI-compatible keypair, with no ambient random source.
#[derive(Clone)]
pub struct KeypairInfo {
    /// Public account address.
    pub public_key: Address,
    secret_key: zeroize::Zeroizing<[u8; 64]>,
}

impl KeypairInfo {
    /// Takes ownership of exactly 64 CLI keypair bytes, matching the source model.
    ///
    /// This metadata constructor does not validate the public-key suffix or sign.
    /// Use Kit's cryptographic key import when a validated signer is needed.
    pub fn new(public_key: Address, secret_key: [u8; 64]) -> Self {
        Self {
            public_key,
            secret_key: zeroize::Zeroizing::new(secret_key),
        }
    }

    /// Alias used by account-oriented APIs.
    pub const fn address(&self) -> Address {
        self.public_key
    }

    /// Explicit read-only access; the library's owned copy is erased on drop.
    pub fn secret_key(&self) -> &[u8; 64] {
        &self.secret_key
    }
}

impl core::fmt::Debug for KeypairInfo {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("KeypairInfo")
            .field("public_key", &self.public_key)
            .finish_non_exhaustive()
    }
}

/// Pinned CLI defaults, retained as host-independent data.
#[derive(Clone)]
pub struct SurfnetConfig {
    /// Run without an upstream unless `remote_rpc_url` is set.
    pub offline: bool,
    /// Explicit upstream URL takes precedence over `offline`.
    pub remote_rpc_url: Option<String>,
    /// Block production policy.
    pub block_production_mode: BlockProductionMode,
    /// Positive slot time in milliseconds.
    pub slot_time_ms: NonZeroU64,
    /// Startup funding in lamports, despite the source field's historic name.
    pub airdrop_sol: u64,
    /// Additional startup funding destinations.
    pub airdrop_addresses: Vec<Address>,
    /// Optional caller-owned payer metadata; no payer is generated implicitly.
    pub payer: Option<KeypairInfo>,
    /// Feature gates to enable.
    pub enable_features: Vec<Address>,
    /// Feature gates to disable.
    pub disable_features: Vec<Address>,
    /// Enable every runtime feature gate.
    pub all_features: bool,
    /// Skip blockhash validation in the local runtime.
    pub skip_blockhash_check: bool,
    /// Host interface passed explicitly to the CLI.
    pub host: String,
    /// Optional fixed HTTP RPC port.
    pub rpc_port: Option<NonZeroU16>,
    /// Optional fixed WebSocket port.
    pub ws_port: Option<NonZeroU16>,
    /// Optional fixed Studio/scenario port, even with Studio UI disabled.
    pub studio_port: Option<NonZeroU16>,
}

impl core::fmt::Debug for SurfnetConfig {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("SurfnetConfig")
            .field("offline", &self.offline)
            .field(
                "remote_rpc_url",
                &self.remote_rpc_url.as_ref().map(|_| "[REDACTED]"),
            )
            .field("block_production_mode", &self.block_production_mode)
            .field("slot_time_ms", &self.slot_time_ms)
            .field("airdrop_sol", &self.airdrop_sol)
            .field("airdrop_addresses", &self.airdrop_addresses)
            .field("payer", &self.payer)
            .field("enable_features", &self.enable_features)
            .field("disable_features", &self.disable_features)
            .field("all_features", &self.all_features)
            .field("skip_blockhash_check", &self.skip_blockhash_check)
            .field("host", &self.host)
            .field("rpc_port", &self.rpc_port)
            .field("ws_port", &self.ws_port)
            .field("studio_port", &self.studio_port)
            .finish()
    }
}

impl Default for SurfnetConfig {
    fn default() -> Self {
        Self {
            offline: true,
            remote_rpc_url: None,
            block_production_mode: BlockProductionMode::Transaction,
            slot_time_ms: NonZeroU64::MIN,
            airdrop_sol: 10_000_000_000,
            airdrop_addresses: Vec::new(),
            payer: None,
            enable_features: Vec::new(),
            disable_features: Vec::new(),
            all_features: false,
            skip_blockhash_check: false,
            host: "127.0.0.1".into(),
            rpc_port: None,
            ws_port: None,
            studio_port: None,
        }
    }
}

impl SurfnetConfig {
    /// Names the funding unit explicitly.
    pub const fn airdrop_lamports(&self) -> u64 {
        self.airdrop_sol
    }

    /// Rejects configured port collisions before the host allocates resources.
    pub fn validate(&self) -> Result<(), ConfigError> {
        ports::validate([self.rpc_port, self.ws_port, self.studio_port])
    }
}
