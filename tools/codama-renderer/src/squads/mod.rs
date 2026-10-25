//! Original source tables derived from the pinned MIT Dart client, using the shared renderer.
mod declarations;
mod errors;
mod instructions;
mod oracle;
mod schema;

#[cfg(all(test, feature = "verification"))]
mod tests;

use crate::{generic, schema::Error};
use serde_json::{Value, json};
use std::path::Path;

const PROGRAM: &str = "SQDS4ep65T869zMMBKyuUq6aD6EgTu8psMjkvj52pCf";

pub(crate) fn source() -> Result<Value, Error> {
    Ok(
        json!({"kind":"rootNode","standard":"codama","version":"1.0.0",
        "additionalPrograms":[],"program":{"kind":"programNode","name":"squadsMultisig",
            "publicKey":PROGRAM,"version":"2.1.0","pdas":[],"events":[],
            "accounts":declarations::accounts()?,"definedTypes":declarations::definitions()?,
            "instructions":instructions::instructions()?,"errors":errors::errors()}}),
    )
}

pub(crate) fn render(root: &Value) -> Result<String, Error> {
    generic::render_complete_program(
        root,
        &generic::Policy {
            name: "squadsMultisig",
            address: PROGRAM.parse()?,
        },
    )
}

/// All validation and formatting complete before the existing atomic replacement boundary.
pub(crate) fn generate(output: &Path) -> Result<(), Error> {
    crate::generate::replace_source(&render(&source()?)?, output)
}

pub(crate) fn audit_primary(path: &Path) -> Result<(), Error> {
    oracle::audit(path)
}
