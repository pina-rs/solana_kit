//! A deliberately narrow Codama encoder experiment over pinned System inputs.

use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path};

type Error = Box<dyn std::error::Error>;

/// Converts validated schema identifiers into unambiguous Rust identifiers.
fn identifier(name: &str) -> Result<String, Error> {
    if name.is_empty() || !name.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
        return Err("schema identifier must be ASCII alphanumeric".into());
    }

    if !name.as_bytes()[0].is_ascii_alphabetic() {
        return Err("schema identifier must start with a letter".into());
    }

    let mut output = String::new();

    for character in name.chars() {
        if character.is_ascii_uppercase() {
            output.push('_');
        }

        output.push(character.to_ascii_lowercase());
    }

    if [
        "self", "super", "crate", "mod", "type", "fn", "match", "struct",
    ]
    .contains(&output.as_str())
    {
        return Err("reserved Rust identifier".into());
    }

    Ok(output)
}

/// Resolves only the three wire types demonstrated by this experiment.
fn wire_type(node: &Value) -> Result<&'static str, Error> {
    match node["kind"].as_str() {
        Some("numberTypeNode") if node["endian"] == "le" && node["format"] == "u64" => Ok("u64"),
        Some("publicKeyTypeNode") => Ok("Address"),
        Some("sizePrefixTypeNode")
            if node.pointer("/type/kind").and_then(Value::as_str) == Some("stringTypeNode")
                && node.pointer("/type/encoding").and_then(Value::as_str) == Some("utf8")
                && node.pointer("/prefix/format").and_then(Value::as_str) == Some("u64")
                && node.pointer("/prefix/endian").and_then(Value::as_str) == Some("le") =>
        {
            Ok("String")
        }
        _ => Err("unsupported wire type; no implicit normalization".into()),
    }
}

/// Renders one validated field and its exact encoding operation.
fn field(argument: &Value) -> Result<(String, String), Error> {
    let name = identifier(argument["name"].as_str().ok_or("argument name missing")?)?;
    let ty = wire_type(&argument["type"])?;
    let member = format!("args.{name}");

    let encoding = match ty {
        "u64" => format!("    data.extend_from_slice(&{member}.to_le_bytes());\n"),
        "Address" => format!("    data.extend_from_slice({member}.as_bytes());\n"),
        "String" => format!(
            "    data.extend_from_slice(&u64::try_from({member}.len())?.to_le_bytes());\n    data.extend_from_slice({member}.as_bytes());\n"
        ),
        _ => return Err("unreachable wire type".into()),
    };

    Ok((format!("    pub {name}: {ty},\n"), encoding))
}

/// Encodes a numeric discriminator only after verifying its node kinds and layout.
fn discriminator(node: &Value) -> Result<u32, Error> {
    let first = node
        .pointer("/arguments/0")
        .ok_or("missing discriminator")?;
    let expected =
        serde_json::json!([{"kind":"fieldDiscriminatorNode","name":"discriminator","offset":0}]);

    if first["name"] != "discriminator"
        || first["type"]["kind"] != "numberTypeNode"
        || first["type"]["format"] != "u32"
        || first["type"]["endian"] != "le"
        || first["defaultValue"]["kind"] != "numberValueNode"
        || node["discriminators"] != expected
    {
        return Err("unsupported discriminator contract".into());
    }

    Ok(u32::try_from(
        first["defaultValue"]["number"]
            .as_u64()
            .ok_or("invalid discriminator")?,
    )?)
}

/// Renders data structs; instruction account policy remains explicit in the probe.
fn instruction(node: &Value) -> Result<String, Error> {
    let original = node["name"].as_str().ok_or("instruction name missing")?;
    let name = identifier(original)?;
    let mut fields = String::new();
    let mut operations = String::new();
    let mut names = BTreeSet::new();
    let tag = discriminator(node)?;

    for argument in node["arguments"]
        .as_array()
        .ok_or("missing arguments")?
        .iter()
        .skip(1)
    {
        let field_name = identifier(argument["name"].as_str().ok_or("missing argument name")?)?;

        if !names.insert(field_name) {
            return Err("duplicate normalized field name".into());
        }

        let (declaration, operation) = field(argument)?;
        fields.push_str(&declaration);
        operations.push_str(&operation);
    }

    let mut struct_name = original.to_owned();
    struct_name[0..1].make_ascii_uppercase();

    Ok(format!(
        "#[derive(Debug)]\npub(crate) struct {struct_name} {{\n{fields}}}\n\npub(crate) fn {name}(\n    args: &{struct_name},\n) -> Result<Vec<u8>, core::num::TryFromIntError> {{\n    let mut data = {tag}_u32.to_le_bytes().to_vec();\n{operations}    Ok(data)\n}}\n\n"
    ))
}

/// Validates all instruction names before selecting the two supported shapes.
pub(crate) fn render(root: &Value) -> Result<String, Error> {
    if root["kind"] != "rootNode" || root["program"]["name"] != "system" {
        return Err("expected pinned Codama System root".into());
    }

    let nodes = root["program"]["instructions"]
        .as_array()
        .ok_or("missing instructions")?;
    let mut names = BTreeSet::new();
    let mut output = "// Generated from fixtures/system.json; see SYSTEM-LICENSE.\nuse solana_kit_core::Address;\n\n".to_owned();
    let mut found = 0;

    for node in nodes {
        let original = node["name"].as_str().ok_or("missing instruction name")?;

        if !names.insert(identifier(original)?) {
            return Err("duplicate normalized instruction name".into());
        }

        if ["createAccount", "createAccountWithSeed"].contains(&original) {
            output.push_str(&instruction(node)?);
            found += 1;
        }
    }

    if found != 2 {
        return Err("required prototype instructions missing".into());
    }

    Ok(output)
}

/// Validates and renders fully before atomically replacing one output file.
pub(crate) fn replace(root: &Value, output: &Path) -> Result<(), Error> {
    use std::io::Write;
    let source = formatted(&render(root)?)?;
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

/// Regenerates the private fixture without touching any public program crate.
pub(crate) fn run() -> Result<(), Error> {
    let root = serde_json::from_str(include_str!("../fixtures/system.json"))?;
    let output = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/generated.rs");
    replace(&root, &output)?;
    println!("generated two private System instruction data encoders");
    Ok(())
}
