//! Raw backend state, before workflow translation, cached identifiers or clock fallbacks.

use super::BackendPublicationBundle;
use crate::portal::{Optional, object};
use serde::Deserialize;
use serde_json::Number;

/// Backend publication-session fields retain absent/null values and open stage names.
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BackendPublicationSession {
    /// Portal session identity.
    pub id: Optional<String>,
    /// Linked ingestion identity.
    pub ingestion_session_id: Optional<String>,
    /// Linked release identity.
    pub release_id: Optional<String>,
    /// Exact backend stage; no checkpoint or completion is synthesized.
    pub stage: Optional<String>,
    /// Metadata location proposed by the backend.
    pub metadata_uri: Optional<String>,
    /// Expected release mint text.
    pub expected_mint_address: Optional<String>,
    /// Mint-submission signature text, without confirming landing.
    pub mint_transaction_signature: Optional<String>,
    /// Collection-verification signature text.
    pub verification_transaction_signature: Optional<String>,
    /// Recorded attestation request identity.
    pub attestation_request_unique_id: Optional<String>,
    /// Recorded store-review ticket.
    pub hubspot_ticket_id: Optional<String>,
    /// Untrusted backend error text, retained only as data.
    pub last_error: Optional<String>,
    /// Exact creation timestamp; no ambient clock fills absence.
    pub created: Optional<String>,
    /// Exact update timestamp; no ambient clock fills absence.
    pub updated: Optional<String>,
}

/// Backend ingestion fields, before source translation or bundle/media hydration.
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BackendIngestionSession {
    /// Portal ingestion identity.
    pub id: Optional<String>,
    /// dApp identity.
    pub dapp_id: Optional<String>,
    /// Caller-issued idempotency identity reported by the portal.
    pub idempotency_key: Optional<String>,
    /// Exact backend source kind, including future names.
    pub source_kind: Optional<String>,
    /// Backend source location; no fetch is triggered by decoding.
    pub source_url: Optional<String>,
    /// Suggested APK file name.
    pub release_file_name: Optional<String>,
    /// Exact backend JSON number; no floating-point conversion is performed.
    #[serde(deserialize_with = "object::number")]
    pub release_file_size: Optional<Number>,
    /// Recorded release notes.
    pub whats_new: Optional<String>,
    /// Exact backend status, including Ready/ready and future values.
    pub status: Optional<String>,
    /// Linked release identity.
    pub release_id: Optional<String>,
    /// Linked publication-session identity.
    pub publication_session_id: Optional<String>,
    /// Optional complete typed bundle; arrays and malformed known fields reject.
    #[serde(deserialize_with = "object::optional")]
    pub bundle: Optional<BackendPublicationBundle>,
    /// Optional typed publication session.
    #[serde(deserialize_with = "object::optional")]
    pub publication_session: Optional<BackendPublicationSession>,
    /// Android package identifier reported by the portal.
    pub android_package: Optional<String>,
    /// Reported release version name.
    pub version_name: Optional<String>,
    /// Exact progress number, without truncating or clamping it.
    #[serde(deserialize_with = "object::number")]
    pub processing_progress: Optional<Number>,
    /// Exact processing stage.
    pub processing_stage: Optional<String>,
    /// Untrusted processing detail text.
    pub processing_detail: Optional<String>,
    /// Untrusted processing error text.
    pub processing_error: Optional<String>,
    /// Untrusted ingestion error text.
    pub error: Optional<String>,
}
