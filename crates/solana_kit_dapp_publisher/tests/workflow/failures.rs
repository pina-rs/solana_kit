//! Uncertain effects survive failures and cancellation without hidden retries or cleanup.

use super::{
    checks::{Outcome, require},
    fixtures::*,
    support::*,
};
use solana_kit_dapp_publisher::workflow::{Effect, Error, ResumeTarget, Stage, Workflow};

#[cfg_attr(test, test)]
pub fn uncertain_create_is_not_retried_or_bypassed_by_a_resume_selector() -> Outcome {
    let mut rig = Rig::new(vec![Step::Fail]);
    require(
        matches!(
            complete(rig.workflow()?.start(&input()))?,
            Err(Error::Portal(_))
        ),
        "workflow failures predicate 1 mismatch",
    )?;
    let snapshot = rig.storage.state.snapshot()?;
    require(
        snapshot.pending() == Some(Effect::CreateIngestion),
        "workflow failures predicate 2 mismatch",
    )?;
    require(
        snapshot.ingestion_id().is_none(),
        "workflow failures predicate 3 mismatch",
    )?;

    for target in [
        ResumeTarget::Snapshot,
        ResumeTarget::Session("pub".into()),
        ResumeTarget::Release("rel".into()),
    ] {
        require(
            matches!(
                complete(rig.workflow()?.resume(target))?,
                Err(Error::Uncertain(Effect::CreateIngestion))
            ),
            "workflow failures predicate 4 mismatch",
        )?;
    }

    require(
        rig.procedures()? == ["publication.createIngestionSession"],
        "workflow failures predicate 5 mismatch",
    )?;
    require(
        rig.uploads.0.observations.borrow().is_empty(),
        "workflow failures predicate 6 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn exhausted_polling_remains_resumable_without_another_create() -> Outcome {
    let mut rig = Rig::new(vec![
        created(),
        queued(),
        queued(),
        ready(),
        session(Some(METADATA), "PreparedForMint"),
        json(&bundle(ICON).to_string()),
    ]);
    let mut bounded = options();
    bounded.limits.max_polls = 2;
    let result = {
        let mut workflow = Workflow::new(
            &rig.client,
            &rig.uploads,
            &mut rig.storage,
            &rig.clock,
            bounded,
        )
        .map_err(|_| "workflow failures fixture 1 missing or invalid")?;
        complete(workflow.start(&input()))?
    };

    require(
        matches!(result, Err(Error::PollLimit)),
        "workflow failures predicate 7 mismatch",
    )?;
    require(
        rig.storage.state.snapshot()?.ingestion_id() == Some("ing"),
        "workflow failures predicate 8 mismatch",
    )?;
    require(
        rig.storage.state.snapshot()?.pending().is_none(),
        "workflow failures predicate 9 mismatch",
    )?;
    let resumed = complete(rig.workflow()?.resume(ResumeTarget::Snapshot))?
        .map_err(|_| "workflow failures fixture 2 missing or invalid")?;
    require(
        resumed.metadata_uri() == METADATA,
        "workflow failures predicate 10 mismatch",
    )?;
    require(
        rig.procedures()?
            .iter()
            .filter(|procedure| *procedure == "publication.createIngestionSession")
            .count()
            == 1,
        "workflow failures predicate 11 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn dropping_pending_poll_cancels_owned_work_and_keeps_acknowledged_ingestion() -> Outcome {
    let mut rig = Rig::new(vec![created(), Step::Pending]);
    {
        let mut workflow = rig.workflow()?;
        let input = input();
        let mut operation = Box::pin(workflow.start(&input));
        poll_pending(operation.as_mut())?;
        drop(operation);
    }

    require(
        rig.portal.cancelled.get() == 1,
        "workflow failures predicate 12 mismatch",
    )?;
    require(
        rig.storage.state.snapshot()?.ingestion_id() == Some("ing"),
        "workflow failures predicate 13 mismatch",
    )?;
    require(
        rig.storage.state.snapshot()?.pending().is_none(),
        "workflow failures predicate 14 mismatch",
    )?;
    rig.portal.steps.borrow_mut().extend([
        ready(),
        session(Some(METADATA), "PreparedForMint"),
        json(&bundle(ICON).to_string()),
    ]);
    require(
        complete(rig.workflow()?.resume(ResumeTarget::Snapshot))?
            .map_err(|_| "workflow failures fixture 3 missing or invalid")?
            .metadata_uri()
            == METADATA,
        "workflow failures predicate 15 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn failed_acknowledgment_write_preserves_create_uncertainty_before_any_poll() -> Outcome {
    let mut rig = Rig::new(vec![created()]);
    rig.storage.state.fail_at.set(Some(2));
    require(
        matches!(
            complete(rig.workflow()?.start(&input()))?,
            Err(Error::Storage(_))
        ),
        "workflow failures predicate 16 mismatch",
    )?;
    require(
        rig.storage.state.snapshot()?.pending() == Some(Effect::CreateIngestion),
        "workflow failures predicate 17 mismatch",
    )?;
    require(
        matches!(
            complete(rig.workflow()?.resume(ResumeTarget::Snapshot))?,
            Err(Error::Uncertain(Effect::CreateIngestion))
        ),
        "workflow failures predicate 18 mismatch",
    )?;
    require(
        rig.procedures()? == ["publication.createIngestionSession"],
        "workflow failures predicate 19 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn interrupted_media_put_retains_public_reconciliation_data_and_never_retries_it() -> Outcome {
    let mut rig = Rig::new(vec![
        session(None, "PreparedForMint"),
        json(&bundle(ICON).to_string()),
        remote_file(),
        target(PUBLIC_ICON),
    ]);
    rig.uploads.0.pending.set(true);
    {
        let mut workflow = rig.workflow()?;
        let mut operation = Box::pin(workflow.resume(ResumeTarget::Release("rel".into())));
        poll_pending(operation.as_mut())?;
        drop(operation);
    }

    let snapshot = rig.storage.state.snapshot()?;
    require(
        snapshot.pending() == Some(Effect::Upload),
        "workflow failures predicate 20 mismatch",
    )?;
    let artifact = snapshot
        .pending_upload()
        .ok_or("workflow failures fixture 4 missing or invalid")?;
    require(
        artifact.sha256() == GIF_HASH,
        "workflow failures predicate 21 mismatch",
    )?;
    require(
        artifact.content_type() == "image/gif",
        "workflow failures predicate 22 mismatch",
    )?;
    require(
        artifact.public_url() == Some(PUBLIC_ICON),
        "workflow failures predicate 23 mismatch",
    )?;
    require(
        rig.uploads.0.cancelled.get() == 1,
        "workflow failures predicate 24 mismatch",
    )?;
    let bytes = rig.storage.state.data.borrow();

    require(
        !std::str::from_utf8(
            bytes
                .as_ref()
                .ok_or("workflow failures fixture 5 missing or invalid")?,
        )
        .map_err(|_| "workflow failures fixture 6 missing or invalid")?
        .contains("signature=synthetic"),
        "workflow failures predicate 25 mismatch",
    )?;
    drop(bytes);
    require(
        matches!(
            complete(rig.workflow()?.resume(ResumeTarget::Snapshot))?,
            Err(Error::Uncertain(Effect::Upload))
        ),
        "workflow failures predicate 26 mismatch",
    )?;
    require(
        rig.uploads.0.observations.borrow().len() == 1,
        "workflow failures predicate 27 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn failure_cleanup_is_explicit_and_an_uncertain_cleanup_cannot_be_replayed() -> Outcome {
    let failed = json(
        r#"{"id":"ing","releaseId":"rel","status":"Failed","processingError":"synthetic-provider-text"}"#,
    );
    let mut rig = Rig::new(vec![created(), failed, Step::Fail]);
    let error = complete(rig.workflow()?.start(&input()))?
        .err()
        .ok_or("workflow failures fixture 7 missing or invalid")?;
    require(
        !format!("{error:?} {error}").contains("synthetic-provider-text"),
        "workflow failures predicate 28 mismatch",
    )?;

    match error {
        Error::IngestionFailed(session) => require(
            session.processing_error
                == solana_kit_client::Presence::Present(Some("synthetic-provider-text".into())),
            "workflow failures predicate 29 mismatch",
        )?,
        _ => return Err("failed ingestion DTO was not retained"),
    }

    require(
        rig.procedures()?
            == [
                "publication.createIngestionSession",
                "publication.getIngestionSession",
            ],
        "workflow failures predicate 30 mismatch",
    )?;
    require(
        rig.storage.state.snapshot()?.release_id() == Some("rel"),
        "workflow failures predicate 31 mismatch",
    )?;
    require(
        matches!(
            complete(rig.workflow()?.cleanup_failed())?,
            Err(Error::Portal(_))
        ),
        "workflow failures predicate 32 mismatch",
    )?;
    require(
        rig.storage.state.snapshot()?.pending() == Some(Effect::Cleanup),
        "workflow failures predicate 33 mismatch",
    )?;
    require(
        matches!(
            complete(rig.workflow()?.cleanup_failed())?,
            Err(Error::Uncertain(Effect::Cleanup))
        ),
        "workflow failures predicate 34 mismatch",
    )?;
    require(
        rig.procedures()?.len() == 3,
        "workflow failures predicate 35 mismatch",
    )?;
    rig.valid()?;

    Ok(())
}

#[cfg_attr(test, test)]
pub fn preserved_submitted_cleanup_reconciles_without_resubmitting_to_store() -> Outcome {
    let failed =
        json(r#"{"id":"ing","releaseId":"rel","publicationSessionId":"pub","status":"failed"}"#);
    let mut rig = Rig::new(vec![
        created(),
        failed,
        json(r#"{"action":"preservedSubmitted"}"#),
        session(Some(METADATA), "Submitted"),
        json(&bundle(ICON).to_string()),
    ]);
    require(
        matches!(
            complete(rig.workflow()?.start(&input()))?,
            Err(Error::IngestionFailed(_))
        ),
        "workflow failures predicate 36 mismatch",
    )?;
    require(
        complete(rig.workflow()?.cleanup_failed())?
            .map_err(|_| "workflow failures fixture 8 missing or invalid")?
            .action
            == "preservedSubmitted",
        "workflow failures predicate 37 mismatch",
    )?;
    require(
        complete(rig.workflow()?.resume(ResumeTarget::Snapshot))?
            .map_err(|_| "workflow failures fixture 9 missing or invalid")?
            .stage()
            == Stage::Submitted,
        "workflow failures predicate 38 mismatch",
    )?;
    require(
        !rig.procedures()?
            .iter()
            .any(|procedure| procedure == "publication.submitToStore"),
        "workflow failures predicate 39 mismatch",
    )?;
    require(
        rig.uploads.0.observations.borrow().is_empty(),
        "workflow failures predicate 40 mismatch",
    )?;

    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn failed_put_does_not_trigger_cleanup_and_acknowledged_deletion_closes_resume() -> Outcome {
    let mut steps = hydrate_steps(ICON);
    steps.pop();
    steps.push(json(r#"{"action":"deleted"}"#));
    let mut rig = Rig::new(steps);
    rig.uploads.0.fail.set(true);
    require(
        matches!(
            complete(rig.workflow()?.resume(ResumeTarget::Release("rel".into())))?,
            Err(Error::Portal(_))
        ),
        "workflow failures predicate 41 mismatch",
    )?;
    require(
        rig.procedures()?
            .last()
            .ok_or("workflow failures fixture 10 missing or invalid")?
            == "publication.createUploadTarget",
        "workflow failures predicate 42 mismatch",
    )?;
    require(
        complete(rig.workflow()?.cleanup_failed())?
            .map_err(|_| "workflow failures fixture 11 missing or invalid")?
            .action
            == "deleted",
        "workflow failures predicate 43 mismatch",
    )?;

    require(
        matches!(
            complete(rig.workflow()?.resume(ResumeTarget::Snapshot))?,
            Err(Error::Invalid(_))
        ),
        "workflow failures predicate 44 mismatch",
    )?;
    require(
        rig.uploads.0.observations.borrow().len() == 1,
        "workflow failures predicate 45 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn ready_storage_completion_at_deadline_blocks_create_admission() -> Outcome {
    let mut rig = Rig::new(vec![]);
    rig.storage.state.advance_save.set(Some(DEADLINE));
    require(
        matches!(
            complete(rig.workflow()?.start(&input()))?,
            Err(Error::Deadline)
        ),
        "workflow failures predicate 46 mismatch",
    )?;
    require(
        rig.portal.requests.borrow().is_empty(),
        "workflow failures predicate 47 mismatch",
    )?;
    require(
        rig.storage.state.snapshot()?.pending() == Some(Effect::CreateIngestion),
        "workflow failures predicate 48 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn ready_upload_completion_at_deadline_keeps_upload_uncertain() -> Outcome {
    let mut rig = Rig::new(vec![
        session(None, "PreparedForMint"),
        json(&bundle(ICON).to_string()),
        remote_file(),
        target(PUBLIC_ICON),
    ]);
    rig.uploads.0.advance.set(Some(DEADLINE));
    require(
        matches!(
            complete(rig.workflow()?.resume(ResumeTarget::Release("rel".into())))?,
            Err(Error::Deadline)
        ),
        "workflow failures predicate 49 mismatch",
    )?;
    require(
        rig.storage.state.snapshot()?.pending() == Some(Effect::Upload),
        "workflow failures predicate 50 mismatch",
    )?;
    require(
        rig.uploads.0.observations.borrow().len() == 1,
        "workflow failures predicate 51 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}

#[cfg_attr(test, test)]
pub fn dropping_a_pending_checkpoint_write_cancels_storage_before_create_admission() -> Outcome {
    let mut rig = Rig::new(vec![]);
    rig.storage.state.pending_save.set(true);
    {
        let mut workflow = rig.workflow()?;
        let input = input();
        let mut operation = Box::pin(workflow.start(&input));
        poll_pending(operation.as_mut())?;
        drop(operation);
    }

    require(
        rig.storage.state.cancelled.get() == 1,
        "workflow failures predicate 52 mismatch",
    )?;
    require(
        rig.storage.state.data.borrow().is_none(),
        "workflow failures predicate 53 mismatch",
    )?;
    require(
        rig.portal.requests.borrow().is_empty(),
        "workflow failures predicate 54 mismatch",
    )?;
    rig.valid()?;
    Ok(())
}
