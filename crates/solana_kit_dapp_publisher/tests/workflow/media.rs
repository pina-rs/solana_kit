//! Independent file/header vectors and invalid metadata exercise real admission boundaries.

use super::{
    checks::{Outcome, require},
    fixtures::*,
    support::*,
};
use serde_json::{Value, json as json_value};
use solana_kit_dapp_publisher::{
    portal::IngestionSource,
    workflow::{
        Error, LocalApk, ResumeTarget, Snapshot, Workflow, prepare_local_apk, read_image_dimensions,
    },
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

fn reader(rig: &Rig, bytes: &[u8]) -> Reader {
    Reader {
        calls: RefCell::new(Vec::new()),
        bytes: bytes.to_vec(),
        pending: false,
        cancelled: Rc::new(Cell::new(0)),
        tick: rig.clock.tick.clone(),
        completion: None,
    }
}

fn local() -> LocalApk {
    LocalApk {
        token: "local-only-file-token".into(),
        file_name: " fixture ".into(),
        content_type: None,
    }
}

#[cfg_attr(test, test)]
pub fn local_apk_uses_one_capped_immutable_read_for_hash_size_and_exact_upload_bytes() -> Outcome {
    let rig = Rig::new(vec![target("https://public.invalid/file.apk")]);
    rig.portal.require_intent.set(false);
    rig.uploads.0.require_intent.set(false);
    let reader = reader(&rig, b"abc");
    let prepared = complete(prepare_local_apk(
        &rig.client,
        &rig.uploads,
        &reader,
        &rig.clock,
        &local(),
        limits(),
    ))?
    .map_err(|_| "workflow media fixture 1 missing or invalid")?;

    assert_local_apk_source(prepared)?;

    require(
        *reader.calls.borrow() == [("local-only-file-token".to_owned(), limits().source_bytes)],
        "workflow media predicate 5 mismatch",
    )?;
    let requests = rig.portal.requests.borrow();
    let request: Value = serde_json::from_slice(
        requests
            .first()
            .ok_or("workflow media observation missing")?
            .body
            .as_deref()
            .ok_or("workflow media fixture 2 missing or invalid")?,
    )
    .map_err(|_| "workflow media fixture 3 missing or invalid")?;
    require(
        request
            == json_value!({"fileHash":ABC_HASH,"fileExtension":"apk","contentType":"application/vnd.android.package-archive"}),
        "workflow media predicate 6 mismatch",
    )?;
    require(
        !serde_json::to_string(&request)
            .map_err(|_| "workflow media fixture 4 missing or invalid")?
            .contains("local-only-file-token"),
        "workflow media predicate 7 mismatch",
    )?;
    require(
        rig.uploads
            .0
            .observations
            .borrow()
            .first()
            .ok_or("workflow upload observation missing")?
            .bytes
            == b"abc",
        "workflow media predicate 8 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn local_read_cancellation_deadline_and_byte_limits_prevent_network_admission() -> Outcome {
    let rig = Rig::new(vec![]);
    let mut reader = reader(&rig, b"abc");
    reader.pending = true;
    let source = local();
    let mut operation = Box::pin(prepare_local_apk(
        &rig.client,
        &rig.uploads,
        &reader,
        &rig.clock,
        &source,
        limits(),
    ));
    poll_pending(operation.as_mut())?;
    drop(operation);
    require(
        reader.cancelled.get() == 1,
        "workflow media predicate 9 mismatch",
    )?;

    assert_local_read_limits(&rig, &mut reader, &source)
}

fn assert_local_read_limits(rig: &Rig, reader: &mut Reader, source: &LocalApk) -> Outcome {
    reader.pending = false;
    let mut small = limits();
    small.source_bytes = 2;
    require(
        matches!(
            complete(prepare_local_apk(
                &rig.client,
                &rig.uploads,
                &*reader,
                &rig.clock,
                source,
                small
            ))?,
            Err(Error::Invalid(_))
        ),
        "workflow media predicate 10 mismatch",
    )?;
    reader.completion = Some(DEADLINE);
    require(
        matches!(
            complete(prepare_local_apk(
                &rig.client,
                &rig.uploads,
                &*reader,
                &rig.clock,
                source,
                limits()
            ))?,
            Err(Error::Deadline)
        ),
        "workflow media predicate 11 mismatch",
    )?;
    require(
        rig.portal.requests.borrow().is_empty(),
        "workflow media predicate 12 mismatch",
    )?;
    require(
        rig.uploads.0.observations.borrow().is_empty(),
        "workflow media predicate 13 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn each_supported_image_header_has_an_independent_nonzero_dimension_vector() -> Outcome {
    let png = b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0dIHDR\x00\x00\x00\x02\x00\x00\x00\x03";
    let jpeg = b"\xff\xd8\xff\xc0\x00\x08\x08\x00\x03\x00\x02\x00";
    let webp_x =
        b"RIFF\x16\x00\x00\x00WEBPVP8X\x0a\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x02\x00\x00";
    let webp_l =
        b"RIFF\x16\x00\x00\x00WEBPVP8L\x0a\x00\x00\x00\x2f\x01\x80\x00\x00\x00\x00\x00\x00\x00";
    let webp_lossy =
        b"RIFF\x16\x00\x00\x00WEBPVP8 \x0a\x00\x00\x00\x00\x00\x00\x9d\x01\x2a\x02\x00\x03\x00";

    for bytes in [
        png.as_slice(),
        jpeg.as_slice(),
        GIF,
        webp_x.as_slice(),
        webp_l.as_slice(),
        webp_lossy.as_slice(),
    ] {
        let dimensions = read_image_dimensions(bytes)
            .map_err(|_| "workflow media fixture 5 missing or invalid")?;
        require(
            (dimensions.width, dimensions.height) == (2, 3),
            "workflow media predicate 14 mismatch",
        )?;
    }

    for bytes in [
        b"".as_slice(),
        b"not-an-image".as_slice(),
        &GIF[..GIF.len() - 1],
        b"GIF89a\x00\x00\x00\x00".as_slice(),
        b"\xff\xd8\xff\xe0\x00\x00\x00\x00\x00\x00\x00\x00".as_slice(),
    ] {
        require(
            read_image_dimensions(bytes).is_err(),
            "workflow media predicate 15 mismatch",
        )?;
    }
    Ok(())
}

#[cfg_attr(test, test)]
pub fn invalid_metadata_is_rejected_before_remote_fetch_or_upload() -> Outcome {
    for fixture in invalid_metadata_fixtures()? {
        let mut rig = Rig::new(vec![
            session(None, "PreparedForMint"),
            json(&fixture.to_string()),
        ]);
        require(
            matches!(
                complete(rig.workflow()?.resume(ResumeTarget::Release("rel".into())))?,
                Err(Error::Invalid(_))
            ),
            "workflow media predicate 16 mismatch",
        )?;
        require(
            rig.procedures()?
                == [
                    "publication.getPublicationSession",
                    "publication.getPublicationBundle",
                ],
            "workflow media predicate 17 mismatch",
        )?;
        require(
            rig.uploads.0.observations.borrow().is_empty(),
            "workflow media predicate 18 mismatch",
        )?;
        rig.valid()?;
    }

    Ok(())
}

#[cfg_attr(test, test)]
pub fn media_item_and_decoded_byte_limits_stop_the_next_effect() -> Outcome {
    let mut many = bundle(ICON);
    many["dapp"]["dappPreviewUrls"] = json_value!(["https://external.invalid/preview.gif"]);
    let mut rig = Rig::new(vec![
        session(None, "PreparedForMint"),
        json(&many.to_string()),
    ]);
    let mut one = options();
    one.limits.media_items = 1;
    let result = {
        let mut workflow =
            Workflow::new(&rig.client, &rig.uploads, &mut rig.storage, &rig.clock, one)
                .map_err(|_| "workflow media fixture 7 missing or invalid")?;
        complete(workflow.resume(ResumeTarget::Release("rel".into())))?
    };
    require(
        matches!(result, Err(Error::Invalid(_))),
        "workflow media predicate 19 mismatch",
    )?;
    require(
        rig.procedures()?.len() == 2,
        "workflow media predicate 20 mismatch",
    )?;

    rig.valid()?;
    assert_decoded_media_limit()
}

fn assert_decoded_media_limit() -> Outcome {
    let mut rig = Rig::new(vec![
        session(None, "PreparedForMint"),
        json(&bundle(ICON).to_string()),
        remote_file(),
    ]);
    let mut small = options();
    small.limits.media_bytes = GIF.len() - 1;
    let result = {
        let mut workflow = Workflow::new(
            &rig.client,
            &rig.uploads,
            &mut rig.storage,
            &rig.clock,
            small,
        )
        .map_err(|_| "workflow media fixture 8 missing or invalid")?;
        complete(workflow.resume(ResumeTarget::Release("rel".into())))?
    };
    require(
        matches!(result, Err(Error::Codec(_))),
        "workflow media predicate 21 mismatch",
    )?;
    require(
        rig.procedures()?.len() == 3,
        "workflow media predicate 22 mismatch",
    )?;
    require(
        rig.uploads.0.observations.borrow().is_empty(),
        "workflow media predicate 23 mismatch",
    )?;

    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn checkpoint_and_timestamp_admission_rejects_ambiguous_or_invalid_input() -> Outcome {
    let mut rig = Rig::new(vec![Step::Fail]);
    require(
        complete(rig.workflow()?.start(&input()))?.is_err(),
        "workflow media predicate 24 mismatch",
    )?;
    let original: Value = serde_json::from_slice(
        rig.storage
            .state
            .data
            .borrow()
            .as_ref()
            .ok_or("workflow media fixture 9 missing or invalid")?,
    )
    .map_err(|_| "workflow media fixture 10 missing or invalid")?;
    assert_invalid_checkpoints(original)?;

    assert_invalid_timestamps(&mut rig)
}

fn assert_invalid_timestamps(rig: &mut Rig) -> Outcome {
    for timestamp in [
        "2026-02-29T00:00:00Z",
        "2026-10-29T24:00:00Z",
        "2026-10-29T00:00:00+00:00",
        "+026-10-29T00:00:00Z",
        "2026-10-29T00:00:00.Z",
    ] {
        let mut invalid = options();
        invalid.updated_on = timestamp.into();
        require(
            matches!(
                Workflow::new(
                    &rig.client,
                    &rig.uploads,
                    &mut rig.storage,
                    &rig.clock,
                    invalid
                ),
                Err(Error::Invalid(_))
            ),
            "workflow media predicate 26 mismatch",
        )?;
    }

    require(
        rig.procedures()? == ["publication.createIngestionSession"],
        "workflow media predicate 27 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn metadata_fallbacks_preserve_normalized_contacts_and_nonempty_subtype_extensions() -> Outcome
{
    let remote =
        json(r#"{"data":"R0lGODlhAgADAA==","fileName":"remote","mimeType":"application/++"}"#);
    let mut payload = bundle(ICON);
    payload["publisher"]
        .as_object_mut()
        .ok_or("workflow media fixture 11 missing or invalid")?
        .remove("supportEmail");
    payload["dapp"]["contactEmail"] = "other-contact@publisher.invalid".into();
    payload["publisher"]["website"] = "publisher.invalid/path?next=https://landing.invalid".into();
    let mut rig = Rig::new(vec![
        session(None, "PreparedForMint"),
        json(&payload.to_string()),
        remote,
        target(PUBLIC_ICON),
        target(METADATA),
    ]);

    complete(rig.workflow()?.resume(ResumeTarget::Release("rel".into())))?
        .map_err(|_| "workflow media fixture 12 missing or invalid")?;
    assert_metadata_fallback_output(&rig)
}

fn assert_metadata_fallback_output(rig: &Rig) -> Outcome {
    let requests = rig.portal.requests.borrow();
    let request = requests
        .iter()
        .find(|request| {
            request.path_segments.last().map(String::as_str)
                == Some("publication.createUploadTarget")
        })
        .ok_or("workflow media fixture 13 missing or invalid")?;
    let body: Value = serde_json::from_slice(
        request
            .body
            .as_deref()
            .ok_or("workflow media fixture 14 missing or invalid")?,
    )
    .map_err(|_| "workflow media fixture 15 missing or invalid")?;

    require(
        body == json_value!({"fileHash":GIF_HASH,"fileExtension":"bin","contentType":"application/++"}),
        "workflow media predicate 28 mismatch",
    )?;
    let uploads = rig.uploads.0.observations.borrow();
    let document: Value = serde_json::from_slice(
        &uploads
            .get(1)
            .ok_or("workflow media observation missing")?
            .bytes,
    )
    .map_err(|_| "workflow media fixture 16 missing or invalid")?;
    require(
        document["extensions"]["solana_dapp_store"]["publisher_details"]["support_email"]
            == "contact@publisher.invalid",
        "workflow media predicate 29 mismatch",
    )?;
    require(
        document["external_url"] == "https://publisher.invalid/path?next=https://landing.invalid",
        "workflow media predicate 30 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn conflicting_backend_identifiers_and_future_stages_cannot_trigger_media_work() -> Outcome {
    let mut other_release = bundle(ICON);
    other_release["releaseId"] = "other-release".into();
    let cases = [
        vec![json(
            r#"{"id":"other-session","ingestionSessionId":"ing","releaseId":"rel","stage":"PreparedForMint"}"#,
        )],
        vec![
            session(None, "PreparedForMint"),
            json(&other_release.to_string()),
        ],
        vec![session(None, "FutureStage")],
    ];

    for steps in cases {
        let mut rig = Rig::new(steps);
        require(
            matches!(
                complete(rig.workflow()?.resume(ResumeTarget::Session("pub".into())))?,
                Err(Error::Invalid(_))
            ),
            "workflow media predicate 31 mismatch",
        )?;
        require(
            !rig.procedures()?
                .iter()
                .any(|procedure| procedure == "fetchRemoteFile"),
            "workflow media predicate 32 mismatch",
        )?;
        require(
            rig.uploads.0.observations.borrow().is_empty(),
            "workflow media predicate 33 mismatch",
        )?;
        rig.valid()?;
    }

    Ok(())
}

#[cfg_attr(test, test)]
pub fn malformed_presigned_targets_do_not_reach_the_uploader_or_get_retried() -> Outcome {
    for upload_url in [
        "upload.invalid/relative",
        " https://upload.invalid/put",
        "https://upload.invalid/put\r\n",
    ] {
        let target =
            json(&json_value!({"uploadUrl":upload_url,"publicUrl":PUBLIC_ICON}).to_string());
        let mut rig = Rig::new(vec![
            session(None, "PreparedForMint"),
            json(&bundle(ICON).to_string()),
            remote_file(),
            target,
        ]);
        require(
            matches!(
                complete(rig.workflow()?.resume(ResumeTarget::Release("rel".into())))?,
                Err(Error::Invalid(_))
            ),
            "workflow media predicate 34 mismatch",
        )?;
        require(
            rig.uploads.0.observations.borrow().is_empty(),
            "workflow media predicate 35 mismatch",
        )?;
        require(
            rig.storage.state.snapshot()?.pending()
                == Some(solana_kit_dapp_publisher::workflow::Effect::CreateUploadTarget),
            "workflow media predicate 36 mismatch",
        )?;
        require(
            matches!(
                complete(rig.workflow()?.resume(ResumeTarget::Snapshot))?,
                Err(Error::Uncertain(
                    solana_kit_dapp_publisher::workflow::Effect::CreateUploadTarget
                ))
            ),
            "workflow media predicate 37 mismatch",
        )?;
        require(
            rig.procedures()?.len() == 4,
            "workflow media predicate 38 mismatch",
        )?;
        rig.valid()?;
    }
    Ok(())
}

fn assert_local_apk_source(prepared: IngestionSource) -> Outcome {
    match prepared {
        IngestionSource::PortalUpload {
            release_file_url,
            release_file_name,
            release_file_size,
            release_file_hash,
        } => {
            require(
                release_file_url == "https://public.invalid/file.apk",
                "workflow media predicate 1 mismatch",
            )?;
            require(
                release_file_name == "fixture.apk",
                "workflow media predicate 2 mismatch",
            )?;
            require(
                release_file_size == 3,
                "workflow media predicate 3 mismatch",
            )?;
            require(
                release_file_hash.as_deref() == Some(ABC_HASH),
                "workflow media predicate 4 mismatch",
            )?;
        }
        _ => return Err("local source did not become a portal upload"),
    }

    Ok(())
}

fn invalid_metadata_fixtures() -> Result<Vec<Value>, &'static str> {
    let mut fixtures = Vec::new();
    let mut missing = bundle(ICON);
    missing["dapp"]
        .as_object_mut()
        .ok_or("workflow media fixture 6 missing or invalid")?
        .remove("dappIconUrl");
    fixtures.push(missing);
    let mut userinfo = bundle("https://user:password@external.invalid/icon.gif");
    userinfo["publisher"]["website"] = "https://publisher.invalid".into();
    fixtures.push(userinfo);
    let mut negative = bundle(ICON);

    negative["release"]["versionCode"] = (-1).into();
    fixtures.push(negative);
    let mut empty_notes = bundle(ICON);
    empty_notes["release"]["newInVersion"] = "".into();
    fixtures.push(empty_notes);
    let mut empty_selected_short = bundle(ICON);
    empty_selected_short["release"]["shortDescription"] = "".into();
    fixtures.push(empty_selected_short);
    let mut empty_selected_uri = bundle(ICON);
    empty_selected_uri["installFile"]["uri"] = "".into();
    empty_selected_uri["release"]["releaseFileUrl"] = "https://fallback.invalid/app.apk".into();
    fixtures.push(empty_selected_uri);
    let mut empty_selected_mint = bundle(ICON);
    empty_selected_mint["signerAuthority"]["appMintAddress"] = "".into();
    fixtures.push(empty_selected_mint);

    Ok(fixtures)
}

fn assert_invalid_checkpoints(original: Value) -> Outcome {
    let mut future = original.clone();
    future["version"] = 2.into();
    let mut array_record = original.clone();
    array_record["uploads"] = json_value!([[GIF_HASH, "image/gif", "gif", PUBLIC_ICON]]);
    let mut unknown = original;

    unknown["future"] = true.into();

    for bytes in [
        b"[]".to_vec(),
        future.to_string().into_bytes(),
        unknown.to_string().into_bytes(),
        array_record.to_string().into_bytes(),
        br#"{"version":1,"version":1}"#.to_vec(),
    ] {
        require(
            Snapshot::decode(&bytes, JSON).is_err(),
            "workflow media predicate 25 mismatch",
        )?;
    }

    Ok(())
}
