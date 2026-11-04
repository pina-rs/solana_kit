//! Publication-session checkpoint, status and timestamp projection.

use super::support::{is_non_empty, string_or_empty, text};
use crate::{
    portal::BackendPublicationSession,
    publication::{
        PublicationCheckpoint, PublicationSession, PublicationSessionStage,
        PublicationSessionStatus, SessionTimestamps,
    },
};

/// Session fields consulted by status and checkpoint normalization.
#[derive(Clone, Copy, Default)]
pub struct SessionMarkers<'a> {
    /// Exact reported stage; absent and explicit null are both `None`.
    pub stage: Option<&'a str>,
    /// Exact mint-submission signature text.
    pub mint_transaction_signature: Option<&'a str>,
    /// Exact collection-verification signature text.
    pub verification_transaction_signature: Option<&'a str>,
    /// Exact attestation request identity.
    pub attestation_request_unique_id: Option<&'a str>,
    /// Exact store-review ticket.
    pub hubspot_ticket_id: Option<&'a str>,
    /// Exact expected release mint text.
    pub expected_mint_address: Option<&'a str>,
    /// Exact metadata location.
    pub metadata_uri: Option<&'a str>,
}

impl<'a> SessionMarkers<'a> {
    /// Reads markers from a typed publication session without trimming them.
    #[must_use]
    pub fn from_backend(session: &'a BackendPublicationSession) -> Self {
        Self {
            stage: text(&session.stage),
            mint_transaction_signature: text(&session.mint_transaction_signature),
            verification_transaction_signature: text(&session.verification_transaction_signature),
            attestation_request_unique_id: text(&session.attestation_request_unique_id),
            hubspot_ticket_id: text(&session.hubspot_ticket_id),
            expected_mint_address: text(&session.expected_mint_address),
            metadata_uri: text(&session.metadata_uri),
        }
    }
}

/// Normalizes the precise checkpoint from stage and receipt evidence.
///
/// Explicit known stages retain precedence. `Attested` maps to
/// [`PublicationCheckpoint::Verified`] exactly as the pinned source does, while
/// `Submitted` maps to [`PublicationCheckpoint::Submitted`]. Any other stage,
/// including `Failed`, blank or a future name, uses receipt precedence:
/// ticket, attestation, verification signature, mint signature, expected mint
/// or metadata, then [`PublicationCheckpoint::Created`].
///
/// An absent or null stage is treated as `PreparedForMint`, matching the source
/// session translator. Blank values are not trimmed into absence.
#[must_use]
pub fn normalize_publication_checkpoint(markers: SessionMarkers<'_>) -> PublicationCheckpoint {
    if let Some(checkpoint) = explicit_checkpoint(markers.stage.unwrap_or("PreparedForMint")) {
        return checkpoint;
    }

    receipt_checkpoint(markers)
}

fn explicit_checkpoint(stage: &str) -> Option<PublicationCheckpoint> {
    match stage {
        "Submitted" => Some(PublicationCheckpoint::Submitted),
        "Attested" | "Verified" | "VerificationSubmitted" => Some(PublicationCheckpoint::Verified),
        "MintSaved" => Some(PublicationCheckpoint::MintSaved),
        "MintSubmitted" => Some(PublicationCheckpoint::MintSubmitted),
        "PreparedForMint" => Some(PublicationCheckpoint::BundleReady),
        _ => None,
    }
}

fn receipt_checkpoint(markers: SessionMarkers<'_>) -> PublicationCheckpoint {
    if is_non_empty(markers.hubspot_ticket_id) {
        return PublicationCheckpoint::Submitted;
    }

    if is_non_empty(markers.attestation_request_unique_id)
        || is_non_empty(markers.verification_transaction_signature)
    {
        return PublicationCheckpoint::Verified;
    }

    mint_checkpoint(markers)
}

fn mint_checkpoint(markers: SessionMarkers<'_>) -> PublicationCheckpoint {
    if is_non_empty(markers.mint_transaction_signature) {
        return PublicationCheckpoint::MintSubmitted;
    }

    if is_non_empty(markers.expected_mint_address) || is_non_empty(markers.metadata_uri) {
        return PublicationCheckpoint::BundleReady;
    }

    PublicationCheckpoint::Created
}

/// Normalizes the coarse session status from stage and ticket evidence.
///
/// `Failed` wins first. `Submitted` or a nonempty ticket completes the session.
/// Every other stage, including a future name, remains running; unknown status
/// is never presented as completion.
#[must_use]
pub fn normalize_publication_status(markers: SessionMarkers<'_>) -> PublicationSessionStatus {
    if markers.stage == Some("Failed") {
        return PublicationSessionStatus::Failed;
    }

    if markers.stage == Some("Submitted") || is_non_empty(markers.hubspot_ticket_id) {
        return PublicationSessionStatus::Completed;
    }

    PublicationSessionStatus::Running
}

/// Projects one typed backend publication session.
///
/// Timestamps are copied exactly. An absent or null timestamp uses the supplied
/// fallback, or remains absent; this function never samples the wall clock.
#[must_use]
pub fn translate_backend_publication_session(
    session: &BackendPublicationSession,
    timestamps: Option<&SessionTimestamps>,
) -> PublicationSession {
    let markers = SessionMarkers::from_backend(session);
    let stage_text = markers.stage.unwrap_or("PreparedForMint");

    PublicationSession {
        id: string_or_empty(&session.id),
        ingestion_session_id: string_or_empty(&session.ingestion_session_id),
        release_id: string_or_empty(&session.release_id),
        status: normalize_publication_status(markers),
        stage: PublicationSessionStage::from_wire(Some(stage_text)),
        checkpoint: normalize_publication_checkpoint(markers),
        metadata_uri: markers.metadata_uri.map(str::to_owned),
        release_mint_address: markers.expected_mint_address.map(str::to_owned),
        mint_transaction_signature: markers.mint_transaction_signature.map(str::to_owned),
        verify_transaction_signature: markers
            .verification_transaction_signature
            .map(str::to_owned),
        attestation_request_unique_id: markers.attestation_request_unique_id.map(str::to_owned),
        hubspot_ticket_id: markers.hubspot_ticket_id.map(str::to_owned),
        last_error: text(&session.last_error).map(str::to_owned),
        created: timestamp(
            &session.created,
            timestamps.map(|value| value.created.as_str()),
        ),
        updated: timestamp(
            &session.updated,
            timestamps.map(|value| value.updated.as_str()),
        ),
    }
}

fn timestamp(value: &crate::portal::Optional<String>, fallback: Option<&str>) -> Option<String> {
    text(value)
        .map(str::to_owned)
        .or_else(|| fallback.map(str::to_owned))
}
