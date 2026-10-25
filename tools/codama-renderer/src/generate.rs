//! Original two-encoder proof using the same validated IR as public generation.

use crate::schema::{self, Error, Instruction, Wire};
use serde_json::Value;
use std::{fs, path::Path};

fn instruction(item: &Instruction) -> String {
    let mut fields = String::new();
    let mut operations = String::new();

    for field in &item.fields {
        let name = &field.name;
        let member = format!("args.{name}");

        let (ty, encoding) = match field.wire {
            Wire::U64 => (
                "u64",
                format!("    data.extend_from_slice(&{member}.to_le_bytes());\n"),
            ),
            Wire::Address => (
                "Address",
                format!("    data.extend_from_slice({member}.as_bytes());\n"),
            ),
            Wire::String => (
                "String",
                format!(
                    "    data.extend_from_slice(&u64::try_from({member}.len())?.to_le_bytes());\n    data.extend_from_slice({member}.as_bytes());\n"
                ),
            ),
        };

        fields.push_str(&format!("    pub {name}: {ty},\n"));
        operations.push_str(&encoding);
    }

    let name = &item.name;
    let rust_name = &item.rust_name;
    let tag = item.tag;
    format!(
        "#[derive(Debug)]\npub(crate) struct {rust_name} {{\n{fields}}}\n\npub(crate) fn {name}(\n    args: &{rust_name},\n) -> Result<Vec<u8>, core::num::TryFromIntError> {{\n    let mut data = {tag}_u32.to_le_bytes().to_vec();\n{operations}    Ok(data)\n}}\n\n"
    )
}

pub(crate) fn render(root: &Value) -> Result<String, Error> {
    let instructions = schema::parse(root)?;
    let mut output = "// Generated from fixtures/system.json; see SYSTEM-LICENSE.\nuse solana_kit_core::Address;\n\n".to_owned();
    let selected: Vec<_> = instructions
        .iter()
        .filter(|item| ["create_account", "create_account_with_seed"].contains(&item.name.as_str()))
        .collect();

    if selected.len() != 2 {
        return Err("required prototype instructions missing".into());
    }

    for item in selected {
        output.push_str(&instruction(item));
    }

    Ok(output)
}

/// Validates and renders fully before atomically replacing one output file.
pub(crate) fn replace(root: &Value, output: &Path) -> Result<(), Error> {
    replace_source(&render(root)?, output)
}

pub(crate) fn replace_source(source: &str, output: &Path) -> Result<(), Error> {
    use std::io::Write;
    let source = formatted(source)?;
    let temporary = output.with_extension("rs.pending");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)?;
    file.write_all(source.as_bytes())?;
    file.sync_all()?;
    fs::rename(temporary, output)?;
    Ok(())
}

/// Formats into memory so formatter failure cannot alter the destination.
fn formatted(source: &str) -> Result<String, Error> {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024", "--emit", "stdout"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .ok_or("formatter stdin missing")?
        .write_all(source.as_bytes())?;
    let result = child.wait_with_output()?;

    if !result.status.success() {
        return Err(format!(
            "rustfmt rejected output: {}",
            String::from_utf8_lossy(&result.stderr)
        )
        .into());
    }

    Ok(String::from_utf8(result.stdout)?)
}

pub(crate) fn run() -> Result<(), Error> {
    let root = serde_json::from_str(include_str!("../fixtures/system.json"))?;
    let output = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/generated.rs");
    replace(&root, &output)?;
    println!("generated two private System instruction data encoders");
    Ok(())
}
