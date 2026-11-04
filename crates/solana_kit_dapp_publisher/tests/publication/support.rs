//! Shared fixtures and parsers for normalized-projection tests.

use serde_json::Value;
pub(super) use serde_json::json;
pub(super) use solana_kit_dapp_publisher::portal::{
    BackendIngestionSession, BackendInstallFile, BackendPublicationBundle,
    BackendPublicationSession, BackendRelease,
};
pub(super) use solana_kit_dapp_publisher::publication::{
    CleanupOutcome, IngestionProgress, InstallFileDetails, MediaPurpose, PortalSourceKind,
    ProgressEvent, ProgressStatus, ProgressStep, PublicationBundle, PublicationCheckpoint,
    PublicationIngestionSession, PublicationSessionStage, PublicationSessionStatus,
    PublicationSource, PublisherKind, SessionMarkers, SessionTimestamps,
    infer_publication_source_kind, map_backend_bundle_to_publication_bundle,
    normalize_publication_checkpoint, normalize_publication_status,
    translate_backend_ingestion_session, translate_backend_ingestion_session_with_attachments,
    translate_backend_publication_session,
};

pub(super) const VERSION_CODE: u64 = 120;
pub(super) const MIN_SDK_VERSION: u64 = 21;
pub(super) const TARGET_SDK_VERSION: u64 = 34;
pub(super) const INSTALL_BYTES: u64 = 2048;
pub(super) const SMALL_FILE_BYTES: u64 = 7;

pub(super) fn bundle(value: Value) -> BackendPublicationBundle {
    serde_json::from_value(value).expect("valid backend bundle fixture")
}

pub(super) fn publication_session(value: Value) -> BackendPublicationSession {
    serde_json::from_value(value).expect("valid backend session fixture")
}

pub(super) fn ingestion_session(value: Value) -> BackendIngestionSession {
    serde_json::from_value(value).expect("valid ingestion fixture")
}

pub(super) fn source_bundle() -> Value {
    json!({
        "ingestionSessionId": "ing-1",
        "publicationSessionId": "pub-1",
        "releaseId": "rel-1",
        "release": {
            "id": "rel-1",
            "dappId": "dapp-1",
            "androidPackage": "com.example.app",
            "versionName": "1.2.0",
            "versionCode": VERSION_CODE,
            "minSdkVersion": MIN_SDK_VERSION,
            "targetSdkVersion": TARGET_SDK_VERSION,
            "permissions": ["INTERNET"],
            "locales": ["en-US"],
            "certificateFingerprint": "AA:BB",
            "newInVersion": "Faster",
            "localizedName": "My App",
            "releaseFileUrl": "https://files.example.com/app.apk",
            "releaseFileName": "app.apk",
            "releaseFileSize": INSTALL_BYTES,
            "releaseFileHash": "abc123",
            "nftMintAddress": "9zM8vL2bFQhR4mkKKB1EJZDP4XcXKXbP7Lz9dH3nV4tQ"
        },
        "dapp": {
            "id": "dapp-1",
            "dappName": "My App",
            "subtitle": "Short",
            "description": "Long description",
            "androidPackage": "com.example.app",
            "dappIconUrl": "https://img.example.com/icon.png",
            "dappPreviewUrls": ["https://img.example.com/1.png"],
            "walletAddress": "Wa11et1111111111111111111111111111111111111",
            "nftMintAddress": "App111111111111111111111111111111111111111",
            "languages": ["en-US"]
        },
        "publisher": {
            "id": "pub-1",
            "type": "organization",
            "name": "Example Inc",
            "website": "https://example.com",
            "email": "contact@example.com",
            "supportEmail": "support@example.com"
        },
        "installFile": {
            "uri": "https://files.example.com/app.apk",
            "mimeType": "application/vnd.android.package-archive",
            "size": INSTALL_BYTES,
            "sha256": "deadbeef"
        },
        "signerAuthority": {
            "dappWalletAddress": "Wa11et1111111111111111111111111111111111111",
            "collectionAuthority": "Co111ection11111111111111111111111111111111",
            "appMintAddress": "App111111111111111111111111111111111111111",
            "sameSignerRequired": true,
            "acceptedSignerRoles": ["publisher", "payer"],
            "feePayer": "Payer111111111111111111111111111111111111111"
        }
    })
}
