//! Portable proof for the pinned simulation cause unwrapping.

use solana_kit_core::errors::{Context, ContextError, Diagnostic, ErrorCode};

/// The three simulation wrapper codes, including the JSON-RPC preflight code.
const SIMULATION_CODES: [i64; 3] = [-32_002, 5_663_019, 5_663_037];

/// The first failed comparison.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Mismatch {
    /// Contract section that failed.
    pub name: &'static str,
    /// Stable explanation.
    pub field: &'static str,
}

impl Mismatch {
    const fn new(name: &'static str, field: &'static str) -> Self {
        Self { name, field }
    }
}

/// Simulation unwrapping exposes exactly one typed cause for pinned codes.
pub fn simulation() -> Result<(), Mismatch> {
    for value in SIMULATION_CODES {
        check_wrapped(value)?;
        check_absent(value)?;
    }

    check_non_simulation()
}

/// A wrapped simulation error returns its typed cause.
fn check_wrapped(value: i64) -> Result<(), Mismatch> {
    let cause = Diagnostic::new(ErrorCode::ACCOUNTS_ACCOUNT_NOT_FOUND, Context::default());
    let outer =
        Diagnostic::new(ErrorCode::from_value(value), Context::default()).with_source(cause);
    let unwrapped = outer.simulation_cause().downcast_ref::<Diagnostic>();

    if unwrapped.map(Diagnostic::code) != Some(ErrorCode::ACCOUNTS_ACCOUNT_NOT_FOUND) {
        return Err(Mismatch::new(
            "unwrapSimulationError",
            "typed simulation cause differs",
        ));
    }

    Ok(())
}

/// A simulation code without a cause returns the same diagnostic.
fn check_absent(value: i64) -> Result<(), Mismatch> {
    let none = Diagnostic::new(ErrorCode::from_value(value), Context::default());

    if !same(none.simulation_cause(), &none) {
        return Err(Mismatch::new(
            "unwrapSimulationError",
            "absent cause was returned as a wrapper",
        ));
    }

    Ok(())
}

/// A non-simulation diagnostic with a source is never unwrapped.
fn check_non_simulation() -> Result<(), Mismatch> {
    let nested = Diagnostic::new(ErrorCode::BLOCK_HEIGHT_EXCEEDED, Context::default())
        .with_source(ContextError::Encoding);

    if !same(nested.simulation_cause(), &nested) {
        return Err(Mismatch::new(
            "unwrapSimulationError",
            "non-simulation error was unwrapped",
        ));
    }

    Ok(())
}

/// Compares two trait-object references by address.
fn same(
    left: &(dyn std::error::Error + 'static),
    right: &(dyn std::error::Error + 'static),
) -> bool {
    std::ptr::addr_eq(left, right)
}
