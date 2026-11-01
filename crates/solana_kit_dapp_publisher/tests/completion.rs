#![cfg(feature = "workflow")]
#![allow(dead_code)]
//! Explicit completion methods keep mint, verification and store mutations caller-driven.

#[path = "support/checks.rs"]
mod checks;

#[path = "workflow/fixtures.rs"]
mod fixtures;

#[path = "workflow/support.rs"]
mod support;

use checks::{Outcome, require};
use fixtures::{
    ADDRESS, ICON, METADATA, bundle, created, hydrate_steps, input, queued, ready, session,
};
use serde_json::json as value;
use solana_kit_client::Presence;
use solana_kit_dapp_publisher::{
    portal::StoreAttestation,
    workflow::{
        Effect, Error, HydratedPublication, MintPreparation, MintSave, ResumeTarget,
        StoreSubmission, TransactionPurpose,
    },
};
use std::vec;
use support::{Rig, Step, complete, json};

fn started() -> Result<(Rig, HydratedPublication), &'static str> {
    let mut steps = vec![created(), queued(), ready()];
    steps.extend(hydrate_steps(ICON));
    let mut rig = Rig::new(steps);
    let hydrated = {
        let mut workflow = rig.workflow()?;
        complete(workflow.start(&input()))?.map_err(|_| "completion start failed")?
    };
    rig.portal.requests.borrow_mut().clear();
    Ok((rig, hydrated))
}

fn at_stage(stage: &str) -> Result<(Rig, HydratedPublication), &'static str> {
    let (mut rig, _) = started()?;
    rig.portal
        .steps
        .borrow_mut()
        .push_back(session(Some(METADATA), stage));
    rig.portal
        .steps
        .borrow_mut()
        .push_back(json(&bundle(ICON).to_string()));
    let hydrated = {
        let mut workflow = rig.workflow()?;
        complete(workflow.resume(ResumeTarget::Snapshot))?
            .map_err(|_| "completion resume failed")?
    };
    rig.portal.requests.borrow_mut().clear();
    Ok((rig, hydrated))
}

fn push(rig: &Rig, payload: &str) {
    rig.portal.steps.borrow_mut().push_back(json(payload));
}

#[test]
fn prepare_release_mint_uses_explicit_caller_inputs() -> Outcome {
    let (mut rig, hydrated) = started()?;
    push(
        &rig,
        r#"{"transaction":"dHg=","mintAddress":"11111111111111111111111111111111","blockhash":"11111111111111111111111111111111"}"#,
    );
    let result = {
        let workflow = rig.workflow()?;
        complete(workflow.prepare_release_mint(
            &hydrated,
            MintPreparation {
                release_name: "Fixture",
                app_mint_address: ADDRESS,
                publisher_address: ADDRESS,
                payer_address: ADDRESS,
            },
        ))?
        .map_err(|_| "prepare release mint failed")?
    };

    require(result.mint_address == ADDRESS, "mint address changed")?;
    require(
        rig.procedures()? == ["publication.prepareReleaseNftTransaction"],
        "unexpected prepare procedure",
    )
}

#[test]
fn prepare_release_rejects_a_mismatched_publisher() -> Outcome {
    let (mut rig, hydrated) = started()?;
    let error = {
        let workflow = rig.workflow()?;
        complete(workflow.prepare_release_mint(
            &hydrated,
            MintPreparation {
                release_name: "Fixture",
                app_mint_address: ADDRESS,
                publisher_address: "other",
                payer_address: ADDRESS,
            },
        ))?
        .err()
        .ok_or("mismatched publisher was accepted")?
    };

    require(
        matches!(error, Error::Invalid(_)),
        "mismatched publisher was not rejected",
    )?;
    require(
        rig.procedures()?.is_empty(),
        "portal was called before policy passed",
    )
}

#[test]
fn prepare_collection_requires_mint_saved_and_bundle_authority() -> Outcome {
    let (mut rig, hydrated) = started()?;
    let error = {
        let workflow = rig.workflow()?;
        complete(workflow.prepare_collection_verification(&hydrated, ADDRESS))?
            .err()
            .ok_or("prepared verification at the wrong stage")?
    };
    require(
        matches!(error, Error::Invalid(_)),
        "wrong-stage verification was not rejected",
    )?;

    let (mut rig, hydrated) = at_stage("MintSaved")?;
    push(
        &rig,
        r#"{"transaction":"dHg=","blockhash":"11111111111111111111111111111111"}"#,
    );
    let result = {
        let workflow = rig.workflow()?;
        complete(workflow.prepare_collection_verification(&hydrated, ADDRESS))?
            .map_err(|_| "prepare collection failed")?
    };

    require(result.transaction == "dHg=", "prepared wire changed")?;
    require(
        rig.procedures()? == ["publication.prepareVerifyCollectionTransaction"],
        "unexpected verification procedure",
    )
}

#[test]
fn submit_records_intent_before_the_call_and_clears_it_after() -> Outcome {
    let (mut rig, hydrated) = started()?;
    push(&rig, r#"{"transactionSignature":"sig"}"#);
    let result = {
        let mut workflow = rig.workflow()?;
        complete(workflow.submit_signed_transaction(
            &hydrated,
            TransactionPurpose::ReleaseMint,
            "dHg=",
        ))?
        .map_err(|_| "submit transaction failed")?
    };

    require(result.transaction_signature == "sig", "signature changed")?;
    require(
        rig.storage.state.snapshot()?.pending().is_none(),
        "acknowledged submission stayed pending",
    )?;
    require(
        rig.procedures()? == ["publication.submitSignedTransaction"],
        "unexpected submission procedure",
    )
}

#[test]
fn failed_submission_remains_uncertain_and_never_replays() -> Outcome {
    let (mut rig, hydrated) = started()?;
    rig.portal.steps.borrow_mut().push_back(Step::Fail);
    let error = {
        let mut workflow = rig.workflow()?;
        complete(workflow.submit_signed_transaction(
            &hydrated,
            TransactionPurpose::ReleaseMint,
            "dHg=",
        ))?
        .err()
        .ok_or("failed submission was accepted")?
    };

    require(
        matches!(error, Error::Portal(_)),
        "wrong submission failure",
    )?;
    require(
        rig.storage.state.snapshot()?.pending() == Some(Effect::SubmitTransaction),
        "failed submission lost its durable intent",
    )?;
    let replay = {
        let mut workflow = rig.workflow()?;
        complete(workflow.resume(ResumeTarget::Snapshot))?
            .err()
            .ok_or("uncertain submission resumed")?
    };
    require(
        matches!(replay, Error::Uncertain(Effect::SubmitTransaction)),
        "uncertain submission was not reported",
    )
}

#[test]
fn save_release_data_requires_the_acknowledged_stage_and_mint() -> Outcome {
    let (mut rig, hydrated) = at_stage("MintSubmitted")?;
    push(&rig, r#"{"success":false}"#);
    let error = {
        let mut workflow = rig.workflow()?;
        complete(workflow.save_release_nft_data(
            &hydrated,
            MintSave {
                mint_address: ADDRESS,
                transaction_signature: "sig",
                owner_address: ADDRESS,
                release_name: "Fixture",
                release_version: "2.0",
                android_package: "test.fixture",
                app_mint_address: ADDRESS,
            },
        ))?
        .err()
        .ok_or("negative save acknowledgement was accepted")?
    };
    require(
        matches!(error, Error::Invalid(_)),
        "negative acknowledgement was not rejected",
    )
}

#[test]
fn collection_verification_requires_the_matching_release() -> Outcome {
    let (mut rig, hydrated) = at_stage("VerificationSubmitted")?;
    push(&rig, r#"{"success":true,"releaseId":"other"}"#);
    let error = {
        let mut workflow = rig.workflow()?;
        complete(workflow.mark_collection_verified(&hydrated))?
            .err()
            .ok_or("mismatched verification was accepted")?
    };
    require(
        matches!(error, Error::Invalid(_)),
        "mismatched verification was not rejected",
    )
}

#[test]
fn store_submission_is_stage_gated_and_records_uncertainty() -> Outcome {
    let (mut rig, hydrated) = started()?;
    let error = {
        let mut workflow = rig.workflow()?;
        complete(workflow.submit_to_store(
            &hydrated,
            StoreSubmission {
                whats_new: "New version",
                critical_update: false,
                testing_instructions: None,
                is_resubmission: None,
                attestation: StoreAttestation {
                    payload: "signed".into(),
                    request_unique_id: "request".into(),
                },
            },
        ))?
        .err()
        .ok_or("store submission at the wrong stage was accepted")?
    };
    require(
        matches!(error, Error::Invalid(_)),
        "wrong-stage store submission was not rejected",
    )?;

    let (mut rig, hydrated) = at_stage("Attested")?;
    rig.portal.steps.borrow_mut().push_back(Step::Fail);
    let error = {
        let mut workflow = rig.workflow()?;
        complete(workflow.submit_to_store(
            &hydrated,
            StoreSubmission {
                whats_new: "New version",
                critical_update: false,
                testing_instructions: None,
                is_resubmission: None,
                attestation: StoreAttestation {
                    payload: "signed".into(),
                    request_unique_id: "request".into(),
                },
            },
        ))?
        .err()
        .ok_or("failed store submission was accepted")?
    };
    require(matches!(error, Error::Portal(_)), "wrong store failure")?;
    require(
        rig.storage.state.snapshot()?.pending() == Some(Effect::SubmitStore),
        "failed store submission lost its durable intent",
    )
}

#[test]
fn store_submission_accepts_a_verified_session_and_returns_the_ticket() -> Outcome {
    let (mut rig, hydrated) = at_stage("Verified")?;
    push(&rig, r#"{"hubspotTicketId":"ticket"}"#);
    let result = {
        let mut workflow = rig.workflow()?;
        complete(workflow.submit_to_store(
            &hydrated,
            StoreSubmission {
                whats_new: "New version",
                critical_update: true,
                testing_instructions: Some("Use the test account"),
                is_resubmission: Some(false),
                attestation: StoreAttestation {
                    payload: "signed".into(),
                    request_unique_id: "request".into(),
                },
            },
        ))?
        .map_err(|_| "store submission failed")?
    };

    require(
        matches!(result.hubspot_ticket_id, Presence::Present(Some(_))),
        "ticket identity was not exposed",
    )?;
    let requests = rig.portal.requests.borrow();
    let body = requests[0].body.as_deref().ok_or("missing body")?;
    let value: serde_json::Value =
        serde_json::from_slice(body).map_err(|_| "invalid submission body")?;
    require(
        value["criticalUpdate"] == value!(true),
        "critical update flag changed",
    )?;
    require(
        value["isResubmission"] == value!(false),
        "explicit false resubmission was dropped",
    )
}
