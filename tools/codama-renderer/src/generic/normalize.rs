//! Hoists anonymous records deterministically before typed parsing.
use crate::schema::{Error, identifier};
use serde_json::{Value, json};

/// Only structural records are rewritten; all other properties reach validation.
pub(super) fn records(root: &Value) -> Result<Value, Error> {
    root.as_object().ok_or("expected root schema object")?;
    root.get("program")
        .and_then(Value::as_object)
        .ok_or("expected program schema object")?;
    let mut root = root.clone();
    let program = &mut root["program"];
    let mut additions = Vec::new();

    for collection in ["instructions", "accounts", "definedTypes"] {
        collection_records(program, collection, &mut additions)?;
    }

    program["definedTypes"]
        .as_array_mut()
        .ok_or("missing types")?
        .extend(additions);
    Ok(root)
}
fn collection_records(
    program: &mut Value,
    collection: &str,
    additions: &mut Vec<Value>,
) -> Result<(), Error> {
    for node in program[collection]
        .as_array_mut()
        .ok_or("missing declarations")?
    {
        let name = node["name"]
            .as_str()
            .ok_or("missing declaration name")?
            .to_owned();
        identifier(&name)?;

        match collection {
            "instructions" => arguments(node, &name, additions)?,
            "accounts" => children(&mut node["data"], &name, additions)?,
            _ => children(&mut node["type"], &name, additions)?,
        }
    }
    Ok(())
}
fn arguments(node: &mut Value, name: &str, additions: &mut Vec<Value>) -> Result<(), Error> {
    for argument in node["arguments"]
        .as_array_mut()
        .ok_or("missing arguments")?
    {
        let field = argument["name"].as_str().ok_or("missing argument name")?;
        let path = format!("{name}{}", upper(field)?);
        hoist(&mut argument["type"], &path, additions)?;
    }
    Ok(())
}
fn upper(name: &str) -> Result<String, Error> {
    identifier(name)?;
    let mut output = name.to_owned();
    output[0..1].make_ascii_uppercase();
    Ok(output)
}
fn children(node: &mut Value, path: &str, additions: &mut Vec<Value>) -> Result<(), Error> {
    if node["kind"] != "structTypeNode" {
        return Ok(());
    }

    for field in node["fields"].as_array_mut().ok_or("missing fields")? {
        let name = field["name"].as_str().ok_or("missing field")?;
        let next = format!("{path}{}", upper(name)?);
        hoist(&mut field["type"], &next, additions)?;
    }

    Ok(())
}
fn hoist(node: &mut Value, path: &str, additions: &mut Vec<Value>) -> Result<(), Error> {
    match node["kind"].as_str() {
        Some("structTypeNode") => {
            children(node, path, additions)?;
            let original =
                std::mem::replace(node, json!({"kind":"definedTypeLinkNode","name":path}));
            additions.push(json!({"kind":"definedTypeNode","name":path,"type":original,"docs":[]}));
        }
        Some("arrayTypeNode" | "optionTypeNode") => {
            hoist(&mut node["item"], &format!("{path}Item"), additions)?
        }
        Some("sizePrefixTypeNode") => hoist(&mut node["type"], path, additions)?,
        _ => {}
    }

    Ok(())
}
