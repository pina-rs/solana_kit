//! Inspects the compiled artifact rather than inferring host needs from target cfg.

use anyhow::{Context, Result, ensure};
use std::collections::BTreeSet;
use wasmparser::{FuncType, Import, Parser, Payload, TypeRef, ValType};

#[cfg(test)]
mod tests;

type Signature = (
    &'static str,
    &'static str,
    &'static [ValType],
    &'static [ValType],
);
use ValType::{I32, I64};

const SUBMIT_ARGUMENTS: &[ValType] = &[I32, I32, I32, I32];
const PROPS_ARGUMENTS: &[ValType] = &[I64, I32, I32];
const IMPORTS: &[Signature] = &[
    ("leptoskit", "node_create", &[I64, I32], &[]),
    ("leptoskit", "node_set_props", PROPS_ARGUMENTS, &[]),
    ("leptoskit", "commit", &[], &[]),
    ("leptoskit_plugins_v1", "version", &[], &[I32]),
    ("leptoskit_plugins_v1", "resolve", &[I32, I32], &[I64]),
    ("leptoskit_plugins_v1", "submit", SUBMIT_ARGUMENTS, &[I64]),
    ("leptoskit_plugins_v1", "result_len", &[I64], &[I64]),
    (
        "leptoskit_plugins_v1",
        "take_result",
        &[I64, I32, I32],
        &[I64],
    ),
    ("leptoskit_plugins_v1", "cancel", &[I64], &[I32]),
    ("leptoskit_plugins_v1", "event_info", &[I32], &[I32]),
    ("leptoskit_plugins_v1", "take_event", &[I32, I32], &[I64]),
];

/// Checks one imported function against the inspected upstream v1 signatures.
fn check<'a>(import: Import<'a>, types: &[FuncType]) -> Result<(&'a str, &'a str)> {
    let expected = IMPORTS
        .iter()
        .find(|entry| entry.0 == import.module && entry.1 == import.name)
        .context("forbidden compiled guest import")?;
    let TypeRef::Func(index) = import.ty else {
        anyhow::bail!("non-function import");
    };
    let ty = types
        .get(usize::try_from(index)?)
        .context("invalid import function type")?;
    ensure!(
        ty.params() == expected.2 && ty.results() == expected.3,
        "wrong bridge ABI signature"
    );
    Ok((import.module, import.name))
}

/// Requires the complete v1 bridge and renderer ABI, with no ambient host imports.
pub(super) fn verify(bytes: &[u8]) -> Result<()> {
    let mut types = Vec::new();
    let mut found = BTreeSet::new();

    for payload in Parser::new(0).parse_all(bytes) {
        let payload = payload?;

        if let Payload::TypeSection(section) = payload {
            types.extend(
                section
                    .into_iter_err_on_gc_types()
                    .collect::<Result<Vec<_>, _>>()?,
            );
            continue;
        }

        let Payload::ImportSection(section) = payload else {
            continue;
        };

        for import in section.into_imports() {
            ensure!(
                found.insert(check(import?, &types)?),
                "duplicate compiled import"
            );
        }
    }

    ensure!(
        found.len() == IMPORTS.len(),
        "missing expected guest imports"
    );
    println!("compiled guest exact v1 ABI imports: {found:?}");
    Ok(())
}
