//! Durable public locations cannot retain an upload query or alias a known PUT capability.

use super::{
    checks::{Outcome, require},
    fixtures::*,
    support::*,
};
use serde_json::{Value, json as json_value};
use solana_kit_dapp_publisher::workflow::{
    Effect, Error, LocalApk, ResumeTarget, Snapshot, prepare_local_apk,
};
use std::{cell::RefCell, rc::Rc};

const SIGNED_UPLOAD: &str = "https://upload.invalid/put?signature=synthetic";
const QUERY_PUBLIC: &str = "https://public.invalid/metadata.json?signature=synthetic";

#[cfg_attr(test, test)]
pub fn upload_target_aliases_and_nonpublic_locations_never_reach_put_or_durable_state() -> Outcome {
    for (upload, public) in [
        (SIGNED_UPLOAD, SIGNED_UPLOAD),
        ("https://upload.invalid/put", "https://upload.invalid/put"),
        (SIGNED_UPLOAD, QUERY_PUBLIC),
        (SIGNED_UPLOAD, "https://public.invalid/icon.gif?"),
        (SIGNED_UPLOAD, "https://user@public.invalid/icon.gif"),
        (SIGNED_UPLOAD, "https://public.invalid/icon.gif#fragment"),
        (SIGNED_UPLOAD, "http://public.invalid/icon.gif"),
    ] {
        reject_target(upload, public)?;
    }
    Ok(())
}

fn reject_target(upload: &str, public: &str) -> Outcome {
    let mut rig = Rig::new(vec![
        session(None, "PreparedForMint"),
        json(&bundle(ICON).to_string()),
        remote_file(),
        upload_target(upload, public),
    ]);
    require(
        matches!(
            complete(rig.workflow()?.resume(ResumeTarget::Release("rel".into())))?,
            Err(Error::Invalid(_))
        ),
        "workflow public_urls predicate 1 mismatch",
    )?;
    require(
        rig.uploads.0.observations.borrow().is_empty(),
        "workflow public_urls predicate 2 mismatch",
    )?;
    let snapshot = rig.storage.state.snapshot()?;
    require(
        snapshot.pending() == Some(Effect::CreateUploadTarget),
        "workflow public_urls predicate 3 mismatch",
    )?;
    require(
        snapshot
            .pending_upload()
            .ok_or("workflow public_urls fixture 1 missing or invalid")?
            .public_url()
            .is_none(),
        "workflow public_urls predicate 4 mismatch",
    )?;
    assert_not_retained(&rig, &[upload, public])?;
    require(
        matches!(
            complete(rig.workflow()?.resume(ResumeTarget::Snapshot))?,
            Err(Error::Uncertain(Effect::CreateUploadTarget))
        ),
        "workflow public_urls predicate 5 mismatch",
    )?;

    require(
        rig.procedures()?.len() == 4,
        "workflow public_urls predicate 6 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn local_apk_uses_the_same_public_target_admission_before_put() -> Outcome {
    for (upload, public) in [
        (SIGNED_UPLOAD, SIGNED_UPLOAD),
        (SIGNED_UPLOAD, QUERY_PUBLIC),
        ("https://upload.invalid/put", "https://upload.invalid/put"),
    ] {
        let rig = Rig::new(vec![upload_target(upload, public)]);
        rig.portal.require_intent.set(false);
        rig.uploads.0.require_intent.set(false);
        let reader = Reader {
            calls: RefCell::new(Vec::new()),
            bytes: b"abc".to_vec(),
            pending: false,
            cancelled: Rc::default(),
            tick: rig.clock.tick.clone(),
            completion: None,
        };
        let source = LocalApk {
            token: "local-only-token".into(),
            file_name: "fixture.apk".into(),
            content_type: None,
        };
        require(
            matches!(
                complete(prepare_local_apk(
                    &rig.client,
                    &rig.uploads,
                    &reader,
                    &rig.clock,
                    &source,
                    limits()
                ))?,
                Err(Error::Invalid(_))
            ),
            "workflow public_urls predicate 7 mismatch",
        )?;
        require(
            reader.calls.borrow().len() == 1,
            "workflow public_urls predicate 8 mismatch",
        )?;
        require(
            rig.procedures()? == ["publication.createUploadTarget"],
            "workflow public_urls predicate 9 mismatch",
        )?;
        require(
            rig.uploads.0.observations.borrow().is_empty(),
            "workflow public_urls predicate 10 mismatch",
        )?;
        require(
            rig.storage.state.history.borrow().is_empty(),
            "workflow public_urls predicate 11 mismatch",
        )?;
        rig.valid()?;
    }
    Ok(())
}

#[cfg_attr(test, test)]
pub fn forged_old_query_locations_cannot_be_loaded_reused_or_written_again() -> Outcome {
    require(
        Snapshot::decode(&checkpoint().to_string().into_bytes(), JSON).is_ok(),
        "workflow public_urls predicate 12 mismatch",
    )?;

    for forged in forged_checkpoints() {
        let bytes = forged.to_string().into_bytes();
        require(
            matches!(Snapshot::decode(&bytes, JSON), Err(Error::Invalid(_))),
            "workflow public_urls predicate 13 mismatch",
        )?;
        let mut rig = Rig::new(vec![]);
        *rig.storage.state.data.borrow_mut() = Some(bytes.clone());
        require(
            matches!(
                complete(rig.workflow()?.resume(ResumeTarget::Snapshot))?,
                Err(Error::Invalid(_))
            ),
            "workflow public_urls predicate 14 mismatch",
        )?;
        require(
            rig.procedures()?.is_empty(),
            "workflow public_urls predicate 15 mismatch",
        )?;
        require(
            rig.uploads.0.observations.borrow().is_empty(),
            "workflow public_urls predicate 16 mismatch",
        )?;
        require(
            rig.storage.state.history.borrow().is_empty(),
            "workflow public_urls predicate 17 mismatch",
        )?;

        require(
            rig.storage.state.data.borrow().as_ref() == Some(&bytes),
            "workflow public_urls predicate 18 mismatch",
        )?;
        rig.valid()?;
    }

    Ok(())
}

fn forged_checkpoints() -> [Value; 3] {
    let mut metadata = checkpoint();
    metadata["metadata_uri"] = QUERY_PUBLIC.into();
    let mut receipt = checkpoint();
    receipt["uploads"] = json_value!([{
        "hash":GIF_HASH,"mime":"image/gif","extension":"gif","public_url":QUERY_PUBLIC
    }]);
    let mut pending = checkpoint();
    pending["pending"] = "Upload".into();
    pending["pending_upload"] = json_value!({
        "hash":GIF_HASH,"mime":"image/gif","extension":"gif","public_url":QUERY_PUBLIC
    });
    [metadata, receipt, pending]
}

#[cfg_attr(test, test)]
pub fn session_bundle_and_attached_metadata_queries_reject_before_cache_or_media() -> Outcome {
    let mut payload = bundle(ICON);
    payload["release"]["nftMetadataUri"] = QUERY_PUBLIC.into();
    let attached = json_value!({
        "id":"ing","releaseId":"rel","publicationSessionId":"pub","status":"ready",
        "publicationSession":{
            "id":"pub","ingestionSessionId":"ing","releaseId":"rel","stage":"PreparedForMint"
        },"bundle":payload.clone()
    });

    for (start, steps) in [
        (false, vec![session(Some(QUERY_PUBLIC), "PreparedForMint")]),
        (
            false,
            vec![session(None, "PreparedForMint"), json(&payload.to_string())],
        ),
        (true, vec![created(), json(&attached.to_string())]),
    ] {
        reject_backend_location(start, steps)?;
    }
    Ok(())
}

fn reject_backend_location(start: bool, steps: Vec<Step>) -> Outcome {
    let mut rig = Rig::new(steps);
    let result = if start {
        complete(rig.workflow()?.start(&input()))?
    } else {
        let mut cached = checkpoint();
        cached["metadata_uri"] = METADATA.into();
        *rig.storage.state.data.borrow_mut() = Some(cached.to_string().into_bytes());
        complete(rig.workflow()?.resume(ResumeTarget::Snapshot))?
    };

    require(
        matches!(result, Err(Error::Invalid(_))),
        "workflow public_urls predicate 19 mismatch",
    )?;

    require(
        rig.uploads.0.observations.borrow().is_empty(),
        "workflow public_urls predicate 20 mismatch",
    )?;
    require(
        rig.procedures()?.iter().all(|procedure| {
            procedure != "fetchRemoteFile" && procedure != "publication.createUploadTarget"
        }),
        "workflow public_urls predicate 21 mismatch",
    )?;
    assert_not_retained(&rig, &[QUERY_PUBLIC])?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn install_artifact_queries_reject_before_fetch_or_metadata_upload() -> Outcome {
    let mut payload = bundle(ICON);
    payload["installFile"]["uri"] = QUERY_PUBLIC.into();
    let mut rig = Rig::new(vec![
        session(None, "PreparedForMint"),
        json(&payload.to_string()),
    ]);
    require(
        matches!(
            complete(rig.workflow()?.resume(ResumeTarget::Release("rel".into())))?,
            Err(Error::Invalid(_))
        ),
        "workflow public_urls predicate 22 mismatch",
    )?;
    require(
        rig.procedures()?.len() == 2,
        "workflow public_urls predicate 23 mismatch",
    )?;
    require(
        rig.uploads.0.observations.borrow().is_empty(),
        "workflow public_urls predicate 24 mismatch",
    )?;
    assert_not_retained(&rig, &[QUERY_PUBLIC])?;
    rig.valid()?;

    Ok(())
}

#[cfg_attr(test, test)]
pub fn query_free_public_locations_succeed_while_signed_queries_stay_in_memory() -> Outcome {
    let mut rig = Rig::new(hydrate_steps(ICON));
    let result = complete(rig.workflow()?.resume(ResumeTarget::Release("rel".into())))?
        .map_err(|_| "workflow public_urls fixture 2 missing or invalid")?;
    require(
        result.metadata_uri() == METADATA,
        "workflow public_urls predicate 25 mismatch",
    )?;
    let uploads = rig.uploads.0.observations.borrow();
    require(
        uploads.len() == 2,
        "workflow public_urls predicate 26 mismatch",
    )?;
    require(
        uploads.iter().all(|upload| upload.url == SIGNED_UPLOAD),
        "workflow public_urls predicate 27 mismatch",
    )?;
    let document: Value = serde_json::from_slice(
        &uploads
            .get(1)
            .ok_or("workflow public_urls observation missing")?
            .bytes,
    )
    .map_err(|_| "workflow public_urls fixture 3 missing or invalid")?;
    require(
        document == expected_document(PUBLIC_ICON),
        "workflow public_urls predicate 28 mismatch",
    )?;

    require(
        !document.to_string().contains("signature=synthetic"),
        "workflow public_urls predicate 29 mismatch",
    )?;
    assert_not_retained(&rig, &[SIGNED_UPLOAD, "signature=synthetic"])?;
    require(
        rig.storage.state.snapshot()?.metadata_uri() == Some(METADATA),
        "workflow public_urls predicate 30 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

fn checkpoint() -> Value {
    json_value!({
        "version":1,"input_hash":null,"idempotency_key":null,
        "ingestion_id":"ing","release_id":"rel","publication_id":"pub",
        "metadata_uri":null,"updated_on":"2026-10-29T12:00:00.000Z",
        "pending":null,"pending_upload":null,"uploads":[],
        "deleted":false,"preserved_submitted":false
    })
}

fn upload_target(upload: &str, public: &str) -> Step {
    json(&json_value!({"uploadUrl":upload,"publicUrl":public}).to_string())
}

fn assert_not_retained(rig: &Rig, forbidden: &[&str]) -> Outcome {
    for bytes in rig.storage.state.history.borrow().iter() {
        let stored = std::str::from_utf8(bytes)
            .map_err(|_| "workflow public_urls fixture 4 missing or invalid")?;
        require(
            forbidden.iter().all(|value| !stored.contains(value)),
            "workflow public_urls predicate 31 mismatch",
        )?;
    }

    Ok(())
}
