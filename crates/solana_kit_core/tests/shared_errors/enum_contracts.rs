//! Executable proof for the pinned `SolanaErrorCode` enum boundaries.
//!
//! Every public associated constant is compared with the independent literal
//! fixture through `from_value`, `value` and `definition`. Unknown integers keep
//! their exact signed value even though the Dart enum cannot represent them.

use solana_kit_core::errors::{ErrorCode, ErrorDomain};
use std::{
    collections::{BTreeSet, HashSet},
    format,
    string::String,
    vec::Vec,
};

/// Number of `SolanaErrorCode` constants pinned by the source.
pub const CODES: usize = 365;

const ROW_FIELDS: usize = 4;

/// The first failed comparison, named for native and portable diagnostics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mismatch {
    /// Pinned constant name or `<enum>`.
    pub name: String,
    /// Stable explanation of the first failed comparison.
    pub field: &'static str,
}

impl Mismatch {
    fn new(name: impl Into<String>, field: &'static str) -> Self {
        Self {
            name: name.into(),
            field,
        }
    }
}

/// Integers the Dart enum cannot represent but Rust preserves exactly.
const UNREPRESENTABLE: [i64; 12] = [
    i64::MIN,
    -32769,
    -1,
    0,
    15,
    2800999,
    3611051,
    5665000,
    8079000,
    9901007,
    3230001,
    i64::MAX,
];

/// Requires every public constant to match the fixture through all accessors.
pub fn exact(fixture: &str) -> Result<(), Mismatch> {
    let values = check_rows(fixture)?;
    check_totals(fixture, &values)?;
    check_unknown_values()?;
    check_identity_traits()
}

/// Checks each fixture row and records its unique integer.
fn check_rows(fixture: &str) -> Result<HashSet<i64>, Mismatch> {
    let mut values = HashSet::new();

    for line in fixture.lines() {
        let value = check_row(line)?;

        if !values.insert(value) {
            let fields = row_fields(line)?;

            return Err(Mismatch::new(fields[0], "enum constant is duplicated"));
        }
    }

    Ok(values)
}

/// Checks one constant's value, name, reverse lookup and round trip.
fn check_row(line: &str) -> Result<i64, Mismatch> {
    let fields = row_fields(line)?;
    let value: i64 = fields[1]
        .parse()
        .map_err(|_| Mismatch::new(fields[0], "enum code is not a signed integer"))?;
    let code = ErrorCode::from_value(value);

    if code.value() != value {
        return Err(Mismatch::new(fields[0], "enum constant differs"));
    }

    let definition = code.definition().ok_or(Mismatch::new(
        fields[0],
        "enum constant is not a definition",
    ))?;

    if definition.name != fields[0] {
        return Err(Mismatch::new(fields[0], "enum constant name differs"));
    }

    if definition.code != code {
        return Err(Mismatch::new(fields[0], "enum constant differs"));
    }

    if code.definition() != Some(definition) {
        return Err(Mismatch::new(
            fields[0],
            "reverse lookup does not return the constant",
        ));
    }

    if ErrorCode::from_value(code.value()) != code {
        return Err(Mismatch::new(
            fields[0],
            "fromValue does not preserve the constant",
        ));
    }

    Ok(value)
}

/// Requires the exact fixture shape used by the catalog proof.
fn row_fields(row: &str) -> Result<[&str; ROW_FIELDS], Mismatch> {
    row.split('\t')
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| Mismatch::new("<enum>", "enum row does not have four fields"))
}

/// Enforces the three independent count and uniqueness guards.
fn check_totals(fixture: &str, values: &HashSet<i64>) -> Result<(), Mismatch> {
    if ErrorCode::definitions().count() != CODES {
        return Err(Mismatch::new("<enum>", "enum definition count is not 365"));
    }

    if fixture.lines().count() != CODES {
        return Err(Mismatch::new("<enum>", "enum fixture row count is not 365"));
    }

    if values.len() != CODES {
        return Err(Mismatch::new(
            "<enum>",
            "enum unique integer count is not 365",
        ));
    }

    Ok(())
}

/// Unknown integers keep their exact value and never acquire a definition.
fn check_unknown_values() -> Result<(), Mismatch> {
    let mut unknown = BTreeSet::new();

    for value in UNREPRESENTABLE {
        let code = ErrorCode::from_value(value);

        if code.value() != value {
            return Err(Mismatch::new("<enum>", "unknown integer is truncated"));
        }

        if code.definition().is_some() {
            return Err(Mismatch::new(
                "<enum>",
                "unknown integer acquired a pinned definition",
            ));
        }

        if !unknown.insert(value) {
            return Err(Mismatch::new("<enum>", "unknown probe is duplicated"));
        }
    }

    if unknown.len() != UNREPRESENTABLE.len() {
        return Err(Mismatch::new("<enum>", "unknown probe count differs"));
    }

    if ErrorCode::from_value(0) == ErrorCode::from_value(1) {
        return Err(Mismatch::new("<enum>", "distinct integers compare equal"));
    }

    Ok(())
}

/// Pins the copy/ordering/hash identity the Dart enum provides by construction.
fn check_identity_traits() -> Result<(), Mismatch> {
    const BLOCK_HEIGHT: ErrorCode = ErrorCode::BLOCK_HEIGHT_EXCEEDED;
    const INVALID_NONCE: ErrorCode = ErrorCode::INVALID_NONCE;
    const UNKNOWN: ErrorCode = ErrorCode::from_value(-1);

    if BLOCK_HEIGHT.value() != 1 || INVALID_NONCE.value() != 2 || UNKNOWN.value() != -1 {
        return Err(Mismatch::new(
            "<enum>",
            "const context does not preserve the pinned value",
        ));
    }

    if !(BLOCK_HEIGHT < INVALID_NONCE) || BLOCK_HEIGHT == INVALID_NONCE {
        return Err(Mismatch::new("<enum>", "enum ordering or equality differs"));
    }

    let mut seen = HashSet::new();

    if !seen.insert(BLOCK_HEIGHT) || seen.insert(BLOCK_HEIGHT) {
        return Err(Mismatch::new("<enum>", "enum hash identity differs"));
    }

    let copy = BLOCK_HEIGHT;

    if copy != BLOCK_HEIGHT {
        return Err(Mismatch::new("<enum>", "enum copy identity differs"));
    }

    if ErrorDomain::General != BLOCK_HEIGHT.domain() {
        return Err(Mismatch::new("<enum>", "enum domain helper differs"));
    }

    Ok(())
}

/// Proves a one-integer fixture corruption is rejected at the intended name.
pub fn rejects_wrong_constant(fixture: &str) -> Result<(), Mismatch> {
    exact(fixture)?;
    let rows: Vec<_> = fixture.lines().collect();
    let last = *rows
        .last()
        .ok_or(Mismatch::new("<enum>", "enum fixture is empty"))?;
    let fields = row_fields(last)?;
    let value: i64 = fields[1]
        .parse()
        .map_err(|_| Mismatch::new(fields[0], "enum code is not a signed integer"))?;
    let wrong = format!("{}\t{}\t{}\t{}", fields[0], value ^ 1, fields[2], fields[3]);
    let mut corrupted = rows[..rows.len() - 1].join("\n");
    corrupted.push('\n');
    corrupted.push_str(&wrong);

    match exact(&corrupted) {
        Ok(()) => Err(Mismatch::new(fields[0], "wrong enum constant was accepted")),
        Err(mismatch)
            if mismatch.name == fields[0]
                && mismatch.field == "enum constant is not a definition" =>
        {
            Ok(())
        }
        Err(_) => Err(Mismatch::new(
            fields[0],
            "wrong enum constant failed for the wrong reason",
        )),
    }
}
