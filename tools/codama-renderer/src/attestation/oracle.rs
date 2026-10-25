//! Read-only complete private primary equality. No primary bytes enter emitted output.
use super::{PROGRAM, schema, source};
use crate::schema::Error;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

const IDL_SHA: &str = "924728c7b8327b5770ef63c34c804430d0a1953adcf96873bcee1f4f4ccb4a75";
const INSTRUCTION_COUNT: usize = 12;

pub(super) fn audit(path: &Path) -> Result<(), Error> {
    let bytes = fs::read(path)?;

    if format!("{:x}", Sha256::digest(&bytes)) != IDL_SHA {
        return Err("private primary IDL pin drift".into());
    }

    let primary: Value = serde_json::from_slice(&bytes)?;
    let public = source()?;
    let model = &public["program"];

    identity(&primary)?;

    let instructions = primary["instructions"]
        .as_array()
        .ok_or("missing primary instructions")?;

    if instructions.len() != INSTRUCTION_COUNT {
        return Err("private primary instruction catalog drift".into());
    }

    for (expected, actual) in model["instructions"]
        .as_array()
        .ok_or("missing model instructions")?
        .iter()
        .zip(instructions)
    {
        instruction(expected, actual)?;
    }

    accounts(model, &primary)?;
    event_inventory(model, &primary)?;
    println!(
        "complete pinned private primary shapes match MIT-derived source model; no license grant or runtime acceptance"
    );
    Ok(())
}

fn identity(primary: &Value) -> Result<(), Error> {
    if primary["name"] != "solana_attestation_service"
        || primary["metadata"]["address"] != PROGRAM
        || primary["metadata"]["origin"] != "shank"
        || primary["version"] != "0.1.0"
    {
        return Err("private primary identity drift".into());
    }

    Ok(())
}

fn event_inventory(model: &Value, primary: &Value) -> Result<(), Error> {
    let types = primary["types"]
        .as_array()
        .ok_or("missing primary named types")?;

    if types.len() != 1
        || types[0]["name"] != "CloseAttestationEvent"
        || record_fields(&model["definedTypes"][1]["type"])? != types[0]["type"]["fields"]
        || primary.get("errors").is_some()
    {
        return Err("private primary event/error inventory differs".into());
    }

    Ok(())
}

/// Compare literal tags, full argument framing and ordered account privilege policies.
fn instruction(model: &Value, primary: &Value) -> Result<(), Error> {
    let name = primary["name"]
        .as_str()
        .ok_or("missing primary instruction name")?;
    let mut expected_name = model["name"]
        .as_str()
        .ok_or("missing model instruction name")?
        .to_owned();
    expected_name[0..1].make_ascii_uppercase();
    let accounts = model["accounts"].as_array().ok_or("missing model roles")?.iter().map(|role| {
        json!({"name":role["name"],"isMut":role["isWritable"],"isSigner":role["isSigner"]})
    }).collect::<Vec<_>>();
    let actual = primary["accounts"]
        .as_array()
        .ok_or("missing primary roles")?
        .iter()
        .map(|role| json!({"name":role["name"],"isMut":role["isMut"],"isSigner":role["isSigner"]}))
        .collect::<Vec<_>>();
    let arguments = model["arguments"]
        .as_array()
        .ok_or("missing model arguments")?[1..]
        .iter()
        .map(|field| {
            let wire = if field["name"] == "layout" {
                json!("bytes")
            } else {
                primary_wire(&field["type"])?
            };

            Ok(json!({"name":field["name"],"type":wire}))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    if name != expected_name
        || accounts != actual
        || json!(arguments) != primary["args"]
        || primary["discriminant"]["type"] != "u8"
        || model["arguments"][0]["defaultValue"]["number"] != primary["discriminant"]["value"]
    {
        return Err("private primary instruction contract differs".into());
    }

    Ok(())
}

fn accounts(model: &Value, primary: &Value) -> Result<(), Error> {
    let accounts = primary["accounts"]
        .as_array()
        .ok_or("missing primary accounts")?;

    if accounts.len() != 3 {
        return Err("private primary account catalog drift".into());
    }

    for account in model["accounts"]
        .as_array()
        .ok_or("missing model accounts")?
    {
        let matched = accounts
            .iter()
            .find(|actual| account["name"] == actual["name"])
            .ok_or("missing primary account")?;

        if record_fields(&account["data"])? != matched["type"]["fields"] {
            return Err("private primary account layout differs".into());
        }
    }

    Ok(())
}

fn record_fields(record: &Value) -> Result<Value, Error> {
    let fields = record["fields"]
        .as_array()
        .ok_or("missing source record fields")?;
    let fields = fields
        .iter()
        .map(|field| Ok(json!({"name":field["name"],"type":primary_wire(&field["type"])?})))
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(json!(fields))
}

fn primary_wire(wire: &Value) -> Result<Value, Error> {
    match wire["kind"].as_str() {
        Some("numberTypeNode") => Ok(wire["format"].clone()),
        Some("publicKeyTypeNode") => Ok(json!("publicKey")),
        Some("booleanTypeNode") => Ok(json!("bool")),

        Some("arrayTypeNode") if wire["item"] == schema::number("u8") => Ok(json!("bytes")),
        Some("arrayTypeNode") => Ok(json!({"vec":primary_wire(&wire["item"])?})),
        Some("sizePrefixTypeNode") => Ok(json!("string")),
        _ => Err("unsupported source wire in private primary comparison".into()),
    }
}
