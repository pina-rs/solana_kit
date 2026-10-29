//! Actual reactive bodies, shared with the client's native test target.
#[path = "../../../crates/solana_kit_client/tests/reactive_contracts/mod.rs"]
mod contracts;

use contracts::{action, bridge, reentry, streams};

const CONTRACTS: [(&str, fn()); 16] = [
    (
        "reactive-action-refresh",
        action::refresh_retains_result_and_error_until_success,
    ),
    (
        "reactive-action-supersede-reset",
        action::supersession_and_reset_drop_work_before_late_producers_complete,
    ),
    (
        "reactive-action-caller-cancel",
        action::caller_cancellation_wins_ready_completion_and_prevents_initial_io,
    ),
    (
        "reactive-action-drop-dispose",
        action::dropped_and_disposed_drivers_release_actual_owned_work,
    ),
    (
        "reactive-stream-reconnect",
        streams::reconnect_retains_data_error_and_drops_both_previous_lanes,
    ),
    (
        "reactive-stream-errors-end",
        streams::recoverable_errors_nulls_and_source_end_preserve_source_semantics,
    ),
    (
        "reactive-stream-cancel-ownership",
        streams::caller_scope_reset_dispose_and_driver_drop_cancel_stream_work,
    ),
    (
        "reactive-stream-factory-drop",
        streams::pending_factory_is_destroyed_by_reset_dispose_and_unpolled_drop,
    ),
    (
        "reactive-bridge-latest-accepted",
        bridge::bridge_seeds_and_coalesces_only_accepted_values,
    ),
    (
        "reactive-bridge-error-priority",
        bridge::bridge_retains_error_ahead_of_buffered_data_and_later_recovery,
    ),
    (
        "reactive-bridge-predicate-drop",
        bridge::fallible_local_predicate_and_drop_release_the_listener,
    ),
    (
        "reactive-bridge-cancel-wake",
        bridge::cancellation_wakes_parked_bridge_and_wins_initial_snapshot,
    ),
    (
        "reactive-local-payload-errors",
        bridge::local_payloads_and_errors_require_neither_send_nor_clone,
    ),
    (
        "reactive-listener-reentry-cycle",
        reentry::listeners_can_reset_and_unsubscribe_without_source_io_or_cycles,
    ),
    (
        "reactive-future-poll-drop-reentry",
        reentry::future_poll_and_destructor_reentry_cannot_restore_or_commit_stale_work,
    ),
    (
        "reactive-stream-predicate-reentry",
        reentry::stream_listeners_and_local_predicates_can_reset_reentrantly,
    ),
];

pub(crate) fn cases() -> [crate::CaseResult; 16] {
    CONTRACTS.map(|(name, body)| {
        body();
        crate::case(name, Ok(()))
    })
}
