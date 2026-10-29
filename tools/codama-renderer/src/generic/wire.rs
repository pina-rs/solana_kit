//! Closed wire families and explicit layout contracts shared by every program.
use super::{
    CountPrefix, Numeric, Wire,
    parse::{keys, name},
};
use crate::schema::Error;
use serde_json::Value;
const MAX_HIDDEN_PADDING: usize = 1024;

pub(super) fn number(node: &Value) -> Result<Numeric, Error> {
    Numeric::parse(node)
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
        Some("fixedSizeTypeNode") => fixed_bytes(node),
        Some("zeroableOptionTypeNode") => zeroable(node),
        Some("arrayTypeNode") => array(node),
        Some("sizePrefixTypeNode") => prefix(node),
        Some("amountTypeNode") => amount(node),
        Some("remainderOptionTypeNode") => {
            keys(node, &["kind", "item"])?;
            Ok(Wire::RemainderOption(Box::new(parse(&node["item"])?)))
        }
        Some("hiddenPrefixTypeNode") => hidden(node),
        Some("mapTypeNode") => map(node),
        Some("tupleTypeNode") => tuple(node),
        _ => Err(format!("unsupported wire node {}", node["kind"]).into()),
    }
}

fn amount(node: &Value) -> Result<Wire, Error> {
    keys(node, &["kind", "number", "decimals", "unit"])?;
    node["decimals"].as_u64().ok_or("invalid amount decimals")?;
    node["unit"].as_str().ok_or("invalid amount unit")?;
    Ok(Wire::Number(number(&node["number"])?))
}

fn tuple(node: &Value) -> Result<Wire, Error> {
    keys(node, &["kind", "items"])?;
    let items = node["items"].as_array().ok_or("missing tuple items")?;

    if items.len() != 2 {
        return Err("only paired tuples supported".into());
    }

    Ok(Wire::Tuple(
        Box::new(parse(&items[0])?),
        Box::new(parse(&items[1])?),
    ))
}
fn boolean(node: &Value) -> Result<Wire, Error> {
    keys(node, &["kind", "size"])?;

    if number(&node["size"])?.format != "u8" {
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
    keys(node, &["kind", "item", "prefix", "fixed", "idlOption"])?;

    if node.get("idlOption").is_some_and(|value| value != "option") {
        return Err("unsupported IDL option".into());
    }

    let prefix = unsigned(&node["prefix"])?;

    let fixed = match node.get("fixed") {
        Some(value) => value.as_bool().ok_or("invalid fixed-option policy")?,
        None => false,
    };

    Ok(Wire::Option {
        item: Box::new(parse(&node["item"])?),
        prefix,
        fixed,
    })
}
fn array(node: &Value) -> Result<Wire, Error> {
    keys(node, &["kind", "item", "count"])?;

    if node["count"]["kind"] == "prefixedCountNode" {
        keys(&node["count"], &["kind", "prefix"])?;
        let prefix = &node["count"]["prefix"];
        let prefix = if prefix
            == &serde_json::json!({"kind":"numberTypeNode","format":"shortU16","endian":"le"})
        {
            CountPrefix::ShortU16
        } else {
            CountPrefix::Number(unsigned(prefix)?)
        };

        return Ok(Wire::Counted {
            item: Box::new(parse(&node["item"])?),
            prefix,
        });
    }

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
        prefix: unsigned(&node["prefix"])?,
    })
}

pub(super) fn unsigned(node: &Value) -> Result<Numeric, Error> {
    Numeric::unsigned(node)
}

fn map(node: &Value) -> Result<Wire, Error> {
    keys(node, &["kind", "key", "value", "count"])?;
    keys(&node["count"], &["kind", "prefix"])?;

    if node["count"]["kind"] != "prefixedCountNode" {
        return Err("map requires an explicit count prefix".into());
    }

    Ok(Wire::Counted {
        item: Box::new(Wire::Tuple(
            Box::new(parse(&node["key"])?),
            Box::new(parse(&node["value"])?),
        )),
        prefix: CountPrefix::Number(unsigned(&node["count"]["prefix"])?),
    })
}

fn hidden(node: &Value) -> Result<Wire, Error> {
    keys(node, &["kind", "type", "prefix"])?;
    let mut bytes = Vec::new();

    for constant in super::parse::array(node, "prefix")? {
        hidden_constant(constant, &mut bytes)?;
    }

    Ok(Wire::HiddenPrefix {
        item: Box::new(parse(&node["type"])?),
        bytes,
    })
}

fn hidden_constant(constant: &Value, bytes: &mut Vec<u8>) -> Result<(), Error> {
    keys(constant, &["kind", "type", "value"])?;
    keys(&constant["value"], &["kind", "number"])?;

    if constant["kind"] != "constantValueNode" || constant["value"]["kind"] != "numberValueNode" {
        return Err("hidden prefix requires literal numeric constants".into());
    }

    let mut binding = &constant["type"];

    if binding["kind"] == "preOffsetTypeNode" {
        hidden_padding(binding, bytes)?;
        binding = &binding["type"];
    }

    if number(binding)?.format != "u8" {
        return Err("hidden prefix constant requires u8".into());
    }

    bytes.push(u8::try_from(
        constant["value"]["number"]
            .as_u64()
            .ok_or("invalid hidden prefix constant")?,
    )?);
    Ok(())
}

fn hidden_padding(binding: &Value, bytes: &mut Vec<u8>) -> Result<(), Error> {
    keys(binding, &["kind", "type", "offset", "strategy"])?;

    if binding["strategy"] != "padded" {
        return Err("unsupported hidden prefix offset strategy".into());
    }

    let offset = usize::try_from(
        binding["offset"]
            .as_u64()
            .ok_or("invalid hidden prefix offset")?,
    )?;

    if offset > MAX_HIDDEN_PADDING {
        return Err("hidden prefix padding exceeds bounded schema budget".into());
    }

    bytes.resize(
        bytes
            .len()
            .checked_add(offset)
            .ok_or("hidden prefix overflow")?,
        0,
    );
    Ok(())
}

fn fixed_bytes(node: &Value) -> Result<Wire, Error> {
    keys(node, &["kind", "type", "size"])?;
    let size = node["size"].as_u64().ok_or("missing fixed byte width")?;
    u32::try_from(size).map_err(|_| "fixed bytes exceed portable bounds")?;
    let size = usize::try_from(size)?;

    if node["type"]["kind"] == "stringTypeNode" {
        text(&node["type"])?;

        return Ok(Wire::FixedText(size));
    }

    keys(&node["type"], &["kind"])?;

    if node["type"]["kind"] != "bytesTypeNode" {
        return Err("fixed byte binding requires a bytes node".into());
    }

    Ok(Wire::FixedBytes(size))
}

fn zeroable(node: &Value) -> Result<Wire, Error> {
    keys(node, &["kind", "item"])?;
    keys(&node["item"], &["kind"])?;

    if node["item"]["kind"] != "publicKeyTypeNode" {
        return Err("zeroable option requires an exact fixed address binding".into());
    }

    Ok(Wire::ZeroableAddress)
}
