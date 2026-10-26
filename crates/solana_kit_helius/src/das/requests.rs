//! Request field spellings follow the pinned Helius DAS source.

use super::{AssetOptions, AssetSortingRequest, DisplayOptions, TokenAccountOptions};
use crate::enums::{HeliusOwnershipModel, HeliusRoyaltyModel};

request!(
    #[serde(deny_unknown_fields)]
    "Fetch one digital asset.",
    GetAssetRequest {
        id: String => "id",
    } optional { options: AssetOptions => "options" }
);
request!(
    #[serde(deny_unknown_fields)]
    "Fetch digital assets in input order.",
    GetAssetBatchRequest {
        ids: Vec<String> => "ids",
    } optional { options: AssetOptions => "options" }
);
request!(
    #[serde(deny_unknown_fields)]
    "Fetch one compressed asset proof.",
    GetAssetProofRequest {
        id: String => "id",
    } optional {}
);
request!(
    #[serde(deny_unknown_fields)]
    "Fetch compressed asset proofs keyed by requested asset ID.",
    GetAssetProofBatchRequest {
        ids: Vec<String> => "ids",
    } optional {}
);
request!(
    #[serde(deny_unknown_fields)]
    "Find assets by authority.",
    GetAssetsByAuthorityRequest {
        authority_address: String => "authorityAddress",
    } optional {
        page: u64 => "page",
        limit: u64 => "limit",
        sort_by: AssetSortingRequest => "sortBy",
        options: DisplayOptions => "options",
        before: String => "before",
        after: String => "after",
    }
);
request!(
    #[serde(deny_unknown_fields)]
    "Find assets by creator.",
    GetAssetsByCreatorRequest {
        creator_address: String => "creatorAddress",
    } optional {
        only_verified: bool => "onlyVerified",
        page: u64 => "page",
        limit: u64 => "limit",
        sort_by: AssetSortingRequest => "sortBy",
        options: DisplayOptions => "options",
        before: String => "before",
        after: String => "after",
    }
);
request!(
    #[serde(deny_unknown_fields)]
    "Find assets by grouping key and value.",
    GetAssetsByGroupRequest {
        group_key: String => "groupKey",
        group_value: String => "groupValue",
    } optional {
        page: u64 => "page",
        limit: u64 => "limit",
        sort_by: AssetSortingRequest => "sortBy",
        options: DisplayOptions => "options",
        before: String => "before",
        after: String => "after",
    }
);
request!(
    #[serde(deny_unknown_fields)]
    "Find assets by owner.",
    GetAssetsByOwnerRequest {
        owner_address: String => "ownerAddress",
    } optional {
        page: u64 => "page",
        limit: u64 => "limit",
        sort_by: AssetSortingRequest => "sortBy",
        options: DisplayOptions => "options",
        display_options: DisplayOptions => "displayOptions",
        before: String => "before",
        after: String => "after",
    }
);

request!(
    #[serde(deny_unknown_fields)]
    "Find edition records for a mint.",
    GetNftEditionsRequest {
        mint: String => "mint",
    } optional { page: u64 => "page",
    limit: u64 => "limit" }
);
request!(
    #[serde(deny_unknown_fields)]
    "Find transaction signatures for an asset.",
    GetSignaturesForAssetRequest {
        id: String => "id",
    } optional {
        page: u64 => "page",
        limit: u64 => "limit",
        before: String => "before",
        after: String => "after",
    }
);
request!(
    #[serde(deny_unknown_fields)]
    "Find token accounts by optional owner and mint.",
    GetTokenAccountsRequest {} optional {
        owner: String => "owner",
        mint: String => "mint",
        page: u64 => "page",
        limit: u64 => "limit",
        cursor: String => "cursor",
        before: String => "before",
        after: String => "after",
        options: TokenAccountOptions => "options",
    }
);
request!(
    #[serde(deny_unknown_fields)]
    "Search the pinned DAS asset filters.",
    SearchAssetsRequest {} optional {
        owner_address: String => "ownerAddress",
        creator_address: String => "creatorAddress",
        grouping: [String; 2] => "grouping",
        token_type: String => "tokenType",
        creator_verified: bool => "creatorVerified",
        authority_address: String => "authorityAddress",
        owner_type: HeliusOwnershipModel => "ownerType",
        tree: String => "tree",
        delegate: String => "delegate",
        supply: u64 => "supply",
        supply_mint: String => "supplyMint",
        interface: String => "interface",
        royalty_target_type: HeliusRoyaltyModel => "royaltyTargetType",
        royalty_target: String => "royaltyTarget",
        royalty_amount: u64 => "royaltyAmount",
        compressed: bool => "compressed",
        compressible: bool => "compressible",
        frozen: bool => "frozen",
        burnt: bool => "burnt",
        json_uri: String => "jsonUri",
        page: u64 => "page",
        limit: u64 => "limit",
        cursor: String => "cursor",
        sort_by: AssetSortingRequest => "sortBy",
        options: DisplayOptions => "options",
        before: String => "before",
        after: String => "after",
    }
);
