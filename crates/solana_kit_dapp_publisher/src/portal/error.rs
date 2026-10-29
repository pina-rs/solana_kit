//! Untrusted bodies and messages require explicit access and never enter ordinary diagnostics.

use solana_kit_client::{TransportError, rest::JsonError, subscriptions::ClockError};
use std::fmt;

/// A request, transport, bounded-response, schema or provider failure.
pub enum Error {
    /// Invalid local request policy, before any exchange begins.
    Request(&'static str),
    /// Original adapter cause behind the shared redacted boundary.
    Transport(TransportError),
    /// HTTP rejection with the exact original bounded body, available explicitly.
    Http {
        /// Original non-success HTTP status.
        status: u16,
        /// Exact bounded response; access explicitly rather than logging diagnostics.
        body: Vec<u8>,
    },
    /// A recognized tRPC error or Left result; message and body are untrusted data.
    Provider {
        /// Untrusted provider message, absent when the error carries no message.
        message: Option<String>,
        /// Exact bounded response; it may contain provider or publisher metadata.
        body: Vec<u8>,
    },
    /// Byte, depth, string, aggregate-item, syntax or duplicate-key rejection.
    Json(JsonError),
    /// Invalid known field or object shape; diagnostics omit the underlying message.
    Schema(serde_json::Error),
    /// Missing or conflicting protocol fields, identified without response data.
    Envelope(&'static str),
    /// Caller-owned monotonic clock failed.
    Clock(ClockError),
    /// The caller's absolute ingestion-retry deadline elapsed.
    Deadline,
}

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request(reason) => formatter.debug_tuple("Request").field(reason).finish(),
            Self::Transport(error) => formatter
                .debug_tuple("Transport")
                .field(&error.kind)
                .finish(),
            Self::Http { status, .. } => formatter
                .debug_struct("Http")
                .field("status", status)
                .finish(),
            Self::Provider { .. } => formatter.write_str("Provider"),
            Self::Json(error) => formatter.debug_tuple("Json").field(error).finish(),
            Self::Schema(_) => formatter.write_str("Schema"),
            Self::Envelope(reason) => formatter.debug_tuple("Envelope").field(reason).finish(),
            Self::Clock(_) => formatter.write_str("Clock"),
            Self::Deadline => formatter.write_str("Deadline"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "portal request failed: {self:?}")
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Transport(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::Clock(error) => Some(error),
            _ => None,
        }
    }
}
