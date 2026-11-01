//! Backend stage normalization never replays a transaction or store submission.

use super::{Error, text};
use crate::portal::BackendPublicationSession;

/// One unambiguous caller-selected recovery lookup.
pub enum ResumeTarget {
    /// Resume the last acknowledged checkpoint at the configured storage key.
    Snapshot,
    /// Resolve a publication session by its exact identity.
    Session(String),
    /// Resolve the current publication session for an exact release identity.
    Release(String),
}

/// Coarse pinned publication stage; future unknown stages require caller review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stage {
    /// Ready for caller-owned mint preparation.
    PreparedForMint,
    /// Mint submission was recorded by the backend.
    MintSubmitted,
    /// Mint data was saved.
    MintSaved,
    /// Collection-verification submission was recorded.
    VerificationSubmitted,
    /// Collection verification was recorded.
    Verified,
    /// Attestation was recorded.
    Attested,
    /// Store submission was recorded; no submission is performed here.
    Submitted,
    /// Publication failed.
    Failed,
}

impl Stage {
    /// Receipt fallbacks refine only PreparedForMint; explicit backend stages retain precedence.
    pub(super) fn resolve(session: &BackendPublicationSession) -> Result<Self, Error> {
        let stage = match text(&session.stage).unwrap_or("PreparedForMint") {
            "PreparedForMint" => Self::PreparedForMint,
            "MintSubmitted" => Self::MintSubmitted,
            "MintSaved" => Self::MintSaved,
            "VerificationSubmitted" => Self::VerificationSubmitted,
            "Verified" => Self::Verified,
            "Attested" => Self::Attested,
            "Submitted" => Self::Submitted,
            "Failed" => Self::Failed,
            _ => return Err(Error::Invalid("unknown publication stage")),
        };

        if stage != Self::PreparedForMint {
            return Ok(stage);
        }

        if text(&session.hubspot_ticket_id).is_some() {
            return Ok(Self::Submitted);
        }

        if text(&session.attestation_request_unique_id).is_some()
            || text(&session.verification_transaction_signature).is_some()
        {
            return Ok(Self::Verified);
        }

        if text(&session.mint_transaction_signature).is_some() {
            return Ok(Self::MintSubmitted);
        }

        Ok(stage)
    }
}
