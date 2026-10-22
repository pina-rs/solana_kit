//! Three typed pilot operations. Remaining catalog entries deliberately have no stubs.

use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize, Serializer};
use solana_kit_core::{Address, Hash};

/// A method determines its own response; the caller cannot substitute a schema.
pub trait Method: Serialize {
    type Response: serde::de::DeserializeOwned;
    const NAME: &'static str;
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Commitment {
    Processed,
    Confirmed,
    Finalized,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub slot: u64,
    pub api_version: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Contextual<T> {
    pub context: Context,
    pub value: T,
}

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
}

#[derive(Debug)]
pub struct LatestBlockhash(pub Option<ReadConfig>);

// Newtype serialization must emit a positional array, including [] when absent.
impl Method for LatestBlockhash {
    type Response = Contextual<BlockhashValue>;
    const NAME: &'static str = "getLatestBlockhash";
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlockhashValue {
    #[serde(deserialize_with = "hash")]
    pub blockhash: Hash,
    pub last_valid_block_height: u64,
}

fn hash<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Hash, D::Error> {
    String::deserialize(d)?
        .parse()
        .map_err(serde::de::Error::custom)
}

fn address<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Address, D::Error> {
    String::deserialize(d)?
        .parse()
        .map_err(serde::de::Error::custom)
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub executable: bool,
    pub lamports: u64,
    #[serde(deserialize_with = "address")]
    pub owner: Address,
    pub rent_epoch: u64,
    pub space: u64,
    pub data: (String, Base64),
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum Base64 {
    #[serde(rename = "base64")]
    Base64,
}

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
    type Response = NullableContext<Account>;
    const NAME: &'static str = "getAccountInfo";
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]

pub struct NullableContext<T> {
    pub context: Context,
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub value: Option<T>,
}

impl Serialize for LatestBlockhash {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(usize::from(self.0.is_some())))?;
        if let Some(config) = &self.0 {
            seq.serialize_element(config)?;
        }
        seq.end()
    }
}
