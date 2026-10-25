//! Read-only comparison with the complete private primary IDL; no primary bytes are emitted.
use super::{schema, source};
use crate::schema::Error;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};

const PRIMARY_HASH: &str = "cb9a0a29040ec3853a5105c547ffc608bb0e3175d78dacf30d463aff939f9b9b";
const ERROR_COUNT: usize = 45;
const ACCOUNT_COUNT: usize = 9;
const INSTRUCTION_COUNT: usize = 36;

/// The primary source remains an externally supplied oracle, never a shipping fixture.
pub(crate) fn audit(path: &Path) -> Result<(), Error> {
    let bytes = fs::read(path)?;

    if format!("{:x}", Sha256::digest(&bytes)) != PRIMARY_HASH {
        return Err("Squads primary oracle does not match the immutable input pin".into());
    }

    let primary: Value = serde_json::from_slice(&bytes)?;
    let model = source()?;
    compare(&primary, &model)?;
    println!(
        "all 36 Squads payload/role contracts, nine accounts, named types and 45 errors agree with the complete private primary oracle"
    );
    Ok(())
}

fn equal(actual: &Value, expected: &Value, contract: &str) -> Result<(), Error> {
    if actual != expected {
        return Err(format!("Squads primary/model mismatch: {contract}").into());
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

/// Follow every named type and complete account/instruction contract before admitting the oracle.
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
        return Err("incomplete Squads program errors".into());
    }

    for (original, prepared) in primary_errors.iter().zip(model_errors) {
        for key in ["name", "code"] {
            equal(&original[key], &prepared[key], "program error identity")?;
        }

        equal(
            &original["msg"],
            &prepared["message"],
            "program error message",
        )?;
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
        return Err("incomplete Squads accounts".into());
    }

    for original in originals {
        let candidate = named(prepared, &original["name"])?;
        equal(
            &primary_shape(&original["type"], types, reached)?,
            &model_shape(&candidate["data"], definitions)?,
            "account fields",
        )?;
        let name = original["name"].as_str().ok_or("missing account name")?;
        equal(
            &schema::constant_header(&schema::discriminator("account", name)),
            &candidate["discriminators"],
            "account header domain",
        )?;
    }

    Ok(())
}

/// Source-specific argument flattening and grouped account names retain their exact contracts.
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
        return Err("incomplete Squads instructions".into());
    }

    for original in originals {
        let candidate = named(prepared, &original["name"])?;
        let arguments = items(candidate, "arguments")?;
        let original_args = flattened_arguments(original, arguments, types, reached)?;
        let mut expected = primary_fields(&original_args, types, reached)?;

        if original["name"] == "multisigSetConfigAuthority" {
            expected[0][0] = json!("newConfigAuthority");
        }

        equal(
            &json!(expected),
            &json!(model_fields(&arguments[1..], definitions)?),
            "instruction payload",
        )?;
        let prefix = if original["name"] == "vaultTransactionCreateFromBuffer" {
            "vaultTransactionCreate"
        } else {
            ""
        };

        equal(
            &json!(primary_roles(items(original, "accounts")?, prefix)?),
            &candidate["accounts"],
            "instruction role order and privileges",
        )?;
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
        && arguments[0]["name"] == "args"
        && prepared
            .get(1)
            .is_some_and(|argument| argument["name"] != "args")
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

fn primary_roles(nodes: &[Value], prefix: &str) -> Result<Vec<Value>, Error> {
    let mut output = Vec::new();

    for node in nodes {
        if let Some(group) = node["accounts"].as_array() {
            let mut children = primary_roles(group, "")?;

            prefix_roles(&mut children, prefix)?;

            output.extend(children);
            continue;
        }

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

fn prefix_roles(children: &mut [Value], prefix: &str) -> Result<(), Error> {
    if prefix.is_empty() {
        return Ok(());
    }

    for child in children {
        let name = child["name"].as_str().ok_or("missing group role name")?;
        let mut suffix = name.to_owned();
        suffix[0..1].make_ascii_uppercase();
        child["name"] = json!(format!("{prefix}{suffix}"));
    }

    Ok(())
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
                field["name"],
                primary_wire(&field["type"], types, reached)?
            ]))
        })
        .collect()
}

fn model_fields(nodes: &[Value], types: &[Value]) -> Result<Vec<Value>, Error> {
    nodes
        .iter()
        .map(|field| Ok(json!([field["name"], model_wire(&field["type"], types)?])))
        .collect()
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

fn primary_primitive(name: &str) -> Result<Value, Error> {
    match name {
        "publicKey" => Ok(json!("address")),
        "string" => Ok(json!("text")),
        "bytes" => Ok(json!(["array", "u8", "u32"])),
        "bool" | "u8" | "u16" | "u32" | "u64" | "i64" => Ok(json!(name)),
        _ => Err("uncompared primary primitive".into()),
    }
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

    Ok(json!([
        variant["name"],
        primary_fields(&fields, types, reached)?
    ]))
}

/// Preserve all source framing instead of comparing only the contained Rust value type.
fn model_wire(node: &Value, types: &[Value]) -> Result<Value, Error> {
    match node["kind"].as_str() {
        Some("numberTypeNode") => Ok(node["format"].clone()),
        Some("publicKeyTypeNode") => Ok(json!("address")),
        Some("booleanTypeNode") => Ok(json!("bool")),
        Some("definedTypeLinkNode") => model_shape(&named(types, &node["name"])?["type"], types),
        Some("optionTypeNode") => Ok(json!(["option", model_wire(&node["item"], types)?])),
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
        Some("enumEmptyVariantTypeNode") => Vec::new(),
        Some("enumStructVariantTypeNode") => {
            model_fields(items(&variant["struct"], "fields")?, types)?
        }
        _ => return Err("uncompared model enum payload".into()),
    };

    Ok(json!([variant["name"], fields]))
}
