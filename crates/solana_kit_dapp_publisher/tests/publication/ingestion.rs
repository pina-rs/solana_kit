//! Independently authored normalized-projection behavior.

use super::support::*;

#[test]
fn source_kind_inference_preserves_unknown_values() {
    assert_eq!(
        infer_publication_source_kind(None),
        PortalSourceKind::PortalUpload
    );
    assert_eq!(
        infer_publication_source_kind(Some("portalUpload")),
        PortalSourceKind::PortalUpload
    );
    assert_eq!(
        infer_publication_source_kind(Some("existingRelease")),
        PortalSourceKind::PortalUpload
    );
    assert_eq!(
        infer_publication_source_kind(Some("externalUrl")),
        PortalSourceKind::ExternalUrl
    );
    assert_eq!(
        infer_publication_source_kind(Some("")),
        PortalSourceKind::Unrecognized(String::new())
    );
    assert_eq!(
        infer_publication_source_kind(Some("signedUrl")),
        PortalSourceKind::Unrecognized("signedUrl".into())
    );
}

fn ready_ingestion() -> PublicationIngestionSession {
    let backend = ingestion_session(json!({
        "id": "ing-3",
        "dappId": "dapp-1",
        "idempotencyKey": "idem-1",
        "whatsNew": "Bug fixes",
        "status": "Ready",
        "sourceKind": "externalUrl",
        "sourceUrl": "https://files.example.com/app.apk",
        "releaseFileName": "app.apk",
        "releaseFileSize": INSTALL_BYTES,
        "processingProgress": 100,
        "processingStage": "Done",
        "processingDetail": "APK processed",
        "androidPackage": "com.example.app",
        "versionName": "1.2.0",
        "publicationSessionId": "pub-3",
        "releaseId": "rel-3"
    }));
    let attached_bundle = bundle(source_bundle());
    let attached_session = publication_session(json!({"id": "pub-3", "stage": "PreparedForMint"}));

    translate_backend_ingestion_session_with_attachments(
        &backend,
        Some(&attached_bundle),
        Some(&attached_session),
        None,
    )
}

#[test]
fn translates_ready_ingestion_identity_and_progress() {
    let projected = ready_ingestion();

    assert!(projected.is_ready());
    assert!(!projected.is_failed());
    assert_eq!(projected.id, "ing-3");
    assert_eq!(projected.dapp_id, "dapp-1");
    assert_eq!(projected.idempotency_key, "idem-1");
    assert_eq!(projected.whats_new, "Bug fixes");

    assert_eq!(projected.release_id.as_deref(), Some("rel-3"));
    assert_eq!(projected.publication_session_id.as_deref(), Some("pub-3"));
    assert_eq!(
        projected.android_package.as_deref(),
        Some("com.example.app")
    );
    assert_eq!(projected.version_name.as_deref(), Some("1.2.0"));

    assert_eq!(
        projected
            .processing_progress
            .as_ref()
            .and_then(|value| value.as_u64()),
        Some(100)
    );
    assert_eq!(projected.processing_stage.as_deref(), Some("Done"));
    assert_eq!(
        projected.processing_detail.as_deref(),
        Some("APK processed")
    );
}

#[test]
fn translates_ready_ingestion_bundle_and_source() {
    let projected = ready_ingestion();
    let bundle = projected.bundle.as_ref().expect("attached bundle");

    assert_eq!(bundle.ingestion_session_id, "ing-3");
    assert_eq!(bundle.publication_session_id, "pub-3");
    assert_eq!(bundle.release_id, "rel-3");
    assert_eq!(
        projected
            .publication_session
            .as_ref()
            .expect("attached session")
            .id,
        "pub-3"
    );

    assert!(matches!(
        projected.source,
        PublicationSource::ExternalUrl { ref url, ref file_name }
            if url == "https://files.example.com/app.apk" && file_name.as_deref() == Some("app.apk")
    ));
}

#[test]
fn ingestion_status_and_source_fallbacks_are_distinct() {
    let defaulted =
        translate_backend_ingestion_session(&ingestion_session(json!({"id": "ing-4"})), None);
    assert_eq!(defaulted.status, "portalUpload");
    assert!(!defaulted.is_ready());
    assert!(!defaulted.is_failed());
    assert!(defaulted.bundle.is_none());
    assert!(defaulted.publication_session.is_none());
    assert!(matches!(
        defaulted.source,
        PublicationSource::PortalUpload {
            release_file_url: ref url,
            release_file_name: ref name,
            release_file_size: ref size,
        } if url.is_empty() && name.is_empty() && size.as_u64() == Some(0)
    ));

    let failed = translate_backend_ingestion_session(
        &ingestion_session(json!({
            "id": "ing-5",
            "status": "Failed",
            "error": "APK rejected",
            "sourceKind": "externalUrl"
        })),
        None,
    );
    assert!(failed.is_failed());
    assert_eq!(failed.error.as_deref(), Some("APK rejected"));

    let blank_status = translate_backend_ingestion_session(
        &ingestion_session(json!({"status": "", "sourceKind": "externalUrl"})),
        None,
    );
    assert_eq!(blank_status.status, "");

    let case_sensitive =
        translate_backend_ingestion_session(&ingestion_session(json!({"status": "READY"})), None);
    assert!(!case_sensitive.is_ready());
}

/// Existing releases reuse the translated bundle identity before the session field.
#[test]
fn existing_release_source_uses_the_translated_identity() {
    let existing = translate_backend_ingestion_session(
        &ingestion_session(json!({
            "id": "ing-6",
            "sourceKind": "existingRelease",
            "releaseId": "rel-6",
            "releaseFileName": "app.apk"
        })),
        None,
    );

    assert!(matches!(
        existing.source,
        PublicationSource::ExistingRelease { ref source_release_id } if source_release_id == "rel-6"
    ));
}

/// Future kinds retain their raw fields; portal uploads infer the final path name.
#[test]
fn unknown_and_portal_upload_sources_preserve_reported_fields() {
    let unknown = translate_backend_ingestion_session(
        &ingestion_session(json!({
            "id": "ing-7",
            "sourceKind": "signedUrl",
            "sourceUrl": "https://files.example.com/future.apk?token=1",
            "releaseFileSize": SMALL_FILE_BYTES
        })),
        None,
    );

    assert!(matches!(
        unknown.source,
        PublicationSource::Unrecognized {
            ref reported_kind,
            ref source_url,
            ref file_name,
            ref release_file_size
        } if reported_kind == "signedUrl"
            && source_url == "https://files.example.com/future.apk?token=1"
            && file_name.as_deref() == Some("future.apk")
            && release_file_size.as_u64() == Some(SMALL_FILE_BYTES)
    ));

    let upload = translate_backend_ingestion_session(
        &ingestion_session(json!({
            "id": "ing-8",
            "sourceKind": "portalUpload",
            "sourceUrl": "https://portal.example.com/app.apk",
            "releaseFileSize": 1024
        })),
        None,
    );

    assert!(matches!(
        upload.source,
        PublicationSource::PortalUpload {
            release_file_url: ref url,
            release_file_name: ref name,
            release_file_size: ref size,
        } if url == "https://portal.example.com/app.apk"
            && name == "app.apk"
            && size.as_u64() == Some(1024)
    ));
}

#[test]
fn explicit_attachments_replace_embedded_records() {
    let backend = ingestion_session(json!({
        "id": "ing-9",
        "releaseId": "rel-9",
        "bundle": source_bundle(),
        "publicationSession": {"id": "pub-9", "stage": "MintSubmitted"}
    }));
    let without = translate_backend_ingestion_session_with_attachments(&backend, None, None, None);
    assert!(without.bundle.is_none());
    assert!(without.publication_session.is_none());

    let projected = translate_backend_ingestion_session(&backend, None);
    assert!(projected.bundle.is_some());
    assert!(projected.publication_session.is_some());
    assert_eq!(
        projected
            .publication_session
            .as_ref()
            .expect("embedded session")
            .checkpoint,
        PublicationCheckpoint::MintSubmitted
    );
}

#[test]
fn metadata_uri_prefers_the_session_then_the_release_record() {
    let backend = ingestion_session(json!({
        "id": "ing-10",
        "releaseId": "rel-10",
        "bundle": {
            "release": {"id": "rel-10", "nftMetadataUri": "https://meta.example/release.json"}
        },
        "publicationSession": {
            "id": "pub-10",
            "metadataUri": "https://meta.example/session.json"
        }
    }));
    let projected = translate_backend_ingestion_session(&backend, None);
    let bundle = projected.bundle.as_ref().expect("bundle");
    assert_eq!(
        bundle.metadata.release_metadata_uri.as_deref(),
        Some("https://meta.example/session.json")
    );
    assert_eq!(
        bundle.release.release_metadata_uri.as_deref(),
        Some("https://meta.example/session.json")
    );

    let fallback = translate_backend_ingestion_session(
        &ingestion_session(json!({
            "id": "ing-11",
            "bundle": {
                "release": {"id": "rel-11", "nftMetadataUri": "https://meta.example/release.json"}
            },
            "publicationSession": {"id": "pub-11", "metadataUri": ""}
        })),
        None,
    );
    assert_eq!(
        fallback
            .bundle
            .as_ref()
            .expect("bundle")
            .metadata
            .release_metadata_uri
            .as_deref(),
        Some("https://meta.example/release.json")
    );
}
