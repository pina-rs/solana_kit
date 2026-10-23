//! Exact dynamic values and caller-selected resource budgets.

use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};
use solana_kit_core::Address;

const MAX_RECURSION_DEPTH: usize = 128;

/// Fields keyed by name; encoding always follows the IDL declaration order.
pub type Values = BTreeMap<String, Value>;

/// A dynamic Anchor value without JSON integer precision loss.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Unsigned integer, checked against the selected wire width.
    Unsigned(u128),
    /// Signed integer, checked against the selected wire width.
    Signed(i128),
    /// IEEE 754 single precision, preserving its wire bits.
    F32(f32),
    /// IEEE 754 double precision, preserving its wire bits.
    F64(f64),
    /// Boolean.
    Bool(bool),
    /// UTF-8 text.
    String(String),
    /// Kit-owned address.
    Address(Address),
    /// Length-prefixed byte string.
    Bytes(Vec<u8>),
    /// Fixed array or length-prefixed vector.
    Sequence(Vec<Value>),
    /// Explicit nullable value; nested options retain every tag.
    Option(Option<Box<Value>>),
    /// Struct fields.
    Struct(Values),
    /// A named enum variant with named or synthesized tuple fields.
    Enum {
        /// Variant name.
        variant: String,
        /// Variant fields (`field0`, `field1`, … for tuples).
        fields: Values,
    },
}

/// Budgets apply independently to each parse, encode, decode, or log operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    /// Maximum JSON, wire, or total log input/output bytes.
    pub max_bytes: usize,
    /// Maximum aggregate collection entries, fields, and allocated byte strings.
    pub max_items: usize,
    /// Maximum nested type/value depth, also capped at 128 for stack safety.
    pub max_depth: usize,
}

impl Limits {
    /// Selects explicit budgets. No ambient defaults are consulted.
    pub const fn new(max_bytes: usize, max_items: usize, max_depth: usize) -> Self {
        Self {
            max_bytes,
            max_items,
            max_depth,
        }
    }

    pub(crate) fn depth(self, depth: usize) -> Result<(), crate::Error> {
        if depth > self.max_depth.min(MAX_RECURSION_DEPTH) {
            return Err(crate::Error::Limit);
        }

        Ok(())
    }
}
