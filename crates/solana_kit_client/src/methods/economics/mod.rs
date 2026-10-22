//! Typed economics RPC for off-chain application code, independent of an executor.

mod accounts;
mod inflation;

pub use accounts::*;
pub use inflation::*;

use super::{Commitment, Contextual, Method, ReadConfig};
use crate::Error;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize, Serializer};
use solana_kit_core::Address;

/// Optional confirmation constraint for methods without a minimum-slot option.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct CommitmentConfig {
    /// Node default when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
}

fn optional_config<S: Serializer, C: Serialize>(
    config: &Option<C>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut seq = serializer.serialize_seq(Some(usize::from(config.is_some())))?;

    if let Some(config) = config {
        seq.serialize_element(config)?;
    }

    seq.end()
}

macro_rules! config_method {
    ($name:ident, $config:ty, $result:ty, $rpc:literal) => {
        #[doc = concat!($rpc, " with optional configuration; None omits the config argument.")]
        #[derive(Debug)]
        pub struct $name(pub Option<$config>);

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                optional_config(&self.0, serializer)
            }
        }

        impl Method for $name {
            type Response = $result;
            const NAME: &'static str = $rpc;
            fn decode(&self, result: &str) -> Result<Self::Response, Error> {
                serde_json::from_str(result).map_err(Error::Schema)
            }
        }
    };
}

pub(super) use config_method;

fn address<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Address, D::Error> {
    String::deserialize(d)?
        .parse()
        .map_err(serde::de::Error::custom)
}

fn addresses<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Vec<Address>, D::Error> {
    Vec::<String>::deserialize(d)?
        .into_iter()
        .map(|text| text.parse().map_err(serde::de::Error::custom))
        .collect()
}

fn optional_address<S: Serializer>(
    address: &Option<Address>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    address.map(|value| value.to_string()).serialize(serializer)
}

config_method!(
    StakeMinimumDelegation,
    ReadConfig,
    Contextual<u64>,
    "getStakeMinimumDelegation"
);
