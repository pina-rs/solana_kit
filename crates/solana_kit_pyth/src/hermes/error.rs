//! Original bounded failures remain accessible without leaking through diagnostics.
use solana_kit_client::subscriptions::ClockError;
use solana_kit_client::{TransportError, TransportFailureKind, rest::JsonError};

const REQUEST_TIMEOUT: u16 = 408;
const TOO_MANY_REQUESTS: u16 = 429;
const SERVER_ERROR: u16 = 500;

/// Failed Hermes exchange, timer, response admission or schema conversion.
pub enum Error {
    /// Categorized adapter failure with its original source.
    Transport(TransportError),
    /// Non-success status and its original bounded response body; never log the body.
    Http {
        /// Original HTTP status.
        status: u16,
        /// Original response bytes, possibly containing sensitive provider data.
        body: alloc::vec::Vec<u8>,
    },
    /// Timer admission, representability or host availability failure.
    Clock(ClockError),
    /// Bounded JSON admission or response-schema failure.
    Json(JsonError),
}

impl Error {
    /// Only transient read failures and the reference retryable statuses repeat.
    pub(super) fn retryable(&self) -> bool {
        match self {
            Self::Http { status, .. } => {
                matches!(*status, REQUEST_TIMEOUT | TOO_MANY_REQUESTS) || *status >= SERVER_ERROR
            }
            Self::Transport(error) => matches!(
                error.kind,
                TransportFailureKind::Io | TransportFailureKind::Deadline
            ),
            _ => false,
        }
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Transport(error) => formatter
                .debug_tuple("Transport")
                .field(&error.kind)
                .finish(),
            Self::Http { status, .. } => formatter.debug_tuple("Http").field(status).finish(),
            Self::Clock(_) => formatter.write_str("Clock"),
            Self::Json(error) => formatter.debug_tuple("Json").field(error).finish(),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "Hermes client error: {self:?}")
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Transport(error) => Some(error),
            Self::Clock(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::Http { .. } => None,
        }
    }
}

impl From<JsonError> for Error {
    fn from(error: JsonError) -> Self {
        Self::Json(error)
    }
}
