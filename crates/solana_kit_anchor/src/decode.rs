//! Dynamic decoding shares a single Kit cursor and allocation budget.

use crate::{Definition, Error, Field, Idl, Limits, Type, Value, Values};
use alloc::{boxed::Box, string::ToString, vec::Vec};
use solana_kit_core::codecs::{Bytes, Decoder, Number, Reader, SizePrefix};

pub(crate) fn value(
    idl: &Idl,
    ty: &Type,
    reader: &mut Reader<'_>,
    limits: Limits,
    depth: usize,
) -> Result<Value, Error> {
    limits.depth(depth)?;

    match ty {
        Type::Primitive(kind) => crate::scalar::decode(*kind, reader),
        Type::Bytes => Ok(Value::Bytes(SizePrefix(Bytes).read(reader)?)),
        Type::Defined(name) => {
            let definition = idl
                .types
                .get(name)
                .ok_or_else(|| Error::Unknown(name.clone()))?;
            defined(idl, definition, reader, limits, depth + 1)
        }
        Type::Option(inner) => option(idl, inner, reader, limits, depth + 1),
        Type::Vec(inner) => {
            let count =
                usize::try_from(Number::<u32>::LE.read(reader)?).map_err(|_| Error::Limit)?;
            sequence(
                idl,
                inner,
                count,
                reader,
                limits,
                // Collection depth is checked even for empty values.
                depth + 1,
            )
        }
        Type::Array(inner, count) => sequence(
            idl,
            inner,
            *count,
            reader,
            limits,
            // Fixed arrays use the same aggregate allocation budget as vectors.
            depth + 1,
        ),
    }
}

fn option(
    idl: &Idl,
    inner: &Type,
    reader: &mut Reader<'_>,
    limits: Limits,
    depth: usize,
) -> Result<Value, Error> {
    let inner = match Number::<u8>::LE.read(reader)? {
        0 => None,
        1 => {
            reader.claim(1)?;
            Some(Box::new(value(idl, inner, reader, limits, depth)?))
        }
        _ => {
            return Err(Error::Codec(
                solana_kit_core::codecs::Error::InvalidDiscriminator,
            ));
        }
    };

    Ok(Value::Option(inner))
}

fn sequence(
    idl: &Idl,
    inner: &Type,
    count: usize,
    reader: &mut Reader<'_>,
    limits: Limits,
    depth: usize,
) -> Result<Value, Error> {
    limits.depth(depth)?;
    reader.claim(count)?;
    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| Error::Codec(solana_kit_core::codecs::Error::AllocationFailed))?;

    for _ in 0..count {
        values.push(value(idl, inner, reader, limits, depth)?);
    }

    Ok(Value::Sequence(values))
}

pub(crate) fn fields(
    idl: &Idl,
    fields: &[Field],
    reader: &mut Reader<'_>,
    limits: Limits,
    depth: usize,
) -> Result<Values, Error> {
    limits.depth(depth)?;
    reader.claim(fields.len())?;
    let mut values = Values::new();

    for field in fields {
        reader.claim(field.name.len())?;
        let ty = field.parsed_type(limits)?;
        values.insert(
            field.name.clone(),
            value(idl, &ty, reader, limits, depth + 1)?,
        );
    }

    Ok(values)
}

pub(crate) fn defined(
    idl: &Idl,
    definition: &Definition,
    reader: &mut Reader<'_>,
    limits: Limits,
    depth: usize,
) -> Result<Value, Error> {
    limits.depth(depth)?;

    match definition {
        Definition::Struct(entries) => Ok(Value::Struct(fields(
            idl,
            entries,
            reader,
            limits,
            depth + 1,
        )?)),
        Definition::Enum(variants) => {
            let tag = usize::from(Number::<u8>::LE.read(reader)?);
            let variant = variants.get(tag).ok_or(Error::Codec(
                solana_kit_core::codecs::Error::InvalidDiscriminator,
            ))?;
            reader.claim(variant.name.len())?;
            let values = fields(idl, &variant.fields, reader, limits, depth + 1)?;
            Ok(Value::Enum {
                variant: variant.name.to_string(),
                fields: values,
            })
        }
    }
}
