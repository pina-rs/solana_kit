//! Concrete Rust types and codec expressions from normalized wire nodes.
use super::{Program, Wire, extent};
use crate::schema::Error;

pub(super) fn rust_type(wire: &Wire) -> String {
    match wire {
        Wire::Number(name) => (*name).to_owned(),
        Wire::Link(name) => name.clone(),
        Wire::Address => "Address".to_owned(),
        Wire::Bool => "bool".to_owned(),
        Wire::Text => "String".to_owned(),
        Wire::Bytes => "Vec<u8>".to_owned(),
        Wire::Option { item, .. } => format!("Option<{}>", rust_type(item)),
        Wire::Array { item, .. } => format!("Vec<{}>", rust_type(item)),
        Wire::Prefix { item, .. } => rust_type(item),
    }
}

pub(super) fn value(wire: &Wire, name: &str) -> String {
    match wire {
        Wire::Bytes => format!("{name}.as_slice()"),
        Wire::Text => format!("{name}.as_str()"),
        Wire::Prefix { item, .. } => value(item, name),
        _ => format!("&{name}"),
    }
}

/// Each node chooses a concrete typed codec; recursive children share validation.
pub(super) fn codec(wire: &Wire, program: &Program) -> Result<String, Error> {
    let output = match wire {
        Wire::Number(name) => format!("Number::<{name}>::LE"),
        Wire::Link(name) => format!("{name}Codec"),
        Wire::Address => address(),
        Wire::Bool => "Bool".to_owned(),
        Wire::Text => "Utf8".to_owned(),
        Wire::Bytes => "Bytes".to_owned(),
        Wire::Option {
            item,
            prefix,
            fixed,
        } => option(item, prefix, *fixed, program)?,
        Wire::Array { item, count } => array(item, *count, program)?,
        Wire::Prefix { item, prefix } => prefix_bytes(item, prefix, program)?,
    };

    Ok(output)
}

fn address() -> String {
    "Mapped::new(FixedBytes::<32>, |value: &Address| Ok::<_, Error>(*value.as_bytes()), |bytes| Ok::<_, Error>(Address::from_bytes(bytes)))".to_owned()
}

fn option(item: &Wire, prefix: &str, fixed: bool, program: &Program) -> Result<String, Error> {
    let inner = codec(item, program)?;

    if !fixed {
        return Ok(format!("OptionCodec::{prefix}({inner})"));
    }

    let width = extent::wire(program, item, &mut Vec::new())?
        .fixed
        .ok_or("fixed width missing")?;
    Ok(format!("OptionCodec::c_option({inner}, {width})"))
}

fn array(item: &Wire, count: Option<usize>, program: &Program) -> Result<String, Error> {
    let inner = codec(item, program)?;
    let count = count
        .map(|count| format!("Count::Fixed(FIXED_COUNT_{count})"))
        .unwrap_or_else(|| "Count::Remainder".to_owned());
    Ok(format!(
        "OwnedSequence(Sequence {{item:{inner},count:{count}}})"
    ))
}

fn prefix_bytes(item: &Wire, prefix: &str, program: &Program) -> Result<String, Error> {
    let inner = codec(item, program)?;
    let prefix = prefix_codec(prefix);
    Ok(format!(
        "ByteSizePrefix {{ item:{inner},prefix:{prefix},consumption:FrameConsumption::Exact }}"
    ))
}
fn prefix_codec(prefix: &str) -> String {
    if prefix == "u64" {
        return "Number::<u64>::LE".to_owned();
    }

    format!(
        "Mapped::new(Number::<{prefix}>::LE, |value: &u64| {prefix}::try_from(*value).map_err(|_| Error::Overflow), |value: {prefix}| Ok::<_, Error>(u64::from(value)))"
    )
}
