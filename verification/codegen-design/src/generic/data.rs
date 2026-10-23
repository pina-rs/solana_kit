//! Data-only projection of complete Codama inputs; wrappers remain caller-owned.
use super::{Definition, Field, Policy, Program, Shape, emit, parse, wire};
use crate::schema::{Error, identifier};
use serde_json::Value;
use std::collections::BTreeSet;
#[cfg(test)]
use std::path::Path;

#[cfg(test)]
pub(crate) fn replace(root: &Value, policy: &Policy<'_>, output: &Path) -> Result<(), Error> {
    crate::generate::replace_source(&render(root, policy)?, output)
}

pub(super) struct Record {
    pub(super) name: String,
    fields: Vec<Field>,
    pub(super) tag: Option<Vec<u8>>,
}

/// Parses actual schema data without inventing replacement JSON or wrapper defaults.
/// The separate collections preserve source order while sharing one layout validator.
pub(crate) fn render(root: &Value, policy: &Policy<'_>) -> Result<String, Error> {
    render_layouts(root, policy, false)
}

pub(crate) fn render_tlv(root: &Value, policy: &Policy<'_>) -> Result<String, Error> {
    render_layouts(root, policy, true)
}

fn render_layouts(root: &Value, policy: &Policy<'_>, tlv_accounts: bool) -> Result<String, Error> {
    super::data_nodes::validate(root)?;
    let normalized = super::normalize_input(root)?;
    let root = &normalized;
    parse::keys(
        root,
        &[
            "kind",
            "program",
            "additionalPrograms",
            "standard",
            "version",
        ],
    )?;

    if root["kind"] != "rootNode" || root["standard"] != "codama" || root["version"] != "1.0.0" {
        return Err("unsupported data root".into());
    }

    let node = &root["program"];
    parse::keys(
        node,
        &[
            "kind",
            "pdas",
            "accounts",
            "instructions",
            "definedTypes",
            "errors",
            "name",
            "prefix",
            "publicKey",
            "version",
            "origin",
            "docs",
        ],
    )?;

    if node["kind"] != "programNode"
        || node["name"] != policy.name
        || node.get("prefix").is_some_and(|prefix| prefix != "")
        || node["publicKey"] != policy.address.to_string()
    {
        return Err("untrusted data program or unsupported errors".into());
    }
    let mut records = Vec::new();
    let mut definitions = Vec::new();

    for error in parse::array(node, "errors")? {
        parse::program_error(error)?;

        if !tlv_accounts {
            return Err("unsupported data projection errors".into());
        }
    }

    for node in parse::array(node, "definedTypes")? {
        super::data_shapes::definition(node, &mut definitions)?;
    }

    for node in parse::array(node, "accounts")? {
        parse::keys(
            node,
            &[
                "kind",
                "data",
                "pda",
                "discriminators",
                "name",
                "idlName",
                "docs",
                "size",
            ],
        )?;

        if node["kind"] != "accountNode" || node["data"]["kind"] != "structTypeNode" {
            return Err("unsupported data account".into());
        }

        parse::keys(&node["data"], &["kind", "fields"])?;
        let nodes = parse::array(&node["data"], "fields")?;

        if !tlv_accounts {
            records.push(record(node, nodes, "structFieldTypeNode")?);
            continue;
        }

        let mut fields = fields(nodes, "structFieldTypeNode")?;

        if tlv_accounts && node["name"] != "multisig" {
            super::data_shapes::tlv_tail(&mut fields)?;
        }

        records.push(Record {
            name: parse::name(node)?,
            fields,
            tag: None,
        });
    }

    let instructions = parse::array(node, "instructions")?;
    let mut tags = BTreeSet::new();

    for node in instructions {
        parse::keys(
            node,
            &[
                "kind",
                "display",
                "accounts",
                "arguments",
                "byteDeltas",
                "discriminators",
                "remainingAccounts",
                "name",
                "idlName",
                "docs",
                "optionalAccountStrategy",
                "provides",
            ],
        )?;

        if node["kind"] != "instructionNode" {
            return Err("unsupported data instruction".into());
        }

        let record = record(
            node,
            parse::array(node, "arguments")?,
            "instructionArgumentNode",
        )?;

        if !tags.insert(record.tag.clone()) || (record.tag.is_none() && instructions.len() != 1) {
            return Err("ambiguous instruction tags".into());
        }

        records.push(record);
    }

    for a in tags.iter().flatten() {
        for b in tags.iter().flatten() {
            if a != b && a.starts_with(b) {
                return Err("ambiguous instruction tag prefix".into());
            }
        }
    }

    output(records, definitions, policy, tlv_accounts)
}

fn record(node: &Value, nodes: &[Value], kind: &str) -> Result<Record, Error> {
    let (tag, skip) = super::tags::parse(node, nodes)?;
    Ok(Record {
        name: parse::name(node)?,
        fields: fields(&nodes[skip..], kind)?,
        tag,
    })
}

pub(super) fn fields(nodes: &[Value], kind: &str) -> Result<Vec<Field>, Error> {
    let mut names = BTreeSet::new();
    let mut fields = Vec::new();

    for node in nodes {
        parse::keys(
            node,
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

        if node["kind"] != kind {
            return Err("unexpected data field".into());
        }

        let name = identifier(node["name"].as_str().ok_or("missing field name")?)?;

        if !names.insert(name.clone()) {
            return Err("duplicate data field".into());
        }

        fields.push(Field {
            name,
            wire: wire::parse(&node["type"])?,
        });
    }

    Ok(fields)
}

fn output(
    records: Vec<Record>,
    mut definitions: Vec<Definition>,
    policy: &Policy<'_>,
    dispatch: bool,
) -> Result<String, Error> {
    let original_count = definitions.len();
    let mut name = policy.name.to_owned();
    name[0..1].make_ascii_uppercase();
    definitions.extend(records.iter().map(|record| Definition {
        name: record.name.clone(),
        shape: Shape::Struct(record.fields.clone()),
    }));
    let program = Program {
        name,
        address: policy.address,
        definitions,
        instructions: Vec::new(),
        accounts: Vec::new(),
        errors: Vec::new(),
    };

    super::layout::validate_data(&program)?;
    let required = super::imports::collect(&program);
    let mut out = preamble(&records, &required);
    out.push_str(&super::constants::emit(&program));

    if required.contains("Counted") {
        out.push_str(super::counted_output::SOURCE);
    }

    if required.contains("TlvSequence") {
        out.push_str(super::tlv_output::SOURCE);
    }

    for definition in &program.definitions[..original_count] {
        out.push_str(&declaration(definition, &program)?);
    }

    if dispatch {
        out.push_str(&super::dispatch_output::data(&program, &records)?);
    }

    for record in records {
        let tag = record
            .tag
            .as_ref()
            .map(|bytes| bytes.iter().map(u8::to_string).collect::<Vec<_>>());
        out.push_str(&emit::data_structure(
            &record.name,
            &record.fields,
            tag.as_deref(),
            &program,
        )?);
    }

    emit::imports(&mut out, &required);
    Ok(out)
}

fn declaration(definition: &Definition, program: &Program) -> Result<String, Error> {
    match &definition.shape {
        Shape::Struct(fields) => emit::data_structure(&definition.name, fields, None, program),
        Shape::Enum(variants) => Ok(super::enum_output::emit(&definition.name, variants)),
        Shape::Tagged { prefix, variants } => {
            super::enum_output::tagged(&definition.name, prefix, variants, program)
        }
    }
}

fn preamble(records: &[Record], required: &super::imports::Imports) -> String {
    let vector = if ["OwnedSequence", "Counted", "Bytes"]
        .iter()
        .any(|name| required.contains(name))
    {
        "use alloc::vec::Vec;\n"
    } else {
        ""
    };
    let address = address_import(required);
    let mut out = format!(
        "// Generated from the exact pinned Codama input; see upstream notices.\n{vector}use solana_kit_core::{{{address}codecs::{{Decoder, Encoder, Error, Reader, Writer}}}};\n"
    );

    if records.iter().any(|record| record.tag.is_some()) {
        out.push_str("use solana_kit_core::codecs::Constant;\n");
    }

    if required.contains("Number") {
        out.push_str("use solana_kit_core::codecs::Number;\n");
    }

    out
}

fn address_import(required: &super::imports::Imports) -> &'static str {
    if required.contains("Address") {
        return "Address, ";
    }

    ""
}
