//! Executes the same System oracle through pinned LeptosKit ABI version 1.

#[cfg(target_arch = "wasm32")]
mod abi {
    #[link(wasm_import_module = "leptoskit")]
    unsafe extern "C" {
        fn node_create(id: u64, kind: i32);
        fn node_set_props(id: u64, ptr: *const u8, len: usize);
        fn commit();
    }

    /// Publishes actual official-constructor results through the host surface.
    fn publish(corrupt: bool) {
        let outcome = solana_kit_system_boundary::verify(corrupt);
        let text = format!("solana-kit-system-create-account-v1: {outcome:?}");
        let props = serde_json::json!({"text": text}).to_string();

        // Safety: the pinned host synchronously copies the live UTF-8 allocation;
        // leptoskit_main has already created the report node with ID 1.
        unsafe {
            node_set_props(1, props.as_ptr(), props.len());
            commit();
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn leptoskit_abi_version() -> u32 {
        1
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn leptoskit_main() {
        // Safety: these integers are the pinned ABI's report ID and text kind.
        unsafe { node_create(1, 1) };
        publish(false);
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn leptoskit_event(id: u64, kind: i32, _ptr: u32, _len: u32) -> i32 {
        if id != 1 || kind != 0 {
            return 0;
        }

        publish(true);
        1
    }
}
