//! Closed Loader Shank projection with exact source-defined discriminator widths.
use crate::schema::Error;
use serde_json::{Value, json};

/// Validates original identity and preserves each source instruction in order.
pub(super) fn project(root: &Value, name: &str, address: &str) -> Result<Value, Error> {
    identity(root, name)?;
    let mut instructions = Vec::new();

    for instruction in root["instructions"]
        .as_array()
        .ok_or("missing instructions")?
    {
        instructions.push(project_instruction(instruction, name)?);
    }

    Ok(json!({
        "kind":"rootNode",
        "standard":"codama",
        "version":"1.0.0",
        "program":{
            "kind":"programNode",
            "name":name,
            "publicKey":address,
            "version":"0.0.0",
            "pdas":[],
            "accounts":[],
            "definedTypes":[],
            "errors":[],
            "instructions":instructions
        }
    }))
}

fn identity(root: &Value, name: &str) -> Result<(), Error> {
    keys(root, &["version", "name", "instructions", "metadata"])?;
    keys(
        &root["metadata"],
        &["origin", "address", "binaryVersion", "libVersion"],
    )?;

    let expected = if name == "loaderV3" {
        "solana_loader_v3_program"
    } else {
        "solana_loader_v4_program"
    };

    if root["version"] != "0.0.0"
        || root["name"] != expected
        || root["metadata"]["origin"] != "shank"
    {
        return Err("unsupported Loader source identity".into());
    }

    Ok(())
}

fn project_instruction(node: &Value, owner: &str) -> Result<Value, Error> {
    keys(node, &["name", "accounts", "args", "discriminant"])?;
    let mut name = node["name"]
        .as_str()
        .ok_or("missing instruction name")?
        .to_owned();
    name[0..1].make_ascii_lowercase();
    let mut arguments = vec![discriminator(node, owner)?];

    for field in node["args"].as_array().ok_or("missing arguments")? {
        arguments.push(argument(field, owner)?);
    }

    let accounts = node["accounts"]
        .as_array()
        .ok_or("missing accounts")?
        .iter()
        .map(account)
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(json!({
        "kind":"instructionNode",
        "name":name,
        "optionalAccountStrategy":"omitted",
        "accounts":accounts,
        "arguments":arguments,
        "discriminators":[{"kind":"fieldDiscriminatorNode","name":"discriminator","offset":0}]
    }))
}

fn discriminator(node: &Value, owner: &str) -> Result<Value, Error> {
    keys(&node["discriminant"], &["type", "value"])?;

    let prefix = if owner == "loaderV3" { "u32" } else { "u8" };
    let tag = node["discriminant"]["value"]
        .as_u64()
        .ok_or("invalid Loader tag")?;

    if node["discriminant"]["type"] != prefix || u8::try_from(tag).is_err() {
        return Err("unsupported Loader discriminator".into());
    }

    Ok(json!({
        "kind":"instructionArgumentNode",
        "name":"discriminator",
        "type":number(prefix),
        "defaultValueStrategy":"omitted",
        "defaultValue":{"kind":"numberValueNode","number":tag}
    }))
}

fn argument(field: &Value, owner: &str) -> Result<Value, Error> {
    keys(field, &["name", "type"])?;

    let prefix = if owner == "loaderV3" { "u64" } else { "u32" };

    let wire = match field["type"].as_str() {
        Some("u32") => number("u32"),
        Some("u64") => number("u64"),
        Some("bytes") => json!({
            "kind":"sizePrefixTypeNode",
            "prefix":number(prefix),
            "type":{"kind":"bytesTypeNode"}
        }),
        _ => return Err("unsupported Loader field type".into()),
    };

    Ok(json!({"kind":"instructionArgumentNode","name":field["name"],"type":wire}))
}

fn account(node: &Value) -> Result<Value, Error> {
    keys(node, &["name", "isMut", "isSigner", "isOptional", "docs"])?;
    let writable = node["isMut"].as_bool().ok_or("invalid writable role")?;
    let signer = node["isSigner"].as_bool().ok_or("invalid signer role")?;

    let optional = match node.get("isOptional") {
        Some(value) => value.as_bool().ok_or("invalid optional role")?,
        None => false,
    };

    Ok(json!({
        "kind":"instructionAccountNode",
        "name":node["name"],
        "isWritable":writable,
        "isSigner":signer,
        "isOptional":optional,
        "docs":node["docs"]
    }))
}

fn number(format: &str) -> Value {
    json!({"kind":"numberTypeNode","format":format,"endian":"le"})
}

fn keys(node: &Value, allowed: &[&str]) -> Result<(), Error> {
    if node
        .as_object()
        .ok_or("expected Loader object")?
        .keys()
        .any(|key| !allowed.contains(&key.as_str()))
    {
        return Err("unsupported Loader source property".into());
    }

    Ok(())
}
