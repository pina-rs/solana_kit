//! Checkpoints record intent before effects and acknowledged receipts afterward.

use super::Error;
use serde::{Deserialize, Serialize};
use solana_kit_client::rest::Budgets;

mod admission;
mod records;

/// An effect whose outcome may require caller reconciliation after failure or cancellation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Effect {
    /// Idempotent ingestion creation has not been acknowledged durably.
    CreateIngestion,
    /// A presigned target creation has not been acknowledged durably.
    CreateUploadTarget,
    /// Binary PUT has not been acknowledged durably.
    Upload,
    /// Explicit failed-release cleanup has not been acknowledged durably.
    Cleanup,
}

/// Public upload identity retained for explicit reconciliation, without the presigned PUT URL.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UploadIntent {
    pub(super) hash: String,
    pub(super) mime: String,
    pub(super) extension: String,
    pub(super) public_url: Option<String>,
}

impl UploadIntent {
    /// Exact SHA-256 of the intended immutable upload bytes.
    pub fn sha256(&self) -> &str {
        &self.hash
    }
    /// Intended media type.
    pub fn content_type(&self) -> &str {
        &self.mime
    }
    /// Acknowledged target's public location, if its creation was received.
    pub fn public_url(&self) -> Option<&str> {
        self.public_url.as_deref()
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct UploadReceipt {
    pub hash: String,
    pub mime: String,
    pub extension: String,
    pub public_url: String,
}

/// Versioned local recovery data. It contains no API key, presigned URL or local path.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub(super) version: u8,
    pub(super) input_hash: Option<String>,
    pub(super) idempotency_key: Option<String>,
    pub(super) ingestion_id: Option<String>,
    pub(super) release_id: Option<String>,
    pub(super) publication_id: Option<String>,
    pub(super) metadata_uri: Option<String>,
    pub(super) updated_on: Option<String>,
    pub(super) pending: Option<Effect>,
    #[serde(default, deserialize_with = "records::intent")]
    pub(super) pending_upload: Option<UploadIntent>,
    #[serde(deserialize_with = "records::receipts")]
    pub(super) uploads: Vec<UploadReceipt>,
    pub(super) deleted: bool,
    pub(super) preserved_submitted: bool,
}

impl Snapshot {
    pub(super) fn new() -> Self {
        Self {
            version: 1,
            input_hash: None,
            idempotency_key: None,
            ingestion_id: None,
            release_id: None,
            publication_id: None,
            metadata_uri: None,
            updated_on: None,
            pending: None,
            pending_upload: None,
            uploads: Vec::new(),
            deleted: false,
            preserved_submitted: false,
        }
    }

    /// Decodes one bounded, duplicate-aware checkpoint; unsupported versions reject.
    pub fn decode(bytes: &[u8], budgets: Budgets) -> Result<Self, Error> {
        budgets
            .validate_json(bytes)
            .map_err(|error| Error::Portal(crate::portal::Error::Json(error)))?;

        if !bytes
            .iter()
            .copied()
            .find(|byte| !byte.is_ascii_whitespace())
            .is_some_and(|byte| byte == b'{')
        {
            return Err(Error::Invalid("checkpoint must be an object"));
        }

        let snapshot: Self = serde_json::from_slice(bytes)
            .map_err(|error| Error::Portal(crate::portal::Error::Schema(error)))?;

        if snapshot.version != 1 {
            return Err(Error::Invalid("unsupported checkpoint version"));
        }

        admission::validate(&snapshot)?;
        Ok(snapshot)
    }

    /// Pending intent, which never authorizes automatic replay.
    pub const fn pending(&self) -> Option<Effect> {
        self.pending
    }
    /// Last acknowledged ingestion identity, when known.
    pub fn ingestion_id(&self) -> Option<&str> {
        self.ingestion_id.as_deref()
    }
    /// Last acknowledged release identity, when known.
    pub fn release_id(&self) -> Option<&str> {
        self.release_id.as_deref()
    }
    /// Last acknowledged publication session identity.
    pub fn publication_session_id(&self) -> Option<&str> {
        self.publication_id.as_deref()
    }
    /// Cached acknowledged metadata URI; no upload is inferred from its absence.
    pub fn metadata_uri(&self) -> Option<&str> {
        self.metadata_uri.as_deref()
    }
    /// Non-secret artifact identity for reconciling an interrupted target creation or PUT.
    pub fn pending_upload(&self) -> Option<&UploadIntent> {
        self.pending_upload.as_ref()
    }

    pub(super) fn admit(&self) -> Result<(), Error> {
        if let Some(effect) = self.pending {
            return Err(Error::Uncertain(effect));
        }

        if self.deleted {
            return Err(Error::Invalid("release was deleted"));
        }

        Ok(())
    }
}
