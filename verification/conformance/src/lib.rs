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
mod registry;
mod transactions;
/// Candidate version-one cases, separate from the established host registry.
pub mod v1;

/// Generated System instruction-data conformance fixtures.
pub mod system;

use alloc::vec::Vec;

/// Immutable suite identity. Changes to fixtures or assertions require a new ID.
pub const SUITE_ID: &str = "solana-kit-core-v6";
/// Required number of cases; a truncated report cannot pass conformance.
pub const EXPECTED_CASE_COUNT: usize = 45;

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
    let mut cases = Vec::with_capacity(EXPECTED_CASE_COUNT);
    cases.extend(registry::values_and_crypto(mode));
    cases.extend(registry::derivation());
    cases.extend(registry::instructions());
    cases.extend(registry::codecs());
    cases.extend(transactions::cases());
    cases.extend(accounts_sysvars::cases());

    cases.extend(fixed_points::cases());
    cases.extend(system::cases());

    Report {
        suite_id: SUITE_ID,
        cases,
    }
}

#[cfg(test)]
mod tests;
