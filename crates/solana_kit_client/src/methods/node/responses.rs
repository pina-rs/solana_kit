//! Owned method-specific response models.
use crate::Presence;
use serde::Deserialize;
use solana_kit_core::Address;
use std::{collections::BTreeMap, net::SocketAddr};

/// Successful health response; unhealthy nodes return a JSON-RPC error.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum Health {
    /// The node reports itself healthy.
    #[serde(rename = "ok")]
    Ok,
}

/// Node software identity and optional advertised feature-set identifier.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct NodeVersion {
    /// Node software version text, retained without semver assumptions.
    pub solana_core: String,
    /// Older nodes may omit this field; explicit null remains distinct.
    #[serde(default)]
    pub feature_set: Presence<Option<u32>>,
}

/// RPC node's validated identity.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct NodeIdentity {
    /// Node identity address.
    #[serde(deserialize_with = "super::serde_address::deserialize")]
    pub identity: Address,
}

/// Highest full snapshot and its nullable incremental snapshot.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct SnapshotSlots {
    /// Highest full snapshot slot.
    pub full: u64,
    /// Present null means no incremental snapshot; a missing field is invalid.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub incremental: Option<u64>,
}

/// Exact epoch counters as observed by the node.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EpochInfo {
    /// Current absolute slot.
    pub absolute_slot: u64,
    /// Current block height.
    pub block_height: u64,
    /// Current epoch.
    pub epoch: u64,
    /// Slot index within the epoch.
    pub slot_index: u64,
    /// Number of slots in the epoch.
    pub slots_in_epoch: u64,
    /// Transaction count, or explicit null if unavailable.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub transaction_count: Option<u64>,
}

/// Cluster epoch schedule from genesis configuration.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EpochSchedule {
    /// First epoch at normal length.
    pub first_normal_epoch: u64,
    /// First slot at normal epoch length.
    pub first_normal_slot: u64,
    /// Leader scheduling lookahead in slots.
    pub leader_schedule_slot_offset: u64,
    /// Normal epoch length.
    pub slots_per_epoch: u64,
    /// Whether initial epochs grow from short lengths.
    pub warmup: bool,
}

/// Advertised cluster-node endpoints and version metadata.
///
/// Fields added across node versions preserve absence separately from null.
/// Endpoints are parsed socket addresses, never opened by this type.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClusterNode {
    /// Validated node identity.
    #[serde(deserialize_with = "super::serde_address::deserialize")]
    pub pubkey: Address,
    /// Validator software client identifier.
    #[serde(default)]
    pub client_id: Presence<Option<String>>,
    /// Advertised feature identifier.
    #[serde(default)]
    pub feature_set: Presence<Option<u32>>,
    /// Gossip endpoint.
    #[serde(default)]
    pub gossip: Presence<Option<SocketAddr>>,
    /// Pubsub endpoint.
    #[serde(default)]
    pub pubsub: Presence<Option<SocketAddr>>,
    /// JSON-RPC endpoint.
    #[serde(default)]
    pub rpc: Presence<Option<SocketAddr>>,
    /// Repair endpoint.
    #[serde(default)]
    pub serve_repair: Presence<Option<SocketAddr>>,
    /// Shred protocol version.
    #[serde(default)]
    pub shred_version: Presence<Option<u16>>,
    /// Legacy UDP transaction endpoint.
    #[serde(default)]
    pub tpu: Presence<Option<SocketAddr>>,
    /// Legacy UDP forwarding endpoint.
    #[serde(default)]
    pub tpu_forwards: Presence<Option<SocketAddr>>,
    /// QUIC forwarding endpoint.
    #[serde(default)]
    pub tpu_forwards_quic: Presence<Option<SocketAddr>>,
    /// QUIC transaction endpoint.
    #[serde(default)]
    pub tpu_quic: Presence<Option<SocketAddr>>,
    /// Vote endpoint.
    #[serde(default)]
    pub tpu_vote: Presence<Option<SocketAddr>>,
    /// Validation endpoint.
    #[serde(default)]
    pub tvu: Presence<Option<SocketAddr>>,
    /// Software version text.
    #[serde(default)]
    pub version: Presence<Option<String>>,
}

/// Slot range returned by the server; both endpoints are required.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProductionRange {
    /// Inclusive first slot.
    pub first_slot: u64,
    /// Inclusive last slot.
    pub last_slot: u64,
}

/// Validator production indexed by validated identity address.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlockProductionValue {
    /// Each pair is [leader slots, produced blocks], in that order.
    #[serde(deserialize_with = "super::serde_address::map")]
    pub by_identity: BTreeMap<Address, [u64; 2]>,
    /// Actual queried range.
    pub range: ProductionRange,
}

/// Leader slot indices, relative to the first slot in the selected epoch.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct LeaderScheduleValue(
    /// Identity-indexed exact slot indices.
    #[serde(deserialize_with = "super::serde_address::map")]
    pub BTreeMap<Address, Vec<u64>>,
);

/// One performance sample, with backwards-compatible vote-count presence.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PerformanceSample {
    /// Sampling slot.
    pub slot: u64,
    /// All transactions in the window.
    pub num_transactions: u64,
    /// Older nodes omit this counter; explicit null is preserved.
    #[serde(default)]
    pub num_non_vote_transactions: Presence<Option<u64>>,
    /// Slots observed in the window.
    pub num_slots: u64,
    /// Sample duration in seconds.
    pub sample_period_secs: u16,
}
