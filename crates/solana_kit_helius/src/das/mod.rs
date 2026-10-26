//! The twelve pinned Digital Asset Standard operations.

mod assets;
mod metadata;
mod options;
mod requests;

pub use assets::*;
pub use metadata::*;
pub use options::*;
pub use requests::*;

method!(GetAssetRequest, "getAsset", HeliusAsset);
method!(GetAssetBatchRequest, "getAssetBatch", Vec<HeliusAsset>);
method!(GetAssetProofRequest, "getAssetProof", AssetProof);
method!(GetAssetProofBatchRequest, "getAssetProofBatch", std::collections::BTreeMap<String, AssetProof>);
method!(
    GetAssetsByAuthorityRequest,
    "getAssetsByAuthority",
    AssetList
);
method!(GetAssetsByCreatorRequest, "getAssetsByCreator", AssetList);
method!(GetAssetsByGroupRequest, "getAssetsByGroup", AssetList);
method!(GetAssetsByOwnerRequest, "getAssetsByOwner", AssetList);
method!(
    GetNftEditionsRequest,
    "getNftEditions",
    NftEditionList,
    object
);
method!(
    GetSignaturesForAssetRequest,
    "getSignaturesForAsset",
    AssetSignatureList
);
method!(
    GetTokenAccountsRequest,
    "getTokenAccounts",
    TokenAccountList,
    object
);
method!(SearchAssetsRequest, "searchAssets", AssetList);
