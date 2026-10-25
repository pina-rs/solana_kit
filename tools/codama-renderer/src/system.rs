//! Emits bounded public System instruction data from the validated schema IR.

use crate::{
    generate,
    schema::{self, Error, Field, Instruction, Wire},
};
use serde_json::Value;
use std::path::Path;

fn field_type(wire: Wire) -> &'static str {
    match wire {
        Wire::U64 => "u64",
        Wire::Address => "Address",
        Wire::String => "String",
    }
}

fn codec(wire: Wire) -> &'static str {
    match wire {
        Wire::U64 => "Number::<u64>::LE",
        Wire::Address => "FixedBytes::<32>",
        Wire::String => "SizePrefix64(Utf8)",
    }
}

fn value(field: &Field) -> String {
    let name = &field.name;

    match field.wire {
        Wire::U64 => format!("&value.{name}"),
        Wire::Address => format!("value.{name}.as_bytes()"),
        Wire::String => format!("value.{name}.as_str()"),
    }
}

/// A single validated field list supplies the struct, size, write and read
/// paths so adding a wire field cannot leave one codec direction stale.
fn structure(item: &Instruction) -> String {
    let name = &item.rust_name;
    let tag = item.tag;
    let mut output = format!(
        "/// System instruction data for `{}` (tag {tag}).\n#[derive(Clone, Debug, Eq, PartialEq)]\npub struct {name} {{\n",
        item.name
    );

    for field in &item.fields {
        output.push_str(&format!(
            "    /// The `{}` wire field.\n    pub {}: {},\n",
            field.name,
            field.name,
            field_type(field.wire)
        ));
    }

    output.push_str(&format!("}}\n\n/// Bounded codec for [`{name}`]; full decoding rejects trailing bytes.\n#[derive(Clone, Copy, Debug)]\npub struct {name}Codec;\n\nimpl Encoder<{name}> for {name}Codec {{\n    fn encoded_len(&self, value: &{name}) -> Result<usize, Error> {{\n        let size = 4usize;\n"));

    for field in &item.fields {
        output.push_str(&format!(
            "        let size = size.checked_add({}.encoded_len({})?).ok_or(Error::Overflow)?;\n",
            codec(field.wire),
            value(field)
        ));
    }

    if item.fields.is_empty() {
        output.push_str("        let _ = value;\n");
    }

    output.push_str(&format!("        Ok(size)\n    }}\n\n    fn write(&self, value: &{name}, writer: &mut Writer<'_>) -> Result<(), Error> {{\n        Constant(&{tag}u32.to_le_bytes()).write(&(), writer)?;\n"));

    for field in &item.fields {
        output.push_str(&format!(
            "        {}.write({}, writer)?;\n",
            codec(field.wire),
            value(field)
        ));
    }

    if item.fields.is_empty() {
        output.push_str("        let _ = value;\n");
    }

    output.push_str(&format!("        Ok(())\n    }}\n}}\n\nimpl Decoder<{name}> for {name}Codec {{\n    fn read(&self, reader: &mut Reader<'_>) -> Result<{name}, Error> {{\n        Constant(&{tag}u32.to_le_bytes()).read(reader)?;\n        Ok({name} {{\n"));

    for field in &item.fields {
        let expression = format!("{}.read(reader)?", codec(field.wire));

        let expression = match field.wire {
            Wire::Address => format!("Address::from_bytes({expression})"),
            _ => expression,
        };

        output.push_str(&format!("            {}: {expression},\n", field.name));
    }

    output.push_str("        })\n    }\n}\n\n");
    output
}

fn dispatch(items: &[Instruction]) -> String {
    let mut output = "/// All fourteen pinned System instruction data variants.\n#[derive(Clone, Debug, Eq, PartialEq)]\npub enum SystemInstructionData {\n".to_owned();

    for item in items {
        output.push_str(&format!(
            "    /// Data for tag {}.\n    {}({}),\n",
            item.tag, item.rust_name, item.rust_name
        ));
    }

    output.push_str("}\n\n/// Strict tagged codec for [`SystemInstructionData`].\n#[derive(Clone, Copy, Debug)]\npub struct SystemInstructionDataCodec;\n\nimpl Encoder<SystemInstructionData> for SystemInstructionDataCodec {\n    fn encoded_len(&self, value: &SystemInstructionData) -> Result<usize, Error> {\n        match value {\n");

    for item in items {
        output.push_str(&format!(
            "            SystemInstructionData::{}(value) => {}Codec.encoded_len(value),\n",
            item.rust_name, item.rust_name
        ));
    }

    output.push_str("        }\n    }\n\n    fn write(&self, value: &SystemInstructionData, writer: &mut Writer<'_>) -> Result<(), Error> {\n        match value {\n");

    for item in items {
        output.push_str(&format!(
            "            SystemInstructionData::{}(value) => {}Codec.write(value, writer),\n",
            item.rust_name, item.rust_name
        ));
    }

    output.push_str("        }\n    }\n}\n\nimpl Decoder<SystemInstructionData> for SystemInstructionDataCodec {\n    fn read(&self, reader: &mut Reader<'_>) -> Result<SystemInstructionData, Error> {\n        let bytes = reader.remaining().get(..4).ok_or(Error::Truncated)?;\n        let tag = Number::<u32>::LE.decode(bytes, solana_kit_core::codecs::Limits::new(4, 0))?;\n\n        match tag {\n");

    for item in items {
        output.push_str(&format!(
            "            {} => Ok(SystemInstructionData::{}({}Codec.read(reader)?)),\n",
            item.tag, item.rust_name, item.rust_name
        ));
    }

    output.push_str("            _ => Err(Error::InvalidDiscriminator),\n        }\n    }\n}\n");
    output
}

pub(crate) fn render(root: &Value) -> Result<String, Error> {
    let items = schema::parse(root)?;
    let mut output = "// Generated by verification/codegen-design from the pinned System IDL.\n// See SYSTEM-LICENSE and README.md for provenance and scope.\nuse alloc::string::String;\nuse solana_kit_core::{Address, codecs::{Constant, Decoder, Encoder, Error, FixedBytes, Number, Reader, SizePrefix64, Utf8, Writer}};\n\n".to_owned();

    for item in &items {
        output.push_str(&structure(item));
    }

    output.push_str(&dispatch(&items));
    Ok(output)
}

pub(crate) fn replace(root: &Value, output: &Path) -> Result<(), Error> {
    generate::replace_source(&render(root)?, output)
}

pub(crate) fn run() -> Result<(), Error> {
    let root = serde_json::from_str(include_str!("../fixtures/system.json"))?;
    let output = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/solana_kit_system/src/generated.rs");
    replace(&root, &output)?;
    println!("generated all fourteen System instruction data codecs");
    Ok(())
}
