//! Exact method names and typed request-selected notification decoding.
use super::{SubscriptionMethod, responses::*, sealed};
use crate::{
    Error,
    methods::{self, Commitment, Contextual, Method, accounts, transaction_results},
};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};
use solana_kit_core::{Address, Signature, accounts::MaybeAccount};

/// Account notification options. Omission preserves node defaults.
#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct AccountConfig {
    /// Requested commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Account representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<accounts::AccountEncoding>,
}
impl AccountConfig {
    fn rpc(self) -> accounts::AccountConfig {
        accounts::AccountConfig {
            read: methods::ReadConfig {
                commitment: self.commitment,
                min_context_slot: None,
            },
            encoding: self.encoding,
            data_slice: None,
        }
    }
}
/// Account changes at one address.
#[derive(Debug)]
pub struct Account(pub Address, pub Option<AccountConfig>);
impl Serialize for Account {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        accounts::requests::params(s, &self.0.to_string(), &self.1)
    }
}
impl Method for Account {
    type Response = Contextual<MaybeAccount<accounts::AccountData>>;
    const NAME: &'static str = "accountSubscribe";
    fn decode(&self, raw: &str) -> Result<Self::Response, Error> {
        accounts::AccountInfoWithConfig(self.0, self.1.map(AccountConfig::rpc)).decode(raw)
    }
}
/// Program account notification options.
#[derive(Debug, Default, Serialize)]
pub struct ProgramConfig {
    /// Representation and commitment.
    #[serde(flatten)]
    pub account: AccountConfig,
    /// At most four conjunctive filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<accounts::Filter>>,
}
/// Account changes owned by one program.
#[derive(Debug)]
pub struct Program(pub Address, pub Option<ProgramConfig>);
impl Serialize for Program {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if self
            .1
            .as_ref()
            .and_then(|c| c.filters.as_ref())
            .is_some_and(|f| f.len() > 4)
        {
            return Err(serde::ser::Error::custom("at most four program filters"));
        }

        accounts::requests::params(s, &self.0.to_string(), &self.1)
    }
}
impl Method for Program {
    type Response = Contextual<solana_kit_core::accounts::Account<accounts::AccountData>>;
    const NAME: &'static str = "programSubscribe";
    fn decode(&self, raw: &str) -> Result<Self::Response, Error> {
        let wire: Contextual<Box<serde_json::value::RawValue>> =
            serde_json::from_str(raw).map_err(Error::Schema)?;
        let config = self
            .1
            .as_ref()
            .map_or_else(AccountConfig::default, |c| c.account)
            .rpc();
        let mut accounts =
            accounts::requests::keyed(&format!("[{}]", wire.value.get()), config, Some(self.0))?;
        Ok(Contextual {
            context: wire.context,
            value: accounts.remove(0),
        })
    }
}
/// Exact logs subscription filter.
#[derive(Debug)]
pub enum LogsFilter {
    /// All transactions except simple votes.
    All,
    /// Include vote transactions.
    AllWithVotes,
    /// Transactions mentioning this single address.
    Mentions(Address),
}
impl Serialize for LogsFilter {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => s.serialize_str("all"),
            Self::AllWithVotes => s.serialize_str("allWithVotes"),
            Self::Mentions(address) => {
                #[derive(Serialize)]
                struct Mentions {
                    mentions: [String; 1],
                }
                Mentions {
                    mentions: [address.to_string()],
                }
                .serialize(s)
            }
        }
    }
}
/// Transaction logs with an optional commitment.
#[derive(Debug)]
pub struct Logs(pub LogsFilter, pub Option<accounts::CommitmentConfig>);
impl Serialize for Logs {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(None)?;
        seq.serialize_element(&self.0)?;

        if let Some(config) = &self.1 {
            seq.serialize_element(config)?;
        }

        seq.end()
    }
}
impl Method for Logs {
    type Response = Contextual<LogsInfo>;
    const NAME: &'static str = "logsSubscribe";
    fn decode(&self, raw: &str) -> Result<Self::Response, Error> {
        serde_json::from_str(raw).map_err(Error::Schema)
    }
}
/// Signature subscription options.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureConfig {
    /// Requested commitment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Receive a preliminary receipt before execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_received_notification: Option<bool>,
}
/// Transaction signature progress.
#[derive(Debug)]
pub struct SignatureNotifications(pub Signature, pub Option<SignatureConfig>);
impl Serialize for SignatureNotifications {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        accounts::requests::params(s, &self.0.to_string(), &self.1)
    }
}
impl Method for SignatureNotifications {
    type Response = Contextual<SignatureInfo>;
    const NAME: &'static str = "signatureSubscribe";
    fn decode(&self, raw: &str) -> Result<Self::Response, Error> {
        let result: Self::Response = serde_json::from_str(raw).map_err(Error::Schema)?;

        if matches!(result.value, SignatureInfo::Received(_))
            && self
                .1
                .as_ref()
                .and_then(|config| config.enable_received_notification)
                != Some(true)
        {
            return Err(transaction_results::schema("unrequested signature receipt"));
        }

        Ok(result)
    }
}
/// Unstable block subscription filter.
#[derive(Debug)]
pub enum BlockFilter {
    /// Every block.
    All,
    /// Blocks mentioning one address.
    Mentions(Address),
}
impl Serialize for BlockFilter {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => s.serialize_str("all"),
            Self::Mentions(a) => {
                #[derive(Serialize)]
                #[serde(rename_all = "camelCase")]
                struct Filter {
                    mentions_account_or_program: String,
                }
                Filter {
                    mentions_account_or_program: a.to_string(),
                }
                .serialize(s)
            }
        }
    }
}
/// Unstable block notification options reuse the historical RPC configuration.
/// `rewards` is serialized as the subscription-specific `showRewards` field.
#[derive(Debug, Default)]
pub struct BlockConfig(pub transaction_results::BlockConfig);
impl Serialize for BlockConfig {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut value = serde_json::to_value(self.0).map_err(serde::ser::Error::custom)?;
        let fields = value
            .as_object_mut()
            .ok_or_else(|| serde::ser::Error::custom("block config"))?;

        if let Some(rewards) = fields.remove("rewards") {
            fields.insert("showRewards".into(), rewards);
        }

        if self
            .0
            .max_supported_transaction_version
            .is_some_and(|v| v > 1)
        {
            return Err(serde::ser::Error::custom("supported version exceeds one"));
        }
        value.serialize(s)
    }
}
/// Unstable block changes. Server support must be enabled explicitly.
#[derive(Debug)]
pub struct Block(pub BlockFilter, pub Option<BlockConfig>);
impl Serialize for Block {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(None)?;
        seq.serialize_element(&self.0)?;

        if let Some(config) = &self.1 {
            seq.serialize_element(config)?;
        }

        seq.end()
    }
}
/// Block notification payload with an explicit unavailable-block result.
#[derive(Debug)]
pub struct BlockInfo {
    /// Slot reported by the node.
    pub slot: u64,
    /// Shared typed RPC block view, absent when unavailable.
    pub block: Option<transaction_results::BlockValue>,
    /// Node block update error, when present.
    pub err: Option<BlockUpdateError>,
}
/// Known unstable block update failure.
#[derive(Debug, Deserialize)]
pub enum BlockUpdateError {
    /// Node failed to construct an update.
    BlockStoreError,
    /// Node cannot encode the transaction version.
    UnsupportedTransactionVersion(u8),
}
impl Method for Block {
    type Response = Contextual<BlockInfo>;
    const NAME: &'static str = "blockSubscribe";
    fn decode(&self, raw: &str) -> Result<Self::Response, Error> {
        #[derive(Deserialize)]
        struct Value {
            slot: u64,
            block: Box<serde_json::value::RawValue>,
            #[serde(deserialize_with = "crate::json::required_nullable")]
            err: Option<BlockUpdateError>,
        }
        let wire: Contextual<Value> = serde_json::from_str(raw).map_err(Error::Schema)?;
        let config = self.1.as_ref().map(|c| c.0);
        let block =
            transaction_results::Block(wire.value.slot, config).decode(wire.value.block.get())?;
        Ok(Contextual {
            context: wire.context,
            value: BlockInfo {
                slot: wire.value.slot,
                block,
                err: wire.value.err,
            },
        })
    }
}
macro_rules! family {
    ($name:ty, $unsubscribe:literal, $notification:literal, $unstable:literal) => {
        impl sealed::Sealed for $name {}
        impl SubscriptionMethod for $name {
            const UNSUBSCRIBE: &'static str = $unsubscribe;
            const NOTIFICATION: &'static str = $notification;
            const UNSTABLE: bool = $unstable;
        }
    };
}
family!(Account, "accountUnsubscribe", "accountNotification", false);
family!(Program, "programUnsubscribe", "programNotification", false);
family!(Logs, "logsUnsubscribe", "logsNotification", false);
impl sealed::Sealed for SignatureNotifications {}
impl SubscriptionMethod for SignatureNotifications {
    const UNSUBSCRIBE: &'static str = "signatureUnsubscribe";
    const NOTIFICATION: &'static str = "signatureNotification";

    fn completes(notification: &Self::Response) -> bool {
        matches!(notification.value, SignatureInfo::Processed { .. })
    }
}
family!(Block, "blockUnsubscribe", "blockNotification", true);
macro_rules! empty {
    ($name:ident, $response:ty, $subscribe:literal, $unsubscribe:literal, $notification:literal, $unstable:literal) => {
        #[doc = concat!("Typed ", $subscribe, " request without parameters.")]
        #[derive(Debug)]
        pub struct $name;
        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_seq(Some(0))?.end()
            }
        }
        impl Method for $name {
            type Response = $response;
            const NAME: &'static str = $subscribe;
            fn decode(&self, raw: &str) -> Result<Self::Response, Error> {
                serde_json::from_str(raw).map_err(Error::Schema)
            }
        }
        family!($name, $unsubscribe, $notification, $unstable);
    };
}
empty!(
    Root,
    u64,
    "rootSubscribe",
    "rootUnsubscribe",
    "rootNotification",
    false
);
empty!(
    Slot,
    SlotInfo,
    "slotSubscribe",
    "slotUnsubscribe",
    "slotNotification",
    false
);
empty!(
    Vote,
    VoteInfo,
    "voteSubscribe",
    "voteUnsubscribe",
    "voteNotification",
    true
);
empty!(
    SlotsUpdates,
    SlotUpdate,
    "slotsUpdatesSubscribe",
    "slotsUpdatesUnsubscribe",
    "slotsUpdatesNotification",
    true
);
