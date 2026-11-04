//! Portable proof for the remaining pinned `solana_kit_errors` contracts.
//!
//! Every body is shared with the native integration tests. The two negative
//! controls precede their matching evidence case so a broken comparison cannot
//! suppress later results.

#[path = "../../../crates/solana_kit_client/tests/shared_errors/context_contracts.rs"]
mod context_contracts;
#[path = "../../../crates/solana_kit_core/tests/shared_errors/diagnostic_contracts.rs"]
mod diagnostic_contracts;
#[path = "../../../crates/solana_kit_core/tests/shared_errors/domain_contracts.rs"]
mod domain_contracts;
#[path = "../../../crates/solana_kit_core/tests/shared_errors/enum_contracts.rs"]
mod enum_contracts;
#[path = "../../../crates/solana_kit_client/tests/shared_errors/execution_contracts.rs"]
mod execution_contracts;
#[path = "../../../crates/solana_kit_core/tests/shared_errors/helper_contracts.rs"]
mod helper_contracts;
#[path = "../../../crates/solana_kit_core/tests/shared_errors/message_contracts.rs"]
mod message_contracts;
#[path = "../../../crates/solana_kit_client/tests/shared_errors/rpc_contracts.rs"]
mod rpc_contracts;
#[path = "../../../crates/solana_kit_client/tests/shared_errors/rpc_enum_contracts.rs"]
mod rpc_enum_contracts;
#[path = "../../../crates/solana_kit_client/tests/shared_errors/simulation_contracts.rs"]
mod simulation_contracts;

use crate::CaseResult;
use std::vec::Vec;

const CATALOG: &str =
    include_str!("../../../crates/solana_kit_core/tests/shared_errors/catalog.tsv");
const DOMAINS: &str =
    include_str!("../../../crates/solana_kit_core/tests/shared_errors/domains.tsv");
const KEYS: &str =
    include_str!("../../../crates/solana_kit_core/tests/shared_errors/error_keys.tsv");
const CONTEXTS: &str =
    include_str!("../../../crates/solana_kit_client/tests/shared_errors/context.txt");
const EXTRA_CONTEXTS: &str =
    include_str!("../../../crates/solana_kit_client/tests/shared_errors/context_extra.txt");
const EXECUTION: &str =
    include_str!("../../../crates/solana_kit_client/tests/shared_errors/execution.tsv");

const CASE_COUNT: usize = 15;

/// Appends the remaining reviewed error-contract groups after the 461-case prefix.
pub(crate) fn cases() -> Vec<CaseResult> {
    let mut cases = Vec::with_capacity(CASE_COUNT);
    cases.extend(control_cases());
    cases.extend(domain_cases());
    cases.extend(execution_cases());
    cases
}

/// Source-owned negative controls precede their matching evidence cases.
fn control_cases() -> [CaseResult; 3] {
    [
        crate::case(
            "error-domain-rejects-wrong-range",
            domain_contracts::rejects_wrong_range(DOMAINS, CATALOG)
                .map_err(|mismatch| mismatch.field),
        ),
        crate::case(
            "error-code-enum-rejects-wrong-constant",
            enum_contracts::rejects_wrong_constant(CATALOG).map_err(|mismatch| mismatch.field),
        ),
        crate::case(
            "error-execution-rejects-wrong-expected",
            execution_contracts::rejects_wrong_execution(EXECUTION)
                .map_err(|mismatch| mismatch.field),
        ),
    ]
}

/// Domain, enum, diagnostic and message evidence groups.
fn domain_cases() -> [CaseResult; 6] {
    [
        crate::case(
            "error-domain-classification-25",
            domain_contracts::exact(DOMAINS, CATALOG).map_err(|mismatch| mismatch.field),
        ),
        crate::case(
            "error-code-enum-boundaries",
            enum_contracts::exact(CATALOG).map_err(|mismatch| mismatch.field),
        ),
        crate::case(
            "error-diagnostic-source-contract",
            diagnostic_contracts::exact().map_err(|mismatch| mismatch.field),
        ),
        crate::case(
            "error-message-rejects-wrong-template",
            message_contracts::rejects_wrong_template(CATALOG).map_err(|mismatch| mismatch.field),
        ),
        crate::case(
            "error-message-template-interpolation",
            message_contracts::exact(CATALOG).map_err(|mismatch| mismatch.field),
        ),
        crate::case(
            "error-context-key-conventions",
            helper_contracts::exact(KEYS).map_err(|mismatch| mismatch.field),
        ),
    ]
}

/// Context wire, RPC enum, execution, JSON-RPC and simulation evidence groups.
fn execution_cases() -> [CaseResult; 6] {
    [
        crate::case(
            "error-context-wire-mounted-values",
            context_contracts::context_extra(EXTRA_CONTEXTS).map_err(|mismatch| mismatch.field),
        ),
        crate::case(
            "error-context-wire-literals",
            context_contracts::context_literals(CONTEXTS).map_err(|mismatch| mismatch.field),
        ),
        crate::case(
            "error-rpc-enum-offset-callback",
            rpc_enum_contracts::rpc_enum().map_err(|mismatch| mismatch.field),
        ),
        crate::case(
            "error-execution-source-oracle",
            execution_contracts::execution(EXECUTION).map_err(|mismatch| mismatch.field),
        ),
        crate::case(
            "error-json-rpc-projection",
            rpc_contracts::json_rpc().map_err(|mismatch| mismatch.field),
        ),
        crate::case(
            "error-simulation-unwrap",
            simulation_contracts::simulation().map_err(|mismatch| mismatch.field),
        ),
    ]
}
