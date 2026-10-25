//! Validates every selected semantic node before constructing the private IR.
use super::{
    Account, Definition, Field, Instruction, Policy, Program, ProgramError, Shape, layout, roles,
    wire,
};
use crate::schema::{Error, identifier};
use serde_json::Value;
use std::collections::BTreeSet;

const U8_VARIANTS: usize = 256;

fn u8_type() -> Value {
    serde_json::json!({"kind":"numberTypeNode","format":"u8","endian":"le"})
}

pub(super) fn keys(node: &Value, allowed: &[&str]) -> Result<(), Error> {
    let object = node.as_object().ok_or("expected schema object")?;

    if let Some(key) = object.keys().find(|key| !allowed.contains(&key.as_str())) {
        return Err(format!(
            "unsupported semantic schema property {key} in {}",
            node["kind"]
        )
        .into());
    }

    Ok(())
}

pub(super) fn array<'a>(node: &'a Value, key: &str) -> Result<&'a [Value], Error> {
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
fn fields(nodes: &[Value], kind: &str) -> Result<Vec<Field>, Error> {
    let mut names = BTreeSet::new();
    let mut output = Vec::new();

    for node in nodes {
        keys(
            node,
            &["kind", "name", "type", "docs", "display", "defaultValue"],
        )?;

        argument_default(node)?;

        if node["kind"] != kind {
            return Err("unexpected field node kind".into());
        }

        let name = identifier(node["name"].as_str().ok_or("missing field name")?)?;

        if !names.insert(name.clone()) {
            return Err("duplicate normalized field".into());
        }

        output.push(Field {
            name,
            wire: wire::parse(&node["type"])?,
        });
    }

    Ok(output)
}

fn argument_default(node: &Value) -> Result<(), Error> {
    let Some(default) = node.get("defaultValue") else {
        return Ok(());
    };
    keys(default, &["kind"])?;

    if default["kind"] != "noneValueNode" || node["type"]["kind"] != "optionTypeNode" {
        return Err("unsupported argument default".into());
    }

    Ok(())
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

fn instruction(node: &Value, accounts: &[Account]) -> Result<Instruction, Error> {
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
            "remainingAccounts",
            "provides",
        ],
    )?;

    if node["kind"] != "instructionNode"
        || ![serde_json::json!("programId"), serde_json::json!("omitted")]
            .contains(&node["optionalAccountStrategy"])
    {
        return Err("unsupported instruction contract".into());
    }

    let arguments = array(node, "arguments")?;
    let tag = discriminator(node)?;
    super::provided::validate(node, accounts)?;

    Ok(Instruction {
        name: name(node)?,
        tag,
        fields: fields(&arguments[1..], "instructionArgumentNode")?,
        roles: roles::parse(array(node, "accounts")?)?,
        remaining: roles::remaining(node)?,
        omit_optional: node["optionalAccountStrategy"] == "omitted",
    })
}

/// Validates the selected schema identity before constructing its private layout graph.
fn selected_program<'a>(
    root: &'a Value,
    policy: &Policy<'_>,
) -> Result<(&'a Value, solana_kit_core::Address), Error> {
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

    empty(node, "pdas")?;

    Ok((node, address))
}

/// Definitions and instructions resolve only after the selected program is trusted.
pub(super) fn program(root: &Value, policy: &Policy<'_>) -> Result<Program, Error> {
    let (node, address) = selected_program(root, policy)?;
    let definitions = array(node, "definedTypes")?
        .iter()
        .map(definition)
        .collect::<Result<_, _>>()?;
    let accounts = array(node, "accounts")?
        .iter()
        .map(account)
        .collect::<Result<Vec<_>, _>>()?;
    let instructions = array(node, "instructions")?
        .iter()
        .map(|node| instruction(node, &accounts))
        .collect::<Result<Vec<_>, _>>()?;

    if instructions.is_empty() {
        return Err("empty selected program".into());
    }

    let output = Program {
        name: name(node)?,
        address,
        definitions,
        instructions,
        accounts,
        errors: array(node, "errors")?
            .iter()
            .map(program_error)
            .collect::<Result<_, _>>()?,
    };
    layout::validate(&output)?;
    Ok(output)
}

fn account(node: &Value) -> Result<Account, Error> {
    keys(
        node,
        &["kind", "name", "data", "size", "discriminators", "docs"],
    )?;
    let size = usize::try_from(
        node["size"]
            .as_u64()
            .ok_or("explicit account size required")?,
    )?;

    if node["kind"] != "accountNode"
        || node["discriminators"]
            != serde_json::json!([{"kind":"sizeDiscriminatorNode","size":size}])
    {
        return Err("unsupported account discriminator".into());
    }
    Ok(Account {
        definition: Definition {
            name: name(node)?,
            shape: shape(&node["data"])?,
        },
        size,
    })
}

pub(super) fn program_error(node: &Value) -> Result<ProgramError, Error> {
    const ERROR_FIELDS: &[&str] = &["kind", "name", "code", "message", "docs"];
    keys(node, ERROR_FIELDS)?;

    if node["kind"] != "errorNode" {
        return Err("unexpected error kind".into());
    }

    Ok(ProgramError {
        name: name(node)?,
        code: u32::try_from(node["code"].as_u64().ok_or("invalid error code")?)?,
        message: node["message"]
            .as_str()
            .ok_or("missing error message")?
            .to_owned(),
    })
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
    let expected_discriminator = serde_json::json!([{
        "kind": "fieldDiscriminatorNode",
        "name": "discriminator",
        "offset": 0
    }]);

    if first["kind"] != "instructionArgumentNode"
        || first["name"] != "discriminator"
        || first["type"] != u8_type()
        || first["defaultValue"]["kind"] != "numberValueNode"
        || first["defaultValueStrategy"] != "omitted"
        || node["discriminators"] != expected_discriminator
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
