//! Enhanced transaction fields follow the provider's wire schema.

use std::collections::BTreeMap;

model!(
    "An enhanced transaction; parsing results may omit all fields except its signature.",
    EnhancedTransaction {
        signature: String => "signature",
    } optional {
        transaction_type: String => "type",
        source: String => "source",
        fee: u64 => "fee",
        fee_payer: String => "feePayer",
        slot: u64 => "slot",
        description: String => "description",
        timestamp: i64 => "timestamp",
        native_transfers: Vec<NativeTransfer> => "nativeTransfers",
        token_transfers: Vec<TokenTransfer> => "tokenTransfers",
        account_data: Vec<AccountData> => "accountData",
        transaction_error: serde_json::Value => "transactionError",
        instructions: Vec<InnerInstruction> => "instructions",
        events: BTreeMap<String,
        serde_json::Value> => "events",
    }
);
model!(
    "A native SOL transfer; unidentified sender and recipient addresses may be null.",
    NativeTransfer {
        amount: u64 => "amount",
    } optional {
        from_user_account: String => "fromUserAccount",
        to_user_account: String => "toUserAccount",
    }
);
model!(
    "A token transfer retaining numeric and string amounts without rounding.",
    TokenTransfer {
        mint: String => "mint",
        token_amount: super::TokenAmount => "tokenAmount",
    } optional {
        from_user_account: String => "fromUserAccount",
        to_user_account: String => "toUserAccount",
        from_token_account: String => "fromTokenAccount",
        to_token_account: String => "toTokenAccount",
        token_standard: String => "tokenStandard",
        decimals: u8 => "decimals",
    }
);
model!(
    "An account's signed native delta and optional post-transaction balance and token changes.",
    AccountData {
        account: String => "account",
    } optional {
        native_balance_change: i64 => "nativeBalanceChange",
        native_token_balance: u64 => "nativeTokenBalance",
        token_balance_changes: Vec<TokenBalanceChange> => "tokenBalanceChanges",
    }
);
model!(
    "A token balance delta with its exact raw amount and decimal precision.",
    TokenBalanceChange {
        mint: String => "mint",
        raw_token_amount: RawTokenAmount => "rawTokenAmount",
        user_account: String => "userAccount",
        token_account: String => "tokenAccount",
    } optional {}
);
model!(
    "A raw token amount represented as a string, with its mint's decimal precision.",
    RawTokenAmount {
        token_amount: String => "tokenAmount",
        decimals: u8 => "decimals",
    } optional {}
);
model!(
    "A parsed or encoded instruction, including recursively typed inner instructions.",
    InnerInstruction {
        program_id: String => "programId",
    } optional {
        program_name: String => "programName",
        accounts: Vec<String> => "accounts",
        data: String => "data",
        inner_instructions: Vec<InnerInstruction> => "innerInstructions",
        parsed: BTreeMap<String, serde_json::Value> => "parsed",
    }
);
