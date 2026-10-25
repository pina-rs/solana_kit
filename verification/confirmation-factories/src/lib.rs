//! Shared deterministic factory contracts. No networking claim is made here.
#[path = "../../../crates/solana_kit_client/tests/confirmation_races.rs"]
mod races;
#[path = "../../../crates/solana_kit_client/tests/shared_factory.rs"]
mod shared;
#[path = "../../../crates/solana_kit_client/tests/confirmation_watch.rs"]
mod watches;

/// Executes the seven original and four shared-operation behavioral bodies on every portable host.
pub fn run() -> u32 {
    races::height_expiry_reconciles_confirmation_that_arrived_during_the_height_read();
    races::nonce_invalidation_reconciles_confirmation_that_arrived_during_the_account_read();
    races::stale_final_status_cannot_establish_height_or_nonce_expiry();
    races::current_absent_status_still_reports_actual_height_and_nonce_expiry();
    races::final_execution_failure_is_preserved_before_lifetime_expiry();
    watches::independent_confirmation_watches_retain_other_notifications_and_socket();
    watches::receipt_notification_rearms_poll_before_later_execution_notification();
    shared::pending_local_rc_operation_releases_owner_and_preserves_unrelated_root();

    shared::one_shot_submission_preserves_original_uncertain_error_and_cleanup();
    shared::submission_guards_run_before_subscription_or_http();
    shared::cancellation_late_ack_false_ack_deadline_and_disconnect_are_explicit();
    11
}

/// Executes the two additive fallible-clock contracts without changing the original eleven.
pub fn run_clock_contracts() -> u32 {
    shared::clock_construction_failures_prevent_submission_and_retain_failed_cleanup();
    shared::poll_clock_failure_retains_original_cause_without_resending_signed_bytes();
    2
}

#[cfg(all(
    target_arch = "wasm32",
    target_os = "unknown",
    feature = "leptoskit-guest"
))]
mod guest {
    #[link(wasm_import_module = "leptoskit")]
    unsafe extern "C" {
        fn node_create(id: u64, kind: i32);
        fn node_set_props(id: u64, ptr: *const u8, len: usize);
        fn commit();
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn leptoskit_abi_version() -> u32 {
        1
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn leptoskit_main() {
        let count = super::run();
        assert_eq!(super::run_clock_contracts(), 2);
        let props = format!(r#"{{"text":"confirmation-factory-contracts {count}/11"}}"#);

        // Safety: the pinned host copies this live UTF-8 allocation synchronously.
        unsafe {
            node_create(1, 1);
            node_set_props(1, props.as_ptr(), props.len());
            commit();
        }
    }
    /// This static report has no event-driven actions.
    #[unsafe(no_mangle)]
    pub extern "C" fn leptoskit_event(_id: u64, _kind: i32, _ptr: u32, _len: u32) -> i32 {
        0
    }
}
#[cfg(all(test, target_arch = "wasm32", target_os = "unknown"))]
mod browser {
    mod network;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn same_eleven_contracts_execute_in_chrome() {
        assert_eq!(super::run(), 11);
    }

    #[wasm_bindgen_test]
    fn fallible_clock_contracts_execute_in_chrome() {
        assert_eq!(super::run_clock_contracts(), 2);
    }
}
