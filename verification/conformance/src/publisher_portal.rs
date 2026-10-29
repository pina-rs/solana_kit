//! Runtime predicates share fallible native/portable bodies; Send witnesses are native-only.

#[path = "../../../crates/solana_kit_dapp_publisher/tests/portal/boundaries.rs"]
mod boundaries;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/portal/retry.rs"]
mod retry;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/portal/routes.rs"]
mod routes;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/portal/strict_boundaries.rs"]
mod strict_boundaries;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/portal/support.rs"]
mod support;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/portal/upload.rs"]
mod upload;

use super::{checks, run_groups};
use crate::CaseResult;
use checks::Outcome;

/// Appends four actual outcomes without stopping after an independent group failure.
pub(crate) fn cases() -> [CaseResult; 4] {
    run_groups([
        ("publisher-portal-thirteen-exact-routes", route_group),
        (
            "publisher-portal-envelopes-admission-and-absence",
            envelopes,
        ),
        ("publisher-portal-retry-deadline-and-cancellation", retries),
        ("publisher-portal-independent-binary-upload", uploads),
    ])
}

fn route_group() -> Outcome {
    routes::all_four_query_routes_keep_json_in_query_data_and_never_hydrate_other_resources()?;
    routes::upload_ingestion_cleanup_and_store_routes_have_exact_source_wire_forms()?;
    routes::transaction_mutations_require_explicit_calls_and_return_untrusted_proposals()?;
    routes::remote_source_variants_omit_only_the_source_optional_fields()
}

fn envelopes() -> Outcome {
    boundaries::direct_data_direct_result_batch_and_right_envelopes_share_the_same_typed_boundary(
    )?;
    boundaries::provider_and_http_errors_preserve_bounded_evidence_without_implicit_disclosure()?;
    boundaries::invalid_envelopes_known_fields_arrays_and_private_number_objects_are_rejected()?;
    boundaries::optional_backend_fields_keep_missing_null_false_and_exact_numbers_distinct()?;
    boundaries::request_and_response_budgets_fail_without_empty_success_or_unbounded_exchange()?;
    boundaries::dropping_a_pending_portal_future_cancels_the_real_owned_exchange()?;
    #[cfg(not(target_arch = "wasm32"))]
    boundaries::concrete_native_adapters_preserve_send_while_rc_adapters_remain_usable()?;
    strict_boundaries::null_envelope_branches_cannot_masquerade_as_missing_fields()?;
    strict_boundaries::legitimate_nullable_backend_fields_still_survive_every_valid_envelope_form()
}

fn retries() -> Outcome {
    retry::explicit_ingestion_retries_reuse_exact_input_with_source_delays_and_three_attempts()?;
    retry::permanent_schema_duplicate_budget_and_auth_failures_do_not_retry()?;
    retry::exhausted_transient_errors_and_side_effecting_operations_never_become_success()?;
    retry::retry_policy_validation_and_deadlines_stop_admission_and_backoff()?;
    retry::cancellation_and_clock_failure_drop_owned_attempts_without_future_admission()?;
    retry::elapsed_deadline_drops_the_pending_attempt_without_retrying()?;
    strict_boundaries::deadline_wins_when_ready_exchange_advances_clock_to_or_past_completion_limit(
    )
}

fn uploads() -> Outcome {
    upload::requesting_a_target_does_not_upload_and_uploading_cannot_access_portal_auth()?;
    upload::body_limits_header_rejection_and_drop_are_enforced_before_or_during_upload()?;
    #[cfg(not(target_arch = "wasm32"))]
    upload::concrete_native_uploader_futures_preserve_send_without_imposing_it_on_local_uploaders(
    )?;
    Ok(())
}

#[cfg(test)]
mod continuation {
    use super::*;

    fn wrong_expected_procedure() -> Outcome {
        routes::check(
            solana_kit_dapp_publisher::portal::GetBlockData {},
            true,
            "deliberately-wrong-procedure",
            serde_json::json!({}),
            r#"{"slot":18446744073709551615,"blockhash":"untrusted"}"#,
        )
        .map(|_| ())
    }

    #[test]
    fn actual_route_failure_is_reported_and_all_later_portal_groups_continue() {
        let results = run_groups([
            ("changed-route-oracle", wrong_expected_procedure),
            ("later-envelope-assertions", envelopes),
            ("later-retry-assertions", retries),
            ("later-upload-assertions", uploads),
        ]);
        assert_eq!(results[0].outcome, Err("portal procedure/path mismatch"));
        assert_eq!(results[3].name, "later-upload-assertions");
        assert!(results[1..].iter().all(|case| case.outcome.is_ok()));
    }
}
