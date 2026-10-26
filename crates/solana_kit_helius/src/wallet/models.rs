//! Wallet identity, historical balances and initial funding retain provider wire fields.

use crate::Decimal;

model!(
    "A known entity's public wallet classification.",
    Identity {
        address: String => "address",
        entity_type: String => "type",
        name: String => "name",
        category: String => "category",
        tags: Vec<String> => "tags",
    } optional {}
);
model!(
    "The transaction that supplied a historical balance.",
    BalanceAtSource {
        slot: u64 => "slot",
        signature: String => "signature",
    } optional { block_time: i64 => "blockTime" }
);
model!(
    "A historical balance with exact decimal strings and a required nullable source.",
    GetBalanceAtResponse {
        wallet: String => "wallet",
        mint: String => "mint",
        is_native: bool => "isNative",
        balance: String => "balance",
        balance_raw: String => "balanceRaw",
        decimals: u8 => "decimals",
        requested: GetBalanceAtRequested => "requested",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        as_of: Option<BalanceAtSource> => "asOf",
    } optional {}
);
model!(
    "The provider echoes all three historical selectors, including unused null fields.",
    GetBalanceAtRequested {
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        time: Option<i64> => "time",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        slot: Option<u64> => "slot",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        datetime: Option<String> => "datetime",
    } optional {}
);
model!(
    "The original funding transfer; displayed amounts remain exact JSON numbers.",
    FundedByResult {
        funder: String => "funder",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        funder_name: Option<String> => "funderName",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        funder_type: Option<String> => "funderType",
        mint: String => "mint",
        symbol: String => "symbol",
        amount: Decimal => "amount",
        amount_raw: String => "amountRaw",
        decimals: u8 => "decimals",
        signature: String => "signature",
        timestamp: i64 => "timestamp",
        date: String => "date",
        slot: u64 => "slot",
        explorer_url: String => "explorerUrl",
    } optional {}
);
