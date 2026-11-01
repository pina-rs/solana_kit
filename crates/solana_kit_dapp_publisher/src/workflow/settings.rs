//! Caller budgets are validated before storage, source reads or network effects.

use super::Error;
use solana_kit_client::rest::Budgets;

const MAX_SOURCE_BYTES: usize = 64 * 1024 * 1024;

/// Explicit bounds for a whole invocation in the caller Clock's namespace.
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    /// Absolute deadline covering I/O, decoding, hashing and durable writes.
    pub deadline: u64,
    /// Maximum ingestion GET attempts; must be nonzero.
    pub max_polls: usize,
    /// Milliseconds between unsuccessful ingestion polls.
    pub poll_interval_ms: u64,
    /// Maximum local APK bytes; nonzero and at most 64 MiB.
    pub source_bytes: usize,
    /// Maximum decoded bytes per media file; nonzero and at most 64 MiB.
    pub media_bytes: usize,
    /// Maximum icon/preview/banner/feature-graphic items before fetching any media.
    pub media_items: usize,
    /// Checkpoint encode/decode budgets, including unknown-field rejection.
    pub snapshot: Budgets,
    /// Final typed metadata document serialization budgets.
    pub metadata: Budgets,
}

impl Limits {
    pub(super) fn validate(self) -> Result<(), Error> {
        let source_range = 1..=MAX_SOURCE_BYTES;

        if self.max_polls == 0
            || self.media_items == 0
            || !source_range.contains(&self.source_bytes)
            || !source_range.contains(&self.media_bytes)
        {
            return Err(Error::Invalid("invalid workflow limits"));
        }

        Ok(())
    }
}

/// Per-invocation local storage identity and caller-issued metadata timestamp.
pub struct Options {
    /// Caller-approved local checkpoint key; never sent in portal request fields.
    pub snapshot_key: String,
    /// Exact UTC ISO-8601 timestamp used for release_details.updated_on.
    pub updated_on: String,
    /// Explicit operation and allocation limits.
    pub limits: Limits,
}
