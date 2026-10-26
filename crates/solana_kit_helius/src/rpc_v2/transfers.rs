//! Positional transfer queries and exact decimal-string amounts.

use crate::enums::CommitmentLevel;
use std::collections::BTreeMap;

address_request!(
    "A transfer query with an optional second positional configuration argument.",
    GetTransfersByAddressRequest,
    address,
    GetTransfersByAddressConfig
);

request!(
    "Transfer query constraints from the pinned schema.",
    GetTransfersByAddressConfig {} optional {
        with_address: String => "with",
        direction: String => "direction",
        mint: String => "mint",
        sol_mode: String => "solMode",
        filters: BTreeMap<String,
        serde_json::Value> => "filters",
        limit: u64 => "limit",
        pagination_token: String => "paginationToken",
        commitment: CommitmentLevel => "commitment",
        sort_order: String => "sortOrder",
    }
);
model!(
    "One transfer page, preserving missing and explicit null cursors.",
    GetTransfersByAddressResponse {
        data: Vec<AddressTransfer> => "data",
    } optional { pagination_token: String => "paginationToken" }
);
model!(
    "A transfer with exact raw and display amount strings.",
    AddressTransfer {
        signature: String => "signature",
        slot: u64 => "slot",
        block_time: i64 => "blockTime",
        transfer_type: String => "type",
        mint: String => "mint",
        amount: String => "amount",
        decimals: u64 => "decimals",
        ui_amount: String => "uiAmount",
        confirmation_status: String => "confirmationStatus",
        transaction_idx: i64 => "transactionIdx",
        instruction_idx: i64 => "instructionIdx",
        inner_instruction_idx: i64 => "innerInstructionIdx",
    } optional {
        from_user_account: String => "fromUserAccount",
        to_user_account: String => "toUserAccount",
        from_token_account: String => "fromTokenAccount",
        to_token_account: String => "toTokenAccount",
        fee_amount: String => "feeAmount",
        fee_account: String => "feeAccount",
        fee_ui_amount: String => "feeUiAmount",
    }
);
