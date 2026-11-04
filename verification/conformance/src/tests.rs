//! Full-report integrity and corruption controls.

use super::{EXPECTED_CASE_COUNT, FixtureMode, SUITE_ID, run};

const CAPTURED_V55: &str = include_str!("../evidence/parity-completion-prefix-v55.txt");
const CURRENT_SUCCESS: &str = include_str!("../evidence/native-success.txt");
const APPENDED_NAMES: [&str; 13] = [
    "grind-entropy-cancellation",
    "message-decompile-roundtrip",
    "signer-explicit-config",
    "token-mint-ata-plans",
    "client-parsed-account-families",
    "fixed-point-codec-pairing",
    "test-support-scripted-client",
    "publisher-publication-bundle-source-projection",
    "publisher-publication-diagnostic-redaction",
    "publisher-publication-ingestion-projection",
    "publisher-publication-session-checkpoint-precedence",
    "error-code-catalog-rejects-wrong-literal",
    "error-code-catalog-exact-365",
];

#[test]
fn canonical_report_preserves_the_captured_v55_prefix() {
    let captured: alloc::vec::Vec<_> = CAPTURED_V55.lines().collect();
    let current: alloc::vec::Vec<_> = CURRENT_SUCCESS.lines().collect();

    assert_eq!(
        captured.last().copied(),
        Some("solana-kit-core-v55: 448/448 cases passed")
    );
    assert_eq!(current.len(), captured.len() + APPENDED_NAMES.len());
    assert_eq!(
        &current[..captured.len() - 1],
        &captured[..captured.len() - 1],
        "the complete pre-suite report prefix changed"
    );

    for (line, name) in current[captured.len() - 1..]
        .iter()
        .zip(APPENDED_NAMES.into_iter().chain([""]))
    {
        if name.is_empty() {
            break;
        }

        assert_eq!(line.split_once(": ").map(|(case, _)| case), Some(name));
    }

    let expected =
        alloc::format!("{SUITE_ID}: {EXPECTED_CASE_COUNT}/{EXPECTED_CASE_COUNT} cases passed");
    assert_eq!(current.last().copied(), Some(expected.as_str()));
}

#[test]
fn canonical_suite_passes_every_case() {
    let report = run(FixtureMode::Canonical);

    assert!(report.is_success(), "{report:#?}");
    assert_eq!(report.cases.len(), EXPECTED_CASE_COUNT);
    let captured_names: alloc::vec::Vec<_> = CURRENT_SUCCESS
        .lines()
        .filter_map(|line| line.strip_suffix(": Ok(())"))
        .collect();
    let live_names: alloc::vec::Vec<_> = report.cases.iter().map(|case| case.name).collect();

    assert_eq!(
        live_names, captured_names,
        "live suite changed the captured case order"
    );
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
