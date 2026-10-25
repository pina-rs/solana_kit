//! Executes original public client test bodies across native and Wasm hosts.
#![forbid(unsafe_code)]
mod registry;
pub use registry::COUNT;

/// Returns the full count only after every original assertion has executed.
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
    fn every_original_contract_executes() {
        assert_eq!(super::run(), super::COUNT);
    }
}
