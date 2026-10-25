//! Caller values for the complete pinned Token-2022 Dart oracle.
use serde_json::Value;
use solana_kit_core::Address;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub(crate) fn title(name: &str) -> String {
    let mut name = name.to_owned();
    name[0..1].make_ascii_uppercase();
    name
}

pub(crate) fn address(case: usize) -> String {
    let byte = [0, 7, 255, 128][case % 4];
    format!("Address('{}')", Address::from_bytes([byte; 32]))
}

/// This emits caller values only; no expected wire byte is synthesized here.
pub(crate) fn value(node: &Value, program: &Value, case: usize) -> Result<String> {
    match node["kind"].as_str() {
        Some("numberTypeNode") => number(node, case),
        Some("publicKeyTypeNode") => Ok(address(case)),
        Some("booleanTypeNode") => Ok((case != 0).to_string()),
        Some("optionTypeNode" | "zeroableOptionTypeNode" | "remainderOptionTypeNode") => {
            if case == 0 {
                return Ok("null".to_owned());
            }

            value(&node["item"], program, case)
        }
        Some("definedTypeLinkNode") => linked(node, program, case),
        Some("stringTypeNode") => {
            Ok(["''", "'42.🦀\\u0000'", "'\\ufeffmax'", "'negative'"][case].to_owned())
        }
        Some("bytesTypeNode") => Ok("Uint8List(0)".to_owned()),
        Some("fixedSizeTypeNode") => {
            let size = node["size"].as_u64().ok_or("missing fixed width")?;
            Ok(format!(
                "Uint8List.fromList(List<int>.filled({size}, {}))",
                [0, 7, 255, 128][case]
            ))
        }
        Some("sizePrefixTypeNode" | "hiddenPrefixTypeNode") => value(&node["type"], program, case),
        Some("amountTypeNode") => value(&node["number"], program, case),
        Some("arrayTypeNode") => array(node, program, case),
        Some("structTypeNode") => fields(node, program, case, None),
        Some("mapTypeNode") => Ok("<String,String>{}".to_owned()),
        _ => Err(format!("unsupported oracle value node: {}", node["kind"]).into()),
    }
}

fn number(node: &Value, case: usize) -> Result<String> {
    let format = node["format"].as_str().ok_or("missing number format")?;

    let (maximum, minimum) = match format {
        "u8" => ("255", "0"),
        "u16" => ("65535", "0"),
        "u32" => ("4294967295", "0"),
        "u64" => ("18446744073709551615", "0"),
        "i8" => ("127", "-128"),
        "i16" => ("32767", "-32768"),
        "i64" => ("9223372036854775807", "-9223372036854775808"),
        "f64" => {
            return Ok(["0.0", "1.5", "double.maxFinite", "-double.maxFinite"][case].to_owned());
        }
        _ => return Err(format!("unsupported oracle number {format}").into()),
    };

    let number = ["0", "7", maximum, minimum][case];

    if ["u64", "i64"].contains(&format) {
        return Ok(format!("BigInt.parse('{number}')"));
    }

    Ok(number.to_owned())
}

fn linked(node: &Value, program: &Value, case: usize) -> Result<String> {
    let name = node["name"].as_str().ok_or("missing link name")?;
    let definition = program["definedTypes"]
        .as_array()
        .ok_or("missing definitions")?
        .iter()
        .find(|item| item["name"] == name)
        .ok_or("unresolved oracle link")?;
    let shape = &definition["type"];

    match shape["kind"].as_str() {
        Some("enumTypeNode") => {
            let variants = shape["variants"].as_array().ok_or("missing variants")?;
            let index = match case {
                2 => variants.len() - 1,
                _ => case.min(variants.len() - 1),
            };

            variant(name, &variants[index], program, case)
        }
        Some("structTypeNode") => fields(shape, program, case, Some(name)),
        _ => value(shape, program, case),
    }
}

pub(crate) fn variant(name: &str, variant: &Value, program: &Value, case: usize) -> Result<String> {
    let label = variant["name"].as_str().ok_or("missing variant name")?;
    let class = format!("t.{}{}", title(name), title(label));

    match variant["kind"].as_str() {
        Some("enumEmptyVariantTypeNode") if name == "extension" || name == "tokenMetadataField" => {
            Ok(format!("{class}()"))
        }

        Some("enumEmptyVariantTypeNode") => Ok(format!("t.{}.{label}", title(name))),
        Some("enumStructVariantTypeNode") => {
            let node = &variant["struct"]["type"];
            let arguments = field_values(node, program, case, true)?;
            Ok(format!("{class}({})", arguments.join(",")))
        }
        Some("enumTupleVariantTypeNode") => {
            let items = variant["tuple"]["items"]
                .as_array()
                .ok_or("missing tuple")?;
            let values = items
                .iter()
                .map(|item| value(item, program, case))
                .collect::<Result<Vec<_>>>()?;
            Ok(format!("{class}({})", values.join(",")))
        }
        _ => Err("unsupported oracle variant".into()),
    }
}

fn field_values(node: &Value, program: &Value, case: usize, named: bool) -> Result<Vec<String>> {
    node["fields"]
        .as_array()
        .ok_or("missing record fields")?
        .iter()
        .map(|field| {
            let name = field["name"].as_str().ok_or("missing field name")?;
            let item = value(&field["type"], program, case)?;
            Ok(if named {
                format!("{name}:{item}")
            } else {
                format!("'{name}':{item}")
            })
        })
        .collect()
}

fn fields(node: &Value, program: &Value, case: usize, name: Option<&str>) -> Result<String> {
    let fields = field_values(node, program, case, name.is_some())?.join(",");
    Ok(match name {
        Some(name) => format!("t.{}({fields})", title(name)),
        None => format!("{{{fields}}}"),
    })
}

fn array(node: &Value, program: &Value, case: usize) -> Result<String> {
    let count = node["count"]["value"]
        .as_u64()
        .map(usize::try_from)
        .transpose()?
        .unwrap_or(case.min(2));
    let item = value(&node["item"], program, case)?;
    Ok(format!("[{}]", vec![item; count].join(",")))
}
