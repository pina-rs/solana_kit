//! Actual fallible workflow bodies preserve independent later groups after failure.

#[path = "../../../crates/solana_kit_dapp_publisher/tests/workflow/completion.rs"]
mod completion;

#[path = "../../../crates/solana_kit_dapp_publisher/tests/workflow/failures.rs"]
mod failures;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/workflow/fixtures.rs"]
mod fixtures;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/workflow/flows.rs"]
mod flows;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/workflow/media.rs"]
mod media;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/workflow/public_urls.rs"]
mod public_urls;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/workflow/support.rs"]
mod support;

use super::{checks, run_groups};
use crate::CaseResult;
use checks::Outcome;

pub(crate) fn cases() -> [CaseResult; 5] {
    run_groups([
        ("publisher-workflow-new-resume-and-metadata", flows),
        (
            "publisher-workflow-durable-effects-and-cancellation",
            durable,
        ),
        ("publisher-workflow-apk-headers-and-admission", media),
        ("publisher-workflow-public-url-boundaries", public_urls),
        ("publisher-workflow-completion-stages", completion_group),
    ])
}

fn completion_group() -> Outcome {
    completion::prepare_release_mint_uses_explicit_caller_inputs()?;
    completion::prepare_release_rejects_a_mismatched_publisher()?;
    completion::prepare_collection_requires_mint_saved_and_bundle_authority()?;
    completion::submit_records_intent_before_the_call_and_clears_it_after()?;
    completion::failed_submission_remains_uncertain_and_never_replays()?;
    completion::save_release_data_requires_the_acknowledged_stage_and_mint()?;
    completion::collection_verification_requires_the_matching_release()?;
    completion::store_submission_is_stage_gated_and_records_uncertainty()?;
    completion::store_submission_accepts_a_verified_session_and_returns_the_ticket()
}

fn flows() -> Outcome {
    flows::new_ingestion_polls_rehosts_media_and_uploads_the_complete_typed_document()?;
    flows::acknowledged_metadata_is_reused_on_resume_without_recreation_or_upload()?;
    flows::attached_ready_bundle_and_session_with_metadata_avoid_redundant_gets()?;
    flows::exact_public_r2_origins_reuse_media_while_host_and_port_lookalikes_get_rehosted()?;
    flows::session_selector_has_precedence_and_submitted_receipts_never_trigger_store_submission()?;
    flows::acknowledged_media_receipt_survives_later_write_failure_without_another_put()
}

fn durable() -> Outcome {
    failures::uncertain_create_is_not_retried_or_bypassed_by_a_resume_selector()?;
    failures::exhausted_polling_remains_resumable_without_another_create()?;
    failures::dropping_pending_poll_cancels_owned_work_and_keeps_acknowledged_ingestion()?;
    failures::failed_acknowledgment_write_preserves_create_uncertainty_before_any_poll()?;
    failures::interrupted_media_put_retains_public_reconciliation_data_and_never_retries_it()?;
    failures::failure_cleanup_is_explicit_and_an_uncertain_cleanup_cannot_be_replayed()?;
    failures::preserved_submitted_cleanup_reconciles_without_resubmitting_to_store()?;
    failures::failed_put_does_not_trigger_cleanup_and_acknowledged_deletion_closes_resume()?;
    failures::ready_storage_completion_at_deadline_blocks_create_admission()?;
    failures::ready_upload_completion_at_deadline_keeps_upload_uncertain()?;
    failures::dropping_a_pending_checkpoint_write_cancels_storage_before_create_admission()
}

fn media() -> Outcome {
    media::local_apk_uses_one_capped_immutable_read_for_hash_size_and_exact_upload_bytes()?;
    media::local_read_cancellation_deadline_and_byte_limits_prevent_network_admission()?;
    media::each_supported_image_header_has_an_independent_nonzero_dimension_vector()?;
    media::invalid_metadata_is_rejected_before_remote_fetch_or_upload()?;
    media::media_item_and_decoded_byte_limits_stop_the_next_effect()?;
    media::checkpoint_and_timestamp_admission_rejects_ambiguous_or_invalid_input()?;
    media::metadata_fallbacks_preserve_normalized_contacts_and_nonempty_subtype_extensions()?;
    media::conflicting_backend_identifiers_and_future_stages_cannot_trigger_media_work()?;
    media::malformed_presigned_targets_do_not_reach_the_uploader_or_get_retried()
}

fn public_urls() -> Outcome {
    public_urls::upload_target_aliases_and_nonpublic_locations_never_reach_put_or_durable_state()?;
    public_urls::local_apk_uses_the_same_public_target_admission_before_put()?;
    public_urls::forged_old_query_locations_cannot_be_loaded_reused_or_written_again()?;
    public_urls::session_bundle_and_attached_metadata_queries_reject_before_cache_or_media()?;
    public_urls::install_artifact_queries_reject_before_fetch_or_metadata_upload()?;
    public_urls::query_free_public_locations_succeed_while_signed_queries_stay_in_memory()
}

#[cfg(test)]
mod continuation {
    use super::*;
    use checks::require;

    fn wrong_metadata_oracle() -> Outcome {
        flows::new_ingestion_metadata_matches("https://wrong.invalid/metadata.json")
    }

    #[test]
    fn a_real_workflow_mismatch_keeps_later_runtime_groups_reportable() -> Outcome {
        let results = run_groups([
            ("changed-workflow-oracle", wrong_metadata_oracle),
            ("later-durable-effects", durable),
            ("later-media-boundaries", media),
            ("later-public-url-boundaries", public_urls),
        ]);
        require(
            results[0].outcome == Err("workflow flows predicate 6 mismatch"),
            "changed workflow oracle did not report its actual predicate failure",
        )?;
        require(
            results[3].name == "later-public-url-boundaries",
            "workflow continuation lost the later group identity",
        )?;
        require(
            results[1..].iter().all(|case| case.outcome.is_ok()),
            "workflow failure prevented later real runtime groups",
        )
    }

    #[test]
    fn a_fake_intent_violation_cannot_be_accepted_as_a_provider_failure() -> Outcome {
        let rig = support::Rig::new(std::vec![fixtures::created()]);
        let result = support::complete(rig.client.call(&fixtures::input()))?;
        require(result.is_err(), "missing durable intent was accepted")?;
        require(
            rig.valid() == Err("portal effect preceded durable intent"),
            "fake effect violation was not retained after its transport failure",
        )
    }
}
