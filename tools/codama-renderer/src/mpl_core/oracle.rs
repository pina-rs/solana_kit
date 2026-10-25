//! Read-only independent wire equality against the complete private pinned primary input.
use crate::schema::{Error, identifier};
use serde_json::{Value, json};
use std::{collections::BTreeSet, fs, path::Path};

const TYPE_COUNT: usize = 116;
const INSTRUCTION_COUNT: usize = 42;
const ACCOUNT_COUNT: usize = 6;
const ERROR_COUNT: usize = 57;

/// Every primary type must be compared independently; a matching selected subset is insufficient.
pub(super) fn audit(path: &Path) -> Result<(), Error> {
    if crate::inputs::hash(path)?
        != "ba75e3dbc1c332f63ddfd0ad4224590c725559679c5aa84eb8c43b6067bd3471"
    {
        return Err("private Core primary pin drift".into());
    }

    let primary: Value = serde_json::from_slice(&fs::read(path)?)?;
    let model = super::source()?;
    let model = &model["program"];

    if primary["name"] != "mpl_core_program" || primary["metadata"]["address"] != super::PROGRAM {
        return Err("private Core primary identity drift".into());
    }

    let mut compared = BTreeSet::new();
    instructions(&primary, model, &mut compared)?;
    definitions(&primary, model, &mut compared)?;
    accounts(&primary, model)?;
    errors(&primary, model)?;
    let actual: BTreeSet<_> = array(&primary["types"])?
        .iter()
        .map(|value| text(&value["name"]).map(str::to_owned))
        .collect::<Result<_, _>>()?;

    if compared != actual || actual.len() != TYPE_COUNT {
        return Err("private Core type without an independently compared contract".into());
    }

    println!(
        "private pinned Core primary matches42 instructions,6 accounts,116 types and57 errors"
    );
    Ok(())
}

fn array(value: &Value) -> Result<&[Value], Error> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or("primary array required".into())
}
fn text(value: &Value) -> Result<&str, Error> {
    value.as_str().ok_or("primary string required".into())
}
fn named<'a>(values: &'a Value, name: &str) -> Result<&'a Value, Error> {
    array(values)?
        .iter()
        .find(|value| value["name"] == name)
        .ok_or("primary named contract missing".into())
}
fn equal(left: Value, right: Value, label: &str) -> Result<(), Error> {
    if left != right {
        return Err(format!("private Core {label} wire contract differs").into());
    }

    Ok(())
}

fn instructions(
    primary: &Value,
    model: &Value,
    compared: &mut BTreeSet<String>,
) -> Result<(), Error> {
    let source = array(&primary["instructions"])?;
    let rust = array(&model["instructions"])?;

    if source.len() != INSTRUCTION_COUNT || rust.len() != INSTRUCTION_COUNT {
        return Err("Core instruction count drift".into());
    }

    for (source, rust) in source.iter().zip(rust) {
        if identifier(text(&source["name"])?)? != identifier(text(&rust["name"])?)?
            || source["discriminant"]["type"] != "u8"
            || source["discriminant"]["value"] != rust["arguments"][0]["defaultValue"]["number"]
        {
            return Err("Core instruction name or literal tag differs".into());
        }

        ordered_roles(source, rust)?;
        let source_fields = instruction_fields(primary, source, compared)?;
        let rust_fields = model_fields(&Value::Array(array(&rust["arguments"])?[1..].to_vec()))?;
        equal(
            json!(source_fields),
            json!(rust_fields),
            "instruction payload",
        )?;
    }

    Ok(())
}

fn ordered_roles(source: &Value, rust: &Value) -> Result<(), Error> {
    let source_roles = array(&source["accounts"])?;
    let rust_roles = array(&rust["accounts"])?;

    if source_roles.len() != rust_roles.len() {
        return Err("Core role count differs".into());
    }

    for (source, rust) in source_roles.iter().zip(rust_roles) {
        equal(
            json!([
                identifier(text(&source["name"])?)?,
                source["isMut"],
                source["isSigner"],
                source.get("isOptional").unwrap_or(&Value::Bool(false))
            ]),
            json!([
                identifier(text(&rust["name"])?)?,
                rust["isWritable"],
                rust["isSigner"],
                rust["isOptional"]
            ]),
            "ordered account privileges",
        )?;
    }

    Ok(())
}

fn instruction_fields(
    primary: &Value,
    source: &Value,
    compared: &mut BTreeSet<String>,
) -> Result<Vec<Value>, Error> {
    let args = array(&source["args"])?;
    Ok(match args {
        [] => Vec::new(),
        [one] => {
            let name = text(&one["type"]["defined"])?;
            compared.insert(name.to_owned());
            let definition = named(&primary["types"], name)?;
            primary_fields(&definition["type"]["fields"])?
        }

        _ => return Err("Core arguments require a complete named record".into()),
    })
}

fn definitions(
    primary: &Value,
    model: &Value,
    compared: &mut BTreeSet<String>,
) -> Result<(), Error> {
    for rust in array(&model["definedTypes"])? {
        let name = text(&rust["name"])?;

        if name == "LifecycleCheck" {
            continue;
        }

        let source = named(&primary["types"], name)?;
        equal(
            primary_shape(&source["type"])?,
            model_shape(&rust["type"])?,
            name,
        )?;
        compared.insert(name.to_owned());
    }

    Ok(())
}

fn accounts(primary: &Value, model: &Value) -> Result<(), Error> {
    let source = array(&primary["accounts"])?;
    let rust = array(&model["accounts"])?;

    if source.len() != ACCOUNT_COUNT || rust.len() != ACCOUNT_COUNT {
        return Err("Core account count drift".into());
    }

    for rust in rust {
        let name = text(&rust["name"])?;
        let source = named(&primary["accounts"], name)?;
        let fields = array(&source["type"]["fields"])?;

        if fields
            .first()
            .is_none_or(|field| field["name"] != "key" || field["type"]["defined"] != "Key")
        {
            return Err("Core account key contract differs".into());
        }

        equal(
            json!(primary_fields(&Value::Array(fields[1..].to_vec()))?),
            json!(model_fields(&rust["data"]["fields"])?),
            name,
        )?;
    }

    Ok(())
}

fn errors(primary: &Value, model: &Value) -> Result<(), Error> {
    let source = array(&primary["errors"])?;
    let rust = array(&model["errors"])?;

    if source.len() != ERROR_COUNT || rust.len() != ERROR_COUNT {
        return Err("Core error count drift".into());
    }

    for (source, rust) in source.iter().zip(rust) {
        equal(
            json!([source["name"], source["code"], source["msg"]]),
            json!([rust["name"], rust["code"], rust["message"]]),
            "error",
        )?;
    }

    Ok(())
}

fn primary_fields(value: &Value) -> Result<Vec<Value>, Error> {
    array(value)?
        .iter()
        .map(|field| {
            let name = text(&field["name"])?;
            let mut wire = field["type"].clone();

            if name == "relationships" && wire == json!({"vec":{"defined":"crate"}}) {
                wire = json!({"vec":{"defined":"RelationshipEntry"}});
            }

            Ok(json!([identifier(name)?, primary_wire(&wire)?]))
        })
        .collect()
}
fn model_fields(value: &Value) -> Result<Vec<Value>, Error> {
    array(value)?
        .iter()
        .map(|field| {
            Ok(json!([
                identifier(text(&field["name"])?)?,
                model_wire(&field["type"])?
            ]))
        })
        .collect()
}

fn primary_shape(value: &Value) -> Result<Value, Error> {
    match value["kind"].as_str() {
        Some("struct") => Ok(json!(["record", primary_fields(&value["fields"])?])),
        Some("enum") => {
            let variants = array(&value["variants"])?
                .iter()
                .map(|variant| {
                    Ok(json!([
                        variant["name"],
                        primary_payload(variant.get("fields"))?
                    ]))
                })
                .collect::<Result<Vec<_>, Error>>()?;
            Ok(json!(["enum", variants]))
        }
        _ => Err("unsupported primary shape".into()),
    }
}

fn primary_payload(fields: Option<&Value>) -> Result<Value, Error> {
    let Some(fields) = fields else {
        return Ok(json!(["unit"]));
    };
    let items = array(fields)?;

    if items
        .first()
        .is_some_and(|field| field.get("name").is_some())
    {
        return Ok(json!(["record", primary_fields(fields)?]));
    }

    if items.len() != 1 {
        return Err("primary enum tuple arity differs".into());
    }

    Ok(json!(["tuple", primary_wire(&items[0])?]))
}
fn model_shape(value: &Value) -> Result<Value, Error> {
    if value["kind"] == "structTypeNode" {
        return Ok(json!(["record", model_fields(&value["fields"])?]));
    }

    if value["kind"] != "enumTypeNode" {
        return Err("unsupported model shape".into());
    }

    let variants = array(&value["variants"])?
        .iter()
        .map(|variant| {
            let payload = match variant["kind"].as_str() {
                Some("enumEmptyVariantTypeNode") => json!(["unit"]),
                Some("enumTupleVariantTypeNode") => {
                    json!(["tuple", model_wire(&variant["tuple"]["items"][0])?])
                }
                Some("enumStructVariantTypeNode") => {
                    json!(["record", model_fields(&variant["struct"]["fields"])?])
                }

                _ => return Err("unsupported model enum payload".into()),
            };

            Ok(json!([variant["name"], payload]))
        })
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(json!(["enum", variants]))
}

fn primary_wire(value: &Value) -> Result<Value, Error> {
    if let Some(name) = value.as_str() {
        return Ok(match name {
            "publicKey" => json!("address"),
            "string" => json!("text"),
            "bytes" => json!(["vector", "u8"]),
            "u8" | "u16" | "u32" | "u64" | "bool" => json!(name),

            _ => return Err("unsupported private primary primitive".into()),
        });
    }

    let object = value
        .as_object()
        .ok_or("private primary wire object required")?;

    if object.len() != 1 {
        return Err("ambiguous private primary wire binding".into());
    }

    let (kind, item) = object
        .iter()
        .next()
        .ok_or("missing private primary binding")?;

    match kind.as_str() {
        "defined" => Ok(json!(["link", text(item)?])),
        "option" => Ok(json!(["option", primary_wire(item)?])),
        "vec" => Ok(json!(["vector", primary_wire(item)?])),
        "array" => Ok(json!(["fixed", primary_wire(&item[0])?, item[1]])),
        "tuple" => Ok(json!([
            "pair",
            array(item)?
                .iter()
                .map(primary_wire)
                .collect::<Result<Vec<_>, _>>()?
        ])),
        _ => Err("unsupported private primary compound type".into()),
    }
}
/// Preserve container framing and the source's single LifecycleCheck tuple alias.
fn model_wire(value: &Value) -> Result<Value, Error> {
    match value["kind"].as_str() {
        Some("numberTypeNode") => Ok(value["format"].clone()),
        Some("publicKeyTypeNode") => Ok(json!("address")),
        Some("booleanTypeNode") => Ok(json!("bool")),
        Some("definedTypeLinkNode") if value["name"] == "LifecycleCheck" => Ok(json!([
            "pair",
            [
                ["link", "HookableLifecycleEvent"],
                ["link", "ExternalCheckResult"]
            ]
        ])),
        Some("definedTypeLinkNode") => Ok(json!(["link", value["name"]])),
        Some("optionTypeNode") => Ok(json!(["option", model_wire(&value["item"])?])),

        Some("arrayTypeNode") if value["count"]["kind"] == "prefixedCountNode" => {
            Ok(json!(["vector", model_wire(&value["item"])?]))
        }

        Some("fixedSizeTypeNode") => Ok(json!(["fixed", "u8", value["size"]])),

        Some("sizePrefixTypeNode") if value["type"]["kind"] == "stringTypeNode" => {
            Ok(json!("text"))
        }

        _ => Err("unsupported model wire comparison".into()),
    }
}
