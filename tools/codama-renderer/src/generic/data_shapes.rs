//! Named records and tagged payloads retain the original source framing.
use super::{Definition, Field, Shape, Variant, Wire, data, parse, wire};
use crate::schema::Error;
use serde_json::Value;
use solana_kit_core::codecs::Endian;
use std::collections::BTreeSet;

#[cfg(feature = "verification")]
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

    output.push(Definition {
        name,
        shape,
        discriminator: None,
    });
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
    let numeric = wire::unsigned(&node["size"])?;
    let prefix = numeric.format;
    let mut variants = Vec::new();
    let mut names = BTreeSet::new();

    if numeric.endian != Endian::Little || !["u8", "u16", "u32"].contains(&prefix) {
        return Err("unsupported enum tag width".into());
    }

    for (index, variant) in parse::array(node, "variants")?.iter().enumerate() {
        variant_discriminator(variant, index)?;
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

    let last = variants.len().checked_sub(1).ok_or("empty enum")?;

    let fits = match prefix {
        "u8" => u8::try_from(last).is_ok(),
        "u16" => u16::try_from(last).is_ok(),
        "u32" => u32::try_from(last).is_ok(),
        _ => false,
    };

    if !fits {
        return Err("invalid enum variant count".into());
    }

    Ok(Shape::Tagged { prefix, variants })
}

/// An explicit source tag must preserve the positional tag used by the wire enum.
fn variant_discriminator(variant: &Value, index: usize) -> Result<(), Error> {
    if let Some(value) = variant.get("discriminator")
        && value.as_u64() != Some(u64::try_from(index)?)
    {
        return Err("explicit enum discriminator must match its contiguous position".into());
    }

    Ok(())
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
            parse::keys(node, &["kind", "name", "docs", "discriminator"])?;
            Ok(None)
        }
        Some("enumStructVariantTypeNode") => {
            parse::keys(node, &["kind", "name", "docs", "struct"])?;
            Ok(Some(struct_payload(owner, name, &node["struct"], output)?))
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

fn struct_payload(
    owner: &str,
    variant: &str,
    node: &Value,
    output: &mut Vec<Definition>,
) -> Result<Wire, Error> {
    let (fields, prefix) = match node["kind"].as_str() {
        Some("structTypeNode") => (record(node)?, None),
        Some("sizePrefixTypeNode") => {
            parse::keys(node, &["kind", "type", "prefix"])?;
            (
                record(&node["type"])?,
                Some(wire::unsigned(&node["prefix"])?),
            )
        }
        _ => return Err("unsupported struct enum payload framing".into()),
    };

    let name = format!("{owner}{variant}Payload");
    output.push(Definition {
        name: name.clone(),
        shape: fields,
        discriminator: None,
    });
    let linked = Wire::Link(name);

    Ok(match prefix {
        Some(prefix) => Wire::Prefix {
            item: Box::new(linked),
            prefix,
        },
        None => linked,
    })
}
