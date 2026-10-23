//! Named records and tagged payloads retain the original source framing.
use super::{Definition, Field, Shape, Variant, Wire, data, parse, wire};
use crate::schema::Error;
use serde_json::Value;
use std::collections::BTreeSet;
const BYTE_VARIANTS: usize = 256;
const SHORT_VARIANTS: usize = 65536;

pub(super) fn tlv_tail(fields: &mut [Field]) -> Result<(), Error> {
    let field = fields
        .last_mut()
        .ok_or("missing extensible account fields")?;

    if field.name != "extensions" {
        return Err("extensible account requires final extension field".into());
    }

    let Wire::RemainderOption(tail) = &mut field.wire else {
        return Err("extension tail requires a remainder option".into());
    };
    let Wire::HiddenPrefix { item, .. } = tail.as_mut() else {
        return Err("extension tail requires its account-type prefix".into());
    };
    let Wire::Array {
        item: element,
        count: None,
    } = item.as_ref()
    else {
        return Err("extension tail requires a remainder array".into());
    };

    if !matches!(element.as_ref(), Wire::Link(name) if name == "Extension") {
        return Err("extension tail requires the source Extension enum".into());
    }

    **item = Wire::TlvSequence(element.clone());
    Ok(())
}

pub(super) fn definition(node: &Value, output: &mut Vec<Definition>) -> Result<(), Error> {
    parse::keys(node, &["kind", "name", "type", "docs"])?;

    if node["kind"] != "definedTypeNode" {
        return Err("unexpected defined type".into());
    }

    let name = parse::name(node)?;

    let shape = match node["type"]["kind"].as_str() {
        Some("structTypeNode") => record(&node["type"])?,
        Some("enumTypeNode") => enumeration(&name, &node["type"], output)?,
        _ => Shape::Struct(vec![Field {
            name: "value".to_owned(),
            wire: wire::parse(&node["type"])?,
        }]),
    };

    output.push(Definition { name, shape });
    Ok(())
}

fn record(node: &Value) -> Result<Shape, Error> {
    parse::keys(node, &["kind", "fields"])?;

    if node["kind"] != "structTypeNode" {
        return Err("enum payload must be a record".into());
    }

    Ok(Shape::Struct(data::fields(
        parse::array(node, "fields")?,
        "structFieldTypeNode",
    )?))
}

/// Unique names and bounded cardinality keep every emitted match arm distinct.
fn enumeration(name: &str, node: &Value, output: &mut Vec<Definition>) -> Result<Shape, Error> {
    parse::keys(node, &["kind", "size", "variants"])?;
    let prefix = wire::unsigned(&node["size"])?;
    let mut variants = Vec::new();
    let mut names = BTreeSet::new();

    if !["u8", "u16"].contains(&prefix) {
        return Err("unsupported enum tag width".into());
    }

    for variant in parse::array(node, "variants")? {
        let label = parse::name(variant)?;

        if !names.insert(label.clone()) {
            return Err("duplicate enum variant".into());
        }

        let payload = payload(name, &label, variant, output)?;
        variants.push(Variant {
            name: label,
            payload,
        });
    }

    let maximum = if prefix == "u8" {
        BYTE_VARIANTS
    } else {
        SHORT_VARIANTS
    };

    if variants.is_empty() || variants.len() > maximum {
        return Err("invalid enum variant count".into());
    }

    Ok(Shape::Tagged { prefix, variants })
}

/// Source variants own their individual size framing, including empty records.
fn payload(
    owner: &str,
    name: &str,
    node: &Value,
    output: &mut Vec<Definition>,
) -> Result<Option<Wire>, Error> {
    match node["kind"].as_str() {
        Some("enumEmptyVariantTypeNode") => {
            parse::keys(node, &["kind", "name", "docs"])?;
            Ok(None)
        }
        Some("enumStructVariantTypeNode") => {
            parse::keys(node, &["kind", "name", "docs", "struct"])?;
            let wrapped = &node["struct"];
            parse::keys(wrapped, &["kind", "type", "prefix"])?;

            if wrapped["kind"] != "sizePrefixTypeNode" {
                return Err("struct enum payload requires explicit size framing".into());
            }

            let name = format!("{owner}{name}Payload");
            output.push(Definition {
                name: name.clone(),
                shape: record(&wrapped["type"])?,
            });
            Ok(Some(Wire::Prefix {
                item: Box::new(Wire::Link(name)),
                prefix: wire::unsigned(&wrapped["prefix"])?,
            }))
        }
        Some("enumTupleVariantTypeNode") => {
            parse::keys(node, &["kind", "name", "docs", "tuple"])?;
            let tuple = &node["tuple"];
            parse::keys(tuple, &["kind", "items"])?;
            let items = parse::array(tuple, "items")?;

            if tuple["kind"] != "tupleTypeNode" || items.len() != 1 {
                return Err("enum tuple requires one explicit item".into());
            }

            Ok(Some(wire::parse(&items[0])?))
        }
        _ => Err("unsupported enum variant".into()),
    }
}
