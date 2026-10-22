//! Private Rust guest for the pinned LeptosKit rendering ABI, version 1.

#[cfg(target_arch = "wasm32")]
mod abi {
    use solana_kit_conformance::{FixtureMode, run};

    #[link(wasm_import_module = "leptoskit")]
    unsafe extern "C" {
        fn node_create(id: u64, kind: i32);
        fn node_set_props(id: u64, ptr: *const u8, len: usize);
        fn commit();
    }

    /// Serializes actual case outcomes; the host checks every field and failure.
    fn publish(mode: FixtureMode) {
        let report = run(mode);
        let cases: Vec<_> = report
            .cases
            .iter()
            .map(|case| serde_json::json!({"name": case.name, "error": case.outcome.err()}))
            .collect();
        let text = serde_json::json!({
            "suite_id": report.suite_id,
            "cases": cases,
            "passed": report.passed_count(),
            "success": report.is_success()
        })
        .to_string();
        let props = serde_json::json!({"text": text}).to_string();

        // Safety: the host synchronously copies this live UTF-8 allocation before return.
        // Node 1 is created by leptoskit_main before this function is called.
        unsafe {
            node_set_props(1, props.as_ptr(), props.len());
            commit();
        }
    }

    /// Declares the exact ABI understood by these private wrappers.
    #[unsafe(no_mangle)]
    pub extern "C" fn leptoskit_abi_version() -> u32 {
        1
    }

    /// Executes canonical fixtures when the actual host starts the guest.
    #[unsafe(no_mangle)]
    pub extern "C" fn leptoskit_main() {
        // Safety: node_create only accepts an owned integer ID and the ABI's text kind.
        unsafe { node_create(1, 1) };
        publish(FixtureMode::Canonical);
    }

    /// A press on the report node re-executes the suite with a corrupt hash oracle.
    #[unsafe(no_mangle)]
    pub extern "C" fn leptoskit_event(id: u64, kind: i32, _ptr: u32, _len: u32) -> i32 {
        if id != 1 || kind != 0 {
            return 0;
        }

        publish(FixtureMode::CorruptHash);
        1
    }
}
