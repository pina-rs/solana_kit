//! Fully typed instruction routing; program-defined parsed content stays exact.
use super::{address, addresses};
use crate::Presence;
use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;
use solana_kit_core::Address;

/// A compiled instruction with checked integer widths and decoded base58 data.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CompiledInstruction {
    /// Index in the resolved account list.
    pub program_id_index: u8,
    /// Indices in the resolved account list.
    pub accounts: Vec<u8>,
    /// Opaque instruction bytes; no program interpretation is implied.
    #[serde(deserialize_with = "base58")]
    pub data: Vec<u8>,
    /// Invocation depth, absent on older nodes and null when unavailable.
    #[serde(default)]
    pub stack_height: Presence<Option<u32>>,
}
pub(crate) fn base58<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
    bs58::decode(String::deserialize(d)?)
        .into_vec()
        .map_err(serde::de::Error::custom)
}
/// An instruction with resolved addresses but unparsed program data.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PartiallyDecodedInstruction {
    /// Program that executes this instruction.
    #[serde(deserialize_with = "address")]
    pub program_id: Address,
    /// Ordered resolved accounts, including duplicates.
    #[serde(deserialize_with = "addresses")]
    pub accounts: Vec<Address>,
    /// Opaque base58-decoded program input.
    #[serde(deserialize_with = "base58")]
    pub data: Vec<u8>,
    /// Invocation depth.
    #[serde(default)]
    pub stack_height: Presence<Option<u32>>,
}
/// Program-specific parsed content with typed routing metadata.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParsedInstruction {
    /// Node parser name.
    pub program: String,
    /// Program whose parser produced this data.
    #[serde(deserialize_with = "address")]
    pub program_id: Address,
    /// Exact bounded program-defined JSON, never normalized through f64.
    pub parsed: Box<RawValue>,
    /// Invocation depth.
    #[serde(default)]
    pub stack_height: Presence<Option<u32>>,
}
/// jsonParsed instructions retain an explicit fallback for unknown programs.
#[derive(Debug)]
pub enum ParsedOrPartialInstruction {
    /// A registered program parser succeeded.
    Parsed(ParsedInstruction),
    /// No registered program parser succeeded.
    Partial(PartiallyDecodedInstruction),
}
/// Inner instructions may be compiled or resolved depending on request encoding.
#[derive(Debug)]
pub enum InnerInstruction {
    /// Unparsed index-based form.
    Compiled(CompiledInstruction),
    /// Address-based parsed or fallback form.
    Resolved(ParsedOrPartialInstruction),
}
/// Inner calls grouped by their outer transaction instruction position.
#[derive(Debug, Deserialize)]
pub struct InnerInstructions {
    /// Outer instruction index.
    pub index: u8,
    /// Ordered inner calls.
    pub instructions: Vec<InnerInstruction>,
}

impl<'de> Deserialize<'de> for ParsedOrPartialInstruction {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(d)?;
        let keys: std::collections::BTreeMap<String, Box<RawValue>> =
            serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;

        if keys.contains_key("parsed") {
            return serde_json::from_str(raw.get())
                .map(Self::Parsed)
                .map_err(serde::de::Error::custom);
        }

        serde_json::from_str(raw.get())
            .map(Self::Partial)
            .map_err(serde::de::Error::custom)
    }
}
impl<'de> Deserialize<'de> for InnerInstruction {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(d)?;
        let keys: std::collections::BTreeMap<String, Box<RawValue>> =
            serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;

        if keys.contains_key("programIdIndex") {
            return serde_json::from_str(raw.get())
                .map(Self::Compiled)
                .map_err(serde::de::Error::custom);
        }

        serde_json::from_str(raw.get())
            .map(Self::Resolved)
            .map_err(serde::de::Error::custom)
    }
}
