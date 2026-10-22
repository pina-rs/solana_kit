//! Typed getTransaction and getBlock requests with fixed owned result graphs.
use super::{
    Reward, TransactionEncoding, TransactionResult, WireTransactionResult, parse, schema,
    signatures,
};
use crate::{
    Error, Presence,
    methods::{Method, history::HistoricalCommitment},
};
use serde::{Deserialize, Serialize, Serializer};
use solana_kit_core::{Hash, Signature};

/// Complete transaction read configuration; omission preserves server defaults.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReadConfig {
    /// Response representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<TransactionEncoding>,
    /// Minimum historical commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<HistoricalCommitment>,
    /// Highest supported numbered version, currently zero or one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_supported_transaction_version: Option<u8>,
}
impl TransactionReadConfig {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self
            .max_supported_transaction_version
            .is_some_and(|v| v > 1)
        {
            return Err(schema("supported transaction version exceeds one"));
        }
        Ok(())
    }
}
/// Reads one signature without inventing confirmation or lifetime state.
#[derive(Debug)]
pub struct Transaction(pub Signature, pub Option<TransactionReadConfig>);
impl Serialize for Transaction {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = s.serialize_seq(None)?;
        seq.serialize_element(&self.0.to_string())?;

        if let Some(config) = self.1 {
            config.validate().map_err(serde::ser::Error::custom)?;
            seq.serialize_element(&config)?;
        }

        seq.end()
    }
}
/// Historical transaction identity and its typed execution view.
#[derive(Debug)]
pub struct ConfirmedTransaction {
    /// Slot containing this transaction.
    pub slot: u64,
    /// Estimated Unix timestamp; explicit null when unavailable.
    pub block_time: Option<i64>,
    /// Versioned position annotation supplied by newer nodes.
    pub transaction_index: Presence<u32>,
    /// Transaction and execution metadata.
    pub details: TransactionResult,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WireConfirmed {
    slot: u64,
    #[serde(deserialize_with = "crate::json::required_nullable")]
    block_time: Option<i64>,
    #[serde(default)]
    transaction_index: Presence<u32>,
}
impl Method for Transaction {
    type Response = Option<ConfirmedTransaction>;
    const NAME: &'static str = "getTransaction";
    fn decode(&self, json: &str) -> Result<Self::Response, Error> {
        let config = self.1.unwrap_or_default();
        config.validate()?;
        let wire: Option<WireConfirmed> = parse(json)?;
        wire.map(|w| {
            Ok(ConfirmedTransaction {
                slot: w.slot,
                block_time: w.block_time,
                transaction_index: w.transaction_index,
                details: parse::<WireTransactionResult>(json)?.decode(
                    config.encoding.unwrap_or_default(),
                    config.max_supported_transaction_version,
                    false,
                    super::VersionRequirement::AllowHistoricalMissingMetadata,
                )?,
            })
        })
        .transpose()
    }
}
/// Level of transaction detail requested for a block.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionDetails {
    /// Full encoded transactions and execution metadata.
    #[default]
    Full,
    /// Signatures only.
    Signatures,
    /// No transaction data.
    None,
    /// Signatures, annotated accounts and execution metadata.
    Accounts,
}
/// Block configuration includes all standard response-shaping options.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlockConfig {
    /// Transaction encoding, irrelevant for accounts/signatures/none modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<TransactionEncoding>,
    /// Historical commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<HistoricalCommitment>,
    /// Highest numbered transaction version accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_supported_transaction_version: Option<u8>,
    /// Include block rewards; server defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards: Option<bool>,
    /// Transaction detail mode; server defaults to full.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_details: Option<TransactionDetails>,
}
/// Reads the requested block, returning null when unavailable.
#[derive(Debug)]
pub struct Block(pub u64, pub Option<BlockConfig>);
impl Serialize for Block {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = s.serialize_seq(None)?;
        seq.serialize_element(&self.0)?;

        if let Some(config) = self.1 {
            if config
                .max_supported_transaction_version
                .is_some_and(|v| v > 1)
            {
                return Err(serde::ser::Error::custom("supported version exceeds one"));
            }
            seq.serialize_element(&config)?;
        }

        seq.end()
    }
}
/// Explicit transaction detail variant corresponding to the request.
#[derive(Debug)]
pub enum BlockTransactions {
    /// Entire transactions in requested encoding.
    Full(Vec<TransactionResult>),
    /// Signatures without transaction data.
    Signatures(Vec<Signature>),
    /// Intentionally omitted transaction details.
    None,
    /// Annotated account-only transaction views.
    Accounts(Vec<TransactionResult>),
}
/// Complete block identity and requested transaction/reward information.
#[derive(Debug)]
pub struct BlockValue {
    /// Block hash.
    pub blockhash: Hash,
    /// Parent block hash.
    pub previous_blockhash: Hash,
    /// Parent slot.
    pub parent_slot: u64,
    /// Estimated production time, null when unavailable.
    pub block_time: Option<i64>,
    /// Block height, null when unavailable.
    pub block_height: Option<u64>,
    /// Request-selected transaction detail variant.
    pub transactions: BlockTransactions,
    /// Rewards are absent when excluded by request.
    pub rewards: Presence<Vec<Reward>>,
    /// Versioned partition count annotation.
    pub num_reward_partitions: Presence<Option<u64>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct WireBlock {
    #[serde(deserialize_with = "crate::wire::hash")]
    blockhash: Hash,
    #[serde(deserialize_with = "crate::wire::hash")]
    previous_blockhash: Hash,
    parent_slot: u64,
    #[serde(deserialize_with = "crate::json::required_nullable")]
    block_time: Option<i64>,
    #[serde(deserialize_with = "crate::json::required_nullable")]
    block_height: Option<u64>,
    #[serde(default)]
    transactions: Presence<Vec<WireTransactionResult>>,
    #[serde(default)]
    signatures: Presence<SignatureList>,
    #[serde(default)]
    rewards: Presence<Vec<Reward>>,
    #[serde(default)]
    num_reward_partitions: Presence<Option<u64>>,
}
#[derive(Deserialize)]
struct SignatureList(#[serde(deserialize_with = "signatures")] Vec<Signature>);
impl Method for Block {
    type Response = Option<BlockValue>;
    const NAME: &'static str = "getBlock";
    fn decode(&self, json: &str) -> Result<Self::Response, Error> {
        let config = self.1.unwrap_or_default();
        let Some(wire) = parse::<Option<WireBlock>>(json)? else {
            return Ok(None);
        };
        let mode = config.transaction_details.unwrap_or_default();
        let transactions =
            decode_block_transactions(wire.transactions, wire.signatures, mode, config)?;

        if matches!(wire.rewards, Presence::Present(_)) != config.rewards.unwrap_or(true) {
            return Err(schema("block rewards contradict request"));
        }

        Ok(Some(BlockValue {
            blockhash: wire.blockhash,
            previous_blockhash: wire.previous_blockhash,
            parent_slot: wire.parent_slot,
            block_time: wire.block_time,
            block_height: wire.block_height,
            transactions,
            rewards: wire.rewards,
            num_reward_partitions: wire.num_reward_partitions,
        }))
    }
}
fn decode_block_transactions(
    transactions: Presence<Vec<WireTransactionResult>>,
    signatures: Presence<SignatureList>,
    mode: TransactionDetails,
    config: BlockConfig,
) -> Result<BlockTransactions, Error> {
    match (mode, transactions, signatures) {
        (TransactionDetails::None, Presence::Missing, Presence::Missing) => {
            Ok(BlockTransactions::None)
        }
        (TransactionDetails::Signatures, Presence::Missing, Presence::Present(v)) => {
            Ok(BlockTransactions::Signatures(v.0))
        }
        (
            TransactionDetails::Full | TransactionDetails::Accounts,
            Presence::Present(v),
            Presence::Missing,
        ) => {
            let accounts = mode == TransactionDetails::Accounts;
            let values = v
                .into_iter()
                .map(|w| {
                    w.decode(
                        config.encoding.unwrap_or_default(),
                        config.max_supported_transaction_version,
                        accounts,
                        super::VersionRequirement::AllowHistoricalMissingMetadata,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(wrap_block_transactions(accounts, values))
        }
        _ => Err(schema("block transaction details contradict request")),
    }
}

fn wrap_block_transactions(accounts: bool, values: Vec<TransactionResult>) -> BlockTransactions {
    if accounts {
        return BlockTransactions::Accounts(values);
    }

    BlockTransactions::Full(values)
}
