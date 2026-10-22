//! Typed filter scalars avoid lossy timestamps and unchecked signature strings.
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use solana_kit_core::Signature;

/// AND-combined range comparisons. Signature ordering is defined by the node.
#[derive(Debug, Default, Clone, Serialize, PartialEq, Eq)]
pub struct Comparison<T> {
    /// Exclusive lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<T>,
    /// Inclusive lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<T>,
    /// Exclusive upper bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<T>,
    /// Inclusive upper bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<T>,
}
/// Block time additionally supports exact equality.
#[derive(Debug, Default, Clone, Serialize, PartialEq, Eq)]
pub struct TimeComparison {
    /// Exact Unix timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<i64>,
    /// AND-combined time bounds.
    #[serde(flatten)]
    pub range: Comparison<i64>,
}
/// Validated signature serialized as its base58 spelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignatureBound(pub Signature);
impl Serialize for SignatureBound {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0.to_string())
    }
}
/// Execution filter.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionFilter {
    /// Successful and failed executions.
    Any,
    /// Failed executions only.
    Failed,
    /// Successful executions only.
    Succeeded,
}
/// Include activity on owned token accounts.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TokenAccountsFilter {
    /// Direct address references only.
    None,
    /// Owned token account balance changes.
    BalanceChanged,
    /// Every owned token account reference.
    All,
}
/// All optional filters combine with AND.
#[derive(Debug, Default, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddressFilters {
    /// Slot comparisons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<Comparison<u64>>,
    /// Signature order comparisons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Comparison<SignatureBound>>,
    /// Block time comparisons, including exact equality.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_time: Option<TimeComparison>,
    /// Execution outcome selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ExecutionFilter>,
    /// Owned-token-account selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_accounts: Option<TokenAccountsFilter>,
}
/// Node ordering within slots and block positions.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// Oldest first.
    Asc,
    /// Newest first.
    Desc,
}
/// Canonical `<slot>:<position>` continuation token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaginationToken {
    /// Slot part.
    pub slot: u64,
    /// Position part. This is a cursor, not an inferred transaction count.
    pub position: u64,
}
impl Serialize for PaginationToken {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&format!("{}:{}", self.slot, self.position))
    }
}
impl<'de> Deserialize<'de> for PaginationToken {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let text = String::deserialize(d)?;
        let (slot, position) = text
            .split_once(':')
            .ok_or_else(|| serde::de::Error::custom("invalid pagination token"))?;
        let canonical = |part: &str| {
            part == "0"
                || (!part.starts_with('0')
                    && !part.is_empty()
                    && part.bytes().all(|b| b.is_ascii_digit()))
        };

        if !canonical(slot) || !canonical(position) {
            return Err(serde::de::Error::custom("invalid pagination token"));
        }

        Ok(Self {
            slot: slot.parse().map_err(serde::de::Error::custom)?,
            position: position.parse().map_err(serde::de::Error::custom)?,
        })
    }
}
