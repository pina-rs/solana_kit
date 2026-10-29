//! Actual composed-store contracts, shared with the client's test target.
#[path = "../../../crates/solana_kit_client/tests/slot_store_contracts/mod.rs"]
mod contracts;

use contracts::{failures, lifecycle, ordering, reentry};

const CASES: [(&str, fn()); 7] = [
    (
        "slot-store-source-order-context",
        ordering::subscription_initial_order_equal_slots_and_complete_context,
    ),
    (
        "slot-store-retained-refresh",
        ordering::reconnect_stale_completion_reuses_retained_arc_exactly_once,
    ),
    (
        "slot-store-shared-first-failure",
        failures::first_failure_is_shared_without_cloning_and_ends_both_children,
    ),
    (
        "slot-store-map-before-watermark",
        failures::failed_mapping_does_not_advance_the_retained_watermark,
    ),
    (
        "slot-store-owned-lifecycle",
        lifecycle::fresh_sources_reset_dispose_drop_and_quiet_completion,
    ),
    (
        "slot-store-caller-cancel",
        lifecycle::caller_cancel_prevents_initial_io_and_wins_ready_results,
    ),
    (
        "slot-store-reentry",
        reentry::listener_poll_and_destructor_reentry_preserve_lifecycle_ownership,
    ),
];

pub(crate) fn cases() -> [crate::CaseResult; 7] {
    CASES.map(|(name, body)| {
        body();
        crate::case(name, Ok(()))
    })
}
