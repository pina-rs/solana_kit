//! Exact Rust values replace the reference's dynamically cast schema map.
use super::scalars::{FieldString, ScalarChar};
use crate::SchemaDataType;
use alloc::{string::String, vec::Vec};
use solana_kit_core::codecs::{
    Bool, Count, Decoder, Encoder, Error, Number, OwnedSequence, Reader, Sequence, Writer,
};

fn vector<C>(item: C) -> OwnedSequence<C> {
    OwnedSequence(Sequence {
        item,
        count: Count::U32,
    })
}

// Keep the generated operations separate from the enum declaration: all four
// dispatches share the same schema-to-codec table passed by `values!`.
macro_rules! value_methods {
    ($($variant:ident($ty:ty) => $codec:expr),+ $(,)?) => {
        /// Returns the source schema tag without numeric coercion.
        #[must_use]
        pub fn data_type(&self) -> SchemaDataType {
            match self { $(Self::$variant(_) => SchemaDataType::$variant),+ }
        }

        pub(super) fn encoded_len(&self) -> Result<usize, Error> {
            match self { $(Self::$variant(value) => $codec.encoded_len(value)),+ }
        }

        pub(super) fn write(&self, writer: &mut Writer<'_>) -> Result<(), Error> {
            match self { $(Self::$variant(value) => $codec.write(value, writer)),+ }
        }

        pub(super) fn read(
            kind: SchemaDataType,
            reader: &mut Reader<'_>,
        ) -> Result<Self, Error> {
            match kind {
                $(SchemaDataType::$variant => Ok(Self::$variant($codec.read(reader)?))),+
            }
        }
    };
}

macro_rules! values {
    ($($variant:ident($ty:ty) => $codec:expr),+ $(,)?) => {
        /// A value with one of the 26 exact types declared by a schema.
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum FieldValue {
            $(#[doc = concat!("A schema `", stringify!($variant), "` value.")]
            $variant($ty)),+
        }

        impl FieldValue {
            value_methods!($($variant($ty) => $codec),+);
        }
    };
}

values! {
    U8(u8) => Number::<u8>::LE,
    U16(u16) => Number::<u16>::LE,
    U32(u32) => Number::<u32>::LE,
    U64(u64) => Number::<u64>::LE,
    U128(u128) => Number::<u128>::LE,
    I8(i8) => Number::<i8>::LE,
    I16(i16) => Number::<i16>::LE,
    I32(i32) => Number::<i32>::LE,
    I64(i64) => Number::<i64>::LE,
    I128(i128) => Number::<i128>::LE,
    Bool(bool) => Bool,
    Char(char) => ScalarChar,
    String(String) => FieldString,
    VecU8(Vec<u8>) => vector(Number::<u8>::LE),
    VecU16(Vec<u16>) => vector(Number::<u16>::LE),
    VecU32(Vec<u32>) => vector(Number::<u32>::LE),
    VecU64(Vec<u64>) => vector(Number::<u64>::LE),
    VecU128(Vec<u128>) => vector(Number::<u128>::LE),
    VecI8(Vec<i8>) => vector(Number::<i8>::LE),
    VecI16(Vec<i16>) => vector(Number::<i16>::LE),
    VecI32(Vec<i32>) => vector(Number::<i32>::LE),
    VecI64(Vec<i64>) => vector(Number::<i64>::LE),
    VecI128(Vec<i128>) => vector(Number::<i128>::LE),
    VecBool(Vec<bool>) => vector(Bool),
    VecChar(Vec<char>) => vector(ScalarChar),
    VecString(Vec<String>) => vector(FieldString),
}
