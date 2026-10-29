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
pub const COUNT: usize = 53;
/// Number of non-signing contract groups when cryptography is excluded.
#[cfg(all(not(feature = "crypto"), feature = "signup"))]
pub const COUNT: usize = 50;

/// Checkout-only count without cryptography or signup.
#[cfg(all(not(feature = "crypto"), feature = "checkout", not(feature = "signup")))]
pub const COUNT: usize = 41;
/// Checkout and cryptography count without signup.
#[cfg(all(feature = "crypto", feature = "checkout", not(feature = "signup")))]
pub const COUNT: usize = 43;

/// Original non-signing contract count when checkout orchestration is absent.
#[cfg(all(
    not(feature = "crypto"),
    not(feature = "signup"),
    not(feature = "checkout")
))]
pub const COUNT: usize = 36;
/// Original signing contract count when signup orchestration is absent.
#[cfg(all(feature = "crypto", not(feature = "signup"), not(feature = "checkout")))]
pub const COUNT: usize = 38;

/// Runs every maintained public assertion before reporting completion.
pub fn run() -> usize {
    let completed =
        execute_cases(contracts::SMART_CASE.1).expect("executed Helius contract group failed");
    verify_report().expect("Helius golden stream does not match executed registry");
    completed
}

/// Native-only genuine outcome capture; behavioral failures remain errors and never PASS rows.
///
/// This deliberately does not compare historical goldens. Normal [`run`] still does.
#[cfg(not(target_arch = "wasm32"))]
pub fn capture() -> Result<usize, &'static str> {
    execute_cases(contracts::SMART_CASE.1)
}

fn execute_cases(
    smart: fn() -> contracts::smart_preparation::Outcome,
) -> Result<usize, &'static str> {
    corruption_control();

    if contracts::CASES.len() + 1 != COUNT {
        return Err("Helius registry count mismatch");
    }

    for (name, body) in contracts::CASES {
        body();
        println!("PASS {name}");
    }

    let name = contracts::SMART_CASE.0;

    if let Err(reason) = smart() {
        println!("FAIL {name}: {reason}");
        return Err(reason);
    }

    println!("PASS {name}");
    Ok(COUNT)
}

fn verify_report() -> Result<(), &'static str> {
    let mut expected = REPORT.lines();
    let names = contracts::CASES
        .iter()
        .map(|(name, _)| *name)
        .chain(core::iter::once(contracts::SMART_CASE.0));

    for name in names {
        let line = format!("PASS {name}");

        if expected.next() != Some(line.as_str()) {
            return Err("Helius golden case order mismatch");
        }
    }

    let summary = format!("PASS all {COUNT} Helius contract groups");

    if expected.next() != Some(summary.as_str()) || expected.next().is_some() {
        return Err("Helius golden summary mismatch");
    }

    Ok(())
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

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn capture_rejects_a_real_smart_assertion_failure() {
        assert_eq!(
            super::execute_cases(super::contracts::smart_preparation::wrong_order_case),
            Err("smart RPC observation order changed")
        );
    }
}
