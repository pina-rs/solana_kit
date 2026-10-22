//! Executes the official constructor and independent oracle inside real Chrome.
#![forbid(unsafe_code)]

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use solana_kit_system_boundary::verify;
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn official_create_account_matches_oracle() {
        assert_eq!(verify(false), Ok(()));
    }

    #[wasm_bindgen_test]
    fn corrupted_fixture_is_rejected() {
        assert_eq!(verify(true), Err("CreateAccount serialized bytes mismatch"));
    }
}
