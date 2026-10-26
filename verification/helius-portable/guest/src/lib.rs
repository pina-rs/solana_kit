//! Commits a report only after every original body returns in the actual interpreter.
#[cfg(target_arch = "wasm32")]
mod abi {
    use solana_kit_helius_portable::CORRUPTION_MARKER;

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
        assert!(!cfg!(feature = "unexpected-panic"), "unrelated guest panic");
        observe_corruption_panic();
        assert_eq!(
            solana_kit_helius_portable::run(),
            solana_kit_helius_portable::COUNT
        );
        let props = serde_json::json!({"text": solana_kit_helius_portable::REPORT}).to_string();

        // Safety: the production host copies the live UTF-8 allocation synchronously.
        unsafe {
            node_create(1, 1);
            node_set_props(1, props.as_ptr(), props.len());
            commit();
        }
    }

    /// Only the intended assertion becomes a marked host trap through the existing ABI.
    fn observe_corruption_panic() {
        let props = format!(r#"{{"hidden":"{CORRUPTION_MARKER}"}}"#);
        std::panic::set_hook(Box::new(move |info| {
            let message = info
                .payload()
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| info.payload().downcast_ref::<String>().map(String::as_str));

            if message == Some(CORRUPTION_MARKER) {
                // Safety: the host reads the live allocation synchronously. Its typed
                // boolean parser rejects this marker string before changing any node.
                unsafe {
                    node_set_props(1, props.as_ptr(), props.len());
                }
            }
        }));
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn leptoskit_event(_: u64, _: i32, _: u32, _: u32) -> i32 {
        0
    }
}
