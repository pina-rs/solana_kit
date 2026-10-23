//! Exact pinned Token-2022 account-role projection; no wire-layout emission.
mod builder_tests;
mod errors;

mod accounts_output;
mod methods_output;
mod output;

#[cfg(test)]
mod atomic_tests;
mod roles;

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{error::Error, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub(super) fn run() -> Result<()> {
    let arguments: Vec<_> = std::env::args().collect();
    let destination = Path::new(arguments.get(1).ok_or("destination required")?);
    let bytes = include_bytes!("../../fixtures/token-2022-original.json");

    if format!("{:x}", Sha256::digest(bytes))
        != "13cbda7213599f3312e1b7ffd1c7f176f36ebd74dba9feabe6e9a6874dc1d5d2"
    {
        return Err("pinned Token-2022 IDL drift".into());
    }

    let root: Value = serde_json::from_slice(bytes)?;
    let items = root["program"]["instructions"]
        .as_array()
        .ok_or("instructions required")?;

    if items.len() != 100 || root["program"]["pdas"] != serde_json::json!([]) {
        return Err("unexpected instruction or PDA inventory".into());
    }

    project(destination, items)
}

fn project(destination: &Path, items: &[Value]) -> Result<()> {
    let mut outputs = output::render(items)?;
    outputs.push((
        "../../tests/builder_literals.rs".to_owned(),
        builder_tests::render(items)?,
    ));

    outputs.push(("../errors.rs".to_owned(), errors::render()?));
    fs::create_dir_all(destination)?;

    replace(destination, outputs)
}

/// Resolve and format every file before replacement. Each rename is atomic;
/// replacing the entire set is not a transaction if a later rename fails.
fn replace(destination: &Path, outputs: Vec<(String, String)>) -> Result<()> {
    let mut formatted = Vec::new();

    for (name, content) in outputs {
        let pending = destination.join(format!("{name}.pending"));
        fs::write(&pending, content)?;
        let status = std::process::Command::new(std::env::var("RUSTFMT")?)
            .args(["--edition", "2024", "--config", "skip_children=true"])
            .arg(&pending)
            .status()?;

        if !status.success() {
            return Err("wrapper formatting failed before replacement".into());
        }

        formatted.push((pending, destination.join(name)));
    }

    for (pending, final_path) in formatted {
        fs::rename(pending, final_path)?;
    }

    Ok(())
}

fn text<'a>(value: &'a Value, name: &str) -> Result<&'a str> {
    value[name]
        .as_str()
        .ok_or_else(|| format!("missing {name}").into())
}

fn snake(value: &str) -> String {
    let mut result = String::new();

    for (index, character) in value.chars().enumerate() {
        if character.is_ascii_uppercase() && index > 0 {
            result.push('_');
        }

        result.push(character.to_ascii_lowercase());
    }

    result
}

fn title(value: &str) -> String {
    let mut characters = value.chars();
    characters.next().map_or_else(String::new, |first| {
        format!(
            "{}{rest}",
            first.to_ascii_uppercase(),
            rest = characters.as_str()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_role_defaults_and_unknown_policies_are_explicit() {
        let node = serde_json::json!({
            "kind":"instructionAccountNode",
            "name":"authority",
            "isWritable":false,
            "isSigner":"either",
            "isOptional":false,
            "defaultValue":{"kind":"identityValueNode"}
        });
        let role = roles::parse(&node).unwrap();
        assert_eq!(role.signer, "Either");
        assert!(role.default.is_none());
        let mut invalid = node;
        invalid["defaultValue"] = serde_json::json!({"kind":"ambientIdentityNode"});
        assert!(roles::parse(&invalid).is_err());
    }
}
