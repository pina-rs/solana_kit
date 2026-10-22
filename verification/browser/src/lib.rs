//! Browser execution of the shared synchronous conformance suite.
#![forbid(unsafe_code)]

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use solana_kit_conformance::{EXPECTED_CASE_COUNT, FixtureMode, SUITE_ID, run};
    use wasm_bindgen_test::{console_log, wasm_bindgen_test, wasm_bindgen_test_configure};

    wasm_bindgen_test_configure!(run_in_browser);

    /// Requires actual suite success, including its identity and complete case list.
    #[wasm_bindgen_test]
    fn canonical_report() {
        let report = run(FixtureMode::Canonical);

        console_log!("{report:#?}");
        assert_eq!(report.suite_id, SUITE_ID);
        assert_eq!(report.cases.len(), EXPECTED_CASE_COUNT);
        assert!(report.is_success());
    }

    /// Makes unexpected corruption success fail the browser harness itself.
    #[wasm_bindgen_test]
    fn corruption_is_observable() {
        let report = run(FixtureMode::CorruptHash);
        let failures: Vec<_> = report
            .cases
            .iter()
            .filter(|case| case.outcome.is_err())
            .collect();

        console_log!("{report:#?}");
        assert_eq!(report.suite_id, SUITE_ID);
        assert_eq!(report.cases.len(), EXPECTED_CASE_COUNT);
        assert!(!report.is_success());
        assert_eq!(report.passed_count(), EXPECTED_CASE_COUNT - 1);
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].name, "sha256-fips-abc");
    }
}
