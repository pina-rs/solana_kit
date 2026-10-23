//! Mobile Wallet Adapter wire protocol, independent of launchers and transports.
//!
//! Private scalars and per-message IVs come from the caller. No host entropy,
//! socket, runtime, application launcher, or default endpoint is installed.

mod association;
mod crypto;
mod rpc;
mod session;
mod siws;
mod types;

pub use association::*;
pub use crypto::*;
pub use rpc::*;
pub use session::*;
pub use types::*;

/// WebSocket binary subprotocol from the pinned protocol contract.
pub const WEBSOCKET_BINARY: &str = "com.solana.mobilewalletadapter.v1";
/// WebSocket base64 subprotocol for reflector connections.
pub const WEBSOCKET_BASE64: &str = "com.solana.mobilewalletadapter.v1.base64";
/// Local WebSocket path. The caller supplies the validated local port.
pub const LOCAL_WEBSOCKET_PATH: &str = "/solana-wallet";
/// Suggested connection retry delays in milliseconds. No retry is started here.
pub const RETRY_DELAYS_MS: [u16; 8] = [150, 150, 200, 500, 500, 750, 750, 1000];
/// Suggested connection deadline in milliseconds. The caller owns its clock.
pub const CONNECTION_TIMEOUT_MS: u32 = 30_000;

/// A protocol boundary failed. Error text never includes secret or peer bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// A private scalar is zero or outside the P-256 group order.
    PrivateKey,
    /// A public point is malformed, noncanonical, or outside P-256.
    PublicKey,
    /// ECDSA authentication failed.
    Signature,
    /// A frame is malformed or AES-GCM authentication failed.
    Authentication,
    /// The frame is out of order, replayed, or the sequence is exhausted.
    Sequence,
    /// A URI, port, reflector ID, or protocol version is invalid.
    Association,
    /// JSON does not match the expected protocol shape or request ID.
    Json,
    /// A SIWS field contains a line break or its signed result is invalid.
    SignIn,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MWA protocol {:?} error", self)
    }
}

impl core::error::Error for Error {}
