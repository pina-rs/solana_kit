//! Historical reads with explicit commitment and pagination.
use super::{BlockCommitmentValue, SignatureHistory, parse};
use crate::{Error, methods::Method};
use serde::{Serialize, Serializer, ser::SerializeSeq};
use solana_kit_core::{Address, Signature};

/// Commitment accepted by historical RPC operations.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HistoricalCommitment {
    /// Confirmed by the cluster.
    Confirmed,
    /// Finalized by the cluster.
    Finalized,
}

/// Optional block-range commitment; omission preserves the node default.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct BlocksConfig {
    /// Historical reads do not accept processed commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<HistoricalCommitment>,
}

macro_rules! scalar {
    ($name:ident, $rpc:literal, $response:ty) => {
        #[doc = concat!("Typed `", $rpc, "` read for one exact slot.")]
        #[derive(Debug, Clone, Copy)]
        pub struct $name(#[doc = "Requested slot."] pub u64);
        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                [self.0].serialize(s)
            }
        }
        impl Method for $name {
            type Response = $response;
            const NAME: &'static str = $rpc;
            fn decode(&self, json: &str) -> Result<Self::Response, Error> {
                parse(json)
            }
        }
    };
}
macro_rules! empty {
    ($name:ident, $rpc:literal) => {
        #[doc = concat!("Typed `", $rpc, "` read with no parameters.")]
        #[derive(Debug, Clone, Copy)]
        pub struct $name;
        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                <[u8; 0]>::default().serialize(s)
            }
        }
        impl Method for $name {
            type Response = u64;
            const NAME: &'static str = $rpc;
            fn decode(&self, json: &str) -> Result<Self::Response, Error> {
                parse(json)
            }
        }
    };
}
scalar!(BlockTime, "getBlockTime", Option<i64>);
scalar!(BlockCommitment, "getBlockCommitment", BlockCommitmentValue);
empty!(FirstAvailableBlock, "getFirstAvailableBlock");
empty!(MinimumLedgerSlot, "minimumLedgerSlot");

/// Inclusive block range; an omitted end permits a config in the second slot.
#[derive(Debug, Clone, Copy)]
pub struct Blocks {
    /// Inclusive lower slot.
    pub start: u64,
    /// Inclusive upper slot, or the node's latest slot.
    pub end: Option<u64>,
    /// Optional historical commitment.
    pub config: Option<BlocksConfig>,
}
impl Serialize for Blocks {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if self.end.is_some_and(|end| end < self.start) {
            return Err(serde::ser::Error::custom("block range ends before start"));
        }

        if self.end.is_some_and(|end| end - self.start > 500_000) {
            return Err(serde::ser::Error::custom(
                "block range exceeds 500000 slots",
            ));
        }

        let mut seq = s.serialize_seq(None)?;
        seq.serialize_element(&self.start)?;

        if let Some(end) = self.end {
            seq.serialize_element(&end)?;
        }

        if let Some(config) = self.config {
            seq.serialize_element(&config)?;
        }

        seq.end()
    }
}
impl Method for Blocks {
    type Response = Vec<u64>;
    const NAME: &'static str = "getBlocks";
    fn decode(&self, json: &str) -> Result<Self::Response, Error> {
        let slots: Vec<u64> = parse(json)?;
        validate_slots(&slots, self.start, self.end)?;
        Ok(slots)
    }
}

/// Returns at most the requested number of produced block slots.
#[derive(Debug, Clone, Copy)]
pub struct BlocksWithLimit {
    /// Inclusive lower slot.
    pub start: u64,
    /// Maximum produced blocks, at most 500,000.
    pub limit: u32,
    /// Optional historical commitment.
    pub config: Option<BlocksConfig>,
}
impl Serialize for BlocksWithLimit {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if self.limit > 500_000 {
            return Err(serde::ser::Error::custom("block limit exceeds 500000"));
        }

        let mut seq = s.serialize_seq(None)?;
        seq.serialize_element(&self.start)?;
        seq.serialize_element(&self.limit)?;

        if let Some(config) = self.config {
            seq.serialize_element(&config)?;
        }

        seq.end()
    }
}
impl Method for BlocksWithLimit {
    type Response = Vec<u64>;
    const NAME: &'static str = "getBlocksWithLimit";
    fn decode(&self, json: &str) -> Result<Self::Response, Error> {
        let slots: Vec<u64> = parse(json)?;
        validate_slots(&slots, self.start, None)?;

        if slots.len() > self.limit as usize {
            return Err(schema("block result exceeds requested limit"));
        }

        Ok(slots)
    }
}
fn validate_slots(slots: &[u64], start: u64, end: Option<u64>) -> Result<(), Error> {
    if slots
        .iter()
        .any(|slot| *slot < start || end.is_some_and(|end| *slot > end))
        || slots.windows(2).any(|pair| pair[0] >= pair[1])
    {
        return Err(schema(
            "block slots must be increasing and within requested range",
        ));
    }

    Ok(())
}
pub(crate) fn schema(message: &str) -> Error {
    Error::Schema(<serde_json::Error as serde::de::Error>::custom(message))
}

/// Signature pagination; before and until are exclusive signature boundaries.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SignaturesConfig {
    /// Start before this signature.
    pub before: Option<Signature>,
    /// Stop before this older boundary.
    pub until: Option<Signature>,
    /// Maximum entries, between 1 and 1000.
    pub limit: Option<u16>,
    /// Required historical commitment.
    pub commitment: Option<HistoricalCommitment>,
    /// Required evaluation slot.
    pub min_context_slot: Option<u64>,
}
impl Serialize for SignaturesConfig {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Wire {
            #[serde(skip_serializing_if = "Option::is_none")]
            before: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            until: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            limit: Option<u16>,
            #[serde(skip_serializing_if = "Option::is_none")]
            commitment: Option<HistoricalCommitment>,
            #[serde(skip_serializing_if = "Option::is_none")]
            min_context_slot: Option<u64>,
        }

        if self.limit.is_some_and(|limit| !(1..=1000).contains(&limit)) {
            return Err(serde::ser::Error::custom(
                "signature limit must be 1 through 1000",
            ));
        }

        Wire {
            before: self.before.map(|v| v.to_string()),
            until: self.until.map(|v| v.to_string()),
            limit: self.limit,
            commitment: self.commitment,
            min_context_slot: self.min_context_slot,
        }
        .serialize(s)
    }
}

/// Reads newest-first signature history without truncation or automatic paging.
#[derive(Debug, Clone, Copy)]
pub struct SignaturesForAddress(pub Address, pub Option<SignaturesConfig>);
impl Serialize for SignaturesForAddress {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(None)?;
        seq.serialize_element(&self.0.to_string())?;

        if let Some(config) = self.1 {
            seq.serialize_element(&config)?;
        }

        seq.end()
    }
}
impl Method for SignaturesForAddress {
    type Response = Vec<SignatureHistory>;
    const NAME: &'static str = "getSignaturesForAddress";
    fn decode(&self, json: &str) -> Result<Self::Response, Error> {
        let history: Vec<SignatureHistory> = parse(json)?;
        let limit = self.1.and_then(|config| config.limit).unwrap_or(1000);

        if history.len() > usize::from(limit) {
            return Err(schema("signature history exceeds requested limit"));
        }

        Ok(history)
    }
}
