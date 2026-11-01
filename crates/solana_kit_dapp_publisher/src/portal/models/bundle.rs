//! Typed backend bundle records; display/media fallbacks belong to later workflow code.

mod release;
use crate::portal::{Optional, object};
pub use release::BackendRelease;
use serde::Deserialize;
use serde_json::Number;

/// Raw bundle returned by the portal. No metadata upload or linked-session fetch occurs.
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BackendPublicationBundle {
    /// Backend ingestion identity.
    pub ingestion_session_id: Optional<String>,
    /// Backend publication-session identity.
    pub publication_session_id: Optional<String>,
    /// Backend release identity, without fallback to another record.
    pub release_id: Optional<String>,
    /// Typed dApp record.
    #[serde(deserialize_with = "object::optional")]
    pub dapp: Optional<BackendDapp>,
    /// Typed publisher record; no contact metadata is transmitted on decode.
    #[serde(deserialize_with = "object::optional")]
    pub publisher: Optional<BackendPublisher>,
    /// Typed APK metadata.
    #[serde(deserialize_with = "object::optional")]
    pub install_file: Optional<BackendInstallFile>,
    /// Proposed signer authority, subject to independent caller policy.
    #[serde(deserialize_with = "object::optional")]
    pub signer_authority: Optional<BackendSignerAuthority>,
    /// Typed release record.
    #[serde(deserialize_with = "object::optional")]
    pub release: Optional<BackendRelease>,
}

/// dApp fields consumed by the source translator, before its display fallbacks.
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BackendDapp {
    /// dApp identity.
    pub id: Optional<String>,
    /// Display name.
    pub dapp_name: Optional<String>,
    /// Short display subtitle.
    pub subtitle: Optional<String>,
    /// Long description.
    pub description: Optional<String>,
    /// Android package identifier.
    pub android_package: Optional<String>,
    /// Icon location; no remote media is fetched.
    pub dapp_icon_url: Optional<String>,
    /// Preview locations in provider order.
    pub dapp_preview_urls: Optional<Vec<String>>,
    /// Banner location.
    pub banner_url: Optional<String>,
    /// Feature graphic location.
    pub feature_graphic_url: Optional<String>,
    /// Editor's choice graphic location.
    pub editors_choice_graphic_url: Optional<String>,
    /// Application website.
    pub app_website: Optional<String>,
    /// Publisher-supplied contact address, retained as data only.
    pub contact_email: Optional<String>,
    /// Publisher-supplied support address.
    pub support_email: Optional<String>,
    /// Language identifiers in provider order.
    pub languages: Optional<Vec<String>>,
    /// License document location.
    pub license_url: Optional<String>,
    /// Copyright document location.
    pub copyright_url: Optional<String>,
    /// Privacy policy location.
    pub privacy_policy_url: Optional<String>,
    /// Proposed dApp wallet text.
    pub wallet_address: Optional<String>,
    /// Proposed dApp collection mint text.
    pub nft_mint_address: Optional<String>,
    /// Previous approved release identity.
    pub last_approved_release_id: Optional<String>,
    /// Website alias, without fallback to appWebsite.
    pub website: Optional<String>,
}

/// Publisher-provided identity/contact data, with no implicit outbound publication.
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BackendPublisher {
    /// Publisher identity.
    pub id: Optional<String>,
    /// Exact publisher type; no organization/individual fallback is synthesized.
    #[serde(rename = "type")]
    pub publisher_type: Optional<String>,
    /// Publisher-supplied display name.
    pub name: Optional<String>,
    /// Publisher-supplied website.
    pub website: Optional<String>,
    /// Publisher-supplied contact address.
    pub email: Optional<String>,
    /// Publisher-supplied support address.
    pub support_email: Optional<String>,
}

/// Backend APK metadata; no file, URL or MIME inference runs during decoding.
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BackendInstallFile {
    /// Exact install-file location.
    pub uri: Optional<String>,
    /// Reported media type.
    pub mime_type: Optional<String>,
    /// Exact reported byte-size number.
    #[serde(deserialize_with = "object::number")]
    pub size: Optional<Number>,
    /// Optional hash text.
    pub sha256: Optional<String>,
    /// Canonical location, without synthesizing an alias.
    pub canonical_url: Optional<String>,
}

/// Proposed backend signer identities, before any fallback or validation.
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BackendSignerAuthority {
    /// Proposed dApp wallet.
    pub dapp_wallet_address: Optional<String>,
    /// Proposed collection authority.
    pub collection_authority: Optional<String>,
    /// Proposed app collection mint.
    pub app_mint_address: Optional<String>,
    /// Exact backend flag; missing/null do not become true.
    pub same_signer_required: Optional<bool>,
    /// Accepted signer roles in provider order.
    pub accepted_signer_roles: Optional<Vec<String>>,
    /// Required signer text, without fallback.
    pub required_signer: Optional<String>,
    /// Mint signer text, without fallback.
    pub mint_signer: Optional<String>,
    /// Optional fee payer text.
    pub fee_payer: Optional<String>,
}
