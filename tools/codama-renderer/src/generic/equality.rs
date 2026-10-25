//! Floating point models expose PartialEq without claiming the Eq contract.
use super::{Field, Program, Shape, Wire};

pub(super) fn derives(program: &Program) -> &'static str {
    let records = program
        .definitions
        .iter()
        .chain(program.accounts.iter().map(|item| &item.definition));
    let floats = records.into_iter().any(|item| match &item.shape {
        Shape::Struct(fields) => floating_fields(fields),
        #[cfg(feature = "verification")]
        Shape::Enum(_) => false,
        Shape::Tagged { variants, .. } => variants
            .iter()
            .filter_map(|variant| variant.payload.as_ref())
            .any(floating),
    }) || program
        .instructions
        .iter()
        .any(|item| floating_fields(&item.fields));

    if floats {
        "Clone, Debug, PartialEq"
    } else {
        "Clone, Debug, Eq, PartialEq"
    }
}

fn floating_fields(fields: &[Field]) -> bool {
    fields.iter().any(|field| floating(&field.wire))
}

fn floating(wire: &Wire) -> bool {
    match wire {
        Wire::Number("f64") => true,
        Wire::Option { item, .. }
        | Wire::Array { item, .. }
        | Wire::Prefix { item, .. }
        | Wire::Counted { item, .. } => floating(item),
        Wire::RemainderOption(item) | Wire::HiddenPrefix { item, .. } => floating(item),
        #[cfg(feature = "verification")]
        Wire::TlvSequence(item) => floating(item),
        Wire::Tuple(a, b) => floating(a) || floating(b),
        _ => false,
    }
}
