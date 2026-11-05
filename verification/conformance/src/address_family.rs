//! Appended source-grounded evidence for the pinned address constants and curve contract.

#[path = "../../../crates/solana_kit_core/tests/core_parity/curve.rs"]
mod curve;

use crate::CaseResult;
use alloc::string::ToString;
use solana_kit_core::{Address, ParseError, address_constants};

/// Two controls, sysvar and curve evidence, and a source-defect check.
pub(crate) fn cases() -> [CaseResult; 5] {
    [
        crate::case(
            "address-constant-rejects-wrong-literal",
            crate::core_parity::constants::rejects_mutated_literal(),
        ),
        crate::case(
            "curve-classification-rejects-wrong-expected",
            curve::rejects_wrong_expected(),
        ),
        crate::case(
            "curve-compressed-point-classification",
            curve::classification(),
        ),
        crate::case(
            "native-loader-source-literal-invalid",
            native_loader_source_literal(),
        ),
        crate::case(
            "address-sysvar-constant-literals",
            crate::core_parity::constants::sysvar_literals(),
        ),
    ]
}

/// The pinned Dart literal is 40 characters and decodes to 30 bytes, not 32.
fn native_loader_source_literal() -> Result<(), &'static str> {
    let source = "NativeLoader1111111111111111111111111111";
    let canonical = "NativeLoader1111111111111111111111111111111";

    require_wrong_length(source)?;

    if address_constants::NATIVE_LOADER_PROGRAM.to_string() != canonical {
        return Err("canonical native loader display changed");
    }

    Ok(())
}

/// Accepts only the exact rejected decoded length from the public parser.
fn require_wrong_length(source: &str) -> Result<(), &'static str> {
    match source.parse::<Address>() {
        Err(ParseError::WrongLength {
            kind: solana_kit_core::ValueKind::Address,
            expected: 32,
            actual: 30,
        }) => Ok(()),
        Err(_) => Err("pinned native loader literal failed for the wrong reason"),
        Ok(_) => Err("pinned native loader literal unexpectedly parsed"),
    }
}
