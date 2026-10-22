//! Namespace and acyclic link validation over the complete selected program.
use super::{Field, Program, Shape, Wire};
use crate::schema::Error;
use std::collections::BTreeSet;

fn links(fields: &[Field]) -> impl Iterator<Item = &str> {
    fields.iter().filter_map(|field| match &field.wire {
        Wire::Link(name) => Some(name.as_str()),
        Wire::Number(_) => None,
    })
}

fn visit(
    program: &Program,
    name: &str,
    active: &mut BTreeSet<String>,
    done: &mut BTreeSet<String>,
) -> Result<(), Error> {
    if done.contains(name) {
        return Ok(());
    }

    if !active.insert(name.to_owned()) {
        return Err("cyclic linked layout".into());
    }

    let definition = program
        .definitions
        .iter()
        .find(|item| item.name == name)
        .ok_or("unresolved linked layout")?;

    if let Shape::Struct(fields) = &definition.shape {
        for link in links(fields) {
            visit(program, link, active, done)?;
        }
    }

    active.remove(name);
    done.insert(name.to_owned());
    Ok(())
}

fn names(program: &Program) -> Result<(), Error> {
    let mut names: BTreeSet<String> = [
        "Address",
        "Instruction",
        "Number",
        "Constant",
        "Encoder",
        "Decoder",
        "Error",
        "Reader",
        "Writer",
        "Limits",
        "Vec",
        "Result",
        "Ok",
        "Err",
        "PROGRAM",
        "identify_instruction",
        "parse_instruction",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    names.insert(format!("{}InstructionData", program.name));
    names.insert(format!("{}InstructionDataCodec", program.name));

    for name in program
        .definitions
        .iter()
        .map(|item| &item.name)
        .chain(program.instructions.iter().map(|item| &item.name))
    {
        if !names.insert(name.clone()) || !names.insert(format!("{name}Codec")) {
            return Err("generated namespace collision".into());
        }
    }
    Ok(())
}

/// Resolve every link under a depth budget before any emitter can choose a layout.
pub(super) fn validate(program: &Program) -> Result<(), Error> {
    const MAX_DEFINITIONS: usize = 64;

    if program.definitions.len() > MAX_DEFINITIONS {
        return Err("linked definition budget exceeded".into());
    }

    names(program)?;
    let tags: BTreeSet<_> = program.instructions.iter().map(|item| item.tag).collect();

    if tags.len() != program.instructions.len() {
        return Err("duplicate instruction discriminator".into());
    }

    let mut done = BTreeSet::new();

    for definition in &program.definitions {
        visit(program, &definition.name, &mut BTreeSet::new(), &mut done)?;
    }

    for item in &program.instructions {
        for link in links(&item.fields) {
            visit(program, link, &mut BTreeSet::new(), &mut done)?;
        }
    }

    Ok(())
}
