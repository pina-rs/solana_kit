//! Errors retain protocol distinctions and adapter causes.
use crate::Presence;
use serde::Deserialize;
use serde_json::value::RawValue;
use std::{error::Error as StdError, fmt};

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
type TransportSource = dyn StdError;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
type TransportSource = dyn StdError + Send + Sync;

/// Portable host failure categories, independent of a particular HTTP library.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportFailureKind {
    /// The host does not provide this capability.
    Unavailable,
    /// The host or destination policy denied access.
    Denied,
    /// The exchange deadline elapsed.
    Deadline,
    /// The owner cancelled the exchange.
    Cancelled,
    /// Network, TLS or other underlying I/O failed.
    Io,
    /// The receive body exceeded the granted limit.
    BodyLimit,
    /// Response headers exceeded the adapter limit.
    HeaderLimit,
    /// HTTP framing was invalid.
    InvalidResponse,
}

/// A categorized adapter failure retaining its original error when available.
///
/// On `wasm32-unknown-unknown`, sources may contain local values, so this error
/// and errors containing it do not promise `Send` or `Sync`. This also applies
/// to JavaScript-free guests on that target and adds no host dependency. Other
/// targets retain `Send + Sync` sources, including native and WASI adapters.
#[derive(Debug)]
pub struct TransportError {
    /// Portable failure category.
    pub kind: TransportFailureKind,
    source: Option<Box<TransportSource>>,
}

impl TransportError {
    /// Creates a failure without an underlying error, such as a policy denial.
    pub fn new(kind: TransportFailureKind) -> Self {
        Self { kind, source: None }
    }

    /// Retains an adapter's original error and portable category.
    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    pub fn with_source(
        kind: TransportFailureKind,
        source: impl StdError + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind,
            source: Some(Box::new(source)),
        }
    }

    /// Retains an adapter's original, possibly local error and category.
    ///
    /// Available on `wasm32-unknown-unknown`, including JavaScript-free guests.
    /// Sources need not implement `Send` or `Sync` on this target.
    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    pub fn with_source(kind: TransportFailureKind, source: impl StdError + 'static) -> Self {
        Self {
            kind,
            source: Some(Box::new(source)),
        }
    }
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "transport failure: {:?}", self.kind)
    }
}

impl StdError for TransportError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn StdError + 'static))
    }
}

/// A JSON-RPC failure with bounded unknown data retained as exact JSON.
#[derive(Deserialize)]
pub struct RemoteError {
    /// Node-reported code; unknown codes remain intact.
    pub code: i64,
    /// Node-reported message.
    pub message: String,
    /// Absent data differs from a present JSON null.
    #[serde(default)]
    pub data: Presence<Box<RawValue>>,
}

impl fmt::Debug for RemoteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RemoteError")
            .field("code", &self.code)
            .finish_non_exhaustive()
    }
}

/// Failure at the client, transport, protocol or typed-domain boundary.
#[derive(Debug)]
pub enum Error {
    /// This client's correlation namespace is exhausted.
    IdExhausted,
    /// The adapter failed.
    Transport(TransportError),
    /// The HTTP status was outside 200 through 299.
    Status(u16),
    /// The request or response body exceeds its byte budget.
    BodyLimit,
    /// JSON containers exceed the nesting budget.
    DepthLimit,
    /// The JSON document or envelope is malformed.
    Json(serde_json::Error),
    /// The method's required schema or domain values are invalid.
    Schema(serde_json::Error),
    /// The protocol version or result/error combination is invalid.
    Envelope,
    /// The response belongs to another request.
    Correlation,
    /// A submission acknowledgement names another transaction.
    SubmissionSignatureMismatch {
        /// Primary signature retained from the exact submitted wire.
        expected: solana_kit_core::Signature,
        /// Canonical signature reported by the endpoint.
        actual: Box<solana_kit_core::Signature>,
    },
    /// The node returned a JSON-RPC error.
    Remote(RemoteError),
    /// Account base64 bytes or reported full allocation size are invalid.
    AccountData,
    /// A snapshot still has encoded or compressed data at this address.
    AccountNotDecoded(solana_kit_core::Address),
    /// Public diagnostic context exceeded its explicit representation contract.
    DiagnosticContext(solana_kit_core::errors::ContextError),
}

impl From<TransportError> for Error {
    fn from(error: TransportError) -> Self {
        Self::Transport(error)
    }
}
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RPC client error: {self:?}")
    }
}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Transport(error) => Some(error),
            Self::Json(error) | Self::Schema(error) => Some(error),
            Self::DiagnosticContext(error) => Some(error),
            _ => None,
        }
    }
}
