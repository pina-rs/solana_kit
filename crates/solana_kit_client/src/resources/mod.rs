//! Version-aware resource limits and one-shot simulation through a caller-owned client.
//!
//! These helpers edit uncompiled instructions so no signature can silently survive a
//! resource change. Callers retain lifetime metadata, lookup snapshots, transport
//! deadlines, cancellation ownership, and the decision to sign or submit. Enable
//! `resource-estimation` to use this module.

mod error;
mod message;
mod simulation;
mod tuning;

pub use error::{ResourceError, TuningError};
pub use message::{ResourceMessage, VersionContext};
pub use simulation::{EstimateConfig, estimate};
pub use tuning::{estimate_and_set, estimate_and_set_with_client};

/// Maximum compute budget used while observing actual execution usage.
pub const MAX_COMPUTE_UNIT_LIMIT: u32 = solana_kit_compute_budget::MAX_COMPUTE_UNIT_LIMIT;
/// Maximum loaded account bytes used for version-one simulation, 64 MiB.
pub const MAX_LOADED_ACCOUNTS_DATA_SIZE_LIMIT: u32 = 64 * 1024 * 1024;
/// Placeholder that reserves room for a compute estimate.
pub const PROVISORY_COMPUTE_UNIT_LIMIT: u32 =
    solana_kit_compute_budget::PROVISORY_COMPUTE_UNIT_LIMIT;
/// Placeholder that reserves room for a version-one loaded account estimate.
pub const PROVISORY_LOADED_ACCOUNTS_DATA_SIZE_LIMIT: u32 = 0;

/// Observed execution usage, with no additional margin or fee policy applied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceLimitsEstimate {
    /// Compute usage capped at the runtime's u32 wire width.
    pub compute_unit_limit: u32,
    /// Loaded account usage when supplied by the node; required for version one.
    pub loaded_accounts_data_size_limit: Option<u32>,
}
