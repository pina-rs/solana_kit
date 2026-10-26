//! Wallet v1 balances keep display quantities exact and nullable metadata explicit.

use crate::Decimal;

/// Token program family reported by the provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WalletTokenProgram {
    /// Original SPL Token program.
    #[serde(rename = "spl-token")]
    SplToken,
    /// Token Extensions program.
    #[serde(rename = "token-2022")]
    Token2022,
}

model!(
    "One page of balances; totalUsdValue describes this page, not the entire portfolio.",
    WalletBalances {
        balances: Vec<WalletTokenBalance> => "balances",
        total_usd_value: Decimal => "totalUsdValue",
        pagination: BalancePagination => "pagination",
    } optional { nfts: Vec<WalletNft> => "nfts" }
);
model!(
    "Explicit page metadata; callers choose whether to request another page.",
    BalancePagination {
        page: u64 => "page",
        limit: u64 => "limit",
        has_more: bool => "hasMore",
    } optional {}
);
model!(
    "A token's exact display balance with required nullable metadata and USD values.",
    WalletTokenBalance {
        mint: String => "mint",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        symbol: Option<String> => "symbol",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        name: Option<String> => "name",
        balance: Decimal => "balance",
        decimals: u8 => "decimals",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        price_per_token: Option<Decimal> => "pricePerToken",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        usd_value: Option<Decimal> => "usdValue",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        logo_uri: Option<String> => "logoUri",
        token_program: WalletTokenProgram => "tokenProgram",
    } optional {}
);
model!(
    "NFT metadata returned only when explicitly requested by the caller.",
    WalletNft {
        mint: String => "mint",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        name: Option<String> => "name",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        image_uri: Option<String> => "imageUri",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        collection_name: Option<String> => "collectionName",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        collection_address: Option<String> => "collectionAddress",
        compressed: bool => "compressed",
    } optional {}
);
