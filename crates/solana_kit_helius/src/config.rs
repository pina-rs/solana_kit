//! Endpoint discovery remains separate from authenticated adapter construction.

use crate::{SensitiveString, enums::HeliusCluster};

/// Validated caller credentials and the selected provider cluster.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    api_key: SensitiveString,
    /// Selected cluster. Mainnet is the reference default.
    pub cluster: HeliusCluster,
}

/// An empty key cannot authenticate a provider exchange.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissingApiKey;

impl core::fmt::Display for MissingApiKey {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("Helius API key is required")
    }
}

impl std::error::Error for MissingApiKey {}

impl Config {
    /// Validates a caller-supplied key and selects mainnet without starting I/O.
    pub fn new(api_key: SensitiveString) -> Result<Self, MissingApiKey> {
        if api_key.expose().trim().is_empty() {
            return Err(MissingApiKey);
        }

        Ok(Self {
            api_key,
            cluster: HeliusCluster::Mainnet,
        })
    }

    /// Borrows the wrapped key for an explicit application adapter.
    pub const fn api_key(&self) -> &SensitiveString {
        &self.api_key
    }

    /// Returns the RPC origin without credentials or query parameters.
    pub const fn rpc_origin(&self) -> &'static str {
        match self.cluster {
            HeliusCluster::Mainnet => "https://mainnet-beta.helius-rpc.com",
            HeliusCluster::Devnet => "https://devnet.helius-rpc.com",
        }
    }

    /// Returns the REST origin without credentials or path prefixes.
    pub const fn rest_origin(&self) -> &'static str {
        match self.cluster {
            HeliusCluster::Mainnet => "https://api-mainnet.helius-rpc.com",
            HeliusCluster::Devnet => "https://api-devnet.helius.xyz",
        }
    }

    /// Returns the Wallet v1 beta origin, whose availability is independent of RPC cluster selection.
    pub const fn wallet_origin(&self) -> &'static str {
        "https://api.helius.xyz"
    }

    /// Returns the WebSocket origin without credentials.
    pub const fn websocket_origin(&self) -> &'static str {
        match self.cluster {
            HeliusCluster::Mainnet => "wss://mainnet-beta.helius-rpc.com",
            HeliusCluster::Devnet => "wss://devnet.helius-rpc.com",
        }
    }

    /// Returns the sender origin without credentials.
    pub const fn sender_origin(&self) -> &'static str {
        match self.cluster {
            HeliusCluster::Mainnet => "https://mainnet.helius-rpc.com",
            HeliusCluster::Devnet => "https://devnet.helius-rpc.com",
        }
    }
}
