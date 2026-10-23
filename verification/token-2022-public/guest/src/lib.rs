//! Executes every Token-2022 body through the pinned LeptosKit host ABI.

#[cfg(target_arch = "wasm32")]
mod abi {
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
        assert_eq!(solana_kit_token_2022_portable::run(), 119);
        let props = br#"{"text":"token-2022-public: 119/119"}"#;

        // Safety: the host synchronously copies the static report bytes.
        unsafe {
            node_create(1, 1);
            node_set_props(1, props.as_ptr(), props.len());
            commit();
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn leptoskit_event(_: u64, _: i32, _: u32, _: u32) -> i32 {
        0
    }
}
