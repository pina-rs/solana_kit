//! MIT Dart contracts lowered into the shared validated program renderer.
mod declarations;
mod instructions;
mod oracle;
mod schema;
#[cfg(all(test, feature = "verification"))]
mod tests;

use crate::{generic, schema::Error};
use serde_json::{Value, json};
use std::path::Path;

const PROGRAM: &str = "22zoJMtdu4tQc2PzL74ZUT7FrwgB1Udec8DdW4yw4BdG";

pub(crate) fn source() -> Result<Value, Error> {
    Ok(
        json!({"kind":"rootNode","standard":"codama","version":"1.0.0",
        "additionalPrograms":[],"program":{"kind":"programNode",
            "name":"attestationService","publicKey":PROGRAM,"version":"0.1.0",
            "pdas":[],"events":[],"accounts":declarations::accounts()?,
            "definedTypes":declarations::definitions()?,
            "instructions":instructions::instructions()?,"errors":declarations::errors()}}),
    )
}

pub(crate) fn render(root: &Value) -> Result<String, Error> {
    generic::render_complete_program(
        root,
        &generic::Policy {
            name: "attestationService",
            address: PROGRAM.parse()?,
        },
    )
}

/// Validation and the existing formatter finish before atomic replacement.
pub(crate) fn generate(output: &Path) -> Result<(), Error> {
    crate::generate::replace_source(&render(&source()?)?, output)
}

pub(crate) fn audit_primary(path: &Path) -> Result<(), Error> {
    oracle::audit(path)
}
