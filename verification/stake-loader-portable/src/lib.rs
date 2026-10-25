//! Exact public client contracts, shared by native and Wasm runtimes.
#![forbid(unsafe_code)]
mod registry;
pub use registry::COUNT;

/// Executes every registered production test body and returns its exact count.
pub fn run() -> usize {
    assert!(
        !cfg!(feature = "corrupt"),
        "intentional portable contract corruption"
    );

    for (name, body) in &registry::CASES {
        body();
        println!("PASS {name}");
    }

    COUNT
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_test_configure!(run_in_browser);
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn all_original_and_new_contracts() {
        assert_eq!(super::run(), super::COUNT);
    }
}
