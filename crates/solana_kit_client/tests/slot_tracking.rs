#![cfg(feature = "reactive")]

mod slot_tracking_contracts;

#[test]
fn subscription_values_never_regress_the_slot() {
    slot_tracking_contracts::ordering::ordered_values_and_contexts();
}

#[test]
fn both_inputs_start_and_pending_values_drain_fairly() {
    slot_tracking_contracts::ordering::first_round_and_equal_slot_drain();
}

#[test]
fn source_and_mapper_errors_preserve_watermark_semantics() {
    slot_tracking_contracts::failures::errors_are_nonterminal_and_mapping_advances_watermark();
}

#[test]
fn termination_waits_for_both_inputs_and_fuses_them() {
    slot_tracking_contracts::lifecycle::completion_orders_and_queued_initial();
}

#[test]
fn actual_owned_work_and_local_values_follow_stream_lifetime() {
    slot_tracking_contracts::lifecycle::drop_local_payloads_and_stale_wakeup();
}
