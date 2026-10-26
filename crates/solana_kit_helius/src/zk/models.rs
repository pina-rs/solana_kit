//! Photon wire models preserve context and encoded data without authenticating proofs.

use crate::Field;
use serde::{Deserialize, Serialize};
use solana_kit_client::methods::Context;

/// A lookup result distinguishes a missing account from an omitted response field.
#[derive(Debug, PartialEq, Deserialize)]
pub struct GetCompressedAccountResponse {
    /// Slot at which the indexer evaluated the lookup.
    pub context: Context,
    /// Null means no matching account; the value field must still be present.
    #[serde(deserialize_with = "serde::Deserialize::deserialize")]
    pub value: Option<CompressedAccount>,
}

/// A non-paginated collection, retaining nullable positions when its item type is optional.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompressedItems<T> {
    /// Records in provider order.
    pub items: Vec<T>,
}

/// A cursor page with distinct missing, null and present cursor states.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompressedPage<T> {
    /// Records in provider order.
    pub items: Vec<T>,
    /// The provider's continuation cursor, when this endpoint returns one.
    #[serde(default, skip_serializing_if = "Field::is_missing")]
    pub cursor: Field<String>,
}

model!(
    "A compressed account, including accounts without assigned addresses or application data.",
    CompressedAccount {
        hash: String => "hash",
        owner: String => "owner",
        lamports: u64 => "lamports",
        leaf_index: u64 => "leafIndex",
        slot_created: u64 => "slotCreated",
        tree: String => "tree",
    } optional {
        address: String => "address",
        data: CompressedAccountData => "data",
        seq: u64 => "seq",
    }
);

model!(
    "Encoded application data and its discriminator and hash; data remains base64 text.",
    CompressedAccountData {
        discriminator: u64 => "discriminator",
        data: String => "data",
        data_hash: String => "dataHash",
    } optional {}
);

model!(
    "A Merkle proof and its tree and root sequence; decoding does not verify the proof.",
    CompressedAccountProof {
        hash: String => "hash",
        root: String => "root",
        proof: Vec<String> => "proof",
        leaf_index: u32 => "leafIndex",
        merkle_tree: String => "merkleTree",
        root_seq: u64 => "rootSeq",
    } optional {}
);

model!(
    "A compressed account together with its SPL token data.",
    CompressedTokenAccount {
        account: CompressedAccount => "account",
        token_data: CompressedTokenData => "tokenData",
    } optional {}
);

model!(
    "Raw token balance, ownership, state and optional extension data.",
    CompressedTokenData {
        amount: u64 => "amount",
        owner: String => "owner",
        mint: String => "mint",
        state: CompressedTokenState => "state",
    } optional {
        delegate: String => "delegate",
        tlv: String => "tlv",
    }
);

/// Token account states emitted by the compression indexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompressedTokenState {
    /// The account is initialized and usable.
    Initialized,
    /// The token authority has frozen the account.
    Frozen,
}

model!(
    "The compressed token balance associated with one owner.",
    OwnerBalance { owner: String => "owner", balance: u64 => "balance" } optional {}
);

model!(
    "One token account's raw compressed balance.",
    CompressedTokenAccountBalance { amount: u64 => "amount" } optional {}
);

model!(
    "A mint and its owner's total raw compressed balance.",
    CompressedTokenBalance { mint: String => "mint", balance: u64 => "balance" } optional {}
);

model!(
    "The V1 balance list retains the provider's token_balances field spelling.",
    CompressedTokenBalanceList {
        token_balances: Vec<CompressedTokenBalance> => "token_balances",
    } optional { cursor: String => "cursor" }
);

model!(
    "A compression transaction signature, slot and signed Unix timestamp.",
    CompressedSignature {
        signature: String => "signature",
        slot: u64 => "slot",
        block_time: i64 => "blockTime",
    } optional {}
);

model!(
    "Non-voting transaction metadata includes an optional failure description.",
    CompressedSignatureWithError {
        #[serde(flatten)]
        info: CompressedSignature => "info",
    } optional { error: String => "error" }
);

/// The healthy indexer result is the JSON string "ok", rather than an object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndexerHealth {
    /// The indexer is within its configured freshness window.
    #[serde(rename = "ok")]
    Healthy,
}

model!(
    "Unverified compressed Groth16 components as the server's JSON byte arrays.",
    CompressedProof {
        a: Vec<u8> => "a", b: Vec<u8> => "b", c: Vec<u8> => "c",
    } optional {}
);

model!(
    "A validity proof and associated tree metadata, without cryptographic verification.",
    ValidityProof {
        compressed_proof: CompressedProof => "compressedProof",
        root_indices: Vec<u64> => "rootIndices",
        roots: Vec<String> => "roots",
        leaf_indices: Vec<u32> => "leafIndices",
        leaves: Vec<String> => "leaves",
        merkle_trees: Vec<String> => "merkleTrees",
    } optional {}
);

model!(
    "A new-address non-inclusion proof and its lower and upper range neighbors.",
    NewAddressProof {
        address: String => "address",
        root: String => "root",
        proof: Vec<String> => "proof",
        lower_range_address: String => "lowerRangeAddress",
        higher_range_address: String => "higherRangeAddress",
        next_index: u32 => "nextIndex",
        low_element_leaf_index: u32 => "lowElementLeafIndex",
        merkle_tree: String => "merkleTree",
        root_seq: u64 => "rootSeq",
    } optional {}
);

model!(
    "A new address paired with the tree used for its non-inclusion proof.",
    AddressWithTree { address: String => "address", tree: String => "tree" } optional {}
);

model!(
    "An affected account with token metadata when it represents an SPL token account.",
    AccountWithOptionalTokenData { account: CompressedAccount => "account" }
    optional { optional_token_data: CompressedTokenData => "optionalTokenData" }
);

model!(
    "Compressed accounts opened and closed by a transaction.",
    CompressionInfo {
        closed_accounts: Vec<AccountWithOptionalTokenData> => "closedAccounts",
        opened_accounts: Vec<AccountWithOptionalTokenData> => "openedAccounts",
    } optional {}
);

model!(
    "The raw transaction and compression_info payload retain their distinct wire spelling.",
    TransactionWithCompressionInfo {
        transaction: serde_json::Value => "transaction",
        compression_info: CompressionInfo => "compression_info",
    } optional {}
);
