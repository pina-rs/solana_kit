//! Host-neutral execution of the accepted opt-in Reader transaction contracts.
#![forbid(unsafe_code)]

mod contracts;

/// Executes all five original Reader control bodies and returns their count.
pub fn run() -> usize {
    contracts::run()
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn every_reader_contract_executes() {
        assert_eq!(super::run(), 5);
    }
}
