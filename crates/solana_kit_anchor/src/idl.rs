//! Typed Anchor IDL 0.30 entries; field types retain deferred validation.

use crate::{Error, Limits, Type};
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use serde_json::Value as Json;

/// A named or tuple field whose raw type is validated when its codec is used.
#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    /// Name, synthesized as `fieldN` for tuple entries.
    pub name: String,
    /// Immutable-by-borrow JSON type node, preserving unsupported generic nodes.
    pub raw_type: Json,
}

impl Field {
    /// Parses a named or synthesized tuple field with deferred type validation.
    pub fn parse(value: &Json, tuple_index: usize, limits: Limits) -> Result<Self, Error> {
        crate::parse::budget(value, limits)?;
        crate::parse::field(value, tuple_index)
    }

    /// Parses this field's deferred type with explicit recursion and node limits.
    pub fn parsed_type(&self, limits: Limits) -> Result<Type, Error> {
        Type::parse(&self.raw_type, limits)
    }
}

/// A unit, named, or tuple enum variant.
#[derive(Clone, Debug, PartialEq)]
pub struct Variant {
    /// Variant name.
    pub name: String,
    /// Fields in wire order.
    pub fields: Vec<Field>,
}

impl Variant {
    /// Parses a unit, named, tuple, or bare single-field variant.
    pub fn parse(value: &Json, limits: Limits) -> Result<Self, Error> {
        crate::parse::budget(value, limits)?;
        crate::parse::variant(value)
    }
}

/// A named struct or one-byte-tagged enum.
#[derive(Clone, Debug, PartialEq)]
pub enum Definition {
    /// Struct fields in wire order.
    Struct(Vec<Field>),
    /// Enum variants in tag order.
    Enum(Vec<Variant>),
}

/// An instruction entry; account metadata is absent from the pinned Dart model.
#[derive(Clone, Debug, PartialEq)]
pub struct Instruction {
    /// Instruction name.
    pub name: String,
    /// Verbatim explicit prefix, with no synthesized sighash fallback.
    pub discriminator: Vec<u8>,
    /// Arguments in wire order.
    pub args: Vec<Field>,
}

/// An account or event with its exact explicit discriminator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NamedDiscriminator {
    /// Entry name.
    pub name: String,
    /// Explicit prefix; IDL documents may supply non-eight-byte prefixes.
    pub discriminator: Vec<u8>,
}

/// Program-defined error information.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomError {
    /// Numeric program error code.
    pub code: i64,
    /// Error name.
    pub name: String,
    /// Optional message; empty text remains distinct from omission.
    pub message: Option<String>,
}

/// A parsed IDL with source-order events for first-matching discriminator behavior.
#[derive(Clone, Debug, PartialEq)]
pub struct Idl {
    /// Address text, empty when absent; parsing the IDL does not validate it.
    pub address: String,
    /// Metadata name, empty when absent.
    pub name: String,
    /// Instructions keyed by their exact names.
    pub instructions: BTreeMap<String, Instruction>,
    /// Account entries keyed by their exact names.
    pub accounts: BTreeMap<String, NamedDiscriminator>,
    /// Events in document order; duplicate names replace their original position.
    pub events: Vec<NamedDiscriminator>,
    /// Program errors, taking precedence over standard errors for every code.
    pub errors: BTreeMap<i64, CustomError>,
    /// Type definitions keyed by module-stripped name.
    pub types: BTreeMap<String, Definition>,
}

impl Idl {
    /// Parses bounded JSON, retaining deferred unsupported field types.
    ///
    /// # Errors
    /// Rejects malformed structure, missing required names/codes, invalid byte
    /// prefixes, unsupported definition kinds, and exceeded caller budgets.
    pub fn parse(text: &str, limits: Limits) -> Result<Self, Error> {
        if text.len() > limits.max_bytes {
            return Err(Error::Limit);
        }

        let value: Json = serde_json::from_str(text).map_err(|_| Error::Idl("invalid JSON"))?;
        Self::from_value(&value, limits)
    }

    /// Parses an already decoded document after checking its node/depth budgets.
    pub fn from_value(value: &Json, limits: Limits) -> Result<Self, Error> {
        crate::parse::document(value, limits)
    }
}
