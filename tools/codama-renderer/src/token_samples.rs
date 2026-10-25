//! Deterministic caller values; expected bytes always come from actual Dart.
use serde_json::Value;
use solana_kit_core::Address;
use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
const NONZERO_BYTE: u8 = 7;
const MAX_BYTE: u8 = u8::MAX;
const FULL_CASE: usize = 2;

pub(crate) fn snake(name: &str) -> String {
    name.chars()
        .flat_map(|c| {
            if c.is_ascii_uppercase() {
                vec!['_', c.to_ascii_lowercase()]
            } else {
                vec![c]
            }
        })
        .collect()
}
pub(crate) fn address(case: usize) -> String {
    Address::from_bytes(
        [match case {
            0 => 0,
            1 => NONZERO_BYTE,
            _ => MAX_BYTE,
        }; 32],
    )
    .to_string()
}

/// Sample synthesis chooses Rust and Dart values, never their encoded bytes.
pub(crate) fn value(
    node: &Value,
    program: &Value,
    case: usize,
    dart: bool,
    path: &str,
) -> Result<String> {
    match node["kind"].as_str() {
        Some("numberTypeNode") => number(node, case, dart),
        Some("publicKeyTypeNode") => Ok(address_value(case, dart)),
        Some("booleanTypeNode") => Ok((case != 0).to_string()),
        Some("optionTypeNode") => option(node, program, case, dart, path),
        Some("definedTypeLinkNode") => enumeration(node, program, case, dart),
        Some("stringTypeNode") => Ok(text(case, dart).to_owned()),
        Some("bytesTypeNode") => Ok(bytes(case, dart)),
        Some("sizePrefixTypeNode") => value(&node["type"], program, case, dart, path),
        Some("arrayTypeNode") => array(node, program, case, dart, path),
        Some("structTypeNode") => record(node, program, case, dart, path),
        _ => Err("unsupported sample shape".into()),
    }
}
fn number(node: &Value, case: usize, dart: bool) -> Result<String> {
    let number = match (node["format"].as_str(), case) {
        (_, 0) => "0",
        (_, 1) => "7",
        (Some("u8"), _) => "255",
        (Some("u64"), _) => "18446744073709551615",
        _ => return Err("unsupported sample number".into()),
    };

    if dart && node["format"] == "u64" {
        return Ok(format!("BigInt.parse('{number}')"));
    }

    if dart {
        return Ok(number.to_owned());
    }

    match (node["format"].as_str(), case) {
        (_, 0) => Ok("0".to_owned()),
        (Some("u8"), 1) => Ok("NONZERO_BYTE".to_owned()),
        (_, 1) => Ok("u64::from(NONZERO_BYTE)".to_owned()),
        (Some("u8"), _) => Ok("u8::MAX".to_owned()),
        _ => Ok("u64::MAX".to_owned()),
    }
}
fn address_value(case: usize, dart: bool) -> String {
    let address = address(case);

    if dart {
        format!("Address('{address}')")
    } else {
        format!("{address:?}.parse().unwrap()")
    }
}
/// Cover absent and populated options in both caller languages.
fn option(node: &Value, program: &Value, case: usize, dart: bool, path: &str) -> Result<String> {
    if case == 0 {
        return Ok(if dart { "null" } else { "None" }.to_owned());
    }

    let inner = value(&node["item"], program, case, dart, path)?;
    Ok(if dart {
        inner
    } else {
        format!("Some({inner})")
    })
}
fn enumeration(node: &Value, program: &Value, case: usize, dart: bool) -> Result<String> {
    let name = node["name"].as_str().ok_or("missing link name")?;
    let definition = program["definedTypes"]
        .as_array()
        .ok_or("types missing")?
        .iter()
        .find(|item| item["name"] == name)
        .ok_or("missing sample link")?;
    let variants = definition["type"]["variants"]
        .as_array()
        .ok_or("sample enum required")?;
    let index = if case == FULL_CASE {
        variants.len() - 1
    } else {
        case
    };

    let variant = variants[index]["name"]
        .as_str()
        .ok_or("missing variant name")?;

    let name = title(name);
    Ok(if dart {
        format!("t.{name}.{variant}")
    } else {
        format!("{name}::{}", title(variant))
    })
}
fn title(name: &str) -> String {
    let mut name = name.to_owned();
    name[0..1].make_ascii_uppercase();
    name
}
fn text(case: usize, dart: bool) -> &'static str {
    match (case, dart) {
        (0, true) => "''",
        (0, false) => "String::new()",
        (1, true) => "'42.🦀\\u0000'",
        (1, false) => "\"42.🦀\\0\".to_owned()",
        (_, true) => "'\\ufeff18446744073709551615.000000001'",
        (_, false) => "\"\\u{feff}18446744073709551615.000000001\".to_owned()",
    }
}
fn bytes(case: usize, dart: bool) -> String {
    let count = if case == FULL_CASE {
        usize::from(MAX_BYTE)
    } else {
        case * 3
    };

    if dart {
        format!("Uint8List.fromList([{}])", vec!["255"; count].join(","))
    } else {
        format!("vec![u8::MAX;{count}]")
    }
}
fn array(node: &Value, program: &Value, case: usize, dart: bool, path: &str) -> Result<String> {
    let count = node["count"]["value"]
        .as_u64()
        .map(usize::try_from)
        .transpose()?
        .unwrap_or(case);
    let item = value(&node["item"], program, case, dart, &format!("{path}Item"))?;
    let list = vec![item; count].join(",");
    Ok(if dart {
        format!("[{list}]")
    } else {
        format!("vec![{list}]")
    })
}
fn record(node: &Value, program: &Value, case: usize, dart: bool, path: &str) -> Result<String> {
    let mut fields = Vec::new();

    for field in node["fields"].as_array().ok_or("struct fields missing")? {
        let name = field["name"].as_str().ok_or("field name missing")?;
        let item = value(&field["type"], program, case, dart, path)?;
        fields.push(if dart {
            format!("'{name}':{item}")
        } else {
            format!("{}:{item}", snake(name))
        });
    }
    Ok(if dart {
        format!("{{{}}}", fields.join(","))
    } else {
        format!("{path}{{{}}}", fields.join(","))
    })
}
