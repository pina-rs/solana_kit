//! Executes the original Helius contract bodies across application hosts.
#![forbid(unsafe_code)]

#[path = "../../../crates/solana_kit_helius/tests/contracts/mod.rs"]
pub mod contracts;

/// Marker shared with the guest panic observer and production-host verifier.
pub const CORRUPTION_MARKER: &str = include_str!("../corruption.txt");

/// Exact full named stream required before a guest may commit success.
#[cfg(feature = "crypto")]
pub const REPORT: &str = include_str!("../report.txt");
/// Exact non-signing named stream when cryptography is excluded.
#[cfg(not(feature = "crypto"))]
pub const REPORT: &str = include_str!("../report-no-crypto.txt");

/// Number of independently listed contract groups in this feature configuration.
#[cfg(feature = "crypto")]
pub const COUNT: usize = 35;
/// Number of non-signing contract groups when cryptography is excluded.
#[cfg(not(feature = "crypto"))]
pub const COUNT: usize = 33;

/// Runs every maintained public assertion before reporting completion.
pub fn run() -> usize {
    corruption_control();
    assert_eq!(contracts::CASES.len(), COUNT);
    let mut expected = REPORT.lines();

    for (name, body) in contracts::CASES {
        let line = format!("PASS {name}");
        assert_eq!(expected.next(), Some(line.as_str()));
        body();
        println!("{line}");
    }

    assert_eq!(
        expected.next(),
        Some(format!("PASS all {COUNT} Helius contract groups").as_str())
    );
    assert!(expected.next().is_none());
    COUNT
}

#[cfg(feature = "corrupt")]
fn corruption_control() {
    panic!("{CORRUPTION_MARKER}");
}

#[cfg(not(feature = "corrupt"))]
fn corruption_control() {}

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
