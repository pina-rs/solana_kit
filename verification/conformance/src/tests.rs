//! Full-report integrity and corruption controls.

use super::{EXPECTED_CASE_COUNT, FixtureMode, run};

#[test]
fn canonical_suite_passes_every_case() {
    let report = run(FixtureMode::Canonical);

    assert!(report.is_success(), "{report:#?}");
    assert_eq!(report.cases.len(), EXPECTED_CASE_COUNT);
}

#[test]
fn corrupted_fixture_fails_the_actual_comparison() {
    let report = run(FixtureMode::CorruptHash);
    let failures: alloc::vec::Vec<_> = report
        .cases
        .iter()
        .filter(|case| case.outcome.is_err())
        .collect();

    assert!(!report.is_success());
    assert_eq!(failures.len(), 1);
    assert_eq!(failures[0].name, "sha256-fips-abc");
    assert_eq!(report.passed_count(), EXPECTED_CASE_COUNT - 1);
}

#[test]
fn missing_cases_and_wrong_suite_identity_fail() {
    let mut report = run(FixtureMode::Canonical);
    report.cases.pop();
    assert!(!report.is_success());

    let mut report = run(FixtureMode::Canonical);
    report.suite_id = "wrong-suite";
    assert!(!report.is_success());
}
