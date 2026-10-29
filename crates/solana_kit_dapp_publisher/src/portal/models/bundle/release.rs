//! Exact release fields before workflow normalization and media inference.

use crate::portal::{Optional, object};
use serde::Deserialize;
use serde_json::Number;

/// Backend release fields used by the source bundle translator.
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BackendRelease {
    /// Release identity.
    pub id: Optional<String>,
    /// Linked dApp identity.
    pub dapp_id: Optional<String>,
    /// APK location, without download or upload.
    pub release_file_url: Optional<String>,
    /// APK file name.
    pub release_file_name: Optional<String>,
    /// Exact reported file-size number.
    #[serde(deserialize_with = "object::number")]
    pub release_file_size: Optional<Number>,
    /// Optional file hash text.
    pub release_file_hash: Optional<String>,
    /// Release version name, without fallback to versionCode.
    pub version_name: Optional<String>,
    /// Exact version-code number, without a default or truncation.
    #[serde(deserialize_with = "object::number")]
    pub version_code: Optional<Number>,
    /// Android package identifier.
    pub android_package: Optional<String>,
    /// Exact minimum SDK number.
    #[serde(deserialize_with = "object::number")]
    pub min_sdk_version: Optional<Number>,
    /// Exact target SDK number.
    #[serde(deserialize_with = "object::number")]
    pub target_sdk_version: Optional<Number>,
    /// Permission identifiers in provider order.
    pub permissions: Optional<Vec<String>>,
    /// Locale identifiers in provider order.
    pub locales: Optional<Vec<String>>,
    /// Certificate fingerprint text.
    pub certificate_fingerprint: Optional<String>,
    /// Optional short description.
    pub short_description: Optional<String>,
    /// Optional long description.
    pub long_description: Optional<String>,
    /// Optional localized display name.
    pub localized_name: Optional<String>,
    /// Release notes.
    pub new_in_version: Optional<String>,
    /// Optional platform-feature text.
    pub saga_features: Optional<String>,
    /// Proposed release NFT mint text.
    pub nft_mint_address: Optional<String>,
    /// Proposed release metadata location.
    pub nft_metadata_uri: Optional<String>,
}
