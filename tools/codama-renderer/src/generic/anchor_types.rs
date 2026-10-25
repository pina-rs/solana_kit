//! Exact Borsh type preparation for the admitted legacy Anchor schema vocabulary.
use super::parse::{array, keys};
use crate::schema::{Error, identifier};
use serde_json::{Value, json};

pub(super) fn number(format: &str) -> Value {
    json!({"kind":"numberTypeNode","format":format,"endian":"le"})
}

/// The closed vocabulary prevents unrecognized Anchor constraints from disappearing.
pub(super) fn wire(node: &Value) -> Result<Value, Error> {
    if let Some(primitive) = node.as_str() {
        return primitive_type(primitive);
    }

    let object = node
        .as_object()
        .ok_or("Anchor type must be a string or object")?;

    if object.len() != 1 {
        return Err("Anchor compound type requires exactly one binding".into());
    }

    let (key, value) = object.iter().next().ok_or("missing Anchor type binding")?;
    compound(key, value)
}

fn compound(key: &str, value: &Value) -> Result<Value, Error> {
    match key {
        "defined" => {
            let name = value.as_str().ok_or("Anchor type link requires a name")?;
            identifier(name)?;
            Ok(json!({"kind":"definedTypeLinkNode","name":name}))
        }
        "vec" => Ok(json!({
            "kind":"arrayTypeNode","item":wire(value)?,
            "count":{"kind":"prefixedCountNode","prefix":number("u32")}
        })),
        "array" => fixed_array(value),
        _ => Err("unsupported Anchor compound wire type".into()),
    }
}

fn primitive_type(name: &str) -> Result<Value, Error> {
    match name {
        "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i64" => Ok(number(name)),
        "publicKey" => Ok(json!({"kind":"publicKeyTypeNode"})),
        "bytes" => Ok(json!({"kind":"arrayTypeNode","item":number("u8"),
            "count":{"kind":"prefixedCountNode","prefix":number("u32")}})),
        _ => Err("unsupported Anchor primitive wire type".into()),
    }
}

fn fixed_array(node: &Value) -> Result<Value, Error> {
    let pair = node
        .as_array()
        .ok_or("Anchor fixed array requires item and length")?;

    if pair.len() != 2 {
        return Err("Anchor fixed array requires exactly two members".into());
    }

    let count = pair[1]
        .as_u64()
        .ok_or("Anchor array count must be unsigned")?;
    u32::try_from(count)?;
    Ok(json!({"kind":"arrayTypeNode","item":wire(&pair[0])?,
        "count":{"kind":"fixedCountNode","value":count}}))
}

pub(super) fn field(node: &Value, kind: &str) -> Result<Value, Error> {
    keys(node, &["name", "type", "docs"])?;
    let name = node["name"].as_str().ok_or("missing Anchor field name")?;
    identifier(name)?;
    let mut output = json!({"kind":kind,"name":name,"type":wire(&node["type"])?});
    super::anchor::docs(node, &mut output)?;
    Ok(output)
}

pub(super) fn definition(node: &Value) -> Result<Value, Error> {
    keys(node, &["name", "type", "docs"])?;
    let name = node["name"]
        .as_str()
        .ok_or("missing Anchor definition name")?;
    identifier(name)?;
    let mut output = json!({"kind":"definedTypeNode","name":name,"type":shape(&node["type"])?});
    super::anchor::docs(node, &mut output)?;
    Ok(output)
}

fn shape(node: &Value) -> Result<Value, Error> {
    match node["kind"].as_str() {
        Some("struct") => {
            keys(node, &["kind", "fields"])?;
            let fields = array(node, "fields")?
                .iter()
                .map(|field_node| field(field_node, "structFieldTypeNode"))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(json!({"kind":"structTypeNode","fields":fields}))
        }
        Some("enum") => {
            keys(node, &["kind", "variants"])?;
            let variants = array(node, "variants")?
                .iter()
                .map(variant)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(json!({"kind":"enumTypeNode","size":number("u8"),"variants":variants}))
        }
        _ => Err("unsupported Anchor named shape".into()),
    }
}

fn variant(node: &Value) -> Result<Value, Error> {
    keys(node, &["name", "fields"])?;
    let name = node["name"].as_str().ok_or("missing Anchor variant name")?;
    identifier(name)?;
    let Some(fields) = node.get("fields") else {
        return Ok(json!({"kind":"enumEmptyVariantTypeNode","name":name}));
    };
    let fields = fields
        .as_array()
        .ok_or("Anchor variant fields must be an array")?;

    if fields.len() != 1 {
        return Err("only evidenced one-item Anchor enum tuples are admitted".into());
    }

    Ok(json!({"kind":"enumTupleVariantTypeNode","name":name,
        "tuple":{"kind":"tupleTypeNode","items":[wire(&fields[0])?]}}))
}
