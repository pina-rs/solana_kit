//! Independently authored normalized-projection behavior.

use super::support::*;

#[test]
fn checkpoint_stage_precedence_matches_the_pinned_translator() {
    let cases = [
        ("Submitted", PublicationCheckpoint::Submitted),
        ("Attested", PublicationCheckpoint::Verified),
        ("Verified", PublicationCheckpoint::Verified),
        ("VerificationSubmitted", PublicationCheckpoint::Verified),
        ("MintSaved", PublicationCheckpoint::MintSaved),
        ("MintSubmitted", PublicationCheckpoint::MintSubmitted),
        ("PreparedForMint", PublicationCheckpoint::BundleReady),
    ];

    for (stage, expected) in cases {
        let session = publication_session(json!({"stage": stage}));
        let projected = translate_backend_publication_session(&session, None);
        assert_eq!(projected.checkpoint, expected, "stage {stage}");
        assert_eq!(projected.stage.wire_name(), stage);
    }
}

#[test]
fn checkpoint_receipt_precedence_falls_through_failed_and_unknown_stages() {
    let cases = [
        (
            json!({"stage": "Failed", "hubspotTicketId": "HS-1"}),
            PublicationCheckpoint::Submitted,
        ),
        (
            json!({"stage": "Failed", "attestationRequestUniqueId": "att-1"}),
            PublicationCheckpoint::Verified,
        ),
        (
            json!({"stage": "Failed", "verificationTransactionSignature": "sig-1"}),
            PublicationCheckpoint::Verified,
        ),
        (
            json!({"stage": "Failed", "mintTransactionSignature": "sig-1"}),
            PublicationCheckpoint::MintSubmitted,
        ),
        (
            json!({"stage": "Failed", "expectedMintAddress": "Mint1111111111111111111111111111111111111"}),
            PublicationCheckpoint::BundleReady,
        ),
        (
            json!({"stage": "Failed", "metadataUri": "https://meta.example/one.json"}),
            PublicationCheckpoint::BundleReady,
        ),
        (json!({"stage": "Failed"}), PublicationCheckpoint::Created),
    ];

    for (value, expected) in cases {
        let projected = translate_backend_publication_session(&publication_session(value), None);
        assert_eq!(projected.checkpoint, expected);
    }
}

#[test]
fn checkpoint_receipt_precedence_orders_ticket_attestation_and_signatures() {
    let precedence = translate_backend_publication_session(
        &publication_session(json!({
            "stage": "Failed",
            "mintTransactionSignature": "mint",
            "verificationTransactionSignature": "verify",
            "attestationRequestUniqueId": "attestation",
            "hubspotTicketId": "ticket"
        })),
        None,
    );
    assert_eq!(precedence.checkpoint, PublicationCheckpoint::Submitted);

    let blanks = translate_backend_publication_session(
        &publication_session(json!({
            "stage": "Failed",
            "hubspotTicketId": "",
            "attestationRequestUniqueId": "",
            "verificationTransactionSignature": "",
            "mintTransactionSignature": "",
            "expectedMintAddress": "",
            "metadataUri": ""
        })),
        None,
    );
    assert_eq!(blanks.checkpoint, PublicationCheckpoint::Created);
}

#[test]
fn absent_null_blank_and_unknown_stages_remain_distinct() {
    let absent = translate_backend_publication_session(&publication_session(json!({})), None);
    assert_eq!(absent.stage, PublicationSessionStage::PreparedForMint);
    assert_eq!(absent.checkpoint, PublicationCheckpoint::BundleReady);
    assert_eq!(absent.status, PublicationSessionStatus::Running);

    let null =
        translate_backend_publication_session(&publication_session(json!({"stage": null})), None);
    assert_eq!(null.stage, PublicationSessionStage::PreparedForMint);
    assert_eq!(null.checkpoint, PublicationCheckpoint::BundleReady);

    let blank =
        translate_backend_publication_session(&publication_session(json!({"stage": ""})), None);
    assert_eq!(blank.stage, PublicationSessionStage::Unknown(String::new()));
    assert_eq!(blank.checkpoint, PublicationCheckpoint::Created);
    assert_eq!(blank.status, PublicationSessionStatus::Running);

    let unknown = translate_backend_publication_session(
        &publication_session(json!({"stage": "FutureStage", "mintTransactionSignature": "mint"})),
        None,
    );
    assert_eq!(
        unknown.stage,
        PublicationSessionStage::Unknown("FutureStage".into())
    );
    assert_eq!(unknown.checkpoint, PublicationCheckpoint::MintSubmitted);
    assert_eq!(unknown.status, PublicationSessionStatus::Running);
    assert_eq!(unknown.stage.wire_name(), "FutureStage");
}

#[test]
fn status_precedence_is_failed_then_completed_then_running() {
    let failed = translate_backend_publication_session(
        &publication_session(json!({"stage": "Failed", "hubspotTicketId": "HS-1"})),
        None,
    );
    assert_eq!(failed.status, PublicationSessionStatus::Failed);
    assert_eq!(failed.checkpoint, PublicationCheckpoint::Submitted);

    let submitted = translate_backend_publication_session(
        &publication_session(json!({"stage": "Submitted"})),
        None,
    );
    assert_eq!(submitted.status, PublicationSessionStatus::Completed);

    let ticket = translate_backend_publication_session(
        &publication_session(json!({"hubspotTicketId": "HS-2"})),
        None,
    );
    assert_eq!(ticket.status, PublicationSessionStatus::Completed);

    let blank_ticket = translate_backend_publication_session(
        &publication_session(json!({"hubspotTicketId": ""})),
        None,
    );
    assert_eq!(blank_ticket.status, PublicationSessionStatus::Running);
}

#[test]
fn publication_session_timestamps_never_sample_a_clock() {
    let absent = translate_backend_publication_session(&publication_session(json!({})), None);
    assert!(absent.created.is_none());
    assert!(absent.updated.is_none());

    let fallback = SessionTimestamps {
        created: "2026-11-04T00:00:00Z".into(),
        updated: "2026-11-04T01:00:00Z".into(),
    };
    let filled = translate_backend_publication_session(
        &publication_session(json!({"created": null})),
        Some(&fallback),
    );
    assert_eq!(filled.created.as_deref(), Some("2026-11-04T00:00:00Z"));
    assert_eq!(filled.updated.as_deref(), Some("2026-11-04T01:00:00Z"));

    let explicit = translate_backend_publication_session(
        &publication_session(json!({
            "created": "",
            "updated": "2026-01-02T00:00:00Z"
        })),
        Some(&fallback),
    );
    assert_eq!(explicit.created.as_deref(), Some(""));
    assert_eq!(explicit.updated.as_deref(), Some("2026-01-02T00:00:00Z"));
}

#[test]
fn publication_session_retains_error_text_only_as_explicit_data() {
    let projected = translate_backend_publication_session(
        &publication_session(json!({
            "id": "pub-2",
            "ingestionSessionId": "ing-2",
            "releaseId": "rel-2",
            "expectedMintAddress": "Mint2222222222222222222222222222222222222",
            "lastError": "provider-secret-detail"
        })),
        None,
    );

    assert_eq!(projected.id, "pub-2");
    assert_eq!(projected.ingestion_session_id, "ing-2");
    assert_eq!(projected.release_id, "rel-2");
    assert_eq!(
        projected.last_error.as_deref(),
        Some("provider-secret-detail")
    );
    assert_eq!(projected.error(), Some("provider-secret-detail"));
    assert_eq!(
        projected.release_mint_address.as_deref(),
        Some("Mint2222222222222222222222222222222222222")
    );

    let diagnostic = format!("{projected:?}");
    assert!(!diagnostic.contains("provider-secret-detail"));
    assert!(!diagnostic.contains("pub-2"));
    assert!(diagnostic.contains("checkpoint"));
}

#[test]
fn checkpoint_wire_names_and_ordering_are_stable() {
    let checkpoints = [
        PublicationCheckpoint::Created,
        PublicationCheckpoint::BundleReady,
        PublicationCheckpoint::MintSubmitted,
        PublicationCheckpoint::MintSaved,
        PublicationCheckpoint::VerificationSubmitted,
        PublicationCheckpoint::Verified,
        PublicationCheckpoint::Attested,
        PublicationCheckpoint::Submitted,
        PublicationCheckpoint::Completed,
    ];

    for (index, checkpoint) in checkpoints.iter().enumerate() {
        assert_eq!(
            PublicationCheckpoint::from_wire(checkpoint.wire_name()),
            *checkpoint
        );
        assert!(checkpoint.is_at_least(*checkpoint));

        if index > 0 {
            let earlier = checkpoints[index - 1];
            assert!(checkpoint.is_at_least(earlier));
            assert!(!earlier.is_at_least(*checkpoint));
        }
    }

    assert_eq!(
        PublicationCheckpoint::from_wire("future-checkpoint"),
        PublicationCheckpoint::Created
    );
    assert_eq!(
        PublicationSessionStatus::from_wire(Some("future-status")),
        PublicationSessionStatus::Running
    );
    assert_eq!(
        PublicationSessionStatus::from_wire(Some("pending")),
        PublicationSessionStatus::Pending
    );
    assert_eq!(
        PublicationSessionStatus::from_wire(None),
        PublicationSessionStatus::Running
    );
    assert_eq!(
        PublicationSessionStage::from_wire(Some("future-stage")),
        PublicationSessionStage::Unknown("future-stage".into())
    );
}

#[test]
fn markers_can_be_normalized_directly() {
    let session = publication_session(json!({
        "stage": "Verified",
        "hubspotTicketId": "ticket"
    }));
    let markers = SessionMarkers::from_backend(&session);

    assert_eq!(
        normalize_publication_checkpoint(markers),
        PublicationCheckpoint::Verified
    );
    assert_eq!(
        normalize_publication_status(markers),
        PublicationSessionStatus::Completed
    );
}
