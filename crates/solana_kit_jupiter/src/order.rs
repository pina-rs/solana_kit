//! Nullable managed-order records retain the complete bounded response body.

use crate::{Integer, scalar};
use serde::{Deserialize, Deserializer, Serialize};

/// The landing mode reported by a managed order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderMode {
    /// Jupiter's managed landing stack.
    Ultra,
    /// Caller-owned RPC landing.
    Manual,
}

/// Quote and optional transaction from `GET /swap/v2/order`.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    /// Exact input amount; missing and null remain unavailable.
    pub in_amount: Option<Integer>,
    /// Exact output amount.
    pub out_amount: Option<Integer>,
    /// Base64 wire transaction to inspect before signing.
    #[serde(rename = "transaction")]
    pub encoded_transaction: Option<String>,
    /// Request identifier required by the execution route.
    pub request_id: Option<String>,
    /// Provider router identifier.
    pub router: Option<String>,
    /// Known landing mode; unknown string modes remain unavailable.
    #[serde(default, deserialize_with = "order_mode")]
    pub mode: Option<OrderMode>,
    /// Exact last valid block height, accepting number or string wire values.
    pub last_valid_block_height: Option<Integer>,
    /// Expiry Unix timestamp in seconds, accepting integer strings.
    #[serde(default, deserialize_with = "scalar::integer")]
    pub expire_at: Option<i64>,
    /// Provider swap type.
    pub swap_type: Option<String>,
    /// Applied slippage in basis points, accepting integer strings.
    #[serde(default, deserialize_with = "scalar::integer")]
    pub slippage_bps: Option<i64>,
    /// Original JSON bytes, including unknown fields; empty for direct construction.
    #[serde(skip)]
    pub raw_response: Vec<u8>,
}

/// Provider execution result; a signature does not establish successful execution.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionResponse {
    /// Legacy fully signed base64 transaction, usually absent in v2 responses.
    #[serde(rename = "swapTransaction")]
    pub encoded_swap_transaction: Option<String>,
    /// Transaction signature, which may also accompany a failed execution.
    pub signature: Option<String>,
    /// String error reported by the provider; other error shapes remain in raw JSON.
    #[serde(default, deserialize_with = "execution_error")]
    pub error: Option<String>,
    /// Provider status, such as `Success` or `Failed`.
    pub status: Option<String>,
    /// Integer result code; zero is the provider's success code.
    pub code: Option<i64>,
    /// Original JSON bytes, including amounts and events; empty for direct construction.
    #[serde(skip)]
    pub raw_response: Vec<u8>,
}

impl std::fmt::Debug for OrderResponse {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("OrderResponse { .. }")
    }
}

impl std::fmt::Debug for ExecutionResponse {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("ExecutionResponse { .. }")
    }
}

fn order_mode<'de, D: Deserializer<'de>>(decoder: D) -> Result<Option<OrderMode>, D::Error> {
    let value = Option::<String>::deserialize(decoder)?;
    Ok(match value.as_deref() {
        Some("ultra") => Some(OrderMode::Ultra),
        Some("manual") => Some(OrderMode::Manual),
        _ => None,
    })
}

fn execution_error<'de, D: Deserializer<'de>>(decoder: D) -> Result<Option<String>, D::Error> {
    let value = serde_json::Value::deserialize(decoder)?;
    Ok(value.as_str().map(str::to_owned))
}
