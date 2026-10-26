//! Wallet history and transfer pages differ from Enhanced Transactions v0 results.

use crate::Decimal;

model!(
    "A provider history page with cursor metadata.",
    WalletHistory {
        data: Vec<HistoryTransaction> => "data",
        pagination: CursorPagination => "pagination",
    } optional {}
);
model!(
    "A history transaction with exact fee and signed balance changes.",
    HistoryTransaction {
        signature: String => "signature",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        timestamp: Option<i64> => "timestamp",
        slot: u64 => "slot",
        fee: Decimal => "fee",
        fee_payer: String => "feePayer",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        error: Option<String> => "error",
        balance_changes: Vec<BalanceChange> => "balanceChanges",
    } optional {}
);
model!(
    "A signed display quantity for one token in a history transaction.",
    BalanceChange {
        mint: String => "mint",
        amount: Decimal => "amount",
        decimals: u8 => "decimals",
    } optional {}
);
model!(
    "Pagination retains an absent, null or supplied next cursor.",
    CursorPagination { has_more: bool => "hasMore" }
    optional { next_cursor: String => "nextCursor" }
);
/// Transfer direction relative to the queried wallet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WalletTransferDirection {
    /// Received by the queried wallet.
    In,
    /// Sent by the queried wallet.
    Out,
}

model!(
    "One token-transfer page with an opaque continuation cursor.",
    WalletTransfers {
        data: Vec<WalletTransfer> => "data",
        pagination: CursorPagination => "pagination",
    } optional {}
);
model!(
    "A transfer preserves its exact raw string and decimal display amount.",
    WalletTransfer {
        signature: String => "signature",
        timestamp: i64 => "timestamp",
        direction: WalletTransferDirection => "direction",
        counterparty: String => "counterparty",
        mint: String => "mint",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        symbol: Option<String> => "symbol",
        amount: Decimal => "amount",
        amount_raw: String => "amountRaw",
        decimals: u8 => "decimals",
    } optional {}
);
