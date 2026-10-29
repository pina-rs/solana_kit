//! Closed endpoint catalog prevents request data from selecting procedures or HTTP verbs.

use super::{models::*, requests::*};
use serde::{Serialize, de::DeserializeOwned};

mod sealed {
    pub trait Sealed {}
}

/// A typed source-audited portal operation with one fixed procedure and response schema.
pub trait Operation: Serialize + sealed::Sealed {
    /// Typed object returned after successful tRPC envelope decoding.
    type Response: DeserializeOwned;
    /// Exact source procedure; request values never become path segments.
    const PROCEDURE: &'static str;
    /// Queries use GET with an encoded input pair; mutations use POST JSON.
    const QUERY: bool;
}

macro_rules! operation {
    ($request:ty, $reply:ty, $procedure:literal, $query:literal) => {
        impl sealed::Sealed for $request {}
        impl Operation for $request {
            type Response = $reply;
            const PROCEDURE: &'static str = $procedure;
            const QUERY: bool = $query;
        }
    };
}

operation!(GetBlockData, BlockData, "attestation.getBlockData", true);
operation!(
    CreateUploadTarget,
    PortalUploadTarget,
    "publication.createUploadTarget",
    false
);
operation!(
    CreateIngestionSession,
    BackendIngestionSession,
    "publication.createIngestionSession",
    false
);
operation!(
    GetIngestionSession,
    BackendIngestionSession,
    "publication.getIngestionSession",
    true
);
operation!(
    GetPublicationBundle,
    BackendPublicationBundle,
    "publication.getPublicationBundle",
    true
);
operation!(
    GetPublicationSession,
    BackendPublicationSession,
    "publication.getPublicationSession",
    true
);
operation!(
    PrepareReleaseNftTransaction,
    PreparedReleaseTransaction,
    "publication.prepareReleaseNftTransaction",
    false
);
operation!(
    PrepareVerifyCollectionTransaction,
    PreparedVerifyCollectionTransaction,
    "publication.prepareVerifyCollectionTransaction",
    false
);
operation!(
    SubmitSignedTransaction,
    SubmitSignedTransactionResult,
    "publication.submitSignedTransaction",
    false
);
operation!(
    SaveReleaseNftData,
    SaveReleaseNftDataResult,
    "publication.saveReleaseNftData",
    false
);
operation!(
    MarkReleaseCollectionAsVerified,
    MarkReleaseCollectionAsVerifiedResult,
    "publication.markReleaseCollectionAsVerified",
    false
);
operation!(
    CleanupRelease,
    CleanupReleaseResult,
    "publication.cleanupRelease",
    false
);
operation!(
    SubmitToStore,
    SubmitToStoreResult,
    "publication.submitToStore",
    false
);
