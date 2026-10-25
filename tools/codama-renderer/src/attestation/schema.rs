//! Closed source vocabulary for the actual Shank/Dart layouts.
use crate::schema::{Error, identifier};
use serde_json::{Value, json};

pub(super) fn number(name: &str) -> Value {
    json!({"kind":"numberTypeNode","format":name,"endian":"le"})
}

fn wire(name: &str) -> Result<Value, Error> {
    if let Some(item) = name.strip_prefix("[]") {
        return Ok(json!({"kind":"arrayTypeNode","item":wire(item)?,
            "count":{"kind":"prefixedCountNode","prefix":number("u32")}}));
    }

    match name {
        "u8" | "u16" | "u64" | "i64" => Ok(number(name)),
        "address" => Ok(json!({"kind":"publicKeyTypeNode"})),
        "bool" => Ok(json!({"kind":"booleanTypeNode","size":number("u8")})),
        "bytes" => wire("[]u8"),
        "text" => Ok(json!({"kind":"sizePrefixTypeNode","prefix":number("u32"),
            "type":{"kind":"stringTypeNode","encoding":"utf8"}})),
        "SchemaDataType" => Ok(json!({"kind":"definedTypeLinkNode","name":name})),
        _ => Err("unknown Attestation source wire name".into()),
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

pub(super) fn header(tag: u8) -> Value {
    json!([{"kind":"constantDiscriminatorNode","offset":0,
        "constant":{"kind":"constantValueNode","type":{"kind":"bytesTypeNode"},
            "value":{"kind":"bytesValueNode","encoding":"base16","data":format!("{tag:02x}")}}}])
}

pub(super) fn roles(source: &str) -> Result<Vec<Value>, Error> {
    source
        .split(',')
        .map(|role| {
            let (name, flags) = role.split_once(':').ok_or("missing source role flags")?;
            identifier(name)?;

            if !["r", "w", "s", "ws"].contains(&flags) {
                return Err("unknown Attestation role policy".into());
            }

            Ok(
                json!({"kind":"instructionAccountNode","name":name,"isOptional":false,
            "isSigner":flags.contains('s'),"isWritable":flags.contains('w')}),
            )
        })
        .collect()
}
