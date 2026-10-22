//! Emits concrete bounded codecs from the validated finite model.
use super::{
    Field, Program, Shape,
    expressions::{codec, rust_type, value},
};
use crate::schema::Error;

const U8_TAG_COUNT: usize = 256;

fn structure(
    name: &str,
    fields: &[Field],
    tag: Option<u8>,
    program: &Program,
) -> Result<String, Error> {
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
            "let size = size.checked_add({}.encoded_len({})?).ok_or(Error::Overflow)?;\n",
            codec(&field.wire, program)?,
            value(&field.wire, &format!("value.{}", field.name))
        ));
    }

    if fields.is_empty() {
        out.push_str("let _ = value;\n");
    }

    out.push_str(&write_and_read(name, fields, tag, program)?);
    Ok(out)
}

/// Both directions consume the same validated field list so linked layouts cannot drift.
fn write_and_read(
    name: &str,
    fields: &[Field],
    tag: Option<u8>,
    program: &Program,
) -> Result<String, Error> {
    let mut out = format!(
        "Ok(size)\n}}\nfn write(&self, value: &{name}, writer: &mut Writer<'_>) -> Result<(), Error> {{\n"
    );

    if let Some(tag) = tag {
        let tag = super::constants::tag(program, tag);
        out.push_str(&format!("Constant(&[{tag}]).write(&(), writer)?;\n"));
    }

    for field in fields {
        out.push_str(&format!(
            "{}.write({}, writer)?;\n",
            codec(&field.wire, program)?,
            value(&field.wire, &format!("value.{}", field.name))
        ));
    }

    if fields.is_empty() {
        out.push_str("let _ = value; let _ = writer;\n");
    }

    out.push_str(&format!("Ok(())\n}}\n}}\nimpl Decoder<{name}> for {name}Codec {{\nfn read(&self, reader: &mut Reader<'_>) -> Result<{name}, Error> {{\n"));

    if let Some(tag) = tag {
        let tag = super::constants::tag(program, tag);
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
            codec(&field.wire, program)?
        ));
    }

    out.push_str("})\n}\n}\n");
    Ok(out)
}

fn builders(name: &str) -> String {
    format!(
        "impl {name} {{\n/// Constructs an owned off-chain program invocation with no accounts.\npub fn instruction(&self, limits: Limits) -> Result<Instruction, Error> {{\nOk(Instruction {{ program_address: PROGRAM, accounts: Vec::new(), data: {name}Codec.encode(self, limits.max_bytes)? }})\n}}\n/// Parses a canonical invocation, rejecting foreign programs, accounts and trailing data.\npub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {{\nif instruction.program_address != PROGRAM || !instruction.accounts.is_empty() {{ return Err(Error::InvalidValue); }}\n\n{name}Codec.decode(&instruction.data, limits)\n}}\n}}\n"
    )
}

/// Dispatch delegates each direction to the concrete codec, preserving its bounds.
fn dispatch(program: &Program) -> String {
    if program.instructions.len() > super::dispatch_output::INLINE_LIMIT {
        return super::dispatch_output::emit(program);
    }

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
            super::constants::tag(program, item.tag),
            item.name,
            item.name
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
            super::constants::tag(program, item.tag),
            item.name,
            item.name
        ));
    }

    out.push_str(fallback);
    out.push_str("}\n}\n}\n");
    out.push_str(&identification(program, &name, fallback));
    out
}

fn identification(program: &Program, name: &str, fallback: &str) -> String {
    let mut out="/// Identifies a tag without asserting the full payload is valid.\npub fn identify_instruction(data: &[u8]) -> Result<u8, Error> {\nlet tag = *data.first().ok_or(Error::Truncated)?;\n\nmatch tag {\n".to_owned();

    for item in &program.instructions {
        out.push_str(&format!(
            "{} => Ok(tag),\n",
            super::constants::tag(program, item.tag)
        ));
    }

    out.push_str(fallback);

    if !super::constants::extended(program) {
        out.push_str(&format!("}}\n}}\n/// Fully parses a canonical invocation with the fixed selected program identity.\npub fn parse_instruction(instruction: &Instruction, limits: Limits) -> Result<{name}, Error> {{\nif instruction.program_address != PROGRAM || !instruction.accounts.is_empty() {{ return Err(Error::InvalidValue); }}\n\n{name}Codec.decode(&instruction.data, limits)\n}}\n"));
    } else {
        out.push_str("}\n}\n");
        out.push_str(&super::invocations::dispatch(program));
    }

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
    let required = super::imports::collect(program);
    let number = if required.contains("Number") {
        "Number, "
    } else {
        ""
    };

    let mut out = format!(
        "// Generated from a validated Codama model. See README.md and upstream notices.\nuse alloc::vec::Vec;\nuse solana_kit_core::{{Address, Instruction, codecs::{{Constant, Decoder, Encoder, Error, Limits, {number}Reader, Writer}}}};\n/// Selected trusted program identity.\npub const PROGRAM: Address = Address::from_bytes([{bytes}]);\n"
    );

    out.push_str(&super::constants::emit(program));

    if super::constants::extended(program) {
        out.push_str(&super::invocations::helpers());
    }

    out.push_str(&declarations(program)?);
    out.push_str(&super::errors::emit(program));
    out.push_str(&super::account_output::emit(program));
    out.push_str(&dispatch(program));
    imports(&mut out, &required);
    Ok(out)
}

/// Preserve declaration order and separate ownership.
fn declarations(program: &Program) -> Result<String, Error> {
    let mut out = String::new();

    for item in &program.definitions {
        out.push_str(&match &item.shape {
            Shape::Struct(fields) => structure(&item.name, fields, None, program)?,
            Shape::Enum(variants) => super::enum_output::emit(&item.name, variants),
        });
    }

    for item in &program.instructions {
        out.push_str(&instruction(program, item)?);
    }

    for account in &program.accounts {
        let Shape::Struct(fields) = &account.definition.shape else {
            return Err("account must be record".into());
        };
        out.push_str(&structure(&account.definition.name, fields, None, program)?);
    }

    Ok(out)
}

fn instruction(program: &Program, item: &super::Instruction) -> Result<String, Error> {
    let mut out = structure(&item.name, &item.fields, Some(item.tag), program)?;

    if !super::constants::extended(program) && item.roles.is_empty() && item.remaining.is_none() {
        out.push_str(&builders(&item.name));
    } else {
        out.push_str(&super::invocations::emit(item));
    }

    Ok(out)
}

/// Render only requirements collected from validated typed layouts.
fn imports(out: &mut String, required: &super::imports::Imports) {
    for (name, import) in [
        ("Mapped", "Mapped"),
        ("FixedBytes", "FixedBytes"),
        ("Bool", "Bool"),
        ("Utf8", "Utf8"),
        ("Bytes", "Bytes"),
        ("OptionCodec", "OptionCodec"),
        ("OwnedSequence", "OwnedSequence, Sequence, Count"),
        ("ByteSizePrefix", "ByteSizePrefix, FrameConsumption"),
    ] {
        if required.contains(name) {
            out.push_str(&format!("use solana_kit_core::codecs::{{{import}}};\n"));
        }
    }

    if required.contains("String") {
        out.push_str("use alloc::string::String;\n");
    }

    if required.contains("AccountMeta") {
        out.push_str("use solana_kit_core::AccountMeta;\n");
    }

    if required.contains("AccountRole") {
        out.push_str("use solana_kit_core::AccountRole;\n");
    }
}
