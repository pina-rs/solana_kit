//! Every operation owns explicit inputs; no current-session or metadata cache is consulted.

mod transactions;
use serde::Serialize;
pub use transactions::*;

/// Source already available to the portal; local paths cannot cross this boundary.
#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum IngestionSource {
    /// An APK already uploaded to portal storage.
    PortalUpload {
        /// Public uploaded file location.
        release_file_url: String,
        /// APK file name, without a local filesystem path.
        release_file_name: String,
        /// Exact file size in bytes.
        release_file_size: u64,
        /// Optional hash; absence omits the field.
        #[serde(skip_serializing_if = "Option::is_none")]
        release_file_hash: Option<String>,
    },
    /// An externally hosted APK selected by the caller.
    ExternalUrl {
        /// Caller-approved remote APK location.
        apk_url: String,
        /// Optional file name; absence omits the field.
        #[serde(skip_serializing_if = "Option::is_none")]
        release_file_name: Option<String>,
    },
    /// A prior portal release reused as the source.
    ExistingRelease {
        /// Existing release identity.
        source_release_id: String,
    },
}

/// Fetch block data without ambient RPC configuration.
#[derive(Default, Serialize)]
pub struct GetBlockData {}

/// Request a separate presigned upload capability; this operation uploads no bytes.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUploadTarget {
    /// Caller-computed lowercase SHA-256 text.
    pub file_hash: String,
    /// File extension without a leading dot.
    pub file_extension: String,
    /// Media type for the future upload.
    pub content_type: String,
}

/// Create one idempotent ingestion session from a prepared portal-visible source.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIngestionSession {
    /// Prepared remote source; there is no local-file variant.
    pub source: IngestionSource,
    /// User-supplied release notes.
    pub whats_new: String,
    /// Caller-issued key reused unchanged by explicit retry.
    pub idempotency_key: String,
    /// Optional dApp override; absence omits the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dapp_id: Option<String>,
}

/// Read an ingestion session without starting polling or uploads.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIngestionSession {
    /// Exact session identity, serialized only as query data.
    pub session_id: String,
}

/// Read an unnormalized backend publication bundle without fetching media or another session.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPublicationBundle {
    /// Release whose raw typed backend bundle is requested.
    pub release_id: String,
}

/// Explicit publication-session selectors. Neither field falls back to client state.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPublicationSession {
    /// Session selector; None is serialized as null, matching the source request.
    pub publication_session_id: Option<String>,
    /// Release selector; None is serialized as null, matching the source request.
    pub release_id: Option<String>,
}

/// Release collection state is changed only by an explicit operation call.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkReleaseCollectionAsVerified {
    /// Release identity.
    pub release_id: String,
}

/// Explicit cleanup request; decoding another reply never performs cleanup.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanupRelease {
    /// Release whose cleanup is requested.
    pub release_id: String,
}

/// Caller-supplied signed attestation data. The portal feature does not sign it.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreAttestation {
    /// Signature-prefixed JSON encoded as base64.
    pub payload: String,
    /// Request identity bound into the caller-approved payload.
    pub request_unique_id: String,
}

/// Explicit store-review submission; this mutation is never automatically retried.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitToStore {
    /// Release identity.
    pub release_id: String,
    /// User-supplied release notes.
    pub whats_new: String,
    /// Caller-selected critical-update flag.
    pub critical_update: bool,
    /// Optional review instructions; absence omits the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testing_instructions: Option<String>,
    /// Optional resubmission flag; explicit false is retained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_resubmission: Option<bool>,
    /// Caller-approved attestation, nested exactly as required by the source.
    pub attestation: StoreAttestation,
}
