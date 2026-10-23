//! All original Token-2022 client contracts on host-neutral runtime boundaries.
#![forbid(unsafe_code)]

const ORIGINAL_BODIES: usize = 116;
const INDEPENDENT_BODIES: usize = 3;

/// Executes every original and independent body, preserving ordinary assertions.
pub fn run() -> usize {
    let original = solana_kit_token_2022_contracts::bodies();
    let independent = solana_kit_token_2022_contracts::independent_bodies();
    assert_eq!(original.len(), ORIGINAL_BODIES);
    assert_eq!(independent.len(), INDEPENDENT_BODIES);

    let bodies = original.into_iter().chain(independent);
    let mut count = 0;

    for (_, body) in bodies {
        body();
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_test_configure!(run_in_browser);

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn every_token_2022_contract_executes() {
        assert_eq!(
            super::run(),
            super::ORIGINAL_BODIES + super::INDEPENDENT_BODIES
        );
    }
}
