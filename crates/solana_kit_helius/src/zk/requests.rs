//! Compression queries preserve object parameters and direct-array batch parameters.

use super::{AccountLocator, AddressWithTree};

/// Reads a single compressed account by its address or leaf hash.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GetCompressedAccountRequest(pub AccountLocator);

/// Reads one compressed account's lamport balance by address or hash.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GetCompressedBalanceRequest(pub AccountLocator);

/// Reads one compressed token account's balance by address or hash.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GetCompressedTokenAccountBalanceRequest(pub AccountLocator);

request!(
    "Read one compressed account proof.",
    GetCompressedAccountProofRequest { hash: String => "hash" } optional {}
);

request!(
    "List compressed accounts by owner with optional slicing and memory-comparison filters.",
    GetCompressedAccountsByOwnerRequest { owner: String => "owner" } optional {
        cursor: String => "cursor", limit: u64 => "limit",
        data_slice: crate::rpc_v2::DataSlice => "dataSlice",
        filters: Vec<CompressedAccountFilter> => "filters",
    }
);

/// A compressed-account filter compares bytes at a specified data offset.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CompressedAccountFilter {
    /// Match the base58 bytes at this offset.
    Memcmp(crate::rpc_v2::MemcmpFilter),
}

request!(
    "Read an owner's total compressed lamport balance.",
    GetCompressedBalanceByOwnerRequest { owner: String => "owner" } optional {}
);

request!(
    "List compressed token owners and their balances for a mint.",
    GetCompressedMintTokenHoldersRequest { mint: String => "mint" } optional {
        cursor: String => "cursor", limit: u64 => "limit",
    }
);

request!(
    "List compressed token accounts by delegate.",
    GetCompressedTokenAccountsByDelegateRequest { delegate: String => "delegate" } optional {
        mint: String => "mint", cursor: String => "cursor", limit: u64 => "limit",
    }
);

request!(
    "List compressed token accounts by owner.",
    GetCompressedTokenAccountsByOwnerRequest { owner: String => "owner" } optional {
        mint: String => "mint", cursor: String => "cursor", limit: u64 => "limit",
    }
);

request!(
    "List an owner's compressed token balances grouped by mint.",
    GetCompressedTokenBalancesByOwnerRequest { owner: String => "owner" } optional {
        mint: String => "mint", cursor: String => "cursor", limit: u64 => "limit",
    }
);

/// The V2 method keeps its own operation name while sharing the request fields.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GetCompressedTokenBalancesByOwnerV2Request(pub GetCompressedTokenBalancesByOwnerRequest);

request!(
    "Read compression signatures for one account hash; this method has no pagination parameters.",
    GetCompressionSignaturesForAccountRequest { hash: String => "hash" } optional {}
);

request!(
    "List compression signatures for an address.",
    GetCompressionSignaturesForAddressRequest { address: String => "address" } optional {
        cursor: String => "cursor", limit: u64 => "limit",
    }
);

request!(
    "List compression signatures for an owner.",
    GetCompressionSignaturesForOwnerRequest { owner: String => "owner" } optional {
        cursor: String => "cursor", limit: u64 => "limit",
    }
);

request!(
    "List compression signatures for a token owner.",
    GetCompressionSignaturesForTokenOwnerRequest { owner: String => "owner" } optional {
        cursor: String => "cursor", limit: u64 => "limit",
    }
);

request!(
    "Read recent compression signatures.",
    GetLatestCompressionSignaturesRequest {} optional {
        cursor: String => "cursor", limit: u64 => "limit",
    }
);

request!(
    "Read recent non-voting signatures, including their failure descriptions.",
    GetLatestNonVotingSignaturesRequest {} optional {
        cursor: String => "cursor", limit: u64 => "limit",
    }
);

/// Hashes form the RPC params array directly, without a hashes object wrapper.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GetMultipleCompressedAccountProofsRequest(pub Vec<String>);

/// New addresses form the V1 RPC params array directly.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GetMultipleNewAddressProofsRequest(pub Vec<String>);

/// Each V2 array entry contains its address and Merkle tree.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GetMultipleNewAddressProofsV2Request(pub Vec<AddressWithTree>);

request!(
    "Read a transaction and its compression state changes.",
    GetTransactionWithCompressionInfoRequest { signature: String => "signature" } optional {}
);

request!(
    "Request inclusion proofs for hashes or non-inclusion proofs for address/tree pairs.",
    GetValidityProofRequest {} optional {
        hashes: Vec<String> => "hashes",
        new_addresses_with_trees: Vec<AddressWithTree> => "newAddressesWithTrees",
    }
);

request!("Read indexer health with empty object parameters.", GetIndexerHealth {} optional {});
request!("Read the indexer's latest slot.", GetIndexerSlot {} optional {});
