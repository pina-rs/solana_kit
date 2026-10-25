//! Checks prepared Anchor layouts through the existing typed wire and namespace owners.
use super::{Definition, Program, Shape, data, data_shapes, layout, parse, roles, tags};
use crate::schema::Error;
use serde_json::Value;
use std::collections::BTreeSet;

/// Reusing the renderer's typed validators keeps normalization from weakening its boundary.
pub(super) fn validate(root: &Value) -> Result<(), Error> {
    let node = &root["program"];
    layout::instruction_count(parse::array(node, "instructions")?.len())?;
    let mut definitions = Vec::new();

    for definition in parse::array(node, "definedTypes")? {
        data_shapes::definition(definition, &mut definitions)?;
    }

    let mut headers = BTreeSet::new();

    for instruction in parse::array(node, "instructions")? {
        let arguments = parse::array(instruction, "arguments")?;
        let (header, skip) = tags::parse(instruction, arguments)?;
        let header = header.ok_or("prepared instruction requires a literal header")?;

        if !headers.insert(header) {
            return Err("duplicate prepared instruction header".into());
        }

        roles::parse(parse::array(instruction, "accounts")?)?;
        roles::remaining(instruction)?;
        definitions.push(Definition {
            name: parse::name(instruction)?,
            shape: Shape::Struct(data::fields(&arguments[skip..], "instructionArgumentNode")?),
            discriminator: None,
        });
    }

    unambiguous_headers(&headers)?;

    let program = Program {
        name: parse::name(node)?,
        address: node["publicKey"]
            .as_str()
            .ok_or("missing prepared address")?
            .parse()?,
        definitions,
        instructions: Vec::new(),
        accounts: Vec::new(),
        errors: parse::array(node, "errors")?
            .iter()
            .map(parse::program_error)
            .collect::<Result<_, _>>()?,
    };
    layout::validate_data(&program)
}

/// Distinct tags may not be prefixes because decoding must select one instruction.
fn unambiguous_headers(headers: &BTreeSet<Vec<u8>>) -> Result<(), Error> {
    for header in headers {
        if headers
            .iter()
            .any(|other| header != other && header.starts_with(other))
        {
            return Err("ambiguous prepared instruction header prefix".into());
        }
    }

    Ok(())
}
