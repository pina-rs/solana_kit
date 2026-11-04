//! Ingestion-session source, status and attached-record projection.

use super::bundle::map_backend_bundle_to_publication_bundle;
use super::session::translate_backend_publication_session;
use super::support::{
    file_name_from_url, first_non_empty, first_present, string_or_empty, text, value,
};
use crate::portal::{BackendIngestionSession, BackendPublicationBundle, BackendPublicationSession};
use crate::publication::{
    PortalSourceKind, PublicationBundle, PublicationIngestionSession, PublicationSession,
    PublicationSource, SessionTimestamps,
};
use serde_json::Number;

/// Infers the portal hosting kind from a backend `sourceKind` value.
///
/// `externalUrl` is the only external value. `portalUpload`, `existingRelease`,
/// absent and explicit null values are portal-managed, matching the source.
/// Blank and future values are retained as [`PortalSourceKind::Unrecognized`]
/// rather than being presented as portal uploads.
#[must_use]
pub fn infer_publication_source_kind(source_kind: Option<&str>) -> PortalSourceKind {
    match source_kind {
        None | Some("portalUpload" | "existingRelease") => PortalSourceKind::PortalUpload,
        Some("externalUrl") => PortalSourceKind::ExternalUrl,
        Some(other) => PortalSourceKind::Unrecognized(other.to_owned()),
    }
}

/// Projects one typed ingestion session using its embedded bundle and session.
#[must_use]
pub fn translate_backend_ingestion_session(
    session: &BackendIngestionSession,
    timestamps: Option<&SessionTimestamps>,
) -> PublicationIngestionSession {
    translate_backend_ingestion_session_with_attachments(
        session,
        value(&session.bundle),
        value(&session.publication_session),
        timestamps,
    )
}

/// Projects an ingestion session with caller-supplied attached records.
///
/// Attachments are explicit and replace any embedded records: passing `None`
/// projects no bundle/session even when the backend record contains one. This
/// mirrors the pinned translator's optional arguments and prevents a second
/// implicit read.
#[must_use]
pub fn translate_backend_ingestion_session_with_attachments(
    session: &BackendIngestionSession,
    bundle: Option<&BackendPublicationBundle>,
    publication_session: Option<&BackendPublicationSession>,
    timestamps: Option<&SessionTimestamps>,
) -> PublicationIngestionSession {
    let source_kind = text(&session.source_kind).unwrap_or("portalUpload");
    let translated_publication = publication_session
        .map(|session| translate_backend_publication_session(session, timestamps));
    let translated_bundle = bundle.map(|bundle| {
        attached_bundle(
            bundle,
            session,
            publication_session,
            translated_publication.as_ref(),
            source_kind,
        )
    });

    PublicationIngestionSession {
        id: string_or_empty(&session.id),
        dapp_id: string_or_empty(&session.dapp_id),
        idempotency_key: string_or_empty(&session.idempotency_key),
        source: projected_source(session, source_kind, translated_bundle.as_ref()),
        whats_new: string_or_empty(&session.whats_new),
        status: text(&session.status)
            .map(str::to_owned)
            .unwrap_or_else(|| source_kind.to_owned()),
        release_id: text(&session.release_id).map(str::to_owned),
        publication_session_id: text(&session.publication_session_id).map(str::to_owned),
        bundle: translated_bundle,
        publication_session: translated_publication,
        android_package: text(&session.android_package).map(str::to_owned),
        version_name: text(&session.version_name).map(str::to_owned),
        processing_progress: value(&session.processing_progress).cloned(),
        processing_stage: text(&session.processing_stage).map(str::to_owned),
        processing_detail: text(&session.processing_detail).map(str::to_owned),
        processing_error: text(&session.processing_error).map(str::to_owned),
        error: text(&session.error).map(str::to_owned),
    }
}

fn attached_bundle(
    bundle: &BackendPublicationBundle,
    session: &BackendIngestionSession,
    publication_session: Option<&BackendPublicationSession>,
    translated_publication: Option<&PublicationSession>,
    source_kind: &str,
) -> PublicationBundle {
    let metadata_uri = first_non_empty(&[
        publication_session.and_then(|session| text(&session.metadata_uri)),
        value(&bundle.release).and_then(|release| text(&release.nft_metadata_uri)),
    ]);
    let projected = map_backend_bundle_to_publication_bundle(
        bundle,
        metadata_uri,
        infer_publication_source_kind(Some(source_kind)),
    );

    with_session_identifiers(projected, session, translated_publication)
}

fn with_session_identifiers(
    bundle: PublicationBundle,
    session: &BackendIngestionSession,
    publication_session: Option<&PublicationSession>,
) -> PublicationBundle {
    let publication_session_id = first_present(&[
        text(&session.publication_session_id),
        publication_session.map(|session| session.id.as_str()),
    ])
    .unwrap_or_default()
    .to_owned();
    let release_id = text(&session.release_id)
        .map(str::to_owned)
        .unwrap_or_else(|| bundle.release_id.clone());

    PublicationBundle {
        ingestion_session_id: string_or_empty(&session.id),
        publication_session_id,
        release_id,
        ..bundle
    }
}

fn projected_source(
    session: &BackendIngestionSession,
    source_kind: &str,
    bundle: Option<&PublicationBundle>,
) -> PublicationSource {
    let source_url = text(&session.source_url).unwrap_or_default().to_owned();
    let file_name = text(&session.release_file_name)
        .map(str::to_owned)
        .or_else(|| file_name_from_url(&source_url));
    let release_file_size = value(&session.release_file_size)
        .cloned()
        .unwrap_or_else(|| Number::from(0));

    publication_source(
        source_kind,
        source_url,
        file_name,
        release_file_size,
        bundle,
        text(&session.release_id),
    )
}

fn publication_source(
    source_kind: &str,
    source_url: String,
    file_name: Option<String>,
    release_file_size: Number,
    bundle: Option<&PublicationBundle>,
    release_id: Option<&str>,
) -> PublicationSource {
    match source_kind {
        "existingRelease" => PublicationSource::ExistingRelease {
            source_release_id: bundle
                .map(|bundle| bundle.release_id.clone())
                .unwrap_or_else(|| release_id.unwrap_or_default().to_owned()),
        },
        "externalUrl" => PublicationSource::ExternalUrl {
            url: source_url,
            file_name,
        },
        "portalUpload" => PublicationSource::PortalUpload {
            release_file_url: source_url,
            release_file_name: file_name.unwrap_or_default(),
            release_file_size,
        },
        other => PublicationSource::Unrecognized {
            reported_kind: other.to_owned(),
            source_url,
            file_name,
            release_file_size,
        },
    }
}
