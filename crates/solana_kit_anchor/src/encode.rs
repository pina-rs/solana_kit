//! Bounded dynamic encoding, sharing Kit's scalar and framing implementations.

use crate::{Definition, Error, Field, Idl, Limits, Type, Value, Values};
use alloc::{string::ToString, vec::Vec};
use solana_kit_core::codecs::{Bytes, Encoder, Number, SizePrefix};

pub(crate) struct Output {
    bytes: Vec<u8>,
    items: usize,
    limits: Limits,
}

impl Output {
    pub(crate) fn new(limits: Limits) -> Self {
        Self {
            bytes: Vec::new(),
            items: limits.max_items,
            limits,
        }
    }

    pub(crate) fn put(&mut self, bytes: &[u8]) -> Result<(), Error> {
        if bytes.len() > self.remaining() {
            return Err(Error::Limit);
        }

        self.bytes
            .try_reserve(bytes.len())
            .map_err(|_| Error::Codec(solana_kit_core::codecs::Error::AllocationFailed))?;
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }

    fn claim(&mut self, count: usize) -> Result<(), Error> {
        self.items = self.items.checked_sub(count).ok_or(Error::Limit)?;
        Ok(())
    }

    fn remaining(&self) -> usize {
        self.limits.max_bytes - self.bytes.len()
    }

    pub(crate) fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

pub(crate) fn value(
    idl: &Idl,
    ty: &Type,
    input: &Value,
    output: &mut Output,
    depth: usize,
) -> Result<(), Error> {
    output.limits.depth(depth)?;

    match ty {
        Type::Primitive(kind) => {
            if let Value::String(text) = input {
                output.claim(text.len())?;
            }

            output.put(&crate::scalar::encode(*kind, input, output.remaining())?)
        }
        Type::Bytes => {
            let Value::Bytes(bytes) = input else {
                return Err(Error::Value);
            };
            output.claim(bytes.len())?;

            output.put(&SizePrefix(Bytes).encode(bytes.as_slice(), output.remaining())?)
        }
        Type::Defined(name) => {
            let definition = idl
                .types
                .get(name)
                .ok_or_else(|| Error::Unknown(name.clone()))?;
            defined(idl, definition, input, output, depth + 1)
        }
        Type::Option(inner) => option(idl, inner, input, output, depth + 1),
        Type::Vec(inner) => sequence(idl, inner, None, input, output, depth + 1),
        Type::Array(inner, count) => sequence(idl, inner, Some(*count), input, output, depth + 1),
    }
}

fn option(
    idl: &Idl,
    inner: &Type,
    input: &Value,
    output: &mut Output,
    depth: usize,
) -> Result<(), Error> {
    let Value::Option(inner_value) = input else {
        return Err(Error::Value);
    };
    output.put(&[u8::from(inner_value.is_some())])?;

    if let Some(inner_value) = inner_value {
        output.claim(1)?;
        value(idl, inner, inner_value, output, depth)?;
    }

    Ok(())
}

fn sequence(
    idl: &Idl,
    inner: &Type,
    count: Option<usize>,
    input: &Value,
    output: &mut Output,
    depth: usize,
) -> Result<(), Error> {
    output.limits.depth(depth)?;
    let Value::Sequence(items) = input else {
        return Err(Error::Value);
    };

    if count.is_some_and(|count| count != items.len()) {
        return Err(Error::Value);
    }

    output.claim(items.len())?;

    if count.is_none() {
        let length = u32::try_from(items.len()).map_err(|_| Error::Limit)?;
        output.put(&Number::<u32>::LE.encode(&length, output.remaining())?)?;
    }

    for item in items {
        value(idl, inner, item, output, depth)?;
    }

    Ok(())
}

pub(crate) fn fields(
    idl: &Idl,
    fields: &[Field],
    values: &Values,
    output: &mut Output,
    depth: usize,
) -> Result<(), Error> {
    output.limits.depth(depth)?;
    output.claim(fields.len())?;

    for field in fields {
        let mut operation = || {
            let input = values
                .get(&field.name)
                .ok_or_else(|| Error::Unknown(field.name.clone()))?;
            let ty = field.parsed_type(output.limits)?;
            value(idl, &ty, input, output, depth + 1)
        };
        operation().map_err(|error: Error| error.context(&field.name))?;
    }

    Ok(())
}

pub(crate) fn defined(
    idl: &Idl,
    definition: &Definition,
    input: &Value,
    output: &mut Output,
    depth: usize,
) -> Result<(), Error> {
    output.limits.depth(depth)?;

    match (definition, input) {
        (Definition::Struct(entries), Value::Struct(values)) => {
            fields(idl, entries, values, output, depth + 1)
        }
        (
            Definition::Enum(variants),
            Value::Enum {
                variant,
                fields: values,
            },
        ) => {
            let index = variants
                .iter()
                .position(|entry| entry.name == *variant)
                .ok_or_else(|| Error::Unknown(variant.to_string()))?;
            let tag = u8::try_from(index).map_err(|_| Error::Idl("enum exceeds one-byte tags"))?;

            output.put(&[tag])?;
            fields(idl, &variants[index].fields, values, output, depth + 1)
        }
        _ => Err(Error::Value),
    }
}
