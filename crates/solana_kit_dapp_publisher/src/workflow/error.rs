//! Ordinary diagnostics never contain paths, URLs, publisher metadata or provider text.

use super::Effect;
use solana_kit_client::{TransportError, subscriptions::ClockError};
use std::fmt;

/// A bounded workflow failure; an uncertain effect must be reconciled before replay.
pub enum Error {
    /// Original typed portal failure, with its existing redaction boundary.
    Portal(crate::portal::Error),
    /// Caller-owned storage failed; explicit source traversal retains its cause.
    Storage(TransportError),
    /// Caller-owned source reader failed.
    Source(TransportError),
    /// Invalid identifiers, metadata, checkpoint or local policy.
    Invalid(&'static str),
    /// Existing Kit codec rejected input or its allocation budget.
    Codec(solana_kit_core::codecs::Error),
    /// Caller-owned clock failed.
    Clock(ClockError),
    /// Deadline elapsed, including during a ready completion or decoding.
    Deadline,
    /// Ingestion exhausted its explicit poll count.
    PollLimit,
    /// Failed ingestion retains its bounded raw DTO; provider text needs explicit access.
    IngestionFailed(Box<crate::portal::BackendIngestionSession>),
    /// Failed publication retains its bounded raw DTO without implicit message logging.
    PublicationFailed(Box<crate::portal::BackendPublicationSession>),
    /// The last persisted effect lacks an acknowledged outcome; it is never replayed.
    Uncertain(Effect),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Portal(error) => f.debug_tuple("Portal").field(error).finish(),
            Self::Storage(_) => f.write_str("Storage"),
            Self::Source(_) => f.write_str("Source"),
            Self::Invalid(reason) => f.debug_tuple("Invalid").field(reason).finish(),
            Self::Codec(error) => f.debug_tuple("Codec").field(error).finish(),
            Self::Clock(_) => f.write_str("Clock"),
            Self::Deadline => f.write_str("Deadline"),
            Self::PollLimit => f.write_str("PollLimit"),
            Self::IngestionFailed(_) => f.write_str("IngestionFailed"),
            Self::PublicationFailed(_) => f.write_str("PublicationFailed"),
            Self::Uncertain(effect) => f.debug_tuple("Uncertain").field(effect).finish(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "publisher workflow failed: {self:?}")
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Portal(error) => Some(error),
            Self::Storage(error) | Self::Source(error) => Some(error),
            Self::Codec(error) => Some(error),
            Self::Clock(error) => Some(error),
            _ => None,
        }
    }
}

impl From<crate::deadline::Error> for Error {
    fn from(error: crate::deadline::Error) -> Self {
        match error {
            crate::deadline::Error::Clock(error) => Self::Clock(error),
            crate::deadline::Error::Elapsed => Self::Deadline,
        }
    }
}
