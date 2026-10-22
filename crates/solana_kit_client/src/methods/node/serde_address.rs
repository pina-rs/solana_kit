//! JSON strings become validated Kit addresses at the response boundary.
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use solana_kit_core::Address;
use std::collections::BTreeMap;

pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Address, D::Error> {
    String::deserialize(d)?
        .parse()
        .map_err(serde::de::Error::custom)
}

pub fn optional<S: Serializer>(address: &Option<Address>, s: S) -> Result<S::Ok, S::Error> {
    address.map(|a| a.to_string()).serialize(s)
}

pub fn map<'de, D, T>(d: D) -> Result<BTreeMap<Address, T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let strings = BTreeMap::<String, T>::deserialize(d)?;
    strings
        .into_iter()
        .map(|(key, value)| {
            key.parse()
                .map(|key| (key, value))
                .map_err(serde::de::Error::custom)
        })
        .collect()
}

pub fn list<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<Address>, D::Error> {
    Vec::<String>::deserialize(d)?
        .into_iter()
        .map(|key| key.parse().map_err(serde::de::Error::custom))
        .collect()
}
