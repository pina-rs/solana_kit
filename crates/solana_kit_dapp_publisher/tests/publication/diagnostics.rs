//! Independently authored normalized-projection behavior.

use std::format;

use super::support::*;

#[cfg_attr(test, test)]
pub fn progress_events_use_static_source_messages() -> Outcome {
    assert_eq!(ProgressStep::MintSave.wire_name(), "mint.save");
    assert_eq!(
        ProgressEvent::new(ProgressStep::MintSave, ProgressStatus::Running).message(),
        Some("Saving release NFT data")
    );
    assert_eq!(
        ProgressEvent::new(ProgressStep::MintSave, ProgressStatus::Complete).message(),
        Some("Release NFT data saved")
    );
    assert_eq!(
        ProgressEvent::new(ProgressStep::MintSubmit, ProgressStatus::Running).message(),
        None
    );
    assert_eq!(
        ProgressEvent::new(ProgressStep::MintSubmit, ProgressStatus::Complete).message(),
        Some("Release NFT transaction submitted")
    );
    assert_eq!(
        ProgressEvent::new(ProgressStep::CleanupRelease, ProgressStatus::Complete).message(),
        None
    );
    assert_eq!(ProgressStatus::Running.wire_name(), "running");
    assert_eq!(ProgressStatus::Complete.wire_name(), "complete");

    assert_eq!(
        CleanupOutcome::from_action("deleted"),
        CleanupOutcome::Deleted
    );
    assert_eq!(
        CleanupOutcome::from_action("preservedSubmitted").message(),
        "Publication already reached the submitted state; preserving release"
    );
    assert_eq!(
        CleanupOutcome::from_action("futureAction"),
        CleanupOutcome::Unrecognized
    );
    assert!(
        CleanupOutcome::from_action("futureAction")
            .message()
            .contains("unrecognized")
    );

    Ok(())
}

#[cfg_attr(test, test)]
pub fn ingestion_progress_never_promotes_provider_text() -> Outcome {
    let detail = translate_backend_ingestion_session(
        &ingestion_session(json!({
            "status": "processing",
            "processingDetail": "provider detail that must not be logged"
        })),
        None,
    );
    let progress = detail.progress().ok_or("missing reported progress")?;
    assert_eq!(progress, IngestionProgress::ReportedDetail);
    assert_eq!(progress.message(), "Portal ingestion is running");
    assert!(!progress.message().contains("provider detail"));

    let stage = translate_backend_ingestion_session(
        &ingestion_session(json!({"status": "processing", "processingStage": "unpacking"})),
        None,
    );
    assert_eq!(stage.progress(), Some(IngestionProgress::ReportedStage));

    for (status, expected) in [
        ("created", IngestionProgress::Created),
        ("queued", IngestionProgress::Queued),
        ("processing", IngestionProgress::Processing),
        ("Ready", IngestionProgress::Ready),
        ("ready", IngestionProgress::Ready),
    ] {
        let session = translate_backend_ingestion_session(
            &ingestion_session(json!({"status": status})),
            None,
        );
        assert_eq!(session.progress(), Some(expected), "status {status}");
    }

    let failed = translate_backend_ingestion_session(
        &ingestion_session(json!({"status": "Failed", "statusDetail": "ignored"})),
        None,
    );
    assert_eq!(failed.progress(), None);

    Ok(())
}

#[cfg_attr(test, test)]
pub fn debug_output_omits_provider_and_application_text() -> Outcome {
    let session = translate_backend_publication_session(
        &publication_session(json!({
            "id": "secret-id",
            "lastError": "secret-provider-error",
            "stage": "Failed"
        })),
        None,
    );
    let diagnostic = format!("{session:?}");
    assert!(!diagnostic.contains("secret-provider-error"));
    assert!(!diagnostic.contains("secret-id"));
    assert!(diagnostic.contains("Failed"));

    let ingestion = translate_backend_ingestion_session(
        &ingestion_session(json!({
            "id": "secret-ingestion",
            "status": "Failed",
            "error": "secret-ingestion-error",
            "processingDetail": "secret-processing-detail"
        })),
        None,
    );
    let diagnostic = format!("{ingestion:?}");
    assert!(!diagnostic.contains("secret-ingestion-error"));
    assert!(!diagnostic.contains("secret-processing-detail"));
    assert!(!diagnostic.contains("secret-ingestion"));
    assert!(diagnostic.contains("failed: true"));

    assert!(
        !format!("{:?}", PublisherKind::Unrecognized("secret-kind".into())).contains("secret-kind")
    );
    assert!(
        !format!(
            "{:?}",
            PortalSourceKind::Unrecognized("secret-source".into())
        )
        .contains("secret-source")
    );

    Ok(())
}
