//! Closed wire families and explicit layout contracts shared by every program.
use super::{
    Wire,
    parse::{keys, name},
};
use crate::schema::Error;
use serde_json::Value;

pub(super) fn number(node: &Value) -> Result<&'static str, Error> {
    keys(node, &["kind", "format", "endian", "display"])?;

    if node["kind"] != "numberTypeNode" || node["endian"] != "le" {
        return Err("unsupported integer encoding".into());
    }

    match node["format"].as_str() {
        Some("u8") => Ok("u8"),
        Some("u32") => Ok("u32"),
        Some("u64") => Ok("u64"),
        _ => Err("unsupported integer width".into()),
    }
}

/// The closed dispatch rejects unfamiliar kinds rather than choosing a fallback.
pub(super) fn parse(node: &Value) -> Result<Wire, Error> {
    match node["kind"].as_str() {
        Some("numberTypeNode") => Ok(Wire::Number(number(node)?)),
        Some("definedTypeLinkNode") => {
            keys(node, &["kind", "name"])?;
            Ok(Wire::Link(name(node)?))
        }
        Some("publicKeyTypeNode") => {
            keys(node, &["kind"])?;
            Ok(Wire::Address)
        }
        Some("booleanTypeNode") => boolean(node),
        Some("stringTypeNode") => text(node),
        Some("bytesTypeNode") => {
            keys(node, &["kind"])?;
            Ok(Wire::Bytes)
        }
        Some("optionTypeNode") => option(node),
        Some("arrayTypeNode") => array(node),
        Some("sizePrefixTypeNode") => prefix(node),
        _ => Err("unsupported wire node".into()),
    }
}
fn boolean(node: &Value) -> Result<Wire, Error> {
    keys(node, &["kind", "size"])?;

    if number(&node["size"])? != "u8" {
        return Err("unsupported boolean size".into());
    }

    Ok(Wire::Bool)
}
fn text(node: &Value) -> Result<Wire, Error> {
    keys(node, &["kind", "encoding"])?;

    if node["encoding"] != "utf8" {
        return Err("unsupported text encoding".into());
    }

    Ok(Wire::Text)
}
fn option(node: &Value) -> Result<Wire, Error> {
    keys(node, &["kind", "item", "prefix", "fixed"])?;
    let prefix = number(&node["prefix"])?;

    if !["u8", "u32"].contains(&prefix) {
        return Err("unsupported option tag".into());
    }

    let fixed = node["fixed"]
        .as_bool()
        .ok_or("missing fixed-option policy")?;

    if fixed && prefix != "u32" {
        return Err("unsupported fixed option tag width".into());
    }

    Ok(Wire::Option {
        item: Box::new(parse(&node["item"])?),
        prefix,
        fixed,
    })
}
fn array(node: &Value) -> Result<Wire, Error> {
    keys(node, &["kind", "item", "count"])?;
    Ok(Wire::Array {
        item: Box::new(parse(&node["item"])?),
        count: count(&node["count"])?,
    })
}
fn count(node: &Value) -> Result<Option<usize>, Error> {
    match node["kind"].as_str() {
        Some("fixedCountNode") => {
            keys(node, &["kind", "value"])?;
            Ok(Some(usize::try_from(
                node["value"].as_u64().ok_or("invalid array count")?,
            )?))
        }
        Some("remainderCountNode") => {
            keys(node, &["kind"])?;
            Ok(None)
        }
        _ => Err("unsupported array count".into()),
    }
}
fn prefix(node: &Value) -> Result<Wire, Error> {
    keys(node, &["kind", "type", "prefix"])?;
    Ok(Wire::Prefix {
        item: Box::new(parse(&node["type"])?),
        prefix: number(&node["prefix"])?,
    })
}
