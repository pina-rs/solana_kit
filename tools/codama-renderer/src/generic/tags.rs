//! Exact fixed byte-sequence discriminator headers for typed data projection.
use super::{parse, wire};
use crate::schema::Error;
use serde_json::Value;
use solana_kit_core::codecs::Endian;
const MAX_HEADER_BYTES: usize = 32;
const HEX_RADIX: u32 = 16;
#[cfg(all(test, feature = "verification"))]
const WRONG_METADATA_WIDTH: usize = 7;

/// Returns every literal header byte and the number of omitted constant fields.
pub(super) fn parse(node: &Value, fields: &[Value]) -> Result<(Option<Vec<u8>>, usize), Error> {
    let Some(discriminators) = node.get("discriminators") else {
        return Ok((None, 0));
    };
    let discriminators = discriminators
        .as_array()
        .ok_or("invalid discriminator list")?;

    if discriminators.is_empty() {
        return Err("empty explicit discriminator list".into());
    }

    let mut bytes = Vec::new();

    for (index, discriminator) in discriminators.iter().enumerate() {
        let field = fields.get(index).ok_or("missing discriminator field")?;
        validate_field(discriminator, field, bytes.len())?;
        bytes.extend(literal(field)?);
    }

    if bytes.is_empty() || bytes.len() > MAX_HEADER_BYTES {
        return Err("discriminator header exceeds bounded byte domain".into());
    }

    Ok((Some(bytes), discriminators.len()))
}

fn validate_field(discriminator: &Value, field: &Value, offset: usize) -> Result<(), Error> {
    parse::keys(discriminator, &["kind", "name", "offset"])?;
    parse::keys(
        field,
        &[
            "kind",
            "display",
            "name",
            "type",
            "docs",
            "defaultValue",
            "defaultValueStrategy",
        ],
    )?;

    if discriminator["kind"] != "fieldDiscriminatorNode"
        || !["instructionArgumentNode", "structFieldTypeNode"]
            .iter()
            .any(|kind| field["kind"] == *kind)
        || discriminator["name"] != field["name"]
        || discriminator["offset"].as_u64() != Some(u64::try_from(offset)?)
        || field["defaultValueStrategy"] != "omitted"
    {
        return Err("noncontiguous or nonconstant discriminator header".into());
    }
    Ok(())
}

fn literal(field: &Value) -> Result<Vec<u8>, Error> {
    let value = &field["defaultValue"];

    if field["type"]["kind"] == "fixedSizeTypeNode" {
        return byte_literal(field);
    }

    parse::keys(value, &["kind", "number"])?;

    if value["kind"] != "numberValueNode" {
        return Err("numeric discriminator requires a numeric literal".into());
    }

    let value = value["number"]
        .as_u64()
        .ok_or("invalid numeric discriminator")?;

    let number = wire::number(&field["type"])?;

    match (number.format, number.endian) {
        ("u8", _) => Ok(vec![u8::try_from(value)?]),
        ("u32", Endian::Little) => Ok(u32::try_from(value)?.to_le_bytes().to_vec()),
        ("u32", Endian::Big) => Ok(u32::try_from(value)?.to_be_bytes().to_vec()),
        _ => Err("unsupported numeric discriminator width".into()),
    }
}

fn byte_literal(field: &Value) -> Result<Vec<u8>, Error> {
    let value = &field["defaultValue"];
    parse::keys(value, &["kind", "data", "encoding"])?;
    let super::Wire::FixedBytes(size) = wire::parse(&field["type"])? else {
        return Err("invalid fixed discriminator binding".into());
    };

    if value["kind"] != "bytesValueNode" || value["encoding"] != "base16" {
        return Err("fixed discriminator requires base16 bytes".into());
    }

    let text = value["data"]
        .as_str()
        .ok_or("missing discriminator bytes")?;

    if text.len() != size.checked_mul(2).ok_or("discriminator width overflow")? || !text.is_ascii()
    {
        return Err("discriminator byte width mismatch".into());
    }

    (0..size)
        .map(|index| {
            u8::from_str_radix(&text[index * 2..index * 2 + 2], HEX_RADIX).map_err(Into::into)
        })
        .collect()
}

#[cfg(all(test, feature = "verification"))]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn all_pinned_token_2022_headers_are_exact_unique_and_prefix_free() {
        let root: Value =
            serde_json::from_str(include_str!("../../fixtures/token-2022-original.json")).unwrap();
        let mut headers = BTreeSet::new();

        for item in root["program"]["instructions"].as_array().unwrap() {
            let (header, fields) = parse(item, item["arguments"].as_array().unwrap()).unwrap();
            let header = header.unwrap();
            assert!([1, 2, 8].contains(&header.len()));
            assert!((1..=2).contains(&fields));
            assert!(headers.insert(header));
        }

        assert_eq!(headers.len(), 100);

        for a in &headers {
            for b in &headers {
                assert!(a == b || !a.starts_with(b));
            }
        }
    }

    #[test]
    fn header_offsets_bytes_width_and_unknown_properties_fail_closed() {
        let root: Value =
            serde_json::from_str(include_str!("../../fixtures/token-2022-original.json")).unwrap();
        let original = root["program"]["instructions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|item| item["name"] == "initializeTokenMetadata")
            .unwrap();
        for (pointer, value) in [
            ("/discriminators/0/offset", serde_json::json!(1)),
            ("/arguments/0/defaultValue/data", serde_json::json!("gg")),
            (
                "/arguments/0/type/size",
                serde_json::json!(WRONG_METADATA_WIDTH),
            ),
            (
                "/arguments/0/defaultValue/encoding",
                serde_json::json!("base64"),
            ),
        ] {
            let mut item = original.clone();
            *item.pointer_mut(pointer).unwrap() = value;
            assert!(parse(&item, item["arguments"].as_array().unwrap()).is_err());
        }

        let mut item = original.clone();
        item["arguments"][0]["type"]["truncate"] = serde_json::json!(true);
        assert!(parse(&item, item["arguments"].as_array().unwrap()).is_err());
    }
}
