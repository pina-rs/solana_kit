//! Address history uses positional parameters and distinct signature and full records.

use crate::enums::{CommitmentLevel, SortOrder};
pub use crate::enums::{
    HistoryTransactionStatus, TokenAccountMode, TransactionDetails, TransactionEncoding,
};
use std::collections::BTreeMap;

address_request!(
    "Read address transaction history using [address, config?] parameters.",
    GetTransactionsForAddressRequest,
    address,
    GetTransactionsForAddressConfig
);

request!(
    #[derive(Default)]
    "Address-history page, representation and filter constraints.",
    GetTransactionsForAddressConfig {} optional {
        commitment: CommitmentLevel => "commitment",
        min_context_slot: u64 => "minContextSlot",
        encoding: TransactionEncoding => "encoding",
        max_supported_transaction_version: u8 => "maxSupportedTransactionVersion",
        limit: u64 => "limit",
        pagination_token: String => "paginationToken",
        sort_order: SortOrder => "sortOrder",
        filters: TransactionFilters => "filters",
        transaction_details: TransactionDetails => "transactionDetails",
    }
);

request!(
    #[derive(Default)]
    "History filters use nested ranges rather than legacy before/until parameters.",
    TransactionFilters {} optional {
        status: HistoryTransactionStatus => "status",
        token_accounts: TokenAccountMode => "tokenAccounts",
        slot: Comparison<u64> => "slot",
        block_time: Comparison<i64> => "blockTime",
        signature: SignatureComparison => "signature",
    }
);

/// A typed numeric range for slots or signed Unix timestamps.
#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(bound(deserialize = "T: serde::Deserialize<'de>"))]
pub struct Comparison<T> {
    /// Equal to this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eq: Option<T>,
    /// Greater than or equal to this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gte: Option<T>,
    /// Greater than this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gt: Option<T>,
    /// Less than or equal to this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lte: Option<T>,
    /// Less than this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lt: Option<T>,
}

request!(
    #[derive(Default)]
    "Lexicographic signature bounds accepted by address history.",
    SignatureComparison {} optional {
        gte: String => "gte", gt: String => "gt", lte: String => "lte", lt: String => "lt",
    }
);

model!(
    "A history page with its required nullable continuation token.",
    GetTransactionsForAddressResponse {
        data: Vec<TransactionForAddress> => "data",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        pagination_token: Option<String> => "paginationToken",
    } optional {}
);

/// The record shape selected by the history request's transactionDetails.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum TransactionForAddress {
    /// Signature metadata, returned by default.
    Signature(TransactionForAddressSignature),
    /// A full transaction and its execution metadata.
    Full(TransactionForAddressFull),
}

model!(
    "Signature-level history metadata.",
    TransactionForAddressSignature {
        signature: String => "signature",
        slot: u64 => "slot",
        transaction_index: u64 => "transactionIndex",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        err: Option<BTreeMap<String, serde_json::Value>> => "err",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        memo: Option<String> => "memo",
        block_time: i64 => "blockTime",
        confirmation_status: CommitmentLevel => "confirmationStatus",
    } optional {}
);

model!(
    "Full transaction history and its nullable execution metadata.",
    TransactionForAddressFull {
        slot: u64 => "slot",
        transaction_index: u64 => "transactionIndex",
        transaction: HistoryTransactionPayload => "transaction",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        meta: Option<BTreeMap<String, serde_json::Value>> => "meta",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        block_time: Option<i64> => "blockTime",
    } optional {}
);

/// A full history transaction in its provider wire form, without signature validation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum HistoryTransactionPayload {
    /// A structured JSON or parsed-JSON transaction.
    Structured(BTreeMap<String, serde_json::Value>),
    /// Encoded transaction text and its explicit encoding; bytes remain undecoded.
    Binary((String, BinaryTransactionEncoding)),
}

/// Encoding tag attached to a binary transaction payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BinaryTransactionEncoding {
    /// Base58-encoded wire text.
    Base58,
    /// Base64-encoded wire text.
    Base64,
}

impl HistoryTransactionPayload {
    pub(crate) fn matches_encoding(&self, encoding: TransactionEncoding) -> bool {
        matches!(
            (self, encoding),
            (
                Self::Structured(_),
                TransactionEncoding::Json | TransactionEncoding::JsonParsed
            ) | (
                Self::Binary((_, BinaryTransactionEncoding::Base58)),
                TransactionEncoding::Base58
            ) | (
                Self::Binary((_, BinaryTransactionEncoding::Base64)),
                TransactionEncoding::Base64
            )
        )
    }
}
