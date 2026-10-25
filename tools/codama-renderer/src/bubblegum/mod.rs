//! MIT Dart-derived declarations, corrected against private independent wire facts.
mod declarations;
mod errors;
mod instructions;
pub(crate) mod oracle;
mod schema;

#[cfg(all(test, feature = "verification"))]
mod tests;

use crate::{generic, schema::Error};
use serde_json::{Value, json};
use std::path::Path;

const PROGRAM: &str = "BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY";

/// Keeps every field and privilege explicit before the shared validation boundary.
pub(crate) fn source() -> Result<Value, Error> {
    Ok(
        json!({"kind":"rootNode","standard":"codama","version":"1.0.0",
        "additionalPrograms":[],"program":{"kind":"programNode","name":"mplBubblegum",
            "publicKey":PROGRAM,"version":"0.12.0","pdas":[],"events":[],
            "accounts":declarations::accounts()?,"definedTypes":declarations::definitions()?,
            "instructions":instructions::instructions()?,"errors":errors::errors()}}),
    )
}

pub(crate) fn render(root: &Value) -> Result<String, Error> {
    generic::render_complete_program(
        root,
        &generic::Policy {
            name: "mplBubblegum",
            address: PROGRAM.parse()?,
        },
    )
}

/// The existing writer formats and validates the whole output before replacement.
pub(crate) fn generate(output: &Path) -> Result<(), Error> {
    crate::generate::replace_source(&render(&source()?)?, output)
}
