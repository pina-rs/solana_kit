//! Names protocol counts and discriminators in generated code.
use super::{Field, Program, Shape, Wire};
use std::collections::BTreeSet;

pub(super) fn extended(program: &Program) -> bool {
    !super::instruction_header::byte_dispatch(program)
        || !program.accounts.is_empty()
        || !program.errors.is_empty()
        || program
            .instructions
            .iter()
            .any(|item| !item.roles.is_empty() || item.remaining.is_some())
}
pub(super) fn tag(_program: &Program, tag: &super::instruction_header::Header) -> String {
    if tag.bytes().len() != 1 {
        return tag.literal();
    }

    format!("INSTRUCTION_TAG_{tag}")
}
/// Name constants by protocol category and retain deterministic order.
pub(super) fn emit(program: &Program) -> String {
    let mut output = String::new();

    for instruction in &program.instructions {
        if instruction.tag.bytes().len() != 1 {
            continue;
        }

        output.push_str(&format!(
            "const INSTRUCTION_TAG_{}:u8={};\n",
            instruction.tag, instruction.tag
        ));
    }

    for error in &program.errors {
        output.push_str(&format!(
            "const PROGRAM_ERROR_{}:u32={};\n",
            error.code, error.code
        ));
    }

    for account in &program.accounts {
        output.push_str(&format!(
            "const ACCOUNT_SIZE_{}:usize={};\n",
            account.size, account.size
        ));
    }

    output.push_str(&fixed_counts(program));
    output
}

fn fixed_counts(program: &Program) -> String {
    let mut output = String::new();
    let mut counts = BTreeSet::new();

    for definition in &program.definitions {
        shape(&definition.shape, &mut counts);
    }

    for account in &program.accounts {
        shape(&account.definition.shape, &mut counts);
    }

    for instruction in &program.instructions {
        fields(&instruction.fields, &mut counts);
    }

    for count in counts {
        output.push_str(&format!("const FIXED_COUNT_{count}:usize={count};\n"));
    }

    output
}
fn shape(shape: &Shape, counts: &mut BTreeSet<usize>) {
    if let Shape::Tagged { variants, .. } = shape {
        for payload in variants
            .iter()
            .filter_map(|variant| variant.payload.as_ref())
        {
            wire(payload, counts);
        }
    }

    if let Shape::Struct(items) = shape {
        fields(items, counts);
    }
}
fn fields(fields: &[Field], counts: &mut BTreeSet<usize>) {
    for field in fields {
        wire(&field.wire, counts);
    }
}
fn wire(value: &Wire, counts: &mut BTreeSet<usize>) {
    match value {
        Wire::Array { item, count } => {
            counts.extend(count);
            wire(item, counts);
        }
        Wire::Option { item, .. } | Wire::Prefix { item, .. } => wire(item, counts),
        Wire::Counted { item, .. } => wire(item, counts),
        Wire::RemainderOption(item) | Wire::HiddenPrefix { item, .. } => wire(item, counts),
        #[cfg(feature = "verification")]
        Wire::TlvSequence(item) => wire(item, counts),
        Wire::Tuple(a, b) => {
            wire(a, counts);
            wire(b, counts);
        }
        _ => {}
    }
}
