//! Public instruction, account, and composable value operations.

use crate::{Definition, Error, Idl, Limits, Type, Value, Values, decode, encode};
use alloc::{string::ToString, vec::Vec};
use solana_kit_core::codecs::{Limits as WireLimits, Reader};

/// A prefix decode preserves the source's trailing-data policy explicitly.
#[derive(Clone, Debug, PartialEq)]
pub struct DecodedValue<T> {
    /// Decoded value.
    pub value: T,
    /// Number of bytes consumed, including a discriminator when present.
    pub consumed: usize,
}

/// Account fields and the exact matched IDL prefix.
#[derive(Clone, Debug, PartialEq)]
pub struct DecodedAccount {
    /// Explicit account discriminator.
    pub discriminator: Vec<u8>,
    /// Struct fields or enum variant, represented without magic map keys.
    pub data: Value,
    /// Total consumed bytes; account allocation padding may follow.
    pub consumed: usize,
}

/// An immutable borrowed IDL and explicit per-operation resource policy.
#[derive(Clone, Copy, Debug)]
pub struct Coder<'a> {
    /// IDL backing every operation.
    pub idl: &'a Idl,
    /// Explicit caller budgets.
    pub limits: Limits,
}

impl<'a> Coder<'a> {
    /// Borrows a document; no cache, executor, or network operation is started.
    pub const fn new(idl: &'a Idl, limits: Limits) -> Self {
        Self { idl, limits }
    }

    /// Encodes arguments in IDL order after its verbatim explicit prefix.
    pub fn encode_instruction(&self, name: &str, args: &Values) -> Result<Vec<u8>, Error> {
        let instruction = self
            .idl
            .instructions
            .get(name)
            .ok_or_else(|| Error::Unknown(name.to_string()))?;
        let mut output = encode::Output::new(self.limits);
        output.put(&instruction.discriminator)?;
        encode::fields(self.idl, &instruction.args, args, &mut output, 0)?;
        Ok(output.finish())
    }

    /// Validates the prefix and returns fields plus the consumed byte count.
    pub fn decode_instruction(
        &self,
        name: &str,
        bytes: &[u8],
    ) -> Result<DecodedValue<Values>, Error> {
        let instruction = self
            .idl
            .instructions
            .get(name)
            .ok_or_else(|| Error::Unknown(name.to_string()))?;
        let mut reader = self.reader(bytes)?;
        strip(
            &mut reader,
            &instruction.discriminator,
            self.limits.max_bytes,
        )?;
        let value = decode::fields(self.idl, &instruction.args, &mut reader, self.limits, 0)?;
        Ok(DecodedValue {
            value,
            consumed: reader.position(),
        })
    }

    /// Prefixes a struct or enum account, attributing failures to its name.
    pub fn encode_account(&self, name: &str, value: &Value) -> Result<Vec<u8>, Error> {
        let operation = || {
            let account = self
                .idl
                .accounts
                .get(name)
                .ok_or_else(|| Error::Unknown(name.to_string()))?;
            let definition = self.definition(name)?;
            let mut output = encode::Output::new(self.limits);
            output.put(&account.discriminator)?;
            encode::defined(self.idl, definition, value, &mut output, 0)?;
            Ok(output.finish())
        };
        operation().map_err(|error: Error| error.context(name))
    }

    /// Decodes an account prefix while retaining allocation padding explicitly.
    pub fn decode_account(&self, name: &str, bytes: &[u8]) -> Result<DecodedAccount, Error> {
        let account = self
            .idl
            .accounts
            .get(name)
            .ok_or_else(|| Error::Unknown(name.to_string()))?;
        let mut reader = self.reader(bytes)?;
        strip(&mut reader, &account.discriminator, self.limits.max_bytes)?;
        let data = decode::defined(
            self.idl,
            self.definition(name)?,
            &mut reader,
            self.limits,
            0,
        )?;
        Ok(DecodedAccount {
            discriminator: account.discriminator.clone(),
            data,
            consumed: reader.position(),
        })
    }

    /// Validates a composable runtime type and all referenced definitions.
    pub fn codec_for<'b>(&'b self, ty: &'b Type) -> Result<ValueCodec<'a, 'b>, Error> {
        let mut items = self.limits.max_items;
        validate(self, ty, 0, &mut items)?;
        Ok(ValueCodec { coder: *self, ty })
    }

    pub(crate) fn reader<'b>(&self, bytes: &'b [u8]) -> Result<Reader<'b>, Error> {
        Ok(Reader::new(
            bytes,
            WireLimits::new(self.limits.max_bytes, self.limits.max_items),
        )?)
    }

    pub(crate) fn definition(&self, name: &str) -> Result<&'a Definition, Error> {
        self.idl
            .types
            .get(name)
            .ok_or_else(|| Error::Unknown(name.to_string()))
    }
}

/// A borrowed, validated runtime codec; full and prefix reads are distinct.
pub struct ValueCodec<'a, 'b> {
    coder: Coder<'a>,
    ty: &'b Type,
}

impl ValueCodec<'_, '_> {
    /// Encodes the selected dynamic type within the coder's byte/item budgets.
    pub fn encode(&self, value: &Value) -> Result<Vec<u8>, Error> {
        let mut output = encode::Output::new(self.coder.limits);
        encode::value(self.coder.idl, self.ty, value, &mut output, 0)?;
        Ok(output.finish())
    }

    /// Reads a value and rejects trailing bytes.
    pub fn decode(&self, bytes: &[u8]) -> Result<Value, Error> {
        let decoded = self.decode_prefix(bytes)?;

        if decoded.consumed != bytes.len() {
            return Err(Error::Codec(solana_kit_core::codecs::Error::TrailingBytes));
        }

        Ok(decoded.value)
    }

    /// Reads one value, returning its consumed offset like the Dart reader pair.
    pub fn decode_prefix(&self, bytes: &[u8]) -> Result<DecodedValue<Value>, Error> {
        let mut reader = self.coder.reader(bytes)?;
        let value = decode::value(self.coder.idl, self.ty, &mut reader, self.coder.limits, 0)?;
        Ok(DecodedValue {
            value,
            consumed: reader.position(),
        })
    }
}

pub(crate) fn strip(
    reader: &mut Reader<'_>,
    expected: &[u8],
    max_bytes: usize,
) -> Result<(), Error> {
    // Mismatch diagnostics must not clone an IDL prefix beyond this operation's
    // byte policy when the IDL was parsed under a larger budget.
    if expected.len() > max_bytes {
        return Err(Error::Limit);
    }

    if !reader.remaining().starts_with(expected) {
        let found = reader
            .remaining()
            .iter()
            .take(expected.len())
            .copied()
            .collect();
        return Err(Error::Discriminator {
            expected: expected.to_vec(),
            found,
        });
    }

    reader.take(expected.len())?;
    Ok(())
}

fn validate(coder: &Coder<'_>, ty: &Type, depth: usize, items: &mut usize) -> Result<(), Error> {
    coder.limits.depth(depth)?;
    *items = items.checked_sub(1).ok_or(Error::Limit)?;

    match ty {
        Type::Defined(name) => {
            validate_definition(coder, coder.definition(name)?, depth + 1, items)?;
        }
        Type::Array(inner, count) => {
            if *count > coder.limits.max_items {
                return Err(Error::Limit);
            }

            validate(coder, inner, depth + 1, items)?;
        }
        Type::Vec(inner) | Type::Option(inner) => validate(coder, inner, depth + 1, items)?,
        _ => {}
    }

    Ok(())
}

fn validate_definition(
    coder: &Coder<'_>,
    definition: &Definition,
    depth: usize,
    items: &mut usize,
) -> Result<(), Error> {
    match definition {
        Definition::Struct(fields) => validate_fields(coder, fields, depth, items),
        Definition::Enum(variants) => {
            for variant in variants {
                validate_fields(coder, &variant.fields, depth, items)?;
            }

            Ok(())
        }
    }
}

fn validate_fields(
    coder: &Coder<'_>,
    fields: &[crate::Field],
    depth: usize,
    items: &mut usize,
) -> Result<(), Error> {
    for field in fields {
        validate(coder, &field.parsed_type(coder.limits)?, depth, items)?;
    }

    Ok(())
}
