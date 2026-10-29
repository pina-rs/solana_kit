//! Prepared transactions remain proposals until the caller independently validates and signs.

use serde::Serialize;

/// Prepare a release mint transaction; this operation neither signs nor submits it.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareReleaseNftTransaction {
    /// Portal release identity.
    pub release_id: String,
    /// Release display name.
    pub release_name: String,
    /// Caller-approved release metadata location.
    pub release_metadata_uri: String,
    /// Proposed app collection mint text.
    pub app_mint_address: String,
    /// Proposed publication signer text.
    pub publisher_address: String,
    /// Proposed fee payer text.
    pub payer_address: String,
}

/// Prepare collection verification without signing or submission.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareVerifyCollectionTransaction {
    /// Portal dApp identity.
    pub dapp_id: String,
    /// Proposed release NFT mint.
    pub nft_mint_address: String,
    /// Proposed app collection mint.
    pub collection_mint_address: String,
    /// Proposed collection authority.
    pub collection_authority: String,
    /// Proposed fee payer.
    pub payer_address: String,
}

/// Explicit caller-approved transaction submission. No submission follows decode automatically.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitSignedTransaction {
    /// Fully signed base64 wire transaction selected by the caller.
    pub signed_transaction: String,
    /// Explicit session identity; None is transmitted as null, with no hidden fallback.
    pub publication_session_id: Option<String>,
}

/// Save the caller's release mint receipt without inferring that it landed or succeeded.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveReleaseNftData {
    /// Portal release identity.
    pub release_id: String,
    /// Release mint text.
    pub mint_address: String,
    /// Caller-observed transaction signature text.
    pub transaction_signature: String,
    /// Release metadata location.
    pub metadata_uri: String,
    /// Proposed release owner.
    pub owner_address: String,
    /// Release display name.
    pub release_name: String,
    /// Release version name.
    pub release_version: String,
    /// Android package identifier, supplied explicitly rather than read from this machine.
    pub android_package: String,
    /// App collection mint text.
    pub app_mint_address: String,
}
