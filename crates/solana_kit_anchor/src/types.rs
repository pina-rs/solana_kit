//! Supported scalar and compound IDL type nodes.

use crate::{Error, Limits};
use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use serde_json::Value as Json;

/// Exact supported scalar types; invalid scalar names are unrepresentable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Primitive {
    /// Unsigned 8-bit integer.
    Uint8,
    /// Unsigned 16-bit integer.
    U16,
    /// Unsigned 32-bit integer.
    U32,
    /// Unsigned 64-bit integer.
    U64,
    /// Unsigned 128-bit integer.
    U128,
    /// Signed 8-bit integer.
    Int8,
    /// Signed 16-bit integer.
    I16,
    /// Signed 32-bit integer.
    I32,
    /// Signed 64-bit integer.
    I64,
    /// Signed 128-bit integer.
    I128,
    /// Single precision float.
    F32,
    /// Double precision float.
    F64,
    /// One-byte boolean.
    Bool,
    /// UTF-8 with a u32 byte length.
    String,
    /// Exactly 32 address bytes.
    Pubkey,
}

impl Primitive {
    /// Exact scalar spelling from the pinned IDL format.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Uint8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::U128 => "u128",
            Self::Int8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::I128 => "i128",
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::Bool => "bool",
            Self::String => "string",
            Self::Pubkey => "pubkey",
        }
    }
}

impl core::fmt::Display for Primitive {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(self.name())
    }
}

/// A supported type node; generic instantiation and wide option tags are rejected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    /// Scalar.
    Primitive(Primitive),
    /// Byte string with a u32 byte length.
    Bytes,
    /// A module-stripped named definition.
    Defined(String),
    /// Vector with a u32 element count.
    Vec(Box<Type>),
    /// Nullable value with a one-byte tag.
    Option(Box<Type>),
    /// Array with an exact count and no length prefix.
    Array(Box<Type>, usize),
}

impl core::fmt::Display for Type {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Primitive(primitive) => primitive.fmt(formatter),
            Self::Bytes => formatter.write_str("bytes"),
            Self::Defined(name) => formatter.write_str(name),
            Self::Vec(inner) => write!(formatter, "vec<{inner}>"),
            Self::Option(inner) => write!(formatter, "option<{inner}>"),
            Self::Array(inner, length) => write!(formatter, "[{inner}; {length}]"),
        }
    }
}

/// Strips module paths, preserving an empty final segment like the source.
pub fn type_name(name: &str) -> &str {
    name.rsplit("::").next().unwrap_or(name)
}

impl Type {
    /// Parses one type with caller-selected node, byte, and recursion limits.
    pub fn parse(value: &Json, limits: Limits) -> Result<Self, Error> {
        crate::parse::budget(value, limits)?;
        parse_type(value, limits, 0)
    }
}

fn parse_type(value: &Json, limits: Limits, depth: usize) -> Result<Type, Error> {
    limits.depth(depth)?;

    if let Some(name) = value.as_str() {
        return scalar(name);
    }

    let object = crate::parse::object(value)?;

    if object.contains_key("bytes") {
        return Ok(Type::Bytes);
    }

    if let Some(defined) = object.get("defined").and_then(Json::as_object)
        && let Some(ty) = defined_type(defined)?
    {
        return Ok(ty);
    }

    compound(value, limits, depth)
}

fn compound(value: &Json, limits: Limits, depth: usize) -> Result<Type, Error> {
    if let Some(inner) = value.get("vec").filter(|node| !node.is_null()) {
        return Ok(Type::Vec(Box::new(parse_type(inner, limits, depth + 1)?)));
    }

    if let Some(inner) = value.get("option").filter(|node| !node.is_null()) {
        option_prefix(value.get("prefix"))?;
        return Ok(Type::Option(Box::new(parse_type(
            inner,
            limits,
            depth + 1,
        )?)));
    }

    array_type(value.get("array"), limits, depth)
}

fn defined_type(defined: &serde_json::Map<String, Json>) -> Result<Option<Type>, Error> {
    if defined
        .get("generics")
        .is_some_and(|value| !value.is_null())
    {
        return Err(Error::Idl("generic instantiation is unsupported"));
    }

    Ok(defined
        .get("name")
        .and_then(Json::as_str)
        .map(|name| Type::Defined(type_name(name).to_string())))
}

fn option_prefix(prefix: Option<&Json>) -> Result<(), Error> {
    if prefix.is_some_and(|tag| !tag.is_null() && tag.as_u64() != Some(1)) {
        return Err(Error::Idl("only one-byte option prefixes are supported"));
    }

    Ok(())
}

fn array_type(node: Option<&Json>, limits: Limits, depth: usize) -> Result<Type, Error> {
    let array = node
        .and_then(Json::as_array)
        .filter(|array| array.len() == 2)
        .ok_or(Error::Idl("unsupported type node"))?;
    let length = array[1]
        .as_u64()
        .or_else(|| array[1].as_str().and_then(|text| text.parse().ok()))
        .and_then(|length| usize::try_from(length).ok())
        .ok_or(Error::Idl("array length must be a nonnegative integer"))?;

    if length > limits.max_items {
        return Err(Error::Limit);
    }

    Ok(Type::Array(
        Box::new(parse_type(&array[0], limits, depth + 1)?),
        length,
    ))
}

fn scalar(name: &str) -> Result<Type, Error> {
    let primitive = match name {
        "u8" => Primitive::Uint8,
        "u16" => Primitive::U16,
        "u32" => Primitive::U32,
        "u64" => Primitive::U64,
        "u128" => Primitive::U128,
        "i8" => Primitive::Int8,
        "i16" => Primitive::I16,
        "i32" => Primitive::I32,
        "i64" => Primitive::I64,
        "i128" => Primitive::I128,
        "f32" => Primitive::F32,
        "f64" => Primitive::F64,
        "bool" => Primitive::Bool,
        "string" => Primitive::String,
        "pubkey" => Primitive::Pubkey,
        "bytes" => return Ok(Type::Bytes),
        _ => return Err(Error::Idl("unknown scalar type")),
    };

    Ok(Type::Primitive(primitive))
}
