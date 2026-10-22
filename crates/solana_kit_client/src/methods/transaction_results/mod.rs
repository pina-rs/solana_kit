// SPDX-License-Identifier: Apache-2.0
// Field layouts adapted from Agave c9c6f3287e26f24e3476e13e751aebe710191e89.
// See APACHE-2.0; request/validation logic is newly authored.

//! Owned RPC transaction views. Wire bytes are not verified transactions.
mod accounts;
mod instructions;
mod metadata;
mod requests;
mod transaction;
pub use accounts::*;
pub use instructions::*;
pub use metadata::*;
pub use requests::*;
pub use transaction::*;

use crate::Error;
use serde::{Deserialize, Deserializer};
use solana_kit_core::{Address, Signature};

pub(crate) fn address<'de, D: Deserializer<'de>>(d: D) -> Result<Address, D::Error> {
    String::deserialize(d)?
        .parse()
        .map_err(serde::de::Error::custom)
}
pub(crate) fn addresses<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<Address>, D::Error> {
    Vec::<String>::deserialize(d)?
        .into_iter()
        .map(|s| s.parse().map_err(serde::de::Error::custom))
        .collect()
}
pub(crate) fn signatures<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<Signature>, D::Error> {
    Vec::<String>::deserialize(d)?
        .into_iter()
        .map(|s| s.parse().map_err(serde::de::Error::custom))
        .collect()
}
pub(crate) fn parse<T: serde::de::DeserializeOwned>(raw: &str) -> Result<T, Error> {
    serde_json::from_str(raw).map_err(Error::Schema)
}
pub(crate) fn schema(message: &str) -> Error {
    Error::Schema(<serde_json::Error as serde::de::Error>::custom(message))
}
