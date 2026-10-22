#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! The exact synchronous suite executed by every portability harness.
//!
//! This library needs `alloc`, but no host imports, randomness, clock, or executor.
//! A host must check [`Report::is_success`] and report every failing case. Use
//! [`FixtureMode::CorruptHash`] as a negative control for the host's failure path.
extern crate alloc;

pub mod compute_budget;
/// Accepted portable codec framing assertions shared by every host.
pub mod framing;
pub mod memo_associated_token;

mod accounts_sysvars;
mod addresses;
mod codecs;
mod composites;
mod cryptography;
mod fixed_points;

mod framing_cases;
mod instructions;
mod nonce_cases;
mod offchain;
mod registry;
/// Portable public RPC protocol fixtures without network I/O.
pub mod rpc;
mod rpc_accounts;
mod rpc_address_simulation;
mod rpc_economics;
mod rpc_fixture;
mod rpc_history;
mod rpc_node;
mod rpc_results;
mod rpc_submission;
/// Public signer fixtures for portable orchestration diagnostics.
pub mod signers;
mod subscriptions;
mod text_numbers;
mod transactions;
/// Version-one client wire fixtures shared by every host.
pub mod v1;
mod wallet_cases;

/// Generated System instruction-data conformance fixtures.
pub mod system;
pub mod system_client;

use alloc::vec::Vec;

/// Immutable suite identity. Changes to fixtures or assertions require a new ID.
pub const SUITE_ID: &str = "solana-kit-core-v23";
/// Required number of cases; a truncated report cannot pass conformance.
pub const EXPECTED_CASE_COUNT: usize = 101;

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
    let mut cases = foundational_cases(mode);

    cases.extend(rpc_economics::cases());
    cases.extend(rpc_accounts::cases());
    cases.push(case("rpc-history-exact-queries", rpc_history::queries()));
    cases.push(case(
        "rpc-submission-status-errors",
        rpc_submission::operations(),
    ));
    cases.push(case(
        "rpc-full-transaction-results",
        rpc_results::full_results(),
    ));
    cases.push(case(
        "rpc-address-simulation",
        rpc_address_simulation::operations(),
    ));
    cases.extend(registry::offchain());
    cases.extend(compute_budget::cases());
    cases.extend(subscriptions::cases());
    cases.extend(memo_associated_token::cases());
    cases.extend(framing_cases::cases());
    cases.extend(wallet_cases::cases());
    cases.extend(signers::cases());
    cases.extend(nonce_cases::cases());

    Report {
        suite_id: SUITE_ID,
        cases,
    }
}

fn foundational_cases(mode: FixtureMode) -> Vec<CaseResult> {
    let mut cases = Vec::with_capacity(EXPECTED_CASE_COUNT);
    cases.extend(registry::values_and_crypto(mode));
    cases.extend(registry::derivation());
    cases.extend(registry::instructions());
    cases.extend(registry::codecs());
    cases.extend(transactions::cases());
    cases.extend(accounts_sysvars::cases());

    cases.extend(fixed_points::cases());
    cases.extend(system::cases());
    cases.extend(v1::cases());
    cases.extend(registry::clients());
    cases.push(case("numeric-text-codecs", text_numbers::text()));
    cases.push(case("numeric-wide-text", text_numbers::wide()));
    cases.push(case("numeric-text-malformed", text_numbers::malformed()));
    cases.push(case(
        "composite-typed-composition",
        composites::composition(),
    ));
    cases.push(case("composite-backward-bits", composites::bits()));
    cases.push(case("composite-malformed-bounds", composites::malformed()));
    cases
}

#[cfg(test)]
mod tests;
