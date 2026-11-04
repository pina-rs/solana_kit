//! Portable proof for the pinned error-code catalog.
//!
//! The shared body is the same one the native `shared_errors` integration test
//! executes. The first appended case confirms a wrong literal is rejected; the
//! second is the exact evidence referenced by the reviewed manifest rows.

#[path = "../../../crates/solana_kit_core/tests/shared_errors/exact_catalog.rs"]
mod exact_catalog;

use crate::CaseResult;

const FIXTURE: &str =
    include_str!("../../../crates/solana_kit_core/tests/shared_errors/catalog.tsv");

/// Appends the negative control before the complete catalog comparison.
pub(crate) fn cases() -> [CaseResult; 2] {
    [
        crate::case(
            "error-code-catalog-rejects-wrong-literal",
            exact_catalog::rejects_wrong_literal(FIXTURE).map_err(|mismatch| mismatch.field),
        ),
        crate::case(
            "error-code-catalog-exact-365",
            exact_catalog::exact(FIXTURE).map_err(|mismatch| mismatch.field),
        ),
    ]
}
