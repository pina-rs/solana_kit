//! Checked-in portable reports must match the registered suite metadata exactly.

use solana_kit_conformance::{EXPECTED_CASE_COUNT, SUITE_ID};

const SUCCESS: &str = include_str!("../../verification/conformance/evidence/native-success.txt");
const CORRUPTION: &str =
    include_str!("../../verification/conformance/evidence/native-corruption.txt");
const CAPTURED_V55: &str =
    include_str!("../../verification/conformance/evidence/parity-completion-prefix-v55.txt");

fn case_lines(lines: &[&str]) -> usize {
    lines
        .iter()
        .filter(|line| line.contains(": Ok(())") || line.contains(": Err("))
        .count()
}

#[test]
fn checked_in_reports_match_exact_suite_metadata() {
    let success: Vec<_> = SUCCESS.lines().collect();
    let corruption: Vec<_> = CORRUPTION.lines().collect();
    let captured: Vec<_> = CAPTURED_V55.lines().collect();
    let canonical = format!("{SUITE_ID}: {EXPECTED_CASE_COUNT}/{EXPECTED_CASE_COUNT} cases passed");
    let corrupted = format!(
        "{SUITE_ID}: {}/{EXPECTED_CASE_COUNT} cases passed",
        EXPECTED_CASE_COUNT - 1
    );

    assert_eq!(success.last().copied(), Some(canonical.as_str()));
    assert_eq!(corruption.last().copied(), Some(corrupted.as_str()));
    assert_eq!(case_lines(&success), EXPECTED_CASE_COUNT);
    assert_eq!(case_lines(&corruption), EXPECTED_CASE_COUNT);
    assert_eq!(
        corruption
            .iter()
            .filter(|line| line.contains(": Err("))
            .count(),
        1
    );
    assert_eq!(
        corruption
            .iter()
            .filter(|line| line.contains(": Err("))
            .filter(|line| line.starts_with("sha256-fips-abc: "))
            .count(),
        1
    );
    assert_eq!(
        &success[..captured.len() - 1],
        &captured[..captured.len() - 1]
    );
    assert_eq!(
        captured.last().copied(),
        Some("solana-kit-core-v55: 448/448 cases passed")
    );
}
