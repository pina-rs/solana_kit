use super::{asynchronous, client, core, diagnostics, packing, resources};
use alloc::vec::Vec;

const HOST_NEUTRAL_CONTRACTS: usize = 43;

pub(crate) fn cases() -> Vec<crate::CaseResult> {
    let mut cases = Vec::with_capacity(HOST_NEUTRAL_CONTRACTS);
    cases.extend(core());
    cases.extend(client());
    cases.extend(asynchronous());
    cases.extend(packing());
    cases.extend(execution());
    cases.extend(diagnostics());

    cases.extend(independent_async());
    cases.extend(resources());
    cases
}

fn core() -> [crate::CaseResult; 11] {
    [
        crate::case(
            "plans-dart-tree-wire",
            core::actual_dart_shapes_and_wire_match(),
        ),
        crate::case(
            "plans-tree-traversal",
            core::tree_inputs_search_transform_and_flatten_preserve_boundaries(),
        ),
        crate::case(
            "plans-invalid-input",
            core::empty_oversized_and_unresolved_fail_without_partial_output(),
        ),
        crate::case(
            "plans-greedy-cursor",
            core::greedy_packer_does_not_consume_rejected_work(),
        ),
        crate::case(
            "plans-packet-linear-limits",
            core::exact_packet_boundary_and_linear_chunks_use_compiler(),
        ),
        crate::case(
            "plans-transactional-append",
            core::stateful_update_overflow_aborts_and_append_is_transactional(),
        ),
        crate::case(
            "plans-invalid-packer",
            core::invalid_custom_packer_cannot_spin_lose_work_or_change_metadata(),
        ),
        crate::case(
            "plans-v1-instruction-limits",
            core::v1_capacity_and_instruction_limits_are_exact_and_version_specific(),
        ),
        crate::case(
            "plans-invocation-limit",
            core::invocation_budget_does_not_leak_into_later_plans(),
        ),
        crate::case(
            "plans-builder-cause",
            core::builder_failure_preserves_original_cause_without_returning_a_partial_plan(),
        ),
        crate::case(
            "plans-candidate-rejection",
            core::custom_rejections_try_fresh_candidates_and_factories_restart_each_invocation(),
        ),
    ]
}

fn client() -> [crate::CaseResult; 8] {
    [
        crate::case(
            "plans-sequence-failure",
            client::failed_sequence_retains_success_original_error_context_and_canceled_tail(),
        ),
        crate::case(
            "plans-local-overlap-wake",
            client::parallel_futures_overlap_preserve_order_and_wake_with_local_rc(),
        ),
        crate::case(
            "plans-sequence-drop",
            client::sequential_pending_cannot_overtake_and_drop_cancels_actual_owned_futures(),
        ),
        crate::case(
            "plans-abort-settle",
            client::abort_settles_started_work_and_cancels_unstarted_tail(),
        ),
        crate::case(
            "plans-concurrent-leaves",
            client::concurrent_leaves_allow_atomic_groups_and_do_not_halt_on_failure(),
        ),
        crate::case(
            "plans-sign-send-errors",
            client::boundary_preserves_unsigned_transaction_and_exact_signing_or_sending_error(),
        ),
        crate::case(
            "plans-preflight-diagnostics",
            client::preflight_diagnostics_preserve_payload_indices_logs_and_signing_indicators(),
        ),
        crate::case(
            "plans-dart-result-oracle",
            client::actual_dart_failure_and_concurrent_result_oracles_match(),
        ),
    ]
}

fn asynchronous() -> [crate::CaseResult; 8] {
    [
        crate::case(
            "plans-pending-context",
            asynchronous::local_pending_callbacks_read_each_actual_proposal_context(),
        ),
        crate::case(
            "plans-later-callback-errors",
            asynchronous::later_construction_and_update_errors_return_original_without_a_tree(),
        ),
        crate::case(
            "plans-pending-callback-drop",
            asynchronous::dropping_pending_create_or_update_releases_owned_callback_and_builder(),
        ),
        crate::case(
            "plans-async-no-progress",
            asynchronous::async_no_progress_aborts_after_one_construction(),
        ),
        crate::case(
            "plans-consumed-update-error",
            asynchronous::stateful_async_updater_failure_after_prior_candidate_aborts_without_retry(
            ),
        ),
        crate::case(
            "plans-awaited-dart-oracle",
            asynchronous::async_dart_callback_trace_and_message_bytes_match(),
        ),
        crate::case(
            "plans-sync-wildcard",
            asynchronous::synchronous_wildcard_imports_keep_unambiguous_builder_methods(),
        ),
        crate::case(
            "plans-await-before-signing",
            asynchronous::boundary_awaits_later_planning_error_before_starting_signing_or_sending(),
        ),
    ]
}

fn packing() -> [crate::CaseResult; 5] {
    [
        crate::case(
            "plans-literal-packet-cursor",
            packing::literal_boundary_fit_overflow_and_rejected_cursor_rollback(),
        ),
        crate::case(
            "plans-later-builder-error",
            packing::builder_error_after_accepted_candidate_returns_original_failure(),
        ),
        crate::case(
            "plans-bounded-no-progress",
            packing::unchanged_empty_candidate_fails_in_one_attempt(),
        ),
        crate::case(
            "plans-complete-packer",
            packing::initially_complete_packer_neither_creates_empty_message_nor_loses_prior_work(),
        ),
        crate::case(
            "plans-lookup-wire-order",
            packing::lookup_signer_duplicate_positions_and_wire_order_survive_planning(),
        ),
    ]
}

fn execution() -> [crate::CaseResult; 3] {
    use super::execution::{
        concurrent_leaves_cross_nested_sequence_edges_and_keep_error_tree as concurrent,
        nested_abort_and_drop_observe_actual_local_future_ownership as abort,
        nested_pending_failure_settles_started_siblings_and_cancels_both_serial_tails as failure,
    };

    [
        crate::case("plans-nested-failure-settle", failure()),
        crate::case("plans-nested-abort-drop", abort()),
        crate::case("plans-nested-concurrent", concurrent()),
    ]
}

fn diagnostics() -> [crate::CaseResult; 2] {
    [
        crate::case(
            "plans-typed-diagnostics",
            diagnostics::nested_diagnostics_keep_typed_payload_flat_indices_and_abort_reason(),
        ),
        crate::case(
            "plans-abort-reason",
            diagnostics::all_canceled_report_keeps_explicit_caller_reason(),
        ),
    ]
}

fn independent_async() -> [crate::CaseResult; 4] {
    use super::independent_async::{
        consumed_cursor_update_rejection_capacity_and_invalid_prefix_abort_without_retry as consumed,
        dropping_pending_consumed_update_releases_cursor_and_callback_owners as drop_owners,
        external_registered_wakes_gate_each_actual_proposal_without_reacquisition as wakes,
        later_nested_update_error_keeps_original_cause_and_stops_unstarted_tail as later,
    };

    [
        crate::case("plans-external-wakes", wakes()),
        crate::case("plans-consumed-update-errors", consumed()),
        crate::case("plans-consumed-update-drop", drop_owners()),
        crate::case("plans-later-nested-create-error", later()),
    ]
}

fn resources() -> [crate::CaseResult; 2] {
    [
        crate::case(
            "plans-depth-width-512",
            resources::explicit_512_depth_and_width_inputs_preserve_output_and_packet_budgets(),
        ),
        crate::case(
            "plans-invalid-consumed-output",
            resources::successful_consumed_packer_with_invalid_compiled_output_never_retries(),
        ),
    ]
}
