//! Exact execution metadata, distinct from RPC transport success.
use super::{InnerInstructions, LoadedAddresses, address};
use crate::{
    Presence,
    methods::submission::{ExecutionStatus, TransactionFailure},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Deserializer};
use solana_kit_core::Address;

/// Reward category reported by the node.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RewardType {
    /// Transaction fees.
    Fee,
    /// Rent distribution.
    Rent,
    /// Staking yield.
    Staking,
    /// Vote participation.
    Voting,
}
/// Signed reward change and exact resulting account balance.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    /// Reward recipient.
    #[serde(deserialize_with = "address")]
    pub pubkey: Address,
    /// Signed lamport change.
    pub lamports: i64,
    /// Balance after applying the change.
    pub post_balance: u64,
    /// Null when reward category is unavailable.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub reward_type: Option<RewardType>,
    /// Vote commission as a whole percentage.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub commission: Option<u8>,
    /// SIMD-0291 commission in basis points, absent on older nodes.
    #[serde(default)]
    pub commission_bps: Presence<Option<u16>>,
}
/// Exact token quantity; UI numbers remain decimals, never monetary arithmetic.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAmount {
    /// Canonical unsigned decimal units.
    #[serde(deserialize_with = "amount")]
    pub amount: String,
    /// Decimal scale.
    pub decimals: u8,
    /// Node-supplied approximate display number retained without f64 conversion.
    #[serde(deserialize_with = "required_nullable_number")]
    pub ui_amount: Option<serde_json::Number>,
    /// Exact display spelling supplied by the node.
    pub ui_amount_string: String,
}
pub(crate) fn required_nullable_number<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<Option<serde_json::Number>, D::Error> {
    let raw = Box::<serde_json::value::RawValue>::deserialize(d)?;
    let text = raw.get();

    if text == "null" {
        return Ok(None);
    }

    // Number's arbitrary-precision visitor also accepts its private object shape.
    // Check the actual JSON token before retaining the exact numeric spelling.
    if !matches!(text.as_bytes().first(), Some(b'-' | b'0'..=b'9')) {
        return Err(serde::de::Error::custom(
            "expected a numeric scalar or null",
        ));
    }

    serde_json::from_str(text).map_err(serde::de::Error::custom)
}
fn amount<'de, D: Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    let text = String::deserialize(d)?;
    let valid = text == "0" || (!text.starts_with('0') && text.bytes().all(|c| c.is_ascii_digit()));

    if text.is_empty() || !valid || text.parse::<u64>().is_err() {
        return Err(serde::de::Error::custom("invalid u64 token amount"));
    }

    Ok(text)
}
/// Token balance for one resolved transaction account.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    /// Position in the resolved account list.
    pub account_index: u8,
    /// Mint address.
    #[serde(deserialize_with = "address")]
    pub mint: Address,
    /// Exact raw units and display annotations.
    pub ui_token_amount: TokenAmount,
    /// Optional versioned owner annotation.
    #[serde(default)]
    pub owner: Presence<Option<AnnotatedAddress>>,
    /// Optional token program annotation.
    #[serde(default)]
    pub program_id: Presence<Option<AnnotatedAddress>>,
}
/// Validated address inside an optional metadata annotation.
#[derive(Debug, Deserialize)]
pub struct AnnotatedAddress(#[serde(deserialize_with = "address")] pub Address);
/// Last program return value emitted during transaction execution.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReturnData {
    /// Program that emitted the bytes.
    #[serde(deserialize_with = "address")]
    pub program_id: Address,
    /// Exact base64-decoded bytes.
    #[serde(deserialize_with = "return_bytes")]
    pub data: Vec<u8>,
}
fn return_bytes<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
    let (text, encoding) = <(String, String)>::deserialize(d)?;

    if encoding != "base64" {
        return Err(serde::de::Error::custom("return data must use base64"));
    }

    STANDARD.decode(text).map_err(serde::de::Error::custom)
}
/// Complete standard transaction metadata; absence and explicit null stay distinct.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMeta {
    /// Null denotes execution success.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub err: Option<TransactionFailure>,
    /// Deprecated but still emitted execution result.
    pub status: ExecutionStatus,
    /// Exact charged fee in lamports.
    pub fee: u64,
    /// Pre-execution lamport balances in resolved account order.
    pub pre_balances: Vec<u64>,
    /// Post-execution lamport balances in resolved account order.
    pub post_balances: Vec<u64>,
    /// Recorded inner calls, absent or null when unavailable.
    #[serde(default)]
    pub inner_instructions: Presence<Option<Vec<InnerInstructions>>>,
    /// Execution logs, absent or null when unavailable.
    #[serde(default)]
    pub log_messages: Presence<Option<Vec<String>>>,
    /// Pre-execution token balances.
    #[serde(default)]
    pub pre_token_balances: Presence<Option<Vec<TokenBalance>>>,
    /// Post-execution token balances.
    #[serde(default)]
    pub post_token_balances: Presence<Option<Vec<TokenBalance>>>,
    /// Transaction rewards.
    #[serde(default)]
    pub rewards: Presence<Option<Vec<Reward>>>,
    /// Versioned lookup resolutions, omitted for parsed transaction requests.
    #[serde(default)]
    pub loaded_addresses: Presence<Option<LoadedAddresses>>,
    /// Last program return value.
    #[serde(default)]
    pub return_data: Presence<Option<ReturnData>>,
    /// Actual compute units consumed.
    #[serde(default)]
    pub compute_units_consumed: Presence<Option<u64>>,
    /// Node cost-model units, versioned separately from compute consumption.
    #[serde(default)]
    pub cost_units: Presence<Option<u64>>,
}
