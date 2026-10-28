//! Raw build payloads preserve optional fields until explicit Kit conversion.

use crate::{Integer, LookupTables, object};
use serde::Deserialize;

/// Account privileges in a raw Jupiter instruction.
#[derive(Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountMetaPayload {
    /// Base58 account address.
    pub pubkey: Option<String>,
    /// Whether this account must sign.
    pub is_signer: Option<bool>,
    /// Whether this account can be written.
    pub is_writable: Option<bool>,
}

/// One raw instruction from the build route.
///
/// The provider may omit fields. [`Self::to_instruction`] requires a complete
/// payload and validates addresses and base64 without assigning missing roles.
#[derive(Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionPayload {
    /// Base58 program address.
    pub program_id: Option<String>,
    /// Positional accounts, including any duplicates.
    #[serde(default, deserialize_with = "object::array")]
    pub accounts: Option<Vec<AccountMetaPayload>>,
    /// Base64 instruction data.
    pub data: Option<String>,
}

/// Raw self-landed instructions from `GET /swap/v2/build`.
#[derive(Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildResponse {
    /// Compute budget instructions in provider order.
    #[serde(default, deserialize_with = "object::array")]
    pub compute_budget_instructions: Option<Vec<InstructionPayload>>,
    /// Setup instructions in provider order.
    #[serde(default, deserialize_with = "object::array")]
    pub setup_instructions: Option<Vec<InstructionPayload>>,
    /// Swap instruction, when available.
    #[serde(default, deserialize_with = "object::optional")]
    pub swap_instruction: Option<InstructionPayload>,
    /// Optional cleanup instruction.
    #[serde(default, deserialize_with = "object::optional")]
    pub cleanup_instruction: Option<InstructionPayload>,
    /// Additional route instructions in provider order.
    #[serde(default, deserialize_with = "object::array")]
    pub other_instructions: Option<Vec<InstructionPayload>>,
    /// Optional tip instruction.
    #[serde(default, deserialize_with = "object::optional")]
    pub tip_instruction: Option<InstructionPayload>,
    /// Lookup table addresses, falling back to map key order in [`crate::decode_build`].
    pub address_lookup_table_addresses: Option<Vec<String>>,
    /// Resolved table entries in response key order, with on-chain address index order.
    pub addresses_by_lookup_table_address: Option<LookupTables>,
    /// Recent blockhash and expiry metadata.
    #[serde(default, deserialize_with = "object::optional")]
    pub blockhash_with_metadata: Option<BlockhashWithMetadata>,
    /// Original JSON bytes, including unknown fields; empty for direct construction.
    #[serde(skip)]
    pub raw_response: Vec<u8>,
}

/// Required recent blockhash bytes and exact expiry inside present build metadata.
#[derive(Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockhashWithMetadata {
    /// Raw blockhash bytes; conversion requires exactly thirty-two bytes.
    pub blockhash: Vec<u8>,
    /// Exact last valid block height, accepting integer or string wire values.
    pub last_valid_block_height: Integer,
}

impl std::fmt::Debug for AccountMetaPayload {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("AccountMetaPayload { .. }")
    }
}

impl std::fmt::Debug for InstructionPayload {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("InstructionPayload { .. }")
    }
}

impl std::fmt::Debug for BuildResponse {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("BuildResponse { .. }")
    }
}

impl std::fmt::Debug for BlockhashWithMetadata {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("BlockhashWithMetadata { .. }")
    }
}
