//! Source-table lowering reuses the common Codama model and its closed validation.
use crate::schema::{Error, identifier};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

pub(super) fn number(name: &str) -> Value {
    json!({"kind":"numberTypeNode","format":name,"endian":"le"})
}

/// This small source vocabulary is not a second wire codec or emitter.
pub(super) fn wire(name: &str) -> Result<Value, Error> {
    if let Some(item) = name.strip_prefix('?') {
        return Ok(json!({"kind":"optionTypeNode","item":wire(item)?,"prefix":number("u8")}));
    }

    if let Some(item) = name.strip_prefix("[]") {
        return Ok(json!({"kind":"arrayTypeNode","item":wire(item)?,
            "count":{"kind":"prefixedCountNode","prefix":number("u32")}}));
    }

    match name {
        "u8" | "u16" | "u32" | "u64" | "i64" => Ok(number(name)),
        "address" => Ok(json!({"kind":"publicKeyTypeNode"})),
        "bool" => Ok(json!({"kind":"booleanTypeNode","size":number("u8")})),
        "bytes" => wire("[]u8"),
        "text" => Ok(json!({"kind":"sizePrefixTypeNode","prefix":number("u32"),
            "type":{"kind":"stringTypeNode","encoding":"utf8"}})),
        "fixed32" | "fixed64" => Ok(json!({"kind":"fixedSizeTypeNode",
            "size":if name == "fixed32" {32} else {64},"type":{"kind":"bytesTypeNode"}})),
        _ => {
            identifier(name)?;

            if !name.chars().next().is_some_and(char::is_uppercase) {
                return Err("unknown Bubblegum source-table wire name".into());
            }

            Ok(json!({"kind":"definedTypeLinkNode","name":name}))
        }
    }
}

pub(super) fn fields(source: &str, kind: &str) -> Result<Vec<Value>, Error> {
    if source.is_empty() {
        return Ok(Vec::new());
    }

    source
        .split(',')
        .map(|field| {
            let (name, ty) = field.split_once(':').ok_or("missing source field type")?;
            identifier(name)?;

            Ok(json!({"kind":kind,"name":name,"type":wire(ty)?}))
        })
        .collect()
}

pub(super) fn record(source: &str) -> Result<Value, Error> {
    Ok(json!({"kind":"structTypeNode","fields":fields(source,"structFieldTypeNode")?}))
}

/// Anchor's hash domain is independently checked against MIT Dart byte literals.
pub(super) fn discriminator(namespace: &str, name: &str) -> String {
    let bytes = Sha256::digest(format!("{namespace}:{name}"));
    bytes[..8]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub(super) fn constant_header(header: &str) -> Value {
    json!([{"kind":"constantDiscriminatorNode","offset":0,
        "constant":{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},
            "value":{"kind":"bytesValueNode","encoding":"base16","data":header}}}])
}

pub(super) fn roles(source: &str) -> Result<Vec<Value>, Error> {
    if source.is_empty() {
        return Ok(Vec::new());
    }

    source
        .split(',')
        .map(|role| {
            let (name, flags) = role.split_once(':').ok_or("missing source role flags")?;
            identifier(name)?;
            let optional = flags.ends_with('?');
            let flags = flags.strip_suffix('?').unwrap_or(flags);

            if !["r", "w", "s", "ws", "e"].contains(&flags) {
                return Err("unknown Bubblegum source role flags".into());
            }

            let signer = if flags == "e" {
                json!("either")
            } else {
                json!(flags.contains('s'))
            };

            Ok(json!({"kind":"instructionAccountNode","name":name,
            "isOptional":optional,"isSigner":signer,"isWritable":flags.contains('w')}))
        })
        .collect()
}
