//! Discriminator predicates inspect the declared record bytes without changing its layout.

pub(super) fn write_start(discriminator: Option<&[u8]>) -> String {
    match discriminator {
        Some(bytes) => format!(
            "const _DISCRIMINATOR: &[u8] = &{bytes:?};\nlet _record_start = writer.position();\n"
        ),
        None => String::new(),
    }
}

pub(super) fn write_end(discriminator: Option<&[u8]>) -> &'static str {
    match discriminator {
        Some(_) => {
            "let _record_bytes = writer.bytes().get(_record_start..writer.position()).ok_or(Error::InvalidDiscriminator)?;\nif !_record_bytes.starts_with(_DISCRIMINATOR) { return Err(Error::InvalidDiscriminator); }\n"
        }
        None => "",
    }
}

pub(super) fn read_start(discriminator: Option<&[u8]>) -> String {
    let Some(bytes) = discriminator else {
        return String::new();
    };

    format!(
        "const _DISCRIMINATOR: &[u8] = &{bytes:?};\nlet _record_start = reader.position();\nlet _actual_discriminator = reader.remaining().get(.._DISCRIMINATOR.len()).ok_or(Error::Truncated)?;\nif _actual_discriminator != _DISCRIMINATOR {{ return Err(Error::InvalidDiscriminator); }}\n"
    )
}

/// A record cannot satisfy its predicate with bytes belonging to a following field.
pub(super) fn read_end(discriminator: Option<&[u8]>) -> &'static str {
    match discriminator {
        Some(_) => {
            "let _record_bytes = reader.bytes().get(_record_start..reader.position()).ok_or(Error::InvalidDiscriminator)?;\nif !_record_bytes.starts_with(_DISCRIMINATOR) { return Err(Error::InvalidDiscriminator); }\n"
        }
        None => "",
    }
}
