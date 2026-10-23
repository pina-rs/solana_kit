//! Scalar wire machinery delegates to Kit's numeric, byte and text codecs.

use crate::{Error, Primitive, Value};
use alloc::vec::Vec;
use solana_kit_core::{
    Address,
    codecs::{Bool, Decoder, Encoder, FixedBytes, Number, Reader, SizePrefix, Utf8},
};

macro_rules! unsigned {
    ($value:expr, $kind:ty, $max:expr) => {{
        let Value::Unsigned(value) = $value else {
            return Err(Error::Value);
        };
        Number::<$kind>::LE.encode(&<$kind>::try_from(*value).map_err(|_| Error::Value)?, $max)
    }};
}

macro_rules! signed {
    ($value:expr, $kind:ty, $max:expr) => {{
        let Value::Signed(value) = $value else {
            return Err(Error::Value);
        };
        Number::<$kind>::LE.encode(&<$kind>::try_from(*value).map_err(|_| Error::Value)?, $max)
    }};
}

pub(crate) fn encode(kind: Primitive, value: &Value, max: usize) -> Result<Vec<u8>, Error> {
    let bytes = match kind {
        Primitive::Uint8 => unsigned!(value, u8, max),
        Primitive::U16 => unsigned!(value, u16, max),
        Primitive::U32 => unsigned!(value, u32, max),
        Primitive::U64 => unsigned!(value, u64, max),
        Primitive::U128 => unsigned!(value, u128, max),
        Primitive::Int8 => signed!(value, i8, max),
        Primitive::I16 => signed!(value, i16, max),
        Primitive::I32 => signed!(value, i32, max),
        Primitive::I64 => signed!(value, i64, max),
        Primitive::I128 => signed!(value, i128, max),
        _ => return encode_other(kind, value, max),
    }?;

    Ok(bytes)
}

fn encode_other(kind: Primitive, value: &Value, max: usize) -> Result<Vec<u8>, Error> {
    let bytes = match (kind, value) {
        (Primitive::F32, Value::F32(value)) => Number::<f32>::LE.encode(value, max),
        (Primitive::F64, Value::F64(value)) => Number::<f64>::LE.encode(value, max),
        (Primitive::Bool, Value::Bool(value)) => Bool.encode(value, max),
        (Primitive::String, Value::String(value)) => SizePrefix(Utf8).encode(value.as_str(), max),
        (Primitive::Pubkey, Value::Address(value)) => {
            FixedBytes::<32>.encode(value.as_bytes(), max)
        }
        _ => return Err(Error::Value),
    }?;

    Ok(bytes)
}

pub(crate) fn decode(kind: Primitive, reader: &mut Reader<'_>) -> Result<Value, Error> {
    let value = match kind {
        Primitive::Uint8 => Value::Unsigned(Number::<u8>::LE.read(reader)?.into()),
        Primitive::U16 => Value::Unsigned(Number::<u16>::LE.read(reader)?.into()),
        Primitive::U32 => Value::Unsigned(Number::<u32>::LE.read(reader)?.into()),
        Primitive::U64 => Value::Unsigned(Number::<u64>::LE.read(reader)?.into()),
        Primitive::U128 => Value::Unsigned(Number::<u128>::LE.read(reader)?),
        Primitive::Int8 => Value::Signed(Number::<i8>::LE.read(reader)?.into()),
        Primitive::I16 => Value::Signed(Number::<i16>::LE.read(reader)?.into()),
        Primitive::I32 => Value::Signed(Number::<i32>::LE.read(reader)?.into()),
        Primitive::I64 => Value::Signed(Number::<i64>::LE.read(reader)?.into()),
        Primitive::I128 => Value::Signed(Number::<i128>::LE.read(reader)?),
        _ => return decode_other(kind, reader),
    };

    Ok(value)
}

fn decode_other(kind: Primitive, reader: &mut Reader<'_>) -> Result<Value, Error> {
    let value = match kind {
        Primitive::F32 => Value::F32(Number::<f32>::LE.read(reader)?),
        Primitive::F64 => Value::F64(Number::<f64>::LE.read(reader)?),
        Primitive::Bool => Value::Bool(Bool.read(reader)?),
        Primitive::String => Value::String(SizePrefix(Utf8).read(reader)?),
        Primitive::Pubkey => Value::Address(Address::from_bytes(FixedBytes::<32>.read(reader)?)),
        _ => return Err(Error::Value),
    };

    Ok(value)
}
