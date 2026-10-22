//! Validates every selected semantic node before constructing the private IR.
use super::{Definition, Field, Instruction, Policy, Program, Shape, Wire, layout};
use crate::schema::{Error, identifier};
use serde_json::Value;
use std::collections::BTreeSet;

const U8_VARIANTS: usize = 256;

fn u8_type() -> Value {
    serde_json::json!({"kind":"numberTypeNode","format":"u8","endian":"le"})
}

pub(super) fn keys(node: &Value, allowed: &[&str]) -> Result<(), Error> {
    let object = node.as_object().ok_or("expected schema object")?;

    if object.keys().any(|key| !allowed.contains(&key.as_str())) {
        return Err("unsupported semantic schema property".into());
    }

    Ok(())
}

fn array<'a>(node: &'a Value, key: &str) -> Result<&'a [Value], Error> {
    Ok(node[key].as_array().ok_or("expected schema array")?)
}

fn empty(node: &Value, key: &str) -> Result<(), Error> {
    if !array(node, key)?.is_empty() {
        return Err("unsupported nonempty schema collection".into());
    }

    Ok(())
}

pub(super) fn name(node: &Value) -> Result<String, Error> {
    let original = node["name"].as_str().ok_or("missing schema name")?;
    identifier(original)?;
    let mut output = original.to_owned();
    output[0..1].make_ascii_uppercase();

    if output == "Self" {
        return Err("reserved Rust type identifier".into());
    }

    Ok(output)
}

/// Unsupported nested properties can carry wire constraints and must fail closed.
fn wire(node: &Value) -> Result<Wire, Error> {
    match node["kind"].as_str() {
        Some("numberTypeNode") => {
            keys(node, &["kind", "format", "endian", "display"])?;
            if node["endian"] != "le" {
                return Err("only little endian numbers are supported".into());
            }
            match node["format"].as_str() {
                Some("u8") => Ok(Wire::Number("u8")),
                Some("u32") => Ok(Wire::Number("u32")),
                Some("u64") => Ok(Wire::Number("u64")),
                _ => Err("unsupported numeric layout".into()),
            }
        }
        Some("definedTypeLinkNode") => {
            keys(node, &["kind", "name"])?;
            Ok(Wire::Link(name(node)?))
        }
        _ => Err("unsupported field layout".into()),
    }
}

fn fields(nodes: &[Value], kind: &str) -> Result<Vec<Field>, Error> {
    let mut names = BTreeSet::new();
    let mut output = Vec::new();

    for node in nodes {
        keys(node, &["kind", "name", "type", "docs", "display"])?;

        if node["kind"] != kind {
            return Err("unexpected field node kind".into());
        }

        let name = identifier(node["name"].as_str().ok_or("missing field name")?)?;

        if !names.insert(name.clone()) {
            return Err("duplicate normalized field".into());
        }

        output.push(Field {
            name,
            wire: wire(&node["type"])?,
        });
    }

    Ok(output)
}

fn shape(node: &Value) -> Result<Shape, Error> {
    match node["kind"].as_str() {
        Some("structTypeNode") => {
            keys(node, &["kind", "fields"])?;
            Ok(Shape::Struct(fields(
                array(node, "fields")?,
                "structFieldTypeNode",
            )?))
        }
        Some("enumTypeNode") => enumeration(node),
        _ => Err("unsupported named layout".into()),
    }
}

fn definition(node: &Value) -> Result<Definition, Error> {
    keys(node, &["kind", "name", "type", "docs"])?;

    if node["kind"] != "definedTypeNode" {
        return Err("unexpected definition kind".into());
    }

    Ok(Definition {
        name: name(node)?,
        shape: shape(&node["type"])?,
    })
}

fn instruction(node: &Value) -> Result<Instruction, Error> {
    keys(
        node,
        &[
            "kind",
            "name",
            "idlName",
            "docs",
            "display",
            "accounts",
            "arguments",
            "discriminators",
            "optionalAccountStrategy",
        ],
    )?;

    if node["kind"] != "instructionNode" || node["optionalAccountStrategy"] != "programId" {
        return Err("unsupported instruction contract".into());
    }

    empty(node, "accounts")?;
    let arguments = array(node, "arguments")?;
    let tag = discriminator(node)?;

    Ok(Instruction {
        name: name(node)?,
        tag,
        fields: fields(&arguments[1..], "instructionArgumentNode")?,
    })
}

pub(super) fn program(root: &Value, policy: &Policy<'_>) -> Result<Program, Error> {
    keys(
        root,
        &[
            "kind",
            "program",
            "additionalPrograms",
            "standard",
            "version",
        ],
    )?;

    if root["kind"] != "rootNode" || root["standard"] != "codama" || root["version"] != "1.0.0" {
        return Err("unsupported Codama root".into());
    }

    empty(root, "additionalPrograms")?;
    let node = &root["program"];
    keys(
        node,
        &[
            "kind",
            "name",
            "publicKey",
            "version",
            "origin",
            "prefix",
            "docs",
            "display",
            "pdas",
            "accounts",
            "instructions",
            "definedTypes",
            "errors",
        ],
    )?;

    if node["kind"] != "programNode" || node["name"] != policy.name || node["prefix"] != "" {
        return Err("unexpected selected program".into());
    }

    let address: solana_kit_core::Address = node["publicKey"]
        .as_str()
        .ok_or("missing program identity")?
        .parse()?;

    if address != policy.address || node["publicKey"] != address.to_string() {
        return Err("untrusted program identity".into());
    }

    for key in ["pdas", "accounts", "errors"] {
        empty(node, key)?;
    }

    let definitions = array(node, "definedTypes")?
        .iter()
        .map(definition)
        .collect::<Result<_, _>>()?;
    let instructions = array(node, "instructions")?
        .iter()
        .map(instruction)
        .collect::<Result<Vec<_>, _>>()?;

    if instructions.is_empty() {
        return Err("empty selected program".into());
    }

    let output = Program {
        name: name(node)?,
        address,
        definitions,
        instructions,
    };
    layout::validate(&output)?;
    Ok(output)
}

/// Only explicit contiguous u8 unit variants have an evidenced layout in this slice.
fn enumeration(node: &Value) -> Result<Shape, Error> {
    keys(node, &["kind", "variants", "size"])?;

    if node["size"] != u8_type() {
        return Err("only explicit u8 enum tags are supported".into());
    }

    let mut variants = Vec::new();
    let mut unique = BTreeSet::new();

    for item in array(node, "variants")? {
        keys(item, &["kind", "name", "docs"])?;

        if item["kind"] != "enumEmptyVariantTypeNode" {
            return Err("only empty finite enum variants are supported".into());
        }

        let name = name(item)?;

        if !unique.insert(name.clone()) {
            return Err("duplicate enum variant".into());
        }

        variants.push(name);
    }

    if variants.is_empty() || variants.len() > U8_VARIANTS {
        return Err("invalid enum variant count".into());
    }

    Ok(Shape::Enum(variants))
}

fn discriminator(node: &Value) -> Result<u8, Error> {
    let first = node["arguments"]
        .as_array()
        .and_then(|items| items.first())
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
    keys(&first["defaultValue"], &["kind", "number"])?;

    if first["kind"] != "instructionArgumentNode"
        || first["name"] != "discriminator"
        || first["type"] != u8_type()
        || first["defaultValue"]["kind"] != "numberValueNode"
        || first["defaultValueStrategy"] != "omitted"
        || node["discriminators"]
            != serde_json::json!([{"kind":"fieldDiscriminatorNode","name":"discriminator","offset":0}])
    {
        return Err("unsupported explicit discriminator".into());
    }
    let tag = u8::try_from(
        first["defaultValue"]["number"]
            .as_u64()
            .ok_or("invalid discriminator")?,
    )?;

    Ok(tag)
}
