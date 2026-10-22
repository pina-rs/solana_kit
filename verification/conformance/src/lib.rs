#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! The exact synchronous suite executed by every portability harness.
//!
//! This library needs `alloc`, but no host imports, randomness, clock, or executor.
//! A host must check [`Report::is_success`] and report every failing case. Use
//! [`FixtureMode::CorruptHash`] as a negative control for the host's failure path.
extern crate alloc;

mod accounts_sysvars;
mod addresses;
mod codecs;
mod cryptography;
mod fixed_points;
mod instructions;
mod transactions;

use alloc::{vec, vec::Vec};

/// Immutable suite identity. Changes to fixtures or assertions require a new ID.
pub const SUITE_ID: &str = "solana-kit-core-v5";
/// Required number of cases; a truncated report cannot pass conformance.
pub const EXPECTED_CASE_COUNT: usize = 42;

/// Selects authentic fixtures or the intentionally incorrect hash oracle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureMode {
    /// Uses fixed reference and standards vectors.
    Canonical,
    /// Flips one bit of the expected SHA-256 digest; this must fail.
    CorruptHash,
}

/// One named assertion group and its failure explanation, if any.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaseResult {
    /// Stable identifier for the fixture/assertion group.
    pub name: &'static str,
    /// Success or a concrete explanation of the mismatched behavior.
    pub outcome: Result<(), &'static str>,
}

/// Complete host-neutral conformance results.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Report {
    /// Suite and fixture version the host executed.
    pub suite_id: &'static str,
    /// All cases in stable order, including every failure.
    pub cases: Vec<CaseResult>,
}

impl Report {
    /// Counts executed cases whose assertions all succeeded.
    #[must_use]
    pub fn passed_count(&self) -> usize {
        self.cases
            .iter()
            .filter(|case| case.outcome.is_ok())
            .count()
    }

    /// Rejects failures, missing cases, or an unexpected suite identity.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.suite_id == SUITE_ID
            && self.cases.len() == EXPECTED_CASE_COUNT
            && self.passed_count() == EXPECTED_CASE_COUNT
    }
}

/// Records an already executed assertion group with its stable case name.
fn case(name: &'static str, outcome: Result<(), &'static str>) -> CaseResult {
    CaseResult { name, outcome }
}

/// Converts a fixture assertion into a reportable error without trapping a guest.
fn require(condition: bool, explanation: &'static str) -> Result<(), &'static str> {
    condition.then_some(()).ok_or(explanation)
}

/// Runs the same real core operations on native, browser, WASI, and plain Wasm.
///
/// Every case executes even after a failure. The corruption mode modifies only
/// the expected hash fixture; the implementation and comparison remain intact.
#[must_use]
pub fn run(mode: FixtureMode) -> Report {
    let mut cases = vec![
        case("address-base58-golden", addresses::golden()),
        case("address-invalid-alphabet", addresses::invalid_alphabet()),
        case("address-invalid-length", addresses::invalid_length()),
        case("address-bounded-input", addresses::bounded_input()),
        case("hash-base58-golden", addresses::hash_value()),
        case("signature-base58-golden", addresses::signature_value()),
        case("sha256-fips-abc", cryptography::sha256(mode)),
        case("ed25519-rfc8032-test1", cryptography::rfc8032_empty()),
        case("ed25519-rfc8032-test2", cryptography::rfc8032_one_byte()),
        case(
            "ed25519-altered-signature",
            cryptography::altered_signature(),
        ),
        case("ed25519-altered-message", cryptography::altered_message()),
        case("ed25519-weak-key", cryptography::weak_key()),
        case("pda-no-seeds", addresses::pda_no_seeds()),
        case("pda-bump-search", addresses::pda_bump_search()),
        case("pda-byte-seed", addresses::pda_byte_seed()),
        case("pda-utf8-seed", addresses::pda_utf8_seed()),
        case("pda-seed-count", addresses::seed_count()),
        case("pda-seed-length", addresses::seed_length()),
        case("pda-on-curve", addresses::on_curve()),
        case("seeded-address-golden", addresses::seeded_address()),
        case("seeded-address-illegal-owner", addresses::illegal_owner()),
        case("account-role-merge-matrix", instructions::role_matrix()),
        case(
            "instruction-ordered-accounts",
            instructions::ordered_accounts(),
        ),
        case("instruction-preserved-data", instructions::preserved_data()),
        case("fixed-value-byte-lengths", addresses::byte_lengths()),
        case("codec-integer-byte-fixtures", codecs::numbers()),
        case("codec-short-u16-canonical", codecs::short_u16()),
        case("codec-string-length-widths", codecs::strings()),
        case("codec-bounded-sequences", codecs::bounded_sequences()),
        case("codec-distinct-options", codecs::options()),
        case("codec-exact-framing", codecs::framing()),
    ];
    cases.extend(transactions::cases());
    cases.extend(accounts_sysvars::cases());

    cases.extend(fixed_points::cases());

    Report {
        suite_id: SUITE_ID,
        cases,
    }
}

#[cfg(test)]
mod tests {
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
}
