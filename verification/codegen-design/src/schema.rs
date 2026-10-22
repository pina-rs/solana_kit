//! Validated System schema shared by the private probe and public data emitter.

use serde_json::Value;
use std::collections::BTreeSet;

pub(crate) type Error = Box<dyn std::error::Error>;
const INSTRUCTION_COUNT: u32 = 14;

/// Unknown shape attributes may carry constraints and must not be discarded.
fn keys(node: &Value, allowed: &[&str]) -> Result<(), Error> {
    let object = node.as_object().ok_or("expected schema object")?;

    if object.keys().any(|key| !allowed.contains(&key.as_str())) {
        return Err("unsupported schema property or range".into());
    }

    Ok(())
}

#[derive(Clone, Copy)]
pub(crate) enum Wire {
    U64,
    Address,
    String,
}

pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) wire: Wire,
}

pub(crate) struct Instruction {
    pub(crate) name: String,
    pub(crate) rust_name: String,
    pub(crate) tag: u32,
    pub(crate) fields: Vec<Field>,
}

/// Produces an ASCII Rust identifier without allowing source or path injection.
pub(crate) fn identifier(name: &str) -> Result<String, Error> {
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
        "self", "super", "crate", "mod", "type", "fn", "match", "struct", "enum", "const", "async",
        "await", "loop", "return", "use", "pub", "move", "ref", "let", "in", "where", "impl",
        "trait", "unsafe", "extern", "dyn", "true", "false", "static", "union", "yield", "try",
        "box", "Self", "super",
    ]
    .contains(&output.as_str())
    {
        return Err("reserved Rust identifier".into());
    }

    Ok(output)
}

/// Every nested kind is checked because matching numeric members alone accepts
/// unrelated Codama nodes and silently invents a wire representation.
fn wire(node: &Value) -> Result<Wire, Error> {
    validate_wire_keys(node)?;

    match node["kind"].as_str() {
        Some("numberTypeNode") if node["endian"] == "le" && node["format"] == "u64" => {
            Ok(Wire::U64)
        }

        Some("publicKeyTypeNode") => Ok(Wire::Address),
        Some("sizePrefixTypeNode")
            if node["type"]["kind"] == "stringTypeNode"
                && node["type"]["encoding"] == "utf8"
                && node["prefix"]["kind"] == "numberTypeNode"
                && node["prefix"]["format"] == "u64"
                && node["prefix"]["endian"] == "le" =>
        {
            Ok(Wire::String)
        }
        _ => Err("unsupported wire type; no implicit normalization".into()),
    }
}

fn validate_wire_keys(node: &Value) -> Result<(), Error> {
    let allowed: &[&str] = match node["kind"].as_str() {
        Some("numberTypeNode") => &["kind", "format", "endian", "display"],
        Some("publicKeyTypeNode") => &["kind"],
        Some("sizePrefixTypeNode") => &["kind", "type", "prefix"],
        _ => return Err("unsupported wire kind".into()),
    };

    keys(node, allowed)?;

    if node["kind"] == "sizePrefixTypeNode" {
        keys(&node["type"], &["kind", "encoding"])?;
        keys(&node["prefix"], &["kind", "format", "endian"])?;
    }

    Ok(())
}

fn discriminator(node: &Value) -> Result<u32, Error> {
    let first = node
        .pointer("/arguments/0")
        .ok_or("missing discriminator")?;
    keys(
        first,
        &[
            "kind",
            "name",
            "type",
            "docs",
            "display",
            "defaultValue",
            "defaultValueStrategy",
        ],
    )?;
    keys(&first["type"], &["kind", "format", "endian"])?;
    keys(&first["defaultValue"], &["kind", "number"])?;
    let expected =
        serde_json::json!([{"kind":"fieldDiscriminatorNode","name":"discriminator","offset":0}]);

    if first["kind"] != "instructionArgumentNode"
        || first["name"] != "discriminator"
        || first["type"]["kind"] != "numberTypeNode"
        || first["type"]["format"] != "u32"
        || first["type"]["endian"] != "le"
        || first["defaultValue"]["kind"] != "numberValueNode"
        || first["defaultValueStrategy"] != "omitted"
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

/// Defaults are accepted only for explicit optional numeric fields. They never
/// erase a field from the public data shape or change its encoded layout.
fn field(node: &Value) -> Result<Field, Error> {
    keys(
        node,
        &[
            "kind",
            "name",
            "type",
            "docs",
            "display",
            "defaultValue",
            "defaultValueStrategy",
        ],
    )?;

    if node["kind"] != "instructionArgumentNode" {
        return Err("unsupported argument node".into());
    }

    let name = identifier(node["name"].as_str().ok_or("argument name missing")?)?;
    let wire = wire(&node["type"])?;

    if let Some(default) = node.get("defaultValue") {
        keys(default, &["kind", "number"])?;
    } else if node.get("defaultValueStrategy").is_some() {
        return Err("default strategy without a supported value".into());
    }

    if let Some(default) = node.get("defaultValue")
        && (!matches!(wire, Wire::U64)
            || default["kind"] != "numberValueNode"
            || default["number"].as_u64().is_none()
            || node["defaultValueStrategy"] != "optional")
    {
        return Err("unsupported field default".into());
    }
    Ok(Field { name, wire })
}

fn instruction(node: &Value) -> Result<Instruction, Error> {
    if node["kind"] != "instructionNode" {
        return Err("unsupported instruction node".into());
    }

    let original = node["name"].as_str().ok_or("instruction name missing")?;
    let name = identifier(original)?;
    let mut rust_name = original.to_owned();
    rust_name[0..1].make_ascii_uppercase();
    let tag = discriminator(node)?;
    let mut names = BTreeSet::from(["discriminator".to_owned()]);
    let mut fields = Vec::new();

    for argument in node["arguments"]
        .as_array()
        .ok_or("missing arguments")?
        .iter()
        .skip(1)
    {
        let field = field(argument)?;

        if !names.insert(field.name.clone()) {
            return Err("duplicate normalized field name".into());
        }

        fields.push(field);
    }

    Ok(Instruction {
        name,
        rust_name,
        tag,
        fields,
    })
}

/// Generated codec names and imported names occupy the same Rust namespace.
fn validate_output_names(items: &[Instruction]) -> Result<(), Error> {
    let mut names: BTreeSet<String> = [
        "Address",
        "String",
        "Constant",
        "Decoder",
        "Encoder",
        "Error",
        "FixedBytes",
        "Number",
        "Reader",
        "SizePrefix64",
        "Utf8",
        "Writer",
        "SystemInstructionData",
        "SystemInstructionDataCodec",
        "Result",
        "Ok",
        "Err",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();

    for name in items
        .iter()
        .flat_map(|item| [item.rust_name.clone(), format!("{}Codec", item.rust_name)])
    {
        if !names.insert(name) {
            return Err("generated output type collision".into());
        }
    }

    Ok(())
}

/// Resolves every selected shape before either emitter can write output.
pub(crate) fn parse(root: &Value) -> Result<Vec<Instruction>, Error> {
    if root["kind"] != "rootNode"
        || root["program"]["kind"] != "programNode"
        || root["program"]["name"] != "system"
    {
        return Err("expected pinned Codama System root".into());
    }

    let output: Vec<_> = root["program"]["instructions"]
        .as_array()
        .ok_or("missing instructions")?
        .iter()
        .map(instruction)
        .collect::<Result<_, _>>()?;
    validate_output_names(&output)?;
    let names: BTreeSet<_> = output.iter().map(|item| &item.name).collect();
    let types: BTreeSet<_> = output.iter().map(|item| &item.rust_name).collect();
    let tags: BTreeSet<_> = output.iter().map(|item| item.tag).collect();

    if names.len() != output.len() || types.len() != output.len() || tags.len() != output.len() {
        return Err("duplicate normalized instruction name, type or discriminator".into());
    }

    if tags != (0..INSTRUCTION_COUNT).collect() {
        return Err("expected all fourteen System instruction tags".into());
    }

    Ok(output)
}
