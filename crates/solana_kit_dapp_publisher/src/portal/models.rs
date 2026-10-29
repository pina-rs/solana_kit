//! Wire DTOs preserve provider proposals; no field is authorization or proof of publication.

mod bundle;
mod session;
use super::Optional;
pub use bundle::*;
use serde::Deserialize;
pub use session::*;
use std::fmt;

/// Slot and blockhash supplied by the portal; no freshness is inferred.
#[derive(Deserialize)]
pub struct BlockData {
    /// Exact unsigned slot. Fractional, negative, missing and null values reject.
    pub slot: u64,
    /// Untrusted blockhash text for explicit caller conversion to Kit Hash.
    pub blockhash: String,
}

/// Separate presigned upload capability, never an approved PortalService origin.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalUploadTarget {
    /// Sensitive presigned PUT URL. Read explicitly; ordinary diagnostics redact it.
    pub upload_url: String,
    /// Public location proposed for the uploaded artifact.
    pub public_url: String,
}

impl fmt::Debug for PortalUploadTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PortalUploadTarget { .. }")
    }
}

/// Untrusted prepared release transaction; caller policy/signing remains a separate step.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedReleaseTransaction {
    /// Base64 transaction text; decoding does not validate publication policy.
    pub transaction: String,
    /// Proposed release mint text.
    pub mint_address: String,
    /// Proposed recent blockhash text.
    pub blockhash: String,
}

/// Untrusted collection-verification proposal, with no automatic signing/submission.
#[derive(Deserialize)]
pub struct PreparedVerifyCollectionTransaction {
    /// Base64 transaction text for explicit caller validation.
    pub transaction: String,
    /// Proposed recent blockhash text.
    pub blockhash: String,
}

/// Submission receipt; a returned signature does not establish landing or confirmation.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitSignedTransactionResult {
    /// Exact signature text reported by the portal.
    pub transaction_signature: String,
}

/// Provider acknowledgment; false remains a failed acknowledgment, not a transport error.
#[derive(Deserialize)]
pub struct SaveReleaseNftDataResult {
    /// Explicit provider acknowledgment; missing/null/wrong type reject.
    pub success: bool,
}

/// Provider acknowledgment of collection verification state.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkReleaseCollectionAsVerifiedResult {
    /// Explicit provider acknowledgment.
    pub success: bool,
    /// Exact reported release identity.
    pub release_id: String,
}

/// Cleanup receipt retaining open provider action text.
#[derive(Deserialize)]
pub struct CleanupReleaseResult {
    /// Reported action, such as deleted or preservedSubmitted.
    pub action: String,
}

/// Store-review submission receipt, without inferring acceptance or completion.
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SubmitToStoreResult {
    /// Absent, explicit null or exact ticket text remain distinct.
    pub hubspot_ticket_id: Optional<String>,
}
