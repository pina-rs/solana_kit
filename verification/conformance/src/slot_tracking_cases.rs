//! Original slot-stream contracts shared with the client's native test target.
#[path = "../../../crates/solana_kit_client/tests/slot_tracking_contracts/mod.rs"]
mod contracts;

use contracts::{failures, lifecycle, ordering};

pub(crate) fn cases() -> [crate::CaseResult; 5] {
    [
        (
            "slot-tracking-order-context",
            ordering::ordered_values_and_contexts as fn(),
        ),
        (
            "slot-tracking-both-inputs-drain",
            ordering::first_round_and_equal_slot_drain,
        ),
        (
            "slot-tracking-error-watermark",
            failures::errors_are_nonterminal_and_mapping_advances_watermark,
        ),
        (
            "slot-tracking-dual-completion",
            lifecycle::completion_orders_and_queued_initial,
        ),
        (
            "slot-tracking-owned-drop-local",
            lifecycle::drop_local_payloads_and_stale_wakeup,
        ),
    ]
    .map(|(name, body)| {
        body();
        crate::case(name, Ok(()))
    })
}
