//! Typed standard operations and an extension contract for provider methods.

use crate::{Error, Presence};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize, Serializer};
use solana_kit_core::{Address, accounts::MaybeEncodedAccount};

/// Economics, inflation and validator voting operations.
pub mod economics;

/// A typed operation whose params and result schema stay owned by its author.
///
/// The client validates HTTP status, JSON syntax and envelope, request identity,
/// byte/depth budgets and remote errors before calling `decode`. Extensions must
/// validate their complete method schema here; they cannot override framing.
pub trait Method: Serialize {
    /// Owned result exposed to the caller.
    type Response;
    /// Exact JSON-RPC method name.
    const NAME: &'static str;
    /// Parses the already bounded, valid JSON success fragment into typed values.
    fn decode(&self, result: &str) -> Result<Self::Response, Error>;
}

/// Requested confirmation level.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Commitment {
    /// The node has processed the value.
    Processed,
    /// The cluster has confirmed the value.
    Confirmed,
    /// The cluster has finalized the value.
    Finalized,
}

/// Shared optional read constraints; absent values are omitted from JSON.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReadConfig {
    /// Required confirmation level, or the node default when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Minimum acceptable evaluation slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
}

/// Evaluation context; missing API version is preserved and null is rejected.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    /// Exact node-reported slot.
    pub slot: u64,
    /// Optional node API version; explicit null is not a version.
    #[serde(default)]
    pub api_version: Presence<String>,
}

/// A typed value together with the context in which it was evaluated.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Contextual<T> {
    /// Node evaluation context.
    pub context: Context,
    /// Method-specific value.
    pub value: T,
}

/// getBalance address and optional read constraints.
#[derive(Debug)]
pub struct Balance(pub Address, pub Option<ReadConfig>);

impl Serialize for Balance {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(1 + usize::from(self.1.is_some())))?;
        seq.serialize_element(&self.0.to_string())?;

        if let Some(config) = &self.1 {
            seq.serialize_element(config)?;
        }

        seq.end()
    }
}

impl Method for Balance {
    type Response = Contextual<u64>;
    const NAME: &'static str = "getBalance";
    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        serde_json::from_str(result).map_err(Error::Schema)
    }
}

/// getLatestBlockhash optional read constraints.
#[derive(Debug)]
pub struct LatestBlockhash(pub Option<ReadConfig>);

impl Serialize for LatestBlockhash {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(usize::from(self.0.is_some())))?;

        if let Some(config) = &self.0 {
            seq.serialize_element(config)?;
        }

        seq.end()
    }
}

impl Method for LatestBlockhash {
    type Response = Contextual<BlockhashValue>;
    const NAME: &'static str = "getLatestBlockhash";
    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        serde_json::from_str(result).map_err(Error::Schema)
    }
}

/// A validated owned blockhash and exact block-height expiry bound.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlockhashValue {
    /// Recently accepted blockhash.
    #[serde(deserialize_with = "crate::wire::hash")]
    pub blockhash: solana_kit_core::Hash,
    /// Last block height where this blockhash remains valid.
    pub last_valid_block_height: u64,
}

/// getAccountInfo requesting the complete account in canonical base64.
///
/// Data slices, zstd and jsonParsed variants require separate typed operations;
/// this request never asks for them and rejects them in a response.
#[derive(Debug)]
pub struct AccountInfo(pub Address, pub ReadConfig);

impl Serialize for AccountInfo {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Config<'a> {
            encoding: &'static str,
            #[serde(flatten)]
            read: &'a ReadConfig,
        }
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.0.to_string())?;
        seq.serialize_element(&Config {
            encoding: "base64",
            read: &self.1,
        })?;
        seq.end()
    }
}

impl Method for AccountInfo {
    type Response = Contextual<MaybeEncodedAccount>;
    const NAME: &'static str = "getAccountInfo";
    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        crate::wire::account(self.0, result)
    }
}

/// Typed node, epoch, health and leader operations.
pub mod node;
