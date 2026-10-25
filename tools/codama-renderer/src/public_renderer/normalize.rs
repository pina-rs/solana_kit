//! Expands only defaults specified by the pinned Codama1.10.2 node factories.

use serde_json::{Map, Value};

const MAX_NODE_DEPTH: usize = 128;

/// Preserves semantic fields for the closed typed frontend to validate.
pub(super) fn root(input: &Value) -> Result<Value, String> {
    let mut output = input.clone();
    visit(&mut output, 0)?;
    Ok(output)
}

fn visit(value: &mut Value, depth: usize) -> Result<(), String> {
    if depth > MAX_NODE_DEPTH {
        return Err("Codama nesting exceeds 128 levels".into());
    }

    match value {
        Value::Array(items) => visit_items(items.iter_mut(), depth)?,
        Value::Object(object) => {
            defaults(object)?;
            visit_items(object.values_mut(), depth)?;
        }
        _ => {}
    }

    Ok(())
}

fn visit_items<'a>(items: impl Iterator<Item = &'a mut Value>, depth: usize) -> Result<(), String> {
    for item in items {
        visit(item, depth + 1)?;
    }

    Ok(())
}

/// Fill only factory defaults, leaving every explicit constraint for typed validation.
fn defaults(object: &mut Map<String, Value>) -> Result<(), String> {
    let kind = object.get("kind").and_then(Value::as_str).unwrap_or("");

    let arrays: &[&str] = match kind {
        "rootNode" => &["additionalPrograms"],
        "programNode" => &[
            "pdas",
            "accounts",
            "instructions",
            "definedTypes",
            "errors",
            "events",
        ],
        "instructionNode" => &[
            "accounts",
            "arguments",
            "discriminators",
            "remainingAccounts",
        ],
        "pdaNode" => &["seeds"],
        "structTypeNode" => &["fields"],
        "tupleTypeNode" => &["items"],
        "enumTypeNode" => &["variants"],
        _ => &[],
    };

    if kind == "programNode" {
        program_metadata(object)?;
    } else if kind == "instructionNode" {
        object
            .entry("optionalAccountStrategy")
            .or_insert_with(|| Value::String("programId".into()));
    } else if kind == "fieldDiscriminatorNode" {
        object.entry("offset").or_insert_with(|| Value::from(0));
    }

    for key in arrays {
        object
            .entry(*key)
            .or_insert_with(|| Value::Array(Vec::new()));
    }

    Ok(())
}

fn program_metadata(object: &mut Map<String, Value>) -> Result<(), String> {
    object
        .entry("version")
        .or_insert_with(|| Value::String("0.0.0".into()));

    validate_origin(object.get("origin"))?;
    validate_docs(object.get("docs"))?;

    if let Some(constants) = object.get("constants") {
        if constants.as_array().is_none_or(|items| !items.is_empty()) {
            return Err("program constants are not supported by this renderer".into());
        }

        object.remove("constants");
    }

    Ok(())
}

fn validate_origin(origin: Option<&Value>) -> Result<(), String> {
    if let Some(origin) = origin
        && !matches!(origin.as_str(), Some("anchor" | "shank"))
    {
        return Err("program origin must be anchor or shank".into());
    }

    Ok(())
}

fn validate_docs(docs: Option<&Value>) -> Result<(), String> {
    if let Some(docs) = docs
        && docs
            .as_array()
            .is_none_or(|items| items.iter().any(|item| !item.is_string()))
    {
        return Err("program documentation must be an array of strings".into());
    }

    Ok(())
}
