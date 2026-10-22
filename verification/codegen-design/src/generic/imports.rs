//! Import requirements follow typed IR nodes, never generated source text.
use super::{Field, Program, Shape, Wire};
use std::collections::BTreeSet;
/// Required codec and ownership symbols, collected from the entire validated model.
pub(super) type Imports = BTreeSet<&'static str>;

/// Traverse every emitted declaration so unused and missing imports cannot drift.
pub(super) fn collect(program: &Program) -> Imports {
    let mut required = Imports::new();

    for definition in program
        .definitions
        .iter()
        .chain(program.accounts.iter().map(|account| &account.definition))
    {
        match &definition.shape {
            Shape::Struct(items) => fields(items, &mut required),
            Shape::Enum(_) => {
                required.insert("Number");
            }
        }
    }

    for instruction in &program.instructions {
        fields(&instruction.fields, &mut required);
    }

    if super::constants::extended(program) {
        required.extend(["AccountMeta", "AccountRole"]);
    }

    required
}
fn fields(fields: &[Field], required: &mut Imports) {
    for field in fields {
        wire(&field.wire, required);
    }
}

/// Codec composition determines imports independently of arbitrary data literals.
fn wire(value: &Wire, required: &mut Imports) {
    match value {
        Wire::Number(_) => {
            required.insert("Number");
        }
        Wire::Link(_) => {}
        Wire::Address => required.extend(["Mapped", "FixedBytes"]),
        Wire::Bool => {
            required.insert("Bool");
        }
        Wire::Text => required.extend(["Utf8", "String"]),
        Wire::Bytes => {
            required.insert("Bytes");
        }
        Wire::Option { item, .. } => {
            required.insert("OptionCodec");
            wire(item, required);
        }
        Wire::Array { item, .. } => {
            required.extend(["OwnedSequence", "Sequence", "Count"]);
            wire(item, required);
        }
        Wire::Prefix { item, prefix } => {
            required.extend(["ByteSizePrefix", "FrameConsumption", "Number"]);
            if *prefix != "u64" {
                required.insert("Mapped");
            }

            wire(item, required);
        }
    }
}
