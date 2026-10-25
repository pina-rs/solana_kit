//! MIT Dart source contracts lowered into the shared typed program renderer.
mod accounts;
mod errors;
mod instructions;
mod oracle;
mod schema;
mod types;
mod variants;

#[cfg(all(test, feature = "verification"))]
mod tests;

use crate::{generic, schema::Error};
use serde_json::{Value, json};
use std::path::Path;

const PROGRAM: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

pub(crate) fn source() -> Result<Value, Error> {
    let mut definitions = types::definitions()?;
    definitions.extend(variants::definitions()?);

    Ok(
        json!({"kind":"rootNode","standard":"codama","version":"1.0.0",
        "additionalPrograms":[],"program":{"kind":"programNode","name":"mplTokenMetadata",
            "publicKey":PROGRAM,"version":"1.14.0","pdas":[],"events":[],
            "accounts":accounts::accounts()?,"definedTypes":definitions,
            "instructions":instructions::instructions()?,"errors":errors::errors()?}}),
    )
}

pub(crate) fn render(root: &Value) -> Result<String, Error> {
    generic::render_complete_program(
        root,
        &generic::Policy {
            name: "mplTokenMetadata",
            address: PROGRAM.parse()?,
        },
    )
}

/// Uses the existing formatter-in-memory and atomic replacement boundary.
pub(crate) fn generate(output: &Path) -> Result<(), Error> {
    crate::generate::replace_source(&render(&source()?)?, output)
}

pub(crate) fn audit_primary(path: &Path) -> Result<(), Error> {
    oracle::audit(path)
}
