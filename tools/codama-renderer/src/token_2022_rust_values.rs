//! Caller values only. Expected bytes come from the independent Dart rows.
use crate::schema::{Error, identifier};
use serde_json::Value;

pub(super) fn title(name: &str) -> String {
    let mut name = name.to_owned();
    name[0..1].make_ascii_uppercase();
    name
}

pub(super) fn value(
    node: &Value,
    program: &Value,
    case: usize,
    path: &str,
) -> Result<String, Error> {
    match node["kind"].as_str() {
        Some("numberTypeNode") => number(node, case),
        Some("publicKeyTypeNode") => Ok(format!(
            "Address::from_bytes([{};32])",
            [0, 7, 255, 128][case]
        )),
        Some("booleanTypeNode") => Ok((case != 0).to_string()),
        Some("optionTypeNode" | "zeroableOptionTypeNode" | "remainderOptionTypeNode") => {
            if case == 0 {
                return Ok("None".to_owned());
            }

            Ok(format!(
                "Some({})",
                value(&node["item"], program, case, path)?
            ))
        }
        Some("definedTypeLinkNode") => linked(node, program, case),
        Some("stringTypeNode") => Ok(format!(
            "String::from({:?})",
            ["", "42.🦀\0", "\u{feff}max", "negative"][case]
        )),
        Some("bytesTypeNode") => Ok("Vec::new()".to_owned()),
        Some("fixedSizeTypeNode") => Ok(format!(
            "[{};{}]",
            [0, 7, 255, 128][case],
            node["size"].as_u64().ok_or("missing fixed size")?
        )),
        Some("sizePrefixTypeNode" | "hiddenPrefixTypeNode") => {
            value(&node["type"], program, case, path)
        }
        Some("amountTypeNode") => value(&node["number"], program, case, path),
        Some("arrayTypeNode") => array(node, program, case, path),
        Some("structTypeNode") => record(path, node, program, case),
        Some("mapTypeNode") => Ok("Vec::new()".to_owned()),
        _ => Err(format!("unsupported Rust caller value {}", node["kind"]).into()),
    }
}

fn array(node: &Value, program: &Value, case: usize, path: &str) -> Result<String, Error> {
    let count = node["count"]["value"]
        .as_u64()
        .map(usize::try_from)
        .transpose()?
        .unwrap_or(case.min(2));

    if count == 0 {
        return Ok("Vec::new()".to_owned());
    }

    Ok(format!(
        "vec![{};{count}]",
        value(&node["item"], program, case, &format!("{path}Item"))?
    ))
}

fn number(node: &Value, case: usize) -> Result<String, Error> {
    let format = node["format"].as_str().ok_or("missing number format")?;

    if format == "f64" {
        return Ok(["0.0", "1.5", "f64::MAX", "-f64::MAX"][case].to_owned());
    }

    Ok(match case {
        0 => format!("0{format}"),
        1 => format!("7{format}"),
        2 => format!("{format}::MAX"),
        _ if format.starts_with('i') => format!("{format}::MIN"),
        _ => format!("0{format}"),
    })
}

fn linked(node: &Value, program: &Value, case: usize) -> Result<String, Error> {
    let name = node["name"].as_str().ok_or("missing link name")?;
    let definition = program["definedTypes"]
        .as_array()
        .ok_or("missing definitions")?
        .iter()
        .find(|item| item["name"] == name)
        .ok_or("unresolved caller link")?;
    let shape = &definition["type"];
    let name = title(name);

    match shape["kind"].as_str() {
        Some("enumTypeNode") => enumeration(&name, shape, program, case),
        Some("structTypeNode") => record(&name, shape, program, case),
        _ => Ok(format!(
            "{name} {{ value: {} }}",
            value(shape, program, case, &name)?
        )),
    }
}

fn enumeration(name: &str, shape: &Value, program: &Value, case: usize) -> Result<String, Error> {
    let variants = shape["variants"].as_array().ok_or("missing variants")?;
    let index = if case == 2 {
        variants.len() - 1
    } else {
        case.min(variants.len() - 1)
    };

    variant(name, &variants[index], program, case)
}

pub(super) fn variant(
    owner: &str,
    node: &Value,
    program: &Value,
    case: usize,
) -> Result<String, Error> {
    let name = title(node["name"].as_str().ok_or("missing variant name")?);
    let label = format!("{owner}::{name}");

    match node["kind"].as_str() {
        Some("enumEmptyVariantTypeNode") => Ok(label),
        Some("enumStructVariantTypeNode") => Ok(format!(
            "{label}({})",
            record(
                &format!("{owner}{name}Payload"),
                &node["struct"]["type"],
                program,
                case
            )?
        )),
        Some("enumTupleVariantTypeNode") => Ok(format!(
            "{label}({})",
            value(&node["tuple"]["items"][0], program, case, &name)?
        )),
        _ => Err("unsupported Rust caller enum variant".into()),
    }
}

pub(super) fn record(
    name: &str,
    node: &Value,
    program: &Value,
    case: usize,
) -> Result<String, Error> {
    let mut fields = Vec::new();

    for field in node["fields"].as_array().ok_or("missing caller fields")? {
        let field_name = field["name"].as_str().ok_or("missing caller field name")?;
        let value = if field_name == "extensions" {
            "None".to_owned()
        } else {
            value(
                &field["type"],
                program,
                case,
                &format!("{name}{}", title(field_name)),
            )?
        };

        fields.push(format!("{}:{value}", identifier(field_name)?));
    }

    Ok(format!("{name} {{ {} }}", fields.join(",")))
}
