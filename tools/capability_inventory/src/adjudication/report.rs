//! Only maintained execution artifacts can support an executable claim.

use crate::model::Result;
use std::{collections::BTreeSet, fs, path::Path};

const REPORT: &str = "verification/conformance/evidence/native-success.txt";

/// This report is tied to the live registry by the conformance integrity test.
/// Additional report formats need an explicit validator before they can qualify.
pub(super) fn contains(repository: &Path, path: &str, case: &str) -> bool {
    if path != REPORT {
        return false;
    }

    let Ok(text) = fs::read_to_string(repository.join(path)) else {
        return false;
    };

    successful_cases(&text).is_ok_and(|cases| cases.contains(case))
}

/// Validates the complete case block and footer, leaving diagnostics as a prefix.
fn successful_cases(text: &str) -> Result<BTreeSet<&str>> {
    let lines: Vec<_> = text.lines().collect();
    let (footer, body) = lines.split_last().ok_or("empty execution report")?;
    let count = case_count(footer)?;

    if count == 0 || count > body.len() {
        return Err("execution report case count is inconsistent".into());
    }

    let mut cases = BTreeSet::new();

    for line in &body[body.len() - count..] {
        let name = line
            .strip_suffix(": Ok(())")
            .ok_or("unsuccessful report case")?;

        if name.is_empty() || !cases.insert(name) {
            return Err("empty or duplicate execution report case".into());
        }
    }

    Ok(cases)
}

/// A full-success footer is required; a success line alone proves no run.
fn case_count(footer: &str) -> Result<usize> {
    let (suite, totals) = footer.split_once(": ").ok_or("missing suite footer")?;
    let version = suite
        .strip_prefix("solana-kit-core-v")
        .ok_or("unknown report suite")?;

    if version.is_empty() || !version.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("invalid report suite version".into());
    }

    let totals = totals
        .strip_suffix(" cases passed")
        .ok_or("missing report totals")?;
    let (passed, expected) = totals.split_once('/').ok_or("invalid report totals")?;
    let passed: usize = passed.parse()?;
    let expected: usize = expected.parse()?;

    if passed != expected {
        return Err("execution report contains failures".into());
    }

    Ok(expected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_a_complete_unique_successful_report() {
        let valid =
            "diagnostic\nalpha: Ok(())\nbeta: Ok(())\nsolana-kit-core-v56: 2/2 cases passed\n";
        let cases = successful_cases(valid).expect("complete report");

        assert_eq!(cases, BTreeSet::from(["alpha", "beta"]));

        for invalid in [
            "Prose quoting a case\nalpha: Ok(())\n",
            "alpha: Ok(())\nsolana-kit-core-v56: 1/2 cases passed\n",
            "alpha: Ok(())\nsolana-kit-core-v56: 2/2 cases passed\n",
            "alpha: Ok(())\nalpha: Ok(())\nsolana-kit-core-v56: 2/2 cases passed\n",
            "alpha: Err(\"failure\")\nsolana-kit-core-v56: 1/1 cases passed\n",
        ] {
            assert!(successful_cases(invalid).is_err(), "{invalid}");
        }
    }

    #[test]
    fn an_unregistered_file_cannot_supply_executable_evidence() {
        assert!(!contains(Path::new("."), "docs/claim.txt", "alpha"));
    }
}
