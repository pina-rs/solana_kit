//! Concrete Rust types and codec expressions from normalized wire nodes.
use super::{Program, Wire, extent};
use crate::schema::Error;

pub(super) fn rust_type(wire: &Wire) -> String {
    match wire {
        Wire::Number(name) => (*name).to_owned(),
        Wire::Link(name) => name.clone(),
        Wire::Address => "Address".to_owned(),
        Wire::Bool => "bool".to_owned(),
        Wire::Text | Wire::FixedText(_) => "String".to_owned(),
        Wire::Bytes => "Vec<u8>".to_owned(),
        Wire::FixedBytes(size) => format!("[u8; {size}]"),
        Wire::ZeroableAddress => "Option<Address>".to_owned(),
        Wire::RemainderOption(item) => format!("Option<{}>", rust_type(item)),
        #[cfg(feature = "verification")]
        Wire::TlvSequence(item) => format!("Vec<{}>", rust_type(item)),
        Wire::HiddenPrefix { item, .. } => rust_type(item),
        Wire::Option { item, .. } => format!("Option<{}>", rust_type(item)),
        Wire::Array { item, .. } => format!("Vec<{}>", rust_type(item)),
        Wire::Prefix { item, .. } => rust_type(item),
        Wire::Counted { item, .. } => format!("Vec<{}>", rust_type(item)),
        Wire::Tuple(a, b) => format!("({}, {})", rust_type(a), rust_type(b)),
    }
}

pub(super) fn value(wire: &Wire, name: &str) -> String {
    match wire {
        Wire::Bytes => format!("{name}.as_slice()"),
        Wire::Text | Wire::FixedText(_) => format!("{name}.as_str()"),
        Wire::Prefix { item, .. } => value(item, name),
        Wire::HiddenPrefix { item, .. } => value(item, name),
        Wire::Counted { .. } => format!("{name}.as_slice()"),
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
        Wire::FixedBytes(size) => format!("FixedBytes::<{size}>"),
        Wire::FixedText(size) => format!("FixedSize {{ item: Utf8Config::default(), size: {size}, decoded_size: None, allow_truncation: false, max_encoded_size: {size} }}"),
        Wire::ZeroableAddress => "Mapped::new(FixedBytes::<32>, |value: &Option<Address>| Ok::<_, Error>(value.map_or([0;32], |address| *address.as_bytes())), |bytes| Ok::<_, Error>((bytes != [0;32]).then_some(Address::from_bytes(bytes))))".to_owned(),
        Wire::RemainderOption(item) => format!("OptionCodec::remainder({})", owned_codec(item, program)?),
        #[cfg(feature = "verification")]
        Wire::TlvSequence(item) => super::tlv_output::codec(item, program)?,
        Wire::HiddenPrefix { item, bytes } => format!("Affix {{ prefix: &[{}], item: {}, suffix: &[] }}", bytes.iter().map(u8::to_string).collect::<Vec<_>>().join(","), codec(item, program)?),
        Wire::Option {
            item,
            prefix,
            fixed,
        } => option(item, prefix, *fixed, program)?,
        Wire::Array { item, count } => array(item, *count, program)?,
        Wire::Prefix { item, prefix } => prefix_bytes(item, prefix, program)?,
        Wire::Counted { item, prefix } => format!(
            "CheckedCounted {{ prefix: {}, item: {}, minimum: {} }}",
            count_prefix(prefix),
            owned_codec(item, program)?,
            extent::wire(program, item, &mut Vec::new())?.minimum
        ),
        Wire::Tuple(a, b) => format!("Tuple({}, {})", owned_codec(a, program)?, owned_codec(b, program)?),
    };

    Ok(output)
}

fn owned_codec(wire: &Wire, program: &Program) -> Result<String, Error> {
    let codec = codec(wire, program)?;

    match owned_adapter(wire) {
        Some(adapter) => Ok(format!("{adapter}({codec})")),
        None => Ok(codec),
    }
}

/// Only codecs borrowing an unsized value need an adapter at an owned boundary.
pub(super) fn owned_adapter(wire: &Wire) -> Option<&'static str> {
    match wire {
        Wire::Text | Wire::FixedText(_) => Some("OwnedString"),
        Wire::Counted { .. } | Wire::Bytes => Some("OwnedVector"),
        Wire::Prefix { item, .. } | Wire::HiddenPrefix { item, .. } => owned_adapter(item),
        _ => None,
    }
}

fn address() -> String {
    "Mapped::new(FixedBytes::<32>, |value: &Address| Ok::<_, Error>(*value.as_bytes()), |bytes| Ok::<_, Error>(Address::from_bytes(bytes)))".to_owned()
}

fn option(item: &Wire, prefix: &str, fixed: bool, program: &Program) -> Result<String, Error> {
    let inner = owned_codec(item, program)?;

    if !fixed {
        return Ok(format!("OptionCodec::{prefix}({inner})"));
    }

    let width = extent::wire(program, item, &mut Vec::new())?
        .fixed
        .ok_or("fixed width missing")?;

    if prefix == "u8" {
        let size = width.checked_add(1).ok_or("fixed option overflow")?;
        return Ok(format!(
            "FixedSize {{ item: OptionCodec::u8({inner}), size: {size}, decoded_size: None, allow_truncation: false, max_encoded_size: {size} }}"
        ));
    }

    Ok(format!("OptionCodec::c_option({inner}, {width})"))
}

fn count_prefix(prefix: &str) -> String {
    match prefix {
        "u32" => "Number::<u32>::LE".to_owned(),
        "shortU16" => "Mapped::new(ShortU16, |value: &u32| u16::try_from(*value).map_err(|_| Error::Overflow), |value: u16| Ok::<_, Error>(u32::from(value)))".to_owned(),
        "u64" => "Mapped::new(Number::<u64>::LE, |value: &u32| Ok::<_, Error>(u64::from(*value)), |value: u64| u32::try_from(value).map_err(|_| Error::Overflow))".to_owned(),
        name => format!("Mapped::new(Number::<{name}>::LE, |value: &u32| {name}::try_from(*value).map_err(|_| Error::Overflow), |value: {name}| Ok::<_, Error>(u32::from(value)))"),
    }
}

fn array(item: &Wire, count: Option<usize>, program: &Program) -> Result<String, Error> {
    let inner = owned_codec(item, program)?;
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
