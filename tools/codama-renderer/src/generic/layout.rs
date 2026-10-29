//! Namespace, acyclic layout and instruction privilege validation.
use super::{Program, Shape, extent};
use crate::schema::Error;
use std::collections::BTreeSet;
#[cfg(feature = "verification")]
const MAX_DEFINITIONS: usize = 64;
const MAX_DATA_DECLARATIONS: usize = 256;
const MAX_INSTRUCTIONS: usize = 256;

/// Literal headers no longer supply the historical u8 instruction-count bound.
pub(super) fn instruction_count(count: usize) -> Result<(), Error> {
    if count > MAX_INSTRUCTIONS {
        return Err("instruction budget exceeded".into());
    }

    Ok(())
}

const RESERVED: &[&str] = &[
    "Address",
    "Instruction",
    "Number",
    "Constant",
    "Encoder",
    "Decoder",
    "Error",
    "DefaultError",
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
    "parse_instruction_with_program",
    "identify_account",
    "AccountMeta",
    "AccountRole",
    "String",
    "Option",
    "Some",
    "None",
    "Bool",
    "Bytes",
    "Utf8",
    "FixedBytes",
    "Mapped",
    "OptionCodec",
    "OwnedSequence",
    "Sequence",
    "Count",
    "ByteSizePrefix",
    "FrameConsumption",
    "Counted",
    "CheckedCounted",
    "ShortU16",
    "Tuple",
    "FixedSize",
    "role_matches",
    "Affix",
    "OwnedString",
    "TlvSequence",
];

fn names(program: &Program) -> Result<(), Error> {
    let mut names: BTreeSet<String> = RESERVED.iter().copied().map(str::to_owned).collect();

    names.extend(
        super::imports::collect(program)
            .into_iter()
            .map(str::to_owned),
    );

    for suffix in [
        "InstructionData",
        "InstructionDataCodec",
        "Error",
        "AccountKind",
    ] {
        names.insert(format!("{}{suffix}", program.name));
    }
    let declarations = program
        .definitions
        .iter()
        .map(|item| &item.name)
        .chain(program.instructions.iter().map(|item| &item.name))
        .chain(program.accounts.iter().map(|item| &item.definition.name));

    for name in declarations {
        insert(&mut names, name)?;
    }

    tagged_constants(program, &mut names)?;

    Ok(())
}

/// Tag constants share Rust's value namespace with record constructors and codecs.
fn tagged_constants(program: &Program, names: &mut BTreeSet<String>) -> Result<(), Error> {
    for definition in &program.definitions {
        let Shape::Tagged { variants, .. } = &definition.shape else {
            continue;
        };

        for index in 0..variants.len() {
            if !names.insert(format!("{}_TAG_{index}", definition.name.to_uppercase())) {
                return Err("generated enum tag namespace collision".into());
            }
        }
    }

    Ok(())
}
fn insert(names: &mut BTreeSet<String>, name: &str) -> Result<(), Error> {
    if !names.insert(name.to_owned()) || !names.insert(format!("{name}Codec")) {
        return Err("generated namespace collision".into());
    }

    Ok(())
}

#[cfg(feature = "verification")]
pub(super) fn validate(program: &Program) -> Result<(), Error> {
    validate_with_budget(program, MAX_DEFINITIONS)
}

pub(super) fn validate_data(program: &Program) -> Result<(), Error> {
    validate_with_budget(program, MAX_DATA_DECLARATIONS)
}

fn validate_with_budget(program: &Program, maximum: usize) -> Result<(), Error> {
    instruction_count(program.instructions.len())?;

    if program.definitions.len() > maximum {
        return Err("linked definition budget exceeded".into());
    }

    names(program)?;
    unique(
        program.instructions.iter().map(|item| &item.tag),
        "duplicate instruction discriminator",
    )?;
    header_prefixes(program)?;
    unique(
        program.accounts.iter().map(|item| item.size),
        "ambiguous account size discriminator",
    )?;
    errors(program)?;
    definitions(program)?;

    for item in &program.instructions {
        instruction(program, item)?;
    }

    Ok(())
}

fn header_prefixes(program: &Program) -> Result<(), Error> {
    for item in &program.instructions {
        if program
            .instructions
            .iter()
            .any(|other| item.tag != other.tag && item.tag.bytes().starts_with(other.tag.bytes()))
        {
            return Err("ambiguous instruction discriminator prefix".into());
        }
    }

    Ok(())
}
fn unique<T: Ord>(values: impl Iterator<Item = T>, error: &str) -> Result<(), Error> {
    let mut seen = BTreeSet::new();

    for value in values {
        if !seen.insert(value) {
            return Err(error.to_owned().into());
        }
    }

    Ok(())
}
fn errors(program: &Program) -> Result<(), Error> {
    unique(
        program.errors.iter().map(|item| item.code),
        "duplicate program error code",
    )?;
    unique(
        program.errors.iter().map(|item| &item.name),
        "duplicate program error name",
    )?;

    if program.errors.iter().any(|item| item.name == "Unknown") {
        return Err("program error collides with unknown-code variant".into());
    }

    Ok(())
}
fn definitions(program: &Program) -> Result<(), Error> {
    for item in &program.definitions {
        extent::wire(
            program,
            &super::Wire::Link(item.name.clone()),
            &mut Vec::new(),
        )?;
    }

    for account in &program.accounts {
        account_size(program, account)?;
    }

    Ok(())
}
fn account_size(program: &Program, account: &super::Account) -> Result<(), Error> {
    let Shape::Struct(fields) = &account.definition.shape else {
        return Err("account must be a record".into());
    };

    if extent::fields(program, fields, &mut Vec::new())?.fixed != Some(account.size) {
        return Err("account size differs from explicit wire layout".into());
    }

    Ok(())
}
/// Reject ambiguous omission and resolve every linked role layout.
fn instruction(program: &Program, item: &super::Instruction) -> Result<(), Error> {
    if item.omit_optional
        && item
            .roles
            .windows(2)
            .any(|pair| pair[0].optional && !pair[1].optional)
    {
        return Err("omitted optional role precedes required role".into());
    }

    if item.omit_optional && item.remaining.is_some() {
        return Err("omitted optional roles with remaining list are ambiguous".into());
    }

    for role in &item.roles {
        role_link(program, role)?;
    }

    extent::fields(program, &item.fields, &mut Vec::new())?;

    Ok(())
}
fn role_link(program: &Program, role: &super::Role) -> Result<(), Error> {
    let Some(link) = &role.link else {
        return Ok(());
    };

    if !program
        .accounts
        .iter()
        .any(|account| &account.definition.name == link)
    {
        return Err("unresolved account layout link".into());
    }
    Ok(())
}
