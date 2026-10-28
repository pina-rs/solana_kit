//! JSON map insertion order remains significant for the reference lookup fallback.

use serde::{
    Deserialize, Deserializer,
    de::{MapAccess, Visitor},
};
use std::fmt;

/// One lookup table with addresses in their original on-chain index order.
#[derive(Clone, PartialEq, Eq)]
pub struct LookupTablePayload {
    /// Base58 lookup table account address.
    pub address: String,
    /// Base58 loaded addresses, preserving duplicates and positions.
    pub addresses: Vec<String>,
}

/// A lookup-table map represented in its original JSON key order.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct LookupTables {
    /// Ordered map entries; JSON admission rejects duplicate keys before decoding.
    pub entries: Vec<LookupTablePayload>,
}

impl<'de> Deserialize<'de> for LookupTables {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        decoder.deserialize_map(TablesVisitor)
    }
}

struct TablesVisitor;

impl<'de> Visitor<'de> for TablesVisitor {
    type Value = LookupTables;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a map of lookup table addresses to address arrays")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
        let mut entries = Vec::new();

        while let Some((address, addresses)) = map.next_entry::<String, Vec<String>>()? {
            entries.push(LookupTablePayload { address, addresses });
        }

        Ok(LookupTables { entries })
    }
}

impl fmt::Debug for LookupTablePayload {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("LookupTablePayload { .. }")
    }
}

impl fmt::Debug for LookupTables {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("LookupTables { .. }")
    }
}
