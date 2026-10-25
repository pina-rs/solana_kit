//! Resolves known errors before generating their lossless numeric representation.

use crate::{
    generate,
    schema::{self, Error},
};
use serde_json::Value;
use std::{collections::BTreeSet, path::Path};

fn validate_node(node: &Value) -> Result<(), Error> {
    let object = node.as_object().ok_or("invalid error object")?;

    if object
        .keys()
        .any(|key| !["kind", "name", "idlName", "code", "message", "docs"].contains(&key.as_str()))
        || node["kind"] != "errorNode"
    {
        return Err("unsupported error property or kind".into());
    }

    Ok(())
}

/// Schema validation ensures names and numeric contracts are safe before source rendering.
pub(crate) fn render(root: &Value) -> Result<String, Error> {
    let nodes = root["program"]["errors"]
        .as_array()
        .ok_or("missing errors")?;
    let mut codes = BTreeSet::new();
    let mut names = BTreeSet::new();
    let mut variants = String::new();
    let mut constants = String::new();
    let mut from = String::new();
    let mut to = String::new();
    let mut messages = String::new();

    for node in nodes {
        validate_node(node)?;
        let original = node["name"].as_str().ok_or("missing error name")?;
        schema::identifier(original)?;
        let mut name = original.to_owned();
        name[0..1].make_ascii_uppercase();
        let code = u32::try_from(node["code"].as_u64().ok_or("invalid error code")?)?;
        let message = node["message"].as_str().ok_or("missing error message")?;

        if name == "Unknown" || !names.insert(name.clone()) || !codes.insert(code) {
            return Err("duplicate error name/code".into());
        }

        let constant = format!(
            "CODE_{}",
            schema::identifier(original)?.to_ascii_uppercase()
        );
        constants.push_str(&format!("const {constant}: u32 = {code};\n"));
        variants.push_str(&format!("/// Pinned System error {code}.\n{name},\n"));
        from.push_str(&format!("{constant} => Self::{name},\n"));
        to.push_str(&format!("Self::{name} => {constant},\n"));
        messages.push_str(&format!("Self::{name} => Some({message:?}),\n"));
    }

    if codes != (0..9).collect() {
        return Err("expected nine pinned System errors".into());
    }

    Ok(format!(
        "// Generated from pinned System errors; see SYSTEM-LICENSE.\n{constants}\n/// Known System custom errors with unknown numbers preserved losslessly.\n#[derive(Clone, Copy, Debug, Eq, PartialEq)]\npub enum SystemError {{\n{variants}/// Future or unknown custom error code.\nUnknown(u32),\n}}\nimpl SystemError {{\n/// Decodes a custom program error number without discarding unknown values.\npub const fn from_code(code: u32) -> Self {{ match code {{ {from}_ => Self::Unknown(code), }} }}\n/// Returns the original custom error number.\npub const fn code(self) -> u32 {{ match self {{ {to}Self::Unknown(code) => code, }} }}\n/// Returns the pinned human-readable description of a known error.\npub const fn message(self) -> Option<&'static str> {{ match self {{ {messages}Self::Unknown(_) => None, }} }}\n}}\n"
    ))
}

pub(crate) fn run() -> Result<(), Error> {
    let root = serde_json::from_str(include_str!("../fixtures/system.json"))?;
    generate::replace_source(
        &render(&root)?,
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/solana_kit_system/src/errors.rs"),
    )
}
