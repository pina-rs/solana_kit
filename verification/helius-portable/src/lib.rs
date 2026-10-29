//! Executes the original Helius contract bodies across application hosts.
#![forbid(unsafe_code)]

#[path = "../../../crates/solana_kit_helius/tests/contracts/mod.rs"]
pub mod contracts;

/// Marker shared with the guest panic observer and production-host verifier.
pub const CORRUPTION_MARKER: &str = include_str!("../corruption.txt");

/// Exact full named stream required before a guest may commit success.
#[cfg(all(feature = "crypto", feature = "signup"))]
pub const REPORT: &str = include_str!("../report.txt");
/// Exact non-signing named stream when cryptography is excluded.
#[cfg(all(not(feature = "crypto"), feature = "signup"))]
pub const REPORT: &str = include_str!("../report-no-crypto.txt");

/// Checkout-only stream without the signup date parser.
#[cfg(all(not(feature = "crypto"), feature = "checkout", not(feature = "signup")))]
pub const REPORT: &str = include_str!("../report-checkout-no-crypto.txt");
/// Checkout and signing stream without unified signup.
#[cfg(all(feature = "crypto", feature = "checkout", not(feature = "signup")))]
pub const REPORT: &str = include_str!("../report-checkout.txt");

/// Exact original contract stream without signing or checkout orchestration.
#[cfg(all(
    not(feature = "crypto"),
    not(feature = "signup"),
    not(feature = "checkout")
))]
pub const REPORT: &str = include_str!("../report-base.txt");
/// Exact original signing contract stream without signup orchestration.
#[cfg(all(feature = "crypto", not(feature = "signup"), not(feature = "checkout")))]
pub const REPORT: &str = include_str!("../report-base-crypto.txt");

/// Number of independently listed contract groups in this feature configuration.
#[cfg(all(feature = "crypto", feature = "signup"))]
pub const COUNT: usize = 46;
/// Number of non-signing contract groups when cryptography is excluded.
#[cfg(all(not(feature = "crypto"), feature = "signup"))]
pub const COUNT: usize = 43;

/// Checkout-only count without cryptography or signup.
#[cfg(all(not(feature = "crypto"), feature = "checkout", not(feature = "signup")))]
pub const COUNT: usize = 39;
/// Checkout and cryptography count without signup.
#[cfg(all(feature = "crypto", feature = "checkout", not(feature = "signup")))]
pub const COUNT: usize = 41;

/// Original non-signing contract count when checkout orchestration is absent.
#[cfg(all(
    not(feature = "crypto"),
    not(feature = "signup"),
    not(feature = "checkout")
))]
pub const COUNT: usize = 34;
/// Original signing contract count when signup orchestration is absent.
#[cfg(all(feature = "crypto", not(feature = "signup"), not(feature = "checkout")))]
pub const COUNT: usize = 36;

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
