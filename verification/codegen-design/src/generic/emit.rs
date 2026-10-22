//! Emits concrete bounded codecs from the validated finite model.
use super::{Field, Program, Shape, Wire};
use crate::schema::Error;

const U8_TAG_COUNT: usize = 256;

fn rust_type(wire: &Wire) -> String {
    match wire {
        Wire::Number(name) => (*name).to_owned(),
        Wire::Link(name) => name.clone(),
    }
}
fn codec(wire: &Wire) -> String {
    match wire {
        Wire::Number(name) => format!("Number::<{name}>::LE"),
        Wire::Link(name) => format!("{name}Codec"),
    }
}

fn structure(name: &str, fields: &[Field], tag: Option<u8>) -> String {
    let mut out = format!(
        "/// Validated data for `{name}`.\n#[derive(Clone, Debug, Eq, PartialEq)]\npub struct {name} {{\n"
    );

    for field in fields {
        out.push_str(&format!(
            "/// The `{}` wire field.\npub {}: {},\n",
            field.name,
            field.name,
            rust_type(&field.wire)
        ));
    }

    out.push_str(&format!("}}\n/// Bounded codec for [`{name}`].\n#[derive(Clone, Copy, Debug)]\npub struct {name}Codec;\nimpl Encoder<{name}> for {name}Codec {{\nfn encoded_len(&self, value: &{name}) -> Result<usize, Error> {{\nlet size = {}usize;\n", usize::from(tag.is_some())));

    for field in fields {
        out.push_str(&format!(
            "let size = size.checked_add({}.encoded_len(&value.{})?).ok_or(Error::Overflow)?;\n",
            codec(&field.wire),
            field.name
        ));
    }

    if fields.is_empty() {
        out.push_str("let _ = value;\n");
    }

    out.push_str(&write_and_read(name, fields, tag));
    out
}

/// Both directions consume the same validated field list so linked layouts cannot drift.
fn write_and_read(name: &str, fields: &[Field], tag: Option<u8>) -> String {
    let mut out = format!(
        "Ok(size)\n}}\nfn write(&self, value: &{name}, writer: &mut Writer<'_>) -> Result<(), Error> {{\n"
    );

    if let Some(tag) = tag {
        out.push_str(&format!("Constant(&[{tag}]).write(&(), writer)?;\n"));
    }

    for field in fields {
        out.push_str(&format!(
            "{}.write(&value.{}, writer)?;\n",
            codec(&field.wire),
            field.name
        ));
    }

    if fields.is_empty() {
        out.push_str("let _ = value; let _ = writer;\n");
    }

    out.push_str(&format!("Ok(())\n}}\n}}\nimpl Decoder<{name}> for {name}Codec {{\nfn read(&self, reader: &mut Reader<'_>) -> Result<{name}, Error> {{\n"));

    if let Some(tag) = tag {
        out.push_str(&format!("Constant(&[{tag}]).read(reader)?;\n"));
    }

    if fields.is_empty() {
        out.push_str("let _ = reader;\n");
    }

    out.push_str(&format!("Ok({name} {{\n"));

    for field in fields {
        out.push_str(&format!(
            "{}: {}.read(reader)?,\n",
            field.name,
            codec(&field.wire)
        ));
    }

    out.push_str("})\n}\n}\n");
    out
}

fn enumeration(name: &str, variants: &[String]) -> String {
    let mut out = format!(
        "/// Finite u8 tagged enum.\n#[derive(Clone, Debug, Eq, PartialEq)]\npub enum {name} {{\n"
    );

    for variant in variants {
        out.push_str(&format!("/// The `{variant}` variant.\n{variant},\n"));
    }

    out.push_str(&format!("}}\n/// Strict codec for [`{name}`].\n#[derive(Clone, Copy, Debug)]\npub struct {name}Codec;\nimpl Encoder<{name}> for {name}Codec {{\nfn encoded_len(&self, _: &{name}) -> Result<usize, Error> {{ Ok(1) }}\nfn write(&self, value: &{name}, writer: &mut Writer<'_>) -> Result<(), Error> {{\nlet tag = match value {{\n"));

    for (tag, variant) in variants.iter().enumerate() {
        out.push_str(&format!("{name}::{variant} => {tag}u8,\n"));
    }

    out.push_str(&format!("}};\nNumber::<u8>::LE.write(&tag, writer)\n}}\n}}\nimpl Decoder<{name}> for {name}Codec {{\nfn read(&self, reader: &mut Reader<'_>) -> Result<{name}, Error> {{\nmatch Number::<u8>::LE.read(reader)? {{\n"));

    for (tag, variant) in variants.iter().enumerate() {
        out.push_str(&format!("{tag} => Ok({name}::{variant}),\n"));
    }

    if variants.len() < U8_TAG_COUNT {
        out.push_str("_ => Err(Error::InvalidDiscriminator),\n");
    }

    out.push_str("}\n}\n}\n");
    out
}

fn builders(name: &str) -> String {
    format!(
        "impl {name} {{\n/// Constructs an owned off-chain program invocation with no accounts.\npub fn instruction(&self, limits: Limits) -> Result<Instruction, Error> {{\nOk(Instruction {{ program_address: PROGRAM, accounts: Vec::new(), data: {name}Codec.encode(self, limits.max_bytes)? }})\n}}\n/// Parses a canonical invocation, rejecting foreign programs, accounts and trailing data.\npub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {{\nif instruction.program_address != PROGRAM || !instruction.accounts.is_empty() {{ return Err(Error::InvalidValue); }}\n\n{name}Codec.decode(&instruction.data, limits)\n}}\n}}\n"
    )
}

/// Dispatch delegates each direction to the concrete codec, preserving its bounds.
fn dispatch(program: &Program) -> String {
    let name = format!("{}InstructionData", program.name);
    let fallback = if program.instructions.len() == U8_TAG_COUNT {
        ""
    } else {
        "_ => Err(Error::InvalidDiscriminator),\n"
    };
    let mut out = format!(
        "/// All selected instruction data variants.\n#[derive(Clone, Debug, Eq, PartialEq)]\npub enum {name} {{\n"
    );

    for item in &program.instructions {
        out.push_str(&format!(
            "/// Data for tag {}.\n{}({}),\n",
            item.tag, item.name, item.name
        ));
    }

    out.push_str(&format!("}}\n/// Strict bounded instruction dispatch codec.\n#[derive(Clone, Copy, Debug)]\npub struct {name}Codec;\nimpl Encoder<{name}> for {name}Codec {{\nfn encoded_len(&self, value: &{name}) -> Result<usize, Error> {{\nmatch value {{\n"));

    for item in &program.instructions {
        out.push_str(&format!(
            "{name}::{}(value) => {}Codec.encoded_len(value),\n",
            item.name, item.name
        ));
    }

    out.push_str(&format!("}}\n}}\nfn write(&self, value: &{name}, writer: &mut Writer<'_>) -> Result<(), Error> {{\nmatch value {{\n"));

    for item in &program.instructions {
        out.push_str(&format!(
            "{name}::{}(value) => {}Codec.write(value, writer),\n",
            item.name, item.name
        ));
    }

    out.push_str(&format!("}}\n}}\n}}\nimpl Decoder<{name}> for {name}Codec {{\nfn read(&self, reader: &mut Reader<'_>) -> Result<{name}, Error> {{\nlet tag = *reader.remaining().first().ok_or(Error::Truncated)?;\n\nmatch tag {{\n"));

    for item in &program.instructions {
        out.push_str(&format!(
            "{} => Ok({name}::{}({}Codec.read(reader)?)),\n",
            item.tag, item.name, item.name
        ));
    }

    out.push_str(fallback);
    out.push_str("}\n}\n}\n/// Identifies a tag without asserting the full payload is valid.\npub fn identify_instruction(data: &[u8]) -> Result<u8, Error> {\nlet tag = *data.first().ok_or(Error::Truncated)?;\n\nmatch tag {\n");

    for item in &program.instructions {
        out.push_str(&format!("{} => Ok(tag),\n", item.tag));
    }

    out.push_str(fallback);
    out.push_str(&format!("}}\n}}\n/// Fully parses a canonical invocation with the fixed selected program identity.\npub fn parse_instruction(instruction: &Instruction, limits: Limits) -> Result<{name}, Error> {{\nif instruction.program_address != PROGRAM || !instruction.accounts.is_empty() {{ return Err(Error::InvalidValue); }}\n\n{name}Codec.decode(&instruction.data, limits)\n}}\n"));
    out
}

/// Import decisions follow emitted layouts, including standalone definitions.
pub(super) fn program(program: &Program) -> Result<String, Error> {
    let bytes = program
        .address
        .as_bytes()
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(",");
    let number = if needs_number(program) {
        "Number, "
    } else {
        ""
    };

    let mut out = format!(
        "// Generated from a validated Codama model. See README.md and upstream notices.\nuse alloc::vec::Vec;\nuse solana_kit_core::{{Address, Instruction, codecs::{{Constant, Decoder, Encoder, Error, Limits, {number}Reader, Writer}}}};\n/// Selected trusted program identity.\npub const PROGRAM: Address = Address::from_bytes([{bytes}]);\n"
    );

    for item in &program.definitions {
        out.push_str(&match &item.shape {
            Shape::Struct(fields) => structure(&item.name, fields, None),
            Shape::Enum(variants) => enumeration(&item.name, variants),
        });
    }

    for item in &program.instructions {
        out.push_str(&structure(&item.name, &item.fields, Some(item.tag)));
        out.push_str(&builders(&item.name));
    }

    out.push_str(&dispatch(program));

    Ok(out)
}

fn needs_number(program: &Program) -> bool {
    let numeric = |fields: &[Field]| {
        fields
            .iter()
            .any(|field| matches!(field.wire, Wire::Number(_)))
    };
    program.definitions.iter().any(|item| match &item.shape {
        Shape::Enum(_) => true,
        Shape::Struct(fields) => numeric(fields),
    }) || program
        .instructions
        .iter()
        .any(|item| numeric(&item.fields))
}
