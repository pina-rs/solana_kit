//! Initialized nonce account payloads.
//!
//! Source: `packages/solana_kit_rpc_parsed_types/lib/src/nonce_accounts.dart`.
//! The pinned model exposes the info payload directly, so the node
//! `type: "initialized"` discriminator is validated but not repeated in the variant.

use super::{FeeCalculator, ParsedAccountData, scalars};
use crate::Error;
use serde::Deserialize;
use serde_json::value::RawValue;
use solana_kit_core::{Address, Hash};

pub(super) fn decode(program: &str, parsed: &RawValue) -> Result<ParsedAccountData, Error> {
    let envelope = super::envelope(parsed)?;

    if envelope.type_name != "initialized" {
        return super::unknown_type(program, envelope.type_name, parsed);
    }

    Ok(ParsedAccountData::Nonce(super::info(&envelope)?))
}

/// Reference `JsonParsedNonceInfo`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NonceInfo {
    /// Authority allowed to advance the nonce.
    #[serde(deserialize_with = "scalars::required_address")]
    pub authority: Address,
    /// Stored durable nonce blockhash.
    #[serde(deserialize_with = "crate::wire::hash")]
    pub blockhash: Hash,
    /// Fee rate recorded with the nonce.
    pub fee_calculator: FeeCalculator,
}
