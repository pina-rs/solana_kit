use crate::{ConfigError, keys::NativeEntropyError};
use std::{error::Error, fmt, io};
use tokio_tungstenite::tungstenite;

/// Redacted mobile-wallet failure with explicit access to original causes.
pub enum MwaError {
    /// The secure provider failed.
    Entropy(NativeEntropyError),
    /// The native socket failed.
    Io(io::Error),
    /// The WebSocket implementation rejected the exchange.
    Socket(tungstenite::Error),
    /// The MWA framing or RPC contract failed.
    Protocol(solana_kit_wallet::mwa::Error),
    /// A selected finite transport configuration is invalid.
    Config(ConfigError),
    /// An owned connect or operation deadline expired.
    Deadline(tokio::time::error::Elapsed),
    /// The session has ended or was cancelled during an operation.
    Closed,
    /// A finite frame, message or nonce-history budget was exhausted.
    Capacity,
    /// The peer did not negotiate the required binary subprotocol.
    Handshake,
    /// The secure provider did not produce a valid scalar within the finite bound.
    Scalar,
}

impl fmt::Debug for MwaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self {
            Self::Entropy(_) => "entropy",
            Self::Io(_) => "I/O",
            Self::Socket(_) => "socket",
            Self::Protocol(_) => "protocol",
            Self::Config(_) => "configuration",
            Self::Deadline(_) => "deadline",
            Self::Closed => "closed",
            Self::Capacity => "capacity",
            Self::Handshake => "handshake",
            Self::Scalar => "scalar",
        };

        f.debug_tuple("MwaError").field(&kind).finish()
    }
}

impl fmt::Display for MwaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("mobile wallet operation failed")
    }
}

impl Error for MwaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Entropy(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::Socket(error) => Some(error),
            Self::Protocol(error) => Some(error),
            Self::Config(error) => Some(error),
            Self::Deadline(error) => Some(error),
            _ => None,
        }
    }
}

impl From<solana_kit_wallet::mwa::Error> for MwaError {
    fn from(error: solana_kit_wallet::mwa::Error) -> Self {
        Self::Protocol(error)
    }
}
