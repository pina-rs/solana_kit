//! Bounded original bodies remain available without appearing in ordinary diagnostics.

use solana_kit_client::{TransportError, rest::JsonError};
use std::{error::Error as StdError, fmt};

/// A Jupiter request, adapter, budget or response-schema failure.
pub enum Error {
    /// More than fifty mints were requested; no exchange began.
    TooManyMints,
    /// A price response key was not a canonical Kit address.
    InvalidMint,
    /// An exact integer was not a signed decimal integer string.
    InvalidInteger,
    /// An order lacked the nonblank request identifier required for execution.
    MissingRequestId,
    /// A raw payload cannot be converted to complete Kit-owned values.
    Payload(&'static str),
    /// Kit's binary or text codec rejected a payload.
    Codec(solana_kit_core::codecs::Error),
    /// Kit's transaction decoder rejected the supplied wire data.
    Transaction(solana_kit_core::transaction::Error),
    /// Caller-owned adapter failure with redacted ordinary diagnostics.
    Transport(TransportError),
    /// HTTP failure and its unchanged bounded body; do not log this body.
    Http {
        /// Original HTTP status, including redirects, which adapters must not follow.
        status: u16,
        /// Exact received body, which may contain sensitive data.
        body: Vec<u8>,
    },
    /// Shared byte, depth, item, string, syntax or duplicate-key admission failure.
    Json(JsonError),
    /// A known response field had an invalid type or value.
    /// Its underlying error can contain response data and is excluded from the source chain.
    Schema(serde_json::Error),
}

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyMints => formatter.write_str("TooManyMints"),
            Self::InvalidMint => formatter.write_str("InvalidMint"),
            Self::InvalidInteger => formatter.write_str("InvalidInteger"),
            Self::MissingRequestId => formatter.write_str("MissingRequestId"),
            Self::Payload(reason) => formatter.debug_tuple("Payload").field(reason).finish(),
            Self::Codec(error) => formatter.debug_tuple("Codec").field(error).finish(),
            Self::Transaction(error) => formatter.debug_tuple("Transaction").field(error).finish(),
            Self::Transport(error) => formatter
                .debug_tuple("Transport")
                .field(&error.kind)
                .finish(),
            Self::Http { status, .. } => formatter
                .debug_struct("Http")
                .field("status", status)
                .finish(),
            Self::Json(error) => formatter.debug_tuple("Json").field(error).finish(),
            Self::Schema(_) => formatter.write_str("Schema"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Jupiter request failed: {self:?}")
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Transport(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::Codec(error) => Some(error),
            Self::Transaction(error) => Some(error),
            _ => None,
        }
    }
}
