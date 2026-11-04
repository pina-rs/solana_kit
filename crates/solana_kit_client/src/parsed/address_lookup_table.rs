//! Address lookup table payloads.
//!
//! Source:
//! `packages/solana_kit_rpc_parsed_types/lib/src/address_lookup_table_accounts.dart`.
//! The pinned model exposes the info payload directly, so the node
//! `type: "lookupTable"` discriminator is validated but not repeated in the variant.

use super::{ParsedAccountData, scalars};
use crate::Error;
use serde::Deserialize;
use serde_json::value::RawValue;
use solana_kit_core::Address;

pub(super) fn decode(program: &str, parsed: &RawValue) -> Result<ParsedAccountData, Error> {
    let envelope = super::envelope(parsed)?;

    if envelope.type_name != "lookupTable" {
        return super::unknown_type(program, envelope.type_name, parsed);
    }

    Ok(ParsedAccountData::AddressLookupTable(super::info(
        &envelope,
    )?))
}

/// Reference `JsonParsedAddressLookupTableInfo`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddressLookupTableInfo {
    /// Stored addresses in table order.
    #[serde(deserialize_with = "scalars::addresses")]
    pub addresses: Vec<Address>,
    /// Optional authority allowed to modify the table.
    #[serde(default, deserialize_with = "scalars::optional_address")]
    pub authority: Option<Address>,
    /// Deactivation slot reported as decimal text.
    #[serde(deserialize_with = "scalars::unsigned_decimal")]
    pub deactivation_slot: u64,
    /// Last extended slot reported as decimal text.
    #[serde(deserialize_with = "scalars::unsigned_decimal")]
    pub last_extended_slot: u64,
    /// Start index of the last extension.
    pub last_extended_slot_start_index: u8,
}
