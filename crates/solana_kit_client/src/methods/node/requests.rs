//! Typed params and method-specific success decoders.
use super::{
    AlpenglowGenesisCertificate, BlockProductionValue, ClusterNode, EpochInfo, EpochSchedule,
    Health, LeaderScheduleValue, NodeIdentity, NodeVersion, PerformanceSample, SnapshotSlots,
};
use crate::{
    Error,
    methods::{Commitment, Contextual, Method, ReadConfig},
};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};
use solana_kit_core::{Address, Hash};

fn parse<T: serde::de::DeserializeOwned>(json: &str) -> Result<T, Error> {
    serde_json::from_str(json).map_err(Error::Schema)
}

macro_rules! method {
    ($name:ident, $rpc:literal, $response:ty, $decode:expr) => {
        impl Method for $name {
            type Response = $response;
            const NAME: &'static str = $rpc;
            fn decode(&self, json: &str) -> Result<Self::Response, Error> {
                ($decode)(json)
            }
        }
    };
}

macro_rules! no_params {
    ($name:ident, $rpc:literal, $response:ty, $decode:expr) => {
        #[doc = concat!("Typed `", $rpc, "` operation with no parameters.")]
        #[derive(Debug, Clone, Copy)]
        pub struct $name;
        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                <[u8; 0]>::default().serialize(s)
            }
        }
        method!($name, $rpc, $response, $decode);
    };
}

macro_rules! read_params {
    ($name:ident, $rpc:literal, $response:ty, $decode:expr) => {
        #[doc = concat!("Typed `", $rpc, "` operation with optional read constraints.")]
        #[derive(Debug, Clone, Copy)]
        pub struct $name(
            /// Omitted entirely when None; Some(empty config) serializes [{}].
            pub Option<ReadConfig>,
        );
        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                self.0.as_slice().serialize(s)
            }
        }
        method!($name, $rpc, $response, $decode);
    };
}

no_params!(
    AgGenesisCert,
    "getAgGenesisCert",
    Option<AlpenglowGenesisCertificate>,
    parse
);
no_params!(ClusterNodes, "getClusterNodes", Vec<ClusterNode>, parse);
no_params!(GetEpochSchedule, "getEpochSchedule", EpochSchedule, parse);
no_params!(GenesisHash, "getGenesisHash", Hash, decode_hash);
no_params!(GetHealth, "getHealth", Health, parse);
no_params!(
    HighestSnapshotSlot,
    "getHighestSnapshotSlot",
    SnapshotSlots,
    parse
);
no_params!(Identity, "getIdentity", NodeIdentity, parse);
no_params!(MaxRetransmitSlot, "getMaxRetransmitSlot", u64, parse);
no_params!(MaxShredInsertSlot, "getMaxShredInsertSlot", u64, parse);
no_params!(Version, "getVersion", NodeVersion, parse);
read_params!(BlockHeight, "getBlockHeight", u64, parse);
read_params!(GetEpochInfo, "getEpochInfo", EpochInfo, parse);
read_params!(Slot, "getSlot", u64, parse);
read_params!(SlotLeader, "getSlotLeader", Address, decode_address);
read_params!(TransactionCount, "getTransactionCount", u64, parse);

#[derive(Deserialize)]
struct AddressResult(#[serde(deserialize_with = "super::serde_address::deserialize")] Address);
#[derive(Deserialize)]
struct HashResult(#[serde(deserialize_with = "crate::wire::hash")] Hash);
#[derive(Deserialize)]
struct AddressesResult(#[serde(deserialize_with = "super::serde_address::list")] Vec<Address>);

fn decode_hash(json: &str) -> Result<Hash, Error> {
    parse::<HashResult>(json).map(|value| value.0)
}
fn decode_address(json: &str) -> Result<Address, Error> {
    parse::<AddressResult>(json).map(|value| value.0)
}
fn decode_addresses(json: &str) -> Result<Vec<Address>, Error> {
    parse::<AddressesResult>(json).map(|value| value.0)
}

/// Inclusive requested slot range; omitted last slot selects the node's tip.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SlotRange {
    /// Inclusive first slot.
    pub first_slot: u64,
    /// Inclusive last slot, omitted when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_slot: Option<u64>,
}

/// Optional production filter.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlockProductionConfig {
    /// Node confirmation policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Filter to one validator identity.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serde_address::optional"
    )]
    pub identity: Option<Address>,
    /// Inclusive slot range, or current epoch when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<SlotRange>,
}

/// getBlockProduction optional constraints.
#[derive(Debug, Clone, Copy)]
pub struct BlockProduction(pub Option<BlockProductionConfig>);
impl Serialize for BlockProduction {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.as_slice().serialize(s)
    }
}
method!(
    BlockProduction,
    "getBlockProduction",
    Contextual<BlockProductionValue>,
    parse
);

/// Optional validator filter for the leader schedule.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LeaderScheduleConfig {
    /// Node confirmation policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Filter to one validator identity.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "super::serde_address::optional"
    )]
    pub identity: Option<Address>,
}

/// getLeaderSchedule; None slot is the explicit positional JSON null.
#[derive(Debug, Clone, Copy)]
pub struct LeaderSchedule(pub Option<u64>, pub Option<LeaderScheduleConfig>);
impl Serialize for LeaderSchedule {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(Some(1 + usize::from(self.1.is_some())))?;
        seq.serialize_element(&self.0)?;

        if let Some(config) = &self.1 {
            seq.serialize_element(config)?;
        }

        seq.end()
    }
}
method!(
    LeaderSchedule,
    "getLeaderSchedule",
    Option<LeaderScheduleValue>,
    parse
);

/// getRecentPerformanceSamples optional limit (0..=720).
#[derive(Debug, Clone, Copy)]
pub struct RecentPerformanceSamples(pub Option<u16>);
impl Serialize for RecentPerformanceSamples {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if self.0.is_some_and(|limit| limit > 720) {
            return Err(serde::ser::Error::custom(
                "performance sample limit exceeds 720",
            ));
        }

        self.0.as_slice().serialize(s)
    }
}
method!(
    RecentPerformanceSamples,
    "getRecentPerformanceSamples",
    Vec<PerformanceSample>,
    parse
);

/// getSlotLeaders exact start slot and number of leaders (1..=5000).
#[derive(Debug, Clone, Copy)]
pub struct SlotLeaders(pub u64, pub u16);
impl Serialize for SlotLeaders {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if !(1..=5000).contains(&self.1) {
            return Err(serde::ser::Error::custom(
                "leader limit must be between 1 and 5000",
            ));
        }

        (self.0, self.1).serialize(s)
    }
}
method!(
    SlotLeaders,
    "getSlotLeaders",
    Vec<Address>,
    decode_addresses
);
