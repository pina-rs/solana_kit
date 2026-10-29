//! Import requirements follow typed IR nodes, never generated source text.
use super::{CountPrefix, Field, Numeric, Program, Shape, Wire};
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
        shape(&definition.shape, &mut required);
    }

    for instruction in &program.instructions {
        fields(&instruction.fields, &mut required);
    }

    if super::constants::extended(program) {
        required.extend(["AccountMeta", "AccountRole"]);
    }

    required
}

fn shape(shape: &Shape, required: &mut Imports) {
    match shape {
        Shape::Struct(items) => fields(items, required),
        #[cfg(feature = "verification")]
        Shape::Enum(_) => {
            required.insert("Number");
        }
        Shape::Tagged { variants, .. } => {
            required.insert("Number");

            for payload in variants
                .iter()
                .filter_map(|variant| variant.payload.as_ref())
            {
                wire(payload, required);
            }
        }
    }
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
        #[cfg(feature = "verification")]
        Wire::TlvSequence(item) => sequence(item, true, required),
        Wire::RemainderOption(item) => optional(item, false, required),
        Wire::HiddenPrefix { item, .. } => {
            required.insert("Affix");
            wire(item, required);
        }
        Wire::Address | Wire::ZeroableAddress => {
            required.extend(["Address", "Mapped", "FixedBytes"])
        }
        Wire::FixedBytes(_) => {
            required.insert("FixedBytes");
        }
        Wire::Bool => {
            required.insert("Bool");
        }
        Wire::Text => required.extend(["Utf8", "String"]),
        Wire::FixedText(_) => required.extend(["FixedSize", "Utf8Config", "String"]),
        Wire::Bytes => {
            required.insert("Bytes");
        }
        Wire::Option {
            item,
            prefix,
            fixed,
        } => prefixed_option(item, prefix, *fixed, required),
        Wire::Array { item, .. } => sequence(item, false, required),
        Wire::Prefix { item, prefix } => {
            prefix_imports(prefix, required);
            wire(item, required);
        }
        Wire::Counted { item, prefix } => {
            counted(prefix, required);
            owned(item, required);
            wire(item, required);
        }
        Wire::Tuple(a, b) => tuple(a, b, required),
    }
}

fn prefixed_option(item: &Wire, prefix: &Numeric, fixed: bool, required: &mut Imports) {
    if prefix.legacy_option() {
        optional(item, fixed && prefix.format == "u8", required);

        return;
    }

    nullable(item, prefix, required);
}

fn optional(item: &Wire, fixed_byte: bool, required: &mut Imports) {
    option(fixed_byte, required);
    owned(item, required);
    wire(item, required);
}

fn sequence(item: &Wire, terminated: bool, required: &mut Imports) {
    required.extend(["OwnedSequence", "Sequence", "Count"]);

    if !terminated {
        owned(item, required);
    }

    if terminated {
        required.insert("TlvSequence");
    }

    wire(item, required);
}

fn prefix_imports(prefix: &Numeric, required: &mut Imports) {
    required.extend(["ByteSizePrefix", "FrameConsumption", "Number"]);

    if prefix.format != "u64" {
        required.insert("Mapped");
    }
}

fn tuple(a: &Wire, b: &Wire, required: &mut Imports) {
    required.insert("Tuple");
    owned(a, required);
    owned(b, required);

    wire(a, required);
    wire(b, required);
}

fn owned(item: &Wire, required: &mut Imports) {
    if let Some(adapter) = super::expressions::owned_adapter(item) {
        required.insert(adapter);
    }
}

fn counted(prefix: &CountPrefix, required: &mut Imports) {
    required.insert("Counted");

    if !matches!(prefix, CountPrefix::Number(number) if number.format == "u32") {
        required.insert("Mapped");
    }

    let number = match prefix {
        CountPrefix::ShortU16 => "ShortU16",
        _ => "Number",
    };

    required.insert(number);
}

fn nullable(item: &Wire, prefix: &Numeric, required: &mut Imports) {
    required.extend(["Nullable", "Boolean", "Absence", "Number"]);

    if prefix.format != "u32" {
        required.insert("Mapped");
    }

    owned(item, required);
    wire(item, required);
}

fn option(fixed_byte: bool, required: &mut Imports) {
    required.insert("OptionCodec");

    if fixed_byte {
        required.insert("FixedSize");
    }
}
