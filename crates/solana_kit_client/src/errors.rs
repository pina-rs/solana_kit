//! Errors retain protocol distinctions and adapter causes.
use crate::Presence;
use serde::Deserialize;
use serde_json::value::RawValue;
use std::{error::Error as StdError, fmt};

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
#[derive(Debug)]
pub struct TransportError {
    /// Portable failure category.
    pub kind: TransportFailureKind,
    source: Option<Box<dyn StdError + Send + Sync>>,
}

impl TransportError {
    /// Creates a failure without an underlying error, such as a policy denial.
    pub fn new(kind: TransportFailureKind) -> Self {
        Self { kind, source: None }
    }

    /// Retains an adapter's original error and portable category.
    pub fn with_source(
        kind: TransportFailureKind,
        source: impl StdError + Send + Sync + 'static,
    ) -> Self {
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
#[derive(Debug, Deserialize)]
pub struct RemoteError {
    /// Node-reported code; unknown codes remain intact.
    pub code: i64,
    /// Node-reported message.
    pub message: String,
    /// Absent data differs from a present JSON null.
    #[serde(default)]
    pub data: Presence<Box<RawValue>>,
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
    /// The node returned a JSON-RPC error.
    Remote(RemoteError),
    /// Account base64 bytes or reported full allocation size are invalid.
    AccountData,
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
            _ => None,
        }
    }
}
