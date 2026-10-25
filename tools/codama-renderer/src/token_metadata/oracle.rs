//! Read-only comparison with the complete private primary IDL; no primary bytes are emitted.
use super::{schema, source};
use crate::schema::Error;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};

const PRIMARY_HASH: &str = "58f1a1397a1e4f6c671d2a3bf8f6e9465eedd8abb0025c943d42030180ff411f";
const ERROR_COUNT: usize = 203;
const ACCOUNT_COUNT: usize = 14;
const INSTRUCTION_COUNT: usize = 58;

/// The primary source remains an externally supplied oracle, never a shipping fixture.
pub(crate) fn audit(path: &Path) -> Result<(), Error> {
    let bytes = fs::read(path)?;

    if format!("{:x}", Sha256::digest(&bytes)) != PRIMARY_HASH {
        return Err("Token Metadata primary oracle does not match the immutable input pin".into());
    }

    let primary: Value = serde_json::from_slice(&bytes)?;
    let model = source()?;
    compare(&primary, &model)?;
    println!("58 payload/role contracts,14 accounts, all named types and203 errors compared");
    Ok(())
}

fn equal(actual: &Value, expected: &Value, contract: &str) -> Result<(), Error> {
    if actual != expected {
        return Err(format!("Token Metadata primary/model mismatch: {contract}").into());
    }

    Ok(())
}

fn items<'a>(node: &'a Value, key: &str) -> Result<&'a [Value], Error> {
    node[key]
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| "missing oracle collection".into())
}

fn named<'a>(nodes: &'a [Value], name: &Value) -> Result<&'a Value, Error> {
    let mut matches = nodes.iter().filter(|node| node["name"] == *name);
    let first = matches.next().ok_or("missing named oracle declaration")?;

    if matches.next().is_some() {
        return Err("duplicate named oracle declaration".into());
    }

    Ok(first)
}

/// Require complete named-type reachability as well as all account and instruction contracts.
fn compare(primary: &Value, model: &Value) -> Result<(), Error> {
    let model = &model["program"];
    equal(
        &primary["metadata"]["address"],
        &model["publicKey"],
        "identity",
    )?;
    equal(&primary["version"], &model["version"], "version")?;
    let types = items(primary, "types")?;
    let definitions = items(model, "definedTypes")?;
    let mut reached = BTreeSet::new();

    for definition in definitions {
        let original = named(types, &definition["name"])?;
        reached.insert(
            original["name"]
                .as_str()
                .ok_or("missing original type name")?
                .to_owned(),
        );
        equal(
            &primary_shape(&original["type"], types, &mut reached)?,
            &model_shape(&definition["type"], definitions)?,
            "named type",
        )?;
    }

    compare_accounts(primary, model, types, definitions, &mut reached)?;
    compare_instructions(primary, model, types, definitions, &mut reached)?;

    if reached.len() != types.len() {
        return Err("a complete primary type has no compared Rust contract".into());
    }

    compare_errors(primary, model)
}

fn compare_errors(primary: &Value, model: &Value) -> Result<(), Error> {
    let primary_errors = items(primary, "errors")?;
    let model_errors = items(model, "errors")?;

    if primary_errors.len() != ERROR_COUNT || model_errors.len() != ERROR_COUNT {
        return Err("incomplete Token Metadata program errors".into());
    }

    for (original, prepared) in primary_errors.iter().zip(model_errors) {
        for key in ["name", "code"] {
            equal(&original[key], &prepared[key], "program error identity")?;
        }

        let message = original.get("msg").cloned().unwrap_or_else(|| json!(""));
        equal(&message, &prepared["message"], "program error message")?;
    }

    Ok(())
}

fn compare_accounts(
    primary: &Value,
    model: &Value,
    types: &[Value],
    definitions: &[Value],
    reached: &mut BTreeSet<String>,
) -> Result<(), Error> {
    let originals = items(primary, "accounts")?;
    let prepared = items(model, "accounts")?;

    if originals.len() != ACCOUNT_COUNT || prepared.len() != ACCOUNT_COUNT {
        return Err("incomplete Token Metadata accounts".into());
    }

    for original in originals {
        let candidate = named(prepared, &original["name"])?;
        let original_type = primary_account_type(original)?;
        equal(
            &primary_shape(&original_type, types, reached)?,
            &model_shape(&candidate["data"], definitions)?,
            "account fields",
        )?;
        equal(
            &account_header(original, types)?,
            &candidate["discriminators"],
            "account header domain",
        )?;
    }

    Ok(())
}

fn primary_account_type(original: &Value) -> Result<Value, Error> {
    let mut original_type = original["type"].clone();
    let fields = original_type["fields"]
        .as_array_mut()
        .ok_or("missing account fields")?;

    if fields
        .first()
        .is_none_or(|field| field["name"] != "key" || field["type"] != json!({"defined":"Key"}))
    {
        return Err("account must retain the explicit Key domain".into());
    }

    fields.remove(0);

    if original["name"] == "CollectionAuthorityRecord" {
        fields[1]["name"] = json!("newUpdateAuthority");
    }

    Ok(original_type)
}

fn account_header(original: &Value, types: &[Value]) -> Result<Value, Error> {
    let name = match original["name"].as_str().ok_or("missing account name")? {
        "Metadata" => "MetadataV1",
        "Edition" => "EditionV1",
        "MetadataDelegateRecord" => "MetadataDelegate",
        "HolderDelegateRecord" => "HolderDelegate",
        name => name,
    };

    let keys = named(types, &json!("Key"))?;
    let key = items(&keys["type"], "variants")?
        .iter()
        .position(|variant| variant["name"] == name)
        .ok_or("unknown account key")?;
    Ok(schema::constant_header(&format!("{key:02x}")))
}

/// Only the pinned argument-name normalization is allowed before literal payload/tag comparison.
fn compare_instructions(
    primary: &Value,
    model: &Value,
    types: &[Value],
    definitions: &[Value],
    reached: &mut BTreeSet<String>,
) -> Result<(), Error> {
    let originals = items(primary, "instructions")?;
    let prepared = items(model, "instructions")?;

    if originals.len() != INSTRUCTION_COUNT || prepared.len() != INSTRUCTION_COUNT {
        return Err("incomplete Token Metadata instructions".into());
    }

    for original in originals {
        let candidate = named(prepared, &original["name"])?;
        let arguments = items(candidate, "arguments")?;
        let original_args = flattened_arguments(original, definitions, types, reached)?;
        let mut expected = primary_fields(&original_args, types, reached)?;

        if original["name"] == "UpdateMetadataAccountV2" {
            expected[1][0] = json!("newupdateauthority");
        }

        equal(
            &json!(expected),
            &json!(model_fields(&arguments[1..], definitions)?),
            "instruction payload",
        )?;
        equal(
            &json!(primary_roles(items(original, "accounts")?)?),
            &candidate["accounts"],
            "instruction role order and privileges",
        )?;
        equal(
            &original["discriminant"]["value"],
            &arguments[0]["defaultValue"]["number"],
            "instruction discriminant",
        )?;

        if original["discriminant"]["type"] != "u8" {
            return Err("unsupported primary instruction discriminant width".into());
        }
    }

    Ok(())
}

fn flattened_arguments(
    original: &Value,
    prepared: &[Value],
    types: &[Value],
    reached: &mut BTreeSet<String>,
) -> Result<Vec<Value>, Error> {
    let arguments = items(original, "args")?;

    if arguments.len() == 1
        && arguments[0]["type"].get("defined").is_some()
        && !prepared
            .iter()
            .any(|definition| definition["name"] == arguments[0]["type"]["defined"])
    {
        let definition = named(types, &arguments[0]["type"]["defined"])?;
        reached.insert(
            definition["name"]
                .as_str()
                .ok_or("missing argument record name")?
                .to_owned(),
        );

        if definition["type"]["kind"] != "struct" {
            return Err("only complete argument records can be flattened".into());
        }

        return Ok(items(&definition["type"], "fields")?.to_vec());
    }

    Ok(arguments.to_vec())
}

fn primary_roles(nodes: &[Value]) -> Result<Vec<Value>, Error> {
    let mut output = Vec::new();

    for node in nodes {
        let signer = node["isSigner"]
            .as_bool()
            .ok_or("missing primary signer flag")?;
        let writable = node["isMut"]
            .as_bool()
            .ok_or("missing primary writable flag")?;

        let optional = match node.get("isOptional") {
            None => false,
            Some(value) => value.as_bool().ok_or("invalid primary optional flag")?,
        };

        output.push(json!({"kind":"instructionAccountNode","name":node["name"],
            "isOptional":optional,"isSigner":signer,"isWritable":writable}));
    }

    Ok(output)
}

fn primary_fields(
    nodes: &[Value],
    types: &[Value],
    reached: &mut BTreeSet<String>,
) -> Result<Vec<Value>, Error> {
    nodes
        .iter()
        .map(|field| {
            Ok(json!([
                field_name(&field["name"])?,
                primary_wire(&field["type"], types, reached)?
            ]))
        })
        .collect()
}

fn model_fields(nodes: &[Value], types: &[Value]) -> Result<Vec<Value>, Error> {
    nodes
        .iter()
        .map(|field| {
            Ok(json!([
                field_name(&field["name"])?,
                model_wire(&field["type"], types)?
            ]))
        })
        .collect()
}

fn field_name(name: &Value) -> Result<String, Error> {
    Ok(name
        .as_str()
        .ok_or("missing field name")?
        .replace('_', "")
        .to_ascii_lowercase())
}

fn primary_wire(
    node: &Value,
    types: &[Value],
    reached: &mut BTreeSet<String>,
) -> Result<Value, Error> {
    if let Some(name) = node.as_str() {
        return primary_primitive(name);
    }

    if let Some(name) = node.get("defined") {
        let declaration = named(types, name)?;
        reached.insert(name.as_str().ok_or("invalid primary type link")?.to_owned());

        return primary_shape(&declaration["type"], types, reached);
    }

    if let Some(item) = node.get("vec") {
        return Ok(json!(["array", primary_wire(item, types, reached)?, "u32"]));
    }

    if let Some(item) = node.get("option") {
        return Ok(json!(["option", primary_wire(item, types, reached)?]));
    }

    primary_pair(node, types, reached)
}

fn primary_primitive(name: &str) -> Result<Value, Error> {
    match name {
        "publicKey" => Ok(json!("address")),
        "string" => Ok(json!("text")),
        "bytes" => Ok(json!(["array", "u8", "u32"])),
        "bool" | "u8" | "u16" | "u32" | "u64" | "i64" => Ok(json!(name)),
        _ => Err("uncompared primary primitive".into()),
    }
}

fn primary_pair(
    node: &Value,
    types: &[Value],
    reached: &mut BTreeSet<String>,
) -> Result<Value, Error> {
    if let Some(pair) = node.get("hashMap").and_then(Value::as_array) {
        if pair.len() != 2 {
            return Err("invalid primary map".into());
        }

        return Ok(json!([
            "map",
            primary_wire(&pair[0], types, reached)?,
            primary_wire(&pair[1], types, reached)?
        ]));
    }

    if let Some(pair) = node.get("array").and_then(Value::as_array) {
        if pair.len() != 2 {
            return Err("invalid primary fixed array".into());
        }

        return Ok(json!([
            "array",
            primary_wire(&pair[0], types, reached)?,
            pair[1]
        ]));
    }

    Err("uncompared primary wire node".into())
}

fn primary_shape(
    node: &Value,
    types: &[Value],
    reached: &mut BTreeSet<String>,
) -> Result<Value, Error> {
    match node["kind"].as_str() {
        Some("struct") => Ok(json!([
            "record",
            primary_fields(items(node, "fields")?, types, reached)?
        ])),
        Some("enum") => {
            let variants = items(node, "variants")?
                .iter()
                .map(|variant| primary_variant(variant, types, reached))
                .collect::<Result<Vec<_>, Error>>()?;
            Ok(json!(["enum", variants]))
        }
        _ => Err("uncompared primary named shape".into()),
    }
}

fn primary_variant(
    variant: &Value,
    types: &[Value],
    reached: &mut BTreeSet<String>,
) -> Result<Value, Error> {
    let fields = match variant.get("fields") {
        None => Vec::new(),
        Some(value) => value
            .as_array()
            .ok_or("invalid primary enum fields")?
            .clone(),
    };
    let payload = if fields
        .first()
        .is_some_and(|field| field.get("name").is_none())
    {
        if fields.len() != 1 {
            return Err("only one-item primary enum tuples are admitted".into());
        }

        json!(["tuple", primary_wire(&fields[0], types, reached)?])
    } else {
        json!(["record", primary_fields(&fields, types, reached)?])
    };

    Ok(json!([variant["name"], payload]))
}

/// Compare exact framing for each supported model container, not only its inner value.
fn model_wire(node: &Value, types: &[Value]) -> Result<Value, Error> {
    match node["kind"].as_str() {
        Some("numberTypeNode") => Ok(node["format"].clone()),
        Some("publicKeyTypeNode") => Ok(json!("address")),
        Some("booleanTypeNode") => Ok(json!("bool")),
        Some("definedTypeLinkNode") => model_shape(&named(types, &node["name"])?["type"], types),
        Some("optionTypeNode") => Ok(json!(["option", model_wire(&node["item"], types)?])),
        Some("mapTypeNode") => Ok(json!([
            "map",
            model_wire(&node["key"], types)?,
            model_wire(&node["value"], types)?
        ])),
        Some("arrayTypeNode") => Ok(json!([
            "array",
            model_wire(&node["item"], types)?,
            if node["count"]["kind"] == "prefixedCountNode" {
                json!("u32")
            } else {
                node["count"]["value"].clone()
            }
        ])),

        Some("sizePrefixTypeNode") if node["type"]["kind"] == "stringTypeNode" => Ok(json!("text")),

        Some("fixedSizeTypeNode") if node["type"]["kind"] == "bytesTypeNode" => {
            Ok(json!(["array", "u8", node["size"]]))
        }

        _ => Err("uncompared model wire node".into()),
    }
}

fn model_shape(node: &Value, types: &[Value]) -> Result<Value, Error> {
    match node["kind"].as_str() {
        Some("structTypeNode") => Ok(json!([
            "record",
            model_fields(items(node, "fields")?, types)?
        ])),
        Some("enumTypeNode") => {
            let variants = items(node, "variants")?
                .iter()
                .map(|variant| model_variant(variant, types))
                .collect::<Result<Vec<_>, Error>>()?;
            Ok(json!(["enum", variants]))
        }
        _ => Err("uncompared model named shape".into()),
    }
}

fn model_variant(variant: &Value, types: &[Value]) -> Result<Value, Error> {
    let fields = match variant["kind"].as_str() {
        Some("enumEmptyVariantTypeNode") => json!(["record", []]),
        Some("enumStructVariantTypeNode") => json!([
            "record",
            model_fields(items(&variant["struct"], "fields")?, types)?
        ]),
        Some("enumTupleVariantTypeNode") => {
            let items = items(&variant["tuple"], "items")?;

            if items.len() != 1 {
                return Err("model enum tuple requires one item".into());
            }

            json!(["tuple", model_wire(&items[0], types)?])
        }
        _ => return Err("uncompared model enum payload".into()),
    };

    Ok(json!([variant["name"], fields]))
}
