//! Executable proof that the Rust error catalog matches the pinned Dart literals.
//!
//! The native integration test and the portable conformance suite share this
//! body. Mismatches return typed facts instead of panicking, so a guest reports
//! the first wrong name, integer, domain, or template and keeps its later cases.

use solana_kit_core::errors::{CodeDefinition, Context, ErrorCode};
use std::{collections::HashSet, format, string::String, vec::Vec};

/// Number of `SolanaErrorCode` constants pinned by the source.
pub const CODES: usize = 365;

const ROW_FIELDS: usize = 4;
const HEX_RADIX: u32 = 16;

/// One wrong catalog fact, named for native diagnostics and portable reports.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Mismatch {
    /// Source declaration whose facts differ, or `<catalog>` for a bad row.
    pub name: &'static str,
    /// Stable explanation of the first failed comparison.
    pub field: &'static str,
}

impl Mismatch {
    const fn new(name: &'static str, field: &'static str) -> Self {
        Self { name, field }
    }
}

/// Compares every definition and every literal row in canonical order.
pub fn exact(fixture: &str) -> Result<(), Mismatch> {
    let mut values = HashSet::new();

    for (definition, row) in ErrorCode::definitions().zip(fixture.lines()) {
        check_row(definition, row, &mut values)?;
    }

    check_totals(fixture, &values)
}

/// Checks one definition against one literal row and records its integer.
fn check_row(
    definition: &CodeDefinition,
    row: &str,
    values: &mut HashSet<i64>,
) -> Result<(), Mismatch> {
    let fields = row_fields(row)?;
    let value = check_name_and_value(definition, fields[0], fields[1])?;

    if !values.insert(value) {
        return Err(Mismatch::new(
            definition.name,
            "catalog integer is duplicated",
        ));
    }

    check_domain(definition, fields[2])?;
    check_template(definition, fields[3])?;
    check_lookup_and_message(definition)
}

/// Splits a row without accepting missing or extra columns.
fn row_fields(row: &str) -> Result<[&str; ROW_FIELDS], Mismatch> {
    row.split('\t')
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| Mismatch::new("<catalog>", "catalog row does not have four fields"))
}

/// Requires the exact name and signed integer, returning the parsed value.
fn check_name_and_value(
    definition: &CodeDefinition,
    name: &str,
    value: &str,
) -> Result<i64, Mismatch> {
    if definition.name != name {
        return Err(Mismatch::new(definition.name, "catalog name differs"));
    }

    let value: i64 = value
        .parse()
        .map_err(|_| Mismatch::new(definition.name, "catalog code is not a signed integer"))?;

    if definition.code.value() != value {
        return Err(Mismatch::new(definition.name, "catalog integer differs"));
    }

    Ok(value)
}

/// Compares the pinned lower-camel domain name with the Rust classification.
fn check_domain(definition: &CodeDefinition, expected: &str) -> Result<(), Mismatch> {
    let domain = format!("{:?}", definition.code.domain());

    if expected_domain(&domain) != expected {
        return Err(Mismatch::new(
            definition.name,
            "catalog domain differs from the pinned rule",
        ));
    }

    Ok(())
}

/// Compares the exact decoded template bytes.
fn check_template(definition: &CodeDefinition, hex: &str) -> Result<(), Mismatch> {
    let template = decode_template(definition, hex)?;

    if definition.template.as_bytes() != template.as_slice() {
        return Err(Mismatch::new(definition.name, "catalog template differs"));
    }

    Ok(())
}

/// Decodes even-length ASCII hex into the original message bytes.
fn decode_template(definition: &CodeDefinition, hex: &str) -> Result<Vec<u8>, Mismatch> {
    if !hex.len().is_multiple_of(2) {
        return Err(Mismatch::new(
            definition.name,
            "catalog template is not even-length hex",
        ));
    }

    let mut template = Vec::new();

    for pair in hex.as_bytes().chunks(2) {
        let pair = std::str::from_utf8(pair)
            .map_err(|_| Mismatch::new(definition.name, "catalog template is not ASCII hex"))?;
        let byte = u8::from_str_radix(pair, HEX_RADIX)
            .map_err(|_| Mismatch::new(definition.name, "catalog template is not hex"))?;
        template.push(byte);
    }

    Ok(template)
}

/// Requires reverse lookup and a non-empty default-context rendering.
fn check_lookup_and_message(definition: &CodeDefinition) -> Result<(), Mismatch> {
    if definition.code.definition() != Some(definition) {
        return Err(Mismatch::new(
            definition.name,
            "catalog lookup does not return its definition",
        ));
    }

    let message = definition
        .code
        .message(&Context::default())
        .map_err(|_| Mismatch::new(definition.name, "catalog message does not render"))?;

    if message.is_empty() {
        return Err(Mismatch::new(definition.name, "catalog message is empty"));
    }

    Ok(())
}

/// Enforces the three independent 365-row count and uniqueness guards.
fn check_totals(fixture: &str, values: &HashSet<i64>) -> Result<(), Mismatch> {
    if ErrorCode::definitions().count() != CODES {
        return Err(Mismatch::new("<catalog>", "definition count is not 365"));
    }

    if fixture.lines().count() != CODES {
        return Err(Mismatch::new("<catalog>", "fixture row count is not 365"));
    }

    if values.len() != CODES {
        return Err(Mismatch::new(
            "<catalog>",
            "unique integer count is not 365",
        ));
    }

    Ok(())
}

/// Proves the same comparison rejects a one-integer corruption at the last row.
///
/// The unmodified fixture is checked first, so the only failing input differs by
/// one literal. The caller appends another case after this control.
pub fn rejects_wrong_literal(fixture: &str) -> Result<(), Mismatch> {
    exact(fixture)?;
    let rows: Vec<_> = fixture.lines().collect();
    let last = *rows
        .last()
        .ok_or(Mismatch::new("<catalog>", "catalog fixture is empty"))?;
    let fields = row_fields(last)?;
    let value: i64 = fields[1]
        .parse()
        .map_err(|_| Mismatch::new("<catalog>", "catalog code is not a signed integer"))?;
    let wrong = format!("{}\t{}\t{}\t{}", fields[0], value ^ 1, fields[2], fields[3]);
    let mut corrupted = rows[..rows.len() - 1].join("\n");
    corrupted.push('\n');
    corrupted.push_str(&wrong);

    match exact(&corrupted) {
        Ok(()) => Err(Mismatch::new(
            "<catalog>",
            "wrong catalog literal was accepted",
        )),
        Err(mismatch)
            if mismatch.name == fields[0] && mismatch.field == "catalog integer differs" =>
        {
            Ok(())
        }
        Err(_) => Err(Mismatch::new(
            "<catalog>",
            "wrong catalog literal failed for the wrong reason",
        )),
    }
}

/// Lowercases the first ASCII letter of a domain name for fixture comparison.
fn expected_domain(domain: &str) -> String {
    let mut characters = domain.chars();
    let Some(first) = characters.next() else {
        return String::new();
    };

    format!("{}{}", first.to_ascii_lowercase(), characters.as_str())
}
