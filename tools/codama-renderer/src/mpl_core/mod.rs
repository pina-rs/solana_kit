//! MIT Dart Core contracts lowered into the shared program renderer.
mod declarations;
mod errors;
mod instructions;
mod oracle;
mod schema;
#[cfg(all(test, feature = "verification"))]
mod tests;
mod variants;

use crate::{generic, schema::Error};
use serde_json::{Value, json};
use std::path::Path;

const PROGRAM: &str = "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d";

pub(crate) fn source() -> Result<Value, Error> {
    Ok(
        json!({"kind":"rootNode","standard":"codama","version":"1.0.0",
        "additionalPrograms":[],"program":{"kind":"programNode","name":"mplCore",
        "publicKey":PROGRAM,"version":"0.2.0","pdas":[],"events":[],
        "accounts":declarations::accounts()?,"definedTypes":declarations::definitions()?,
        "instructions":instructions::instructions()?,"errors":errors::errors()}}),
    )
}

pub(crate) fn render(root: &Value) -> Result<String, Error> {
    generic::render_complete_program(
        root,
        &generic::Policy {
            name: "mplCore",
            address: PROGRAM.parse()?,
        },
    )
}

/// The shared atomic writer validates and formats before replacing the destination.
pub(crate) fn generate(output: &Path) -> Result<(), Error> {
    crate::generate::replace_source(&render(&source()?)?, output)
}

pub(crate) fn audit_primary(path: &Path) -> Result<(), Error> {
    oracle::audit(path)
}
