//! Asset identity, compression, ownership, supply and list contracts.

use super::metadata::*;
use crate::Decimal;
use crate::enums::HeliusUseMethod;

model!(
    "A digital asset with explicit absent and null metadata.",
    HeliusAsset {
        id: String => "id",
    } optional {
        last_indexed_slot: u64 => "last_indexed_slot",
        interface: String => "interface",
        content: AssetContent => "content",
        authorities: Vec<AssetAuthority> => "authorities",
        compression: AssetCompression => "compression",
        grouping: Vec<AssetGrouping> => "grouping",
        royalty: AssetRoyalty => "royalty",
        creators: Vec<AssetCreator> => "creators",
        ownership: AssetOwnership => "ownership",
        supply: AssetSupply => "supply",
        mutable: bool => "mutable",
        burnt: bool => "burnt",
        token_info: AssetTokenInfo => "token_info",
        uses: AssetUses => "uses",
        inscription: AssetInscription => "inscription",
        spl20: std::collections::BTreeMap<String, serde_json::Value> => "spl20",
        mint_extensions: std::collections::BTreeMap<String,
        serde_json::Value> => "mint_extensions",
    }
);
model!(
    "Authority address and granted scopes.",
    AssetAuthority {
        address: String => "address",
    } optional { scopes: Vec<String> => "scopes" }
);
model!(
    "Compression tree and leaf facts returned by the provider.",
    AssetCompression {} optional {
        eligible: bool => "eligible",
        compressed: bool => "compressed",
        data_hash: String => "data_hash",
        creator_hash: String => "creator_hash",
        asset_hash: String => "asset_hash",
        tree: String => "tree",
        seq: u64 => "seq",
        leaf_id: u64 => "leaf_id",
    }
);
model!(
    "An asset grouping pair.",
    AssetGrouping {
        group_key: String => "group_key",
        group_value: String => "group_value",
    } optional {
        verified: bool => "verified",
        collection_metadata: AssetCollectionMetadata => "collection_metadata",
    }
);
model!(
    "Royalty terms without float coercion.",
    AssetRoyalty {} optional {
        royalty_model: String => "royalty_model",
        target: String => "target",
        percent: Decimal => "percent",
        basis_points: u64 => "basis_points",
        primary_sale_happened: bool => "primary_sale_happened",
        locked: bool => "locked",
    }
);
model!(
    "Creator royalty share and verification.",
    AssetCreator {
        address: String => "address",
        share: u64 => "share",
        verified: bool => "verified",
    } optional {}
);
model!(
    "Asset ownership and delegation.",
    AssetOwnership {} optional {
        frozen: bool => "frozen",
        delegated: bool => "delegated",
        delegate: String => "delegate",
        ownership_model: String => "ownership_model",
        owner: String => "owner",
    }
);
model!(
    "Print supply and edition nonce.",
    AssetSupply {} optional {
        print_max_supply: u64 => "print_max_supply",
        print_current_supply: u64 => "print_current_supply",
        edition_nonce: u64 => "edition_nonce",
    }
);

model!(
    "Token metadata with exact raw supply.",
    AssetTokenInfo {} optional {
        symbol: String => "symbol",
        balance: u64 => "balance",
        supply: u64 => "supply",
        decimals: u64 => "decimals",
        token_program: String => "token_program",
        associated_token_address: String => "associated_token_address",
        mint_authority: String => "mint_authority",
        freeze_authority: String => "freeze_authority",
        price_info: AssetPriceInfo => "price_info",
    }
);
model!(
    "Exact provider price decimals and currency.",
    AssetPriceInfo {} optional {
        price_per_token: Decimal => "price_per_token",
        total_price: Decimal => "total_price",
        currency: String => "currency",
    }
);
model!(
    "Merkle proof data; authenticity requires independent proof verification.",
    AssetProof {
        root: String => "root",
        proof: Vec<String> => "proof",
        node_index: u64 => "node_index",
        leaf: String => "leaf",
        tree_id: String => "tree_id",
    } optional {
        last_indexed_slot: u64 => "last_indexed_slot",
        burnt: bool => "burnt",
    }
);
model!(
    "An NFT edition record.",
    NftEdition {} optional {
        mint: String => "mint",
        edition_address: String => "edition_address",
        edition: u64 => "edition",
        burnt: bool => "burnt",
    }
);
model!(
    "A master edition and its paginated print editions.",
    NftEditionList {} optional {
        last_indexed_slot: u64 => "last_indexed_slot",
        total: u64 => "total",
        limit: u64 => "limit",
        page: u64 => "page",
        master_edition_address: String => "master_edition_address",
        supply: u64 => "supply",
        max_supply: u64 => "max_supply",
        editions: Vec<NftEdition> => "editions",
    }
);
/// A signature/operation pair; operation names remain open to future programs.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AssetSignature(
    /// Transaction signature.
    pub String,
    /// Provider operation label.
    pub String,
);
model!(
    "One page of signature events.",
    AssetSignatureList {
        total: u64 => "total",
        limit: u64 => "limit",
        items: Vec<AssetSignature> => "items",
    } optional {
        last_indexed_slot: u64 => "last_indexed_slot",
        page: u64 => "page",
        before: String => "before",
        after: String => "after",
    }
);
model!(
    "One page of assets.",
    AssetList {
        total: u64 => "total",
        limit: u64 => "limit",
        items: Vec<HeliusAsset> => "items",
    } optional {
        last_indexed_slot: u64 => "last_indexed_slot",
        grand_total: u64 => "grand_total",
        page: u64 => "page",
        cursor: String => "cursor",
        native_balance: NativeBalanceInfo => "nativeBalance",
    }
);
model!(
    "One page of token accounts.",
    TokenAccountList {} optional {
        last_indexed_slot: u64 => "last_indexed_slot",
        total: u64 => "total",
        limit: u64 => "limit",
        token_accounts: Vec<TokenAccount> => "token_accounts",
        page: u64 => "page",
        cursor: String => "cursor",
    }
);
model!(
    "A token account with exact raw balance.",
    TokenAccount {} optional {
        address: String => "address",
        mint: String => "mint",
        owner: String => "owner",
        amount: u64 => "amount",
        delegated_amount: u64 => "delegated_amount",
        frozen: bool => "frozen",
        burnt: bool => "burnt",
        token_extensions: serde_json::Value => "token_extensions",
    }
);
model!(
    "Native SOL balance with exact provider prices.",
    NativeBalanceInfo {
        lamports: u64 => "lamports",
        price_per_sol: Decimal => "price_per_sol",
        total_price: Decimal => "total_price",
    } optional {}
);
model!(
    "Asset consumption restrictions.",
    AssetUses {
        use_method: HeliusUseMethod => "use_method",
        remaining: u64 => "remaining",
        total: u64 => "total",
    } optional {}
);
