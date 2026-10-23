//! IDL structure checks and bounded raw JSON traversal.

use crate::{
    CustomError, Definition, Error, Field, Idl, Instruction, Limits, NamedDiscriminator, Variant,
    type_name,
};
use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use serde_json::{Map, Value as Json};

pub(crate) fn budget(value: &Json, limits: Limits) -> Result<(), Error> {
    let mut pending = vec![(value, 0)];

    let mut items = limits.max_items;

    let mut bytes = limits.max_bytes;

    while let Some((node, depth)) = pending.pop() {
        limits.depth(depth)?;
        items = items.checked_sub(1).ok_or(Error::Limit)?;

        budget_node(node, depth, items, &mut bytes, &mut pending)?;
    }

    Ok(())
}

fn budget_node<'a>(
    node: &'a Json,
    depth: usize,
    items: usize,
    bytes: &mut usize,
    pending: &mut Vec<(&'a Json, usize)>,
) -> Result<(), Error> {
    match node {
        Json::String(text) => *bytes = bytes.checked_sub(text.len()).ok_or(Error::Limit)?,
        Json::Array(list) => {
            check_count(list.len(), items)?;
            pending.extend(list.iter().map(|child| (child, depth + 1)));
        }
        Json::Object(map) => {
            check_count(map.len(), items)?;

            for (key, child) in map {
                *bytes = bytes.checked_sub(key.len()).ok_or(Error::Limit)?;
                pending.push((child, depth + 1));
            }
        }
        _ => {}
    }

    Ok(())
}

fn check_count(count: usize, maximum: usize) -> Result<(), Error> {
    if count > maximum {
        return Err(Error::Limit);
    }

    Ok(())
}

pub(crate) fn object(value: &Json) -> Result<&Map<String, Json>, Error> {
    value.as_object().ok_or(Error::Idl("expected an object"))
}

fn name(value: &Json) -> Result<String, Error> {
    value
        .get("name")
        .and_then(Json::as_str)
        .filter(|name| !name.is_empty())
        .map(ToString::to_string)
        .ok_or(Error::Idl("missing entry name"))
}

fn optional_text<'a>(value: &'a Json, key: &str) -> Result<Option<&'a str>, Error> {
    match value.get(key) {
        None | Some(Json::Null) => Ok(None),
        Some(Json::String(text)) => Ok(Some(text)),
        _ => Err(Error::Idl("expected a string field")),
    }
}

fn list<'a>(value: &'a Json, key: &str) -> Result<&'a [Json], Error> {
    match value.get(key) {
        None | Some(Json::Null) => Ok(&[]),
        Some(Json::Array(list)) => Ok(list),
        _ => Err(Error::Idl("expected an entry array")),
    }
}

fn discriminator(value: &Json) -> Result<Vec<u8>, Error> {
    list(value, "discriminator")?
        .iter()
        .map(|byte| {
            byte.as_u64()
                .and_then(|number| u8::try_from(number).ok())
                .ok_or(Error::Idl("discriminator byte out of range"))
        })
        .collect()
}

pub(crate) fn field(value: &Json, index: usize) -> Result<Field, Error> {
    let fallback = || format!("field{index}");

    if let Some(raw_type) = value.get("type") {
        if raw_type.is_null() {
            return Err(Error::Idl("missing field type"));
        }

        return Ok(Field {
            name: optional_text(value, "name")?
                .map(ToString::to_string)
                .unwrap_or_else(fallback),
            raw_type: raw_type.clone(),
        });
    }

    Ok(Field {
        name: fallback(),
        raw_type: value.clone(),
    })
}

fn fields(value: Option<&Json>) -> Result<Vec<Field>, Error> {
    let Some(Json::Array(list)) = value else {
        return Ok(Vec::new());
    };

    list.iter()
        .enumerate()
        .map(|(index, value)| field(value, index))
        .collect()
}

pub(crate) fn variant(value: &Json) -> Result<Variant, Error> {
    object(value)?;

    let name = optional_text(value, "name")?
        .unwrap_or("variant")
        .to_string();

    let fields = match value.get("fields") {
        None | Some(Json::Null) => Vec::new(),
        Some(Json::Array(_)) => fields(value.get("fields"))?,
        Some(other) => vec![field(other, 0)?],
    };

    Ok(Variant { name, fields })
}

fn definition(value: &Json) -> Result<Definition, Error> {
    let body = value.get("type").ok_or(Error::Idl("missing type body"))?;

    match body.get("kind").and_then(Json::as_str) {
        Some("struct") => Ok(Definition::Struct(fields(body.get("fields"))?)),
        Some("enum") => Ok(Definition::Enum(
            list(body, "variants")?
                .iter()
                .map(variant)
                .collect::<Result<_, _>>()?,
        )),
        _ => Err(Error::Idl("unsupported defined type kind")),
    }
}

fn named(value: &Json) -> Result<NamedDiscriminator, Error> {
    object(value)?;

    Ok(NamedDiscriminator {
        name: optional_text(value, "name")?.unwrap_or("").to_string(),
        discriminator: discriminator(value)?,
    })
}

pub(crate) fn document(value: &Json, limits: Limits) -> Result<Idl, Error> {
    budget(value, limits)?;
    object(value)?;

    let mut idl = Idl {
        address: optional_text(value, "address")?.unwrap_or("").to_string(),
        name: value
            .get("metadata")
            .and_then(|metadata| metadata.get("name"))
            .and_then(Json::as_str)
            .unwrap_or("")
            .to_string(),
        instructions: BTreeMap::new(),
        accounts: BTreeMap::new(),
        events: Vec::new(),
        errors: BTreeMap::new(),
        types: BTreeMap::new(),
    };

    parse_instructions(value, &mut idl)?;
    parse_named(value, &mut idl)?;

    for node in list(value, "types")? {
        let name = name(node)?;
        idl.types
            .insert(type_name(&name).to_string(), definition(node)?);
    }

    parse_errors(value, &mut idl)?;

    Ok(idl)
}

fn parse_instructions(value: &Json, idl: &mut Idl) -> Result<(), Error> {
    for node in list(value, "instructions")? {
        object(node)?;
        let name = name(node)?;
        let instruction = Instruction {
            name: name.clone(),
            discriminator: discriminator(node)?,
            args: fields(node.get("args"))?,
        };
        idl.instructions.insert(name, instruction);
    }

    Ok(())
}

fn parse_errors(value: &Json, idl: &mut Idl) -> Result<(), Error> {
    for node in list(value, "errors")? {
        let code = node
            .get("code")
            .and_then(Json::as_i64)
            .ok_or(Error::Idl("missing signed 64-bit error code"))?;
        let error = CustomError {
            code,
            name: name(node)?,
            message: optional_text(node, "msg")?.map(ToString::to_string),
        };
        idl.errors.insert(code, error);
    }

    Ok(())
}

fn parse_named(value: &Json, idl: &mut Idl) -> Result<(), Error> {
    for node in list(value, "accounts")? {
        let entry = named(node)?;
        idl.accounts.insert(entry.name.clone(), entry);
    }

    for node in list(value, "events")? {
        let entry = named(node)?;

        if let Some(previous) = idl.events.iter_mut().find(|old| old.name == entry.name) {
            *previous = entry;
        } else {
            idl.events.push(entry);
        }
    }

    Ok(())
}
