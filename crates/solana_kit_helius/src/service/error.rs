//! Errors preserve bounded server bodies without exposing them through formatting.

use solana_kit_client::TransportError;
use std::{error::Error as StdError, fmt};

impl From<solana_kit_client::rest::JsonError> for Error {
    fn from(error: solana_kit_client::rest::JsonError) -> Self {
        use solana_kit_client::rest::JsonError;

        match error {
            JsonError::Bytes => Self::Bytes,
            JsonError::Depth => Self::Depth,
            JsonError::Items => Self::Items,
            JsonError::String => Self::String,
            JsonError::Schema(error) => Self::Schema(error),
        }
    }
}

/// An observable service, schema or budget failure.
pub enum Error {
    /// Adapter capability, policy, timeout, cancellation or I/O failure.
    Transport(TransportError),
    /// Non-success HTTP status and its original bounded body.
    Http {
        /// Original HTTP status.
        status: u16,
        /// Unmodified body, which may contain secrets and must not be logged.
        body: Vec<u8>,
    },
    /// Request or reply exceeded the byte budget.
    Bytes,
    /// JSON nesting exceeded the depth budget.
    Depth,
    /// JSON values and object keys exceeded the item budget.
    Items,
    /// A JSON string exceeded the encoded UTF-8 byte budget, including escapes.
    String,
    /// JSON syntax or a known typed field was invalid.
    Schema(serde_json::Error),
    /// An endpoint's non-null protocol error, preserved as bounded exact JSON.
    Provider(Box<serde_json::value::RawValue>),
}

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(error) => formatter
                .debug_tuple("Transport")
                .field(&error.kind)
                .finish(),
            Self::Http { status, .. } => formatter.debug_tuple("Http").field(status).finish(),
            Self::Bytes => formatter.write_str("Bytes"),
            Self::Depth => formatter.write_str("Depth"),
            Self::Items => formatter.write_str("Items"),
            Self::String => formatter.write_str("String"),
            Self::Schema(_) => formatter.write_str("Schema"),
            Self::Provider(_) => formatter.write_str("Provider"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Helius service error: {self:?}")
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Transport(error) => Some(error),
            Self::Schema(error) => Some(error),
            _ => None,
        }
    }
}
