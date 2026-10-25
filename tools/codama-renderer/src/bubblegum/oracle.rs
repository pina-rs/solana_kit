//! Private external schema equality. This auditor never emits or distributes the input.
use super::{schema, source};
use crate::schema::Error;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

const PRIMARY_SHA256: &str = "3c7e0f6d57375b5204d7c831b4cd4fed2ae7ae045f6fd458b695fbde1cfc246f";
const INSTRUCTION_COUNT: usize = 36;
const MESSAGE_DIFFERENCES: &[u64] = &[
    6000, 6001, 6002, 6003, 6014, 6016, 6018, 6020, 6021, 6022, 6023, 6024, 6026, 6029, 6030, 6031,
    6033,
];

pub(crate) fn audit(path: &Path) -> Result<(), Error> {
    let bytes = fs::read(path)?;

    if format!("{:x}", Sha256::digest(&bytes)) != PRIMARY_SHA256 {
        return Err("Bubblegum private oracle pin mismatch".into());
    }

    let primary: Value = serde_json::from_slice(&bytes)?;
    let model = source()?;
    let prepared = &model["program"];

    for (key, count) in [
        ("instructions", INSTRUCTION_COUNT),
        ("accounts", 2),
        ("errors", 60),
    ] {
        if array(&primary, key)?.len() != count || array(prepared, key)?.len() != count {
            return Err(format!("Bubblegum oracle cardinality mismatch for {key}").into());
        }
    }

    audit_instructions(&primary, prepared)?;
    audit_accounts(&primary, prepared)?;
    audit_errors(&primary, prepared)?;
    audit_types(&primary, prepared)?;
    println!(
        "complete private Bubblegum wire comparison;17 explicit MIT message differences; no source emitted"
    );
    Ok(())
}

fn audit_instructions(primary: &Value, prepared: &Value) -> Result<(), Error> {
    for instruction in array(primary, "instructions")? {
        let name = text(instruction, "name")?;
        let actual = named(array(prepared, "instructions")?, name)?;
        let fields = fields(
            array(instruction, "args")?,
            "instructionArgumentNode",
            name == "decompressV1",
        )?;

        if actual["arguments"].as_array().ok_or("missing arguments")?[1..] != fields {
            return Err(format!("Bubblegum payload mismatch for {name}").into());
        }

        let expected_roles = array(instruction, "accounts")?
            .iter()
            .map(|role| expected_role(role, name))
            .collect::<Result<Vec<_>, Error>>()?;

        if actual["accounts"] != json!(expected_roles) {
            return Err(format!("Bubblegum ordered account policy mismatch for {name}").into());
        }
    }

    Ok(())
}

fn expected_role(role: &Value, instruction: &str) -> Result<Value, Error> {
    let name = text(role, "name")?;
    let conditional = ["burn", "transfer"].contains(&instruction)
        && ["leafOwner", "leafDelegate"].contains(&name);
    let signer = if conditional {
        json!("either")
    } else {
        role["isSigner"].clone()
    };

    Ok(json!({"kind":"instructionAccountNode","name":name,
        "isSigner":signer,"isWritable":role["isMut"],
        "isOptional":role.get("isOptional").cloned().unwrap_or(Value::Bool(false))}))
}

fn audit_accounts(primary: &Value, prepared: &Value) -> Result<(), Error> {
    for account in array(primary, "accounts")? {
        let name = text(account, "name")?;
        let actual = named(array(prepared, "accounts")?, name)?;

        if actual["data"]["fields"]
            != json!(fields(
                array(&account["type"], "fields")?,
                "structFieldTypeNode",
                false
            )?)
        {
            return Err(format!("Bubblegum account payload mismatch for {name}").into());
        }
    }

    Ok(())
}

fn audit_errors(primary: &Value, prepared: &Value) -> Result<(), Error> {
    for error in array(primary, "errors")? {
        let code = error["code"].as_u64().ok_or("missing error code")?;
        let actual = array(prepared, "errors")?
            .iter()
            .find(|item| item["code"] == code)
            .ok_or("missing Bubblegum program error")?;

        let differs = actual["message"] != error["msg"];

        if differs != MESSAGE_DIFFERENCES.contains(&code) {
            return Err(format!("Bubblegum error message discrepancy at {code}").into());
        }
    }

    Ok(())
}

fn audit_types(primary: &Value, prepared: &Value) -> Result<(), Error> {
    for definition in array(primary, "types")? {
        let name = text(definition, "name")?;
        let actual = named(array(prepared, "definedTypes")?, name)?;

        let expected = match definition["type"]["kind"].as_str() {
            Some("struct") => json!({
                "kind":"structTypeNode",
                "fields":fields(
                    array(&definition["type"], "fields")?, "structFieldTypeNode", false
                )?
            }),
            Some("enum") => enumeration(&definition["type"])?,
            _ => return Err("unknown private named type shape".into()),
        };

        if actual["type"] != expected {
            return Err(format!("Bubblegum named type mismatch for {name}").into());
        }
    }

    Ok(())
}

fn enumeration(node: &Value) -> Result<Value, Error> {
    let variants = array(node, "variants")?
        .iter()
        .map(|variant| {
            let name = text(variant, "name")?;

            if let Some(fields_node) = variant.get("fields") {
                let items = fields_node.as_array().ok_or("invalid enum fields")?;
                return Ok(json!({
                    "kind":"enumStructVariantTypeNode","name":name,
                    "struct":{"kind":"structTypeNode",
                        "fields":fields(items, "structFieldTypeNode", false)?}
                }));
            }

            Ok(json!({"kind":"enumEmptyVariantTypeNode","name":name}))
        })
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(json!({"kind":"enumTypeNode","size":schema::number("u8"),"variants":variants}))
}

fn fields(nodes: &[Value], kind: &str, rename_metadata: bool) -> Result<Vec<Value>, Error> {
    nodes
        .iter()
        .map(|field| {
            let original = text(field, "name")?;
            let name = if rename_metadata && original == "metadata" {
                "metadataArgs"
            } else {
                original
            };

            Ok(json!({"kind":kind,"name":name,"type":wire(&field["type"])?}))
        })
        .collect()
}

/// Translate only the pinned private vocabulary; each recursive container keeps its framing.
fn wire(node: &Value) -> Result<Value, Error> {
    if let Some(name) = node.as_str() {
        return schema::wire(match name {
            "publicKey" => "address",
            "string" => "text",
            other => other,
        });
    }

    let object = node.as_object().ok_or("invalid private wire shape")?;

    if object.len() != 1 {
        return Err("unknown private wire constraint".into());
    }

    if let Some(name) = node.get("defined").and_then(Value::as_str) {
        return schema::wire(name);
    }

    if let Some(item) = node.get("option") {
        return Ok(
            json!({"kind":"optionTypeNode","item":wire(item)?,"prefix":schema::number("u8")}),
        );
    }

    if let Some(item) = node.get("vec") {
        return Ok(json!({"kind":"arrayTypeNode","item":wire(item)?,
            "count":{"kind":"prefixedCountNode","prefix":schema::number("u32")}}));
    }

    if node["array"] == json!(["u8", 32]) {
        return schema::wire("fixed32");
    }

    Err("unsupported private wire shape".into())
}

fn array<'a>(node: &'a Value, key: &str) -> Result<&'a [Value], Error> {
    Ok(node[key].as_array().ok_or("missing private schema array")?)
}

fn text<'a>(node: &'a Value, key: &str) -> Result<&'a str, Error> {
    node[key]
        .as_str()
        .ok_or("missing private schema text".into())
}

fn named<'a>(items: &'a [Value], name: &str) -> Result<&'a Value, Error> {
    let matches = items
        .iter()
        .filter(|item| item["name"] == name)
        .collect::<Vec<_>>();

    if matches.len() != 1 {
        return Err(format!("missing or duplicate public contract {name}").into());
    }

    Ok(matches[0])
}
