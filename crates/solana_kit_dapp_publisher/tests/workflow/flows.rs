//! Real public start/resume methods consume independently authored portal and storage responses.

use super::{
    checks::{Outcome, require},
    fixtures::*,
    support::*,
};
use serde_json::{Value, json as json_value};
use solana_kit_client::{Presence, rest};
use solana_kit_dapp_publisher::workflow::{HydratedPublication, ResumeTarget, Stage, Workflow};
use std::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[cfg_attr(test, test)]
pub fn new_ingestion_polls_rehosts_media_and_uploads_the_complete_typed_document() -> Outcome {
    new_ingestion_metadata_matches(METADATA)
}

pub fn new_ingestion_metadata_matches(expected_metadata: &str) -> Outcome {
    let mut steps = vec![created(), queued(), ready()];
    steps.extend(hydrate_steps(ICON));
    let mut rig = Rig::new(steps);
    let result = complete(rig.workflow()?.start(&input()))?
        .map_err(|_| "workflow flows fixture 1 missing or invalid")?;

    assert_hydrated(&result, &rig, expected_metadata)?;
    require(
        rig.procedures()?
            == [
                "publication.createIngestionSession",
                "publication.getIngestionSession",
                "publication.getIngestionSession",
                "publication.getPublicationSession",
                "publication.getPublicationBundle",
                "fetchRemoteFile",
                "publication.createUploadTarget",
                "publication.createUploadTarget",
            ],
        "workflow flows predicate 1 mismatch",
    )?;
    let waits: Vec<_> = rig
        .clock
        .sleeps
        .borrow()
        .iter()
        .copied()
        .filter(|tick| *tick != DEADLINE)
        .collect();
    require(
        waits == [limits().poll_interval_ms],
        "workflow flows predicate 2 mismatch",
    )?;
    assert_uploads(&rig)?;
    assert_fetch_and_target(&rig)?;
    rig.valid()?;
    Ok(())
}

fn assert_hydrated(result: &HydratedPublication, rig: &Rig, expected_metadata: &str) -> Outcome {
    require(
        result.release_id() == "rel",
        "workflow flows predicate 3 mismatch",
    )?;
    require(
        result.ingestion_id() == "ing",
        "workflow flows predicate 4 mismatch",
    )?;
    require(
        result.publication_session_id() == "pub",
        "workflow flows predicate 5 mismatch",
    )?;
    require(
        result.metadata_uri() == expected_metadata,
        "workflow flows predicate 6 mismatch",
    )?;
    require(
        result.stage() == Stage::PreparedForMint,
        "workflow flows predicate 7 mismatch",
    )?;
    require(
        result.backend_bundle().release_id == Presence::Present(Some("rel".into())),
        "workflow flows predicate 8 mismatch",
    )?;
    require(
        result.backend_session().id == Presence::Present(Some("pub".into())),
        "workflow flows predicate 9 mismatch",
    )?;

    require(
        rig.storage.state.snapshot()?.metadata_uri() == Some(METADATA),
        "workflow flows predicate 10 mismatch",
    )?;
    require(
        rig.storage.state.snapshot()?.pending().is_none(),
        "workflow flows predicate 11 mismatch",
    )?;
    Ok(())
}

fn assert_uploads(rig: &Rig) -> Outcome {
    let uploads = rig.uploads.0.observations.borrow();
    require(uploads.len() == 2, "workflow flows predicate 12 mismatch")?;
    require(
        uploads
            .first()
            .ok_or("workflow flows observation missing")?
            .bytes
            == GIF,
        "workflow flows predicate 13 mismatch",
    )?;
    require(
        uploads
            .first()
            .ok_or("workflow flows observation missing")?
            .mime
            == "image/gif",
        "workflow flows predicate 14 mismatch",
    )?;
    require(
        uploads
            .get(1)
            .ok_or("workflow flows observation missing")?
            .mime
            == "application/json",
        "workflow flows predicate 15 mismatch",
    )?;
    require(
        serde_json::from_slice::<Value>(
            &uploads
                .get(1)
                .ok_or("workflow flows observation missing")?
                .bytes,
        )
        .map_err(|_| "workflow flows fixture 2 missing or invalid")?
            == expected_document(PUBLIC_ICON),
        "workflow flows predicate 16 mismatch",
    )?;
    require(
        uploads
            .first()
            .ok_or("workflow flows observation missing")?
            .url
            == "https://upload.invalid/put?signature=synthetic",
        "workflow flows predicate 17 mismatch",
    )?;
    Ok(())
}

fn assert_fetch_and_target(rig: &Rig) -> Outcome {
    let requests = rig.portal.requests.borrow();
    let fetch = requests
        .iter()
        .find(|request| request.path_segments.last().map(String::as_str) == Some("fetchRemoteFile"))
        .ok_or("workflow flows fixture 3 missing or invalid")?;
    require(
        fetch.method == rest::Method::Get,
        "workflow flows predicate 18 mismatch",
    )?;
    require(
        fetch.query.len() == 1,
        "workflow flows predicate 19 mismatch",
    )?;
    require(
        fetch
            .query
            .first()
            .ok_or("workflow query observation missing")?
            .0
            == "input",
        "workflow flows predicate 20 mismatch",
    )?;
    require(
        serde_json::from_str::<Value>(
            &fetch
                .query
                .first()
                .ok_or("workflow query observation missing")?
                .1,
        )
        .map_err(|_| "workflow flows fixture 4 missing or invalid")?
            == json_value!({"url":ICON,"fileName":"release-icon.png"}),
        "workflow flows predicate 21 mismatch",
    )?;
    require(
        fetch.body.as_deref().is_none(),
        "workflow flows predicate 22 mismatch",
    )?;
    assert_upload_target(rig)
}

fn assert_upload_target(rig: &Rig) -> Outcome {
    let requests = rig.portal.requests.borrow();
    let target = requests
        .iter()
        .find(|request| {
            request.path_segments.last().map(String::as_str)
                == Some("publication.createUploadTarget")
        })
        .ok_or("workflow flows fixture 5 missing or invalid")?;

    let body: Value = serde_json::from_slice(
        target
            .body
            .as_deref()
            .ok_or("workflow flows fixture 6 missing or invalid")?,
    )
    .map_err(|_| "workflow flows fixture 7 missing or invalid")?;
    require(
        body == json_value!({"fileHash":GIF_HASH,"fileExtension":"gif","contentType":"image/gif"}),
        "workflow flows predicate 23 mismatch",
    )?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn acknowledged_metadata_is_reused_on_resume_without_recreation_or_upload() -> Outcome {
    let mut steps = vec![created(), ready()];
    steps.extend(hydrate_steps(ICON));
    steps.extend([session(None, "MintSaved"), json(&bundle(ICON).to_string())]);
    let mut rig = Rig::new(steps);
    complete(rig.workflow()?.start(&input()))?
        .map_err(|_| "workflow flows fixture 8 missing or invalid")?;
    let result = resume_later(&mut rig)?;

    require(
        result.stage() == Stage::MintSaved,
        "workflow flows predicate 24 mismatch",
    )?;
    require(
        result.metadata_uri() == METADATA,
        "workflow flows predicate 25 mismatch",
    )?;
    require(
        rig.uploads.0.observations.borrow().len() == 2,
        "workflow flows predicate 26 mismatch",
    )?;
    require(
        rig.procedures()?
            .iter()
            .filter(|procedure| *procedure == "publication.createIngestionSession")
            .count()
            == 1,
        "workflow flows predicate 27 mismatch",
    )?;
    require(
        rig.procedures()?
            .iter()
            .filter(|procedure| *procedure == "fetchRemoteFile")
            .count()
            == 1,
        "workflow flows predicate 28 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn attached_ready_bundle_and_session_with_metadata_avoid_redundant_gets() -> Outcome {
    let mut payload = bundle(ICON);
    payload["release"]["nftMetadataUri"] = METADATA.into();
    let ready = json_value!({
        "id":"ing","releaseId":"rel","publicationSessionId":"pub","status":"ready",
        "publicationSession":{
            "id":"pub","ingestionSessionId":"ing","releaseId":"rel","stage":"PreparedForMint"
        },"bundle":payload
    });
    let mut rig = Rig::new(vec![created(), json(&ready.to_string())]);
    let result = complete(rig.workflow()?.start(&input()))?
        .map_err(|_| "workflow flows fixture 9 missing or invalid")?;

    require(
        result.metadata_uri() == METADATA,
        "workflow flows predicate 29 mismatch",
    )?;
    require(
        rig.procedures()?
            == [
                "publication.createIngestionSession",
                "publication.getIngestionSession",
            ],
        "workflow flows predicate 30 mismatch",
    )?;
    require(
        rig.uploads.0.observations.borrow().is_empty(),
        "workflow flows predicate 31 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn exact_public_r2_origins_reuse_media_while_host_and_port_lookalikes_get_rehosted() -> Outcome
{
    const PRODUCTION: &str = "https://r2.solanamobiledappstore.com/icon.gif";
    const STAGING: &str = "https://r2-staging.solanamobiledappstore.com:443/icon.gif";

    for (uri, image, uploads) in [
        (PRODUCTION, PRODUCTION, 1),
        (STAGING, STAGING, 1),
        (
            "https://r2.solanamobiledappstore.com/icon.gif?variant=synthetic/",
            PUBLIC_ICON,
            2,
        ),
        (
            "https://r2.solanamobiledappstore.com.evil.invalid/icon.gif",
            PUBLIC_ICON,
            2,
        ),
        (
            "https://r2.solanamobiledappstore.com:8443/icon.gif",
            PUBLIC_ICON,
            2,
        ),
    ] {
        assert_origin_behavior(uri, image, uploads)?;
    }
    Ok(())
}

fn assert_origin_behavior(uri: &str, image: &str, expected_uploads: usize) -> Outcome {
    let mut steps = vec![
        session(None, "PreparedForMint"),
        json(&bundle(uri).to_string()),
        remote_file(),
    ];

    if expected_uploads == 2 {
        steps.push(target(PUBLIC_ICON));
    }

    steps.push(target(METADATA));
    let mut rig = Rig::new(steps);
    let result = complete(rig.workflow()?.resume(ResumeTarget::Release("rel".into())))?
        .map_err(|_| "workflow flows fixture 10 missing or invalid")?;
    let uploads = rig.uploads.0.observations.borrow();

    require(
        result.metadata_uri() == METADATA,
        "workflow flows predicate 32 mismatch",
    )?;
    require(
        uploads.len() == expected_uploads,
        "workflow flows predicate 33 mismatch",
    )?;
    require(
        serde_json::from_slice::<Value>(
            &uploads
                .last()
                .ok_or("workflow flows fixture 11 missing or invalid")?
                .bytes,
        )
        .map_err(|_| "workflow flows fixture 12 missing or invalid")?
            == expected_document(image),
        "workflow flows predicate 34 mismatch",
    )?;
    assert_origin_request_counts(&rig, expected_uploads)
}

fn assert_origin_request_counts(rig: &Rig, expected_uploads: usize) -> Outcome {
    require(
        rig.procedures()?
            .iter()
            .filter(|procedure| *procedure == "fetchRemoteFile")
            .count()
            == 1,
        "workflow flows predicate 35 mismatch",
    )?;
    require(
        rig.procedures()?
            .iter()
            .filter(|procedure| *procedure == "publication.createUploadTarget")
            .count()
            == expected_uploads,
        "workflow flows predicate 36 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn session_selector_has_precedence_and_submitted_receipts_never_trigger_store_submission()
-> Outcome {
    let mut rig = Rig::new(vec![
        session(Some(METADATA), "Submitted"),
        json(&bundle(ICON).to_string()),
    ]);
    let result = complete(rig.workflow()?.resume(ResumeTarget::Session("pub".into())))?
        .map_err(|_| "workflow flows fixture 13 missing or invalid")?;
    let requests = rig.portal.requests.borrow();
    let query: Value = serde_json::from_str(
        &requests
            .first()
            .ok_or("workflow flows observation missing")?
            .query
            .first()
            .ok_or("workflow query observation missing")?
            .1,
    )
    .map_err(|_| "workflow flows fixture 14 missing or invalid")?;

    require(
        query == json_value!({"publicationSessionId":"pub","releaseId":null}),
        "workflow flows predicate 37 mismatch",
    )?;
    require(
        result.stage() == Stage::Submitted,
        "workflow flows predicate 38 mismatch",
    )?;
    require(
        rig.procedures()?
            == [
                "publication.getPublicationSession",
                "publication.getPublicationBundle",
            ],
        "workflow flows predicate 39 mismatch",
    )?;
    require(
        rig.uploads.0.observations.borrow().is_empty(),
        "workflow flows predicate 40 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn acknowledged_media_receipt_survives_later_write_failure_without_another_put() -> Outcome {
    let mut rig = Rig::new(vec![
        session(None, "PreparedForMint"),
        json(&bundle(ICON).to_string()),
        remote_file(),
        target(PUBLIC_ICON),
    ]);
    rig.storage
        .state
        .fail_at
        .set(Some(METADATA_TARGET_INTENT_WRITE));
    require(
        matches!(
            complete(rig.workflow()?.resume(ResumeTarget::Release("rel".into())))?,
            Err(solana_kit_dapp_publisher::workflow::Error::Storage(_))
        ),
        "workflow flows predicate 41 mismatch",
    )?;
    require(
        rig.storage.state.snapshot()?.pending().is_none(),
        "workflow flows predicate 42 mismatch",
    )?;
    require(
        rig.uploads.0.observations.borrow().len() == 1,
        "workflow flows predicate 43 mismatch",
    )?;

    assert_receipt_resume(&mut rig)
}

fn assert_receipt_resume(rig: &mut Rig) -> Outcome {
    rig.storage.state.fail_at.set(None);
    rig.portal.steps.borrow_mut().extend([
        session(None, "PreparedForMint"),
        json(&bundle(ICON).to_string()),
        remote_file(),
        target(METADATA),
    ]);
    require(
        resume_later(rig)?.metadata_uri() == METADATA,
        "workflow flows predicate 44 mismatch",
    )?;

    let uploads = rig.uploads.0.observations.borrow();
    require(uploads.len() == 2, "workflow flows predicate 45 mismatch")?;
    require(
        serde_json::from_slice::<Value>(
            &uploads
                .get(1)
                .ok_or("workflow flows observation missing")?
                .bytes,
        )
        .map_err(|_| "workflow flows fixture 15 missing or invalid")?
            == expected_document(PUBLIC_ICON),
        "workflow flows predicate 46 mismatch",
    )?;
    require(
        rig.procedures()?
            .iter()
            .filter(|procedure| *procedure == "publication.createUploadTarget")
            .count()
            == 2,
        "workflow flows predicate 47 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

const METADATA_TARGET_INTENT_WRITE: usize = 5;

fn resume_later(rig: &mut Rig) -> Result<HydratedPublication, &'static str> {
    let mut later = options();
    later.updated_on = "2026-11-01T00:00:00Z".into();
    let mut workflow = Workflow::new(
        &rig.client,
        &rig.uploads,
        &mut rig.storage,
        &rig.clock,
        later,
    )
    .map_err(|_| "workflow flows fixture 16 missing or invalid")?;
    complete(workflow.resume(ResumeTarget::Snapshot))?
        .map_err(|_| "workflow flows fixture 17 missing or invalid")
}
