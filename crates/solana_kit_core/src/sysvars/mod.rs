//! Pinned sysvar layouts for portable off-chain clients.
//!
//! These codecs perform no RPC. Full decoding rejects trailing data; prefix
//! decoding uses the common cursor contract. Historical RecentBlockhashes is
//! retained for reading snapshots; use RPC fee estimation for new transactions.

mod fixed;
mod history;

pub use fixed::*;
pub use history::*;

use crate::{
    Hash,
    codecs::{Decoder, Encoder, Error, FixedBytes, Reader, Writer},
};
use core::marker::PhantomData;

/// Stateless codec selected by its Kit-owned sysvar value type.
#[derive(Clone, Copy, Debug, Default)]
pub struct SysvarCodec<T>(PhantomData<T>);

impl<T> SysvarCodec<T> {
    /// Selects the exact pinned layout for `T`.
    pub const NEW: Self = Self(PhantomData);
}

/// Internal hash adapter composes the existing fixed-byte wire codec.
struct HashCodec;

impl Encoder<Hash> for HashCodec {
    fn encoded_len(&self, _: &Hash) -> Result<usize, Error> {
        Ok(32)
    }
    fn write(&self, value: &Hash, writer: &mut Writer<'_>) -> Result<(), Error> {
        FixedBytes::<32>.write(value.as_bytes(), writer)
    }
}

impl Decoder<Hash> for HashCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Hash, Error> {
        FixedBytes::<32>.read(reader).map(Hash::from_bytes)
    }
}

/// Defines field-order composition without duplicating numeric byte operations.
macro_rules! layout {
    ($name:ident, $size:expr, $($field:ident: $codec:expr),+ $(,)?) => {
        impl Encoder<$name> for SysvarCodec<$name> {
            fn encoded_len(&self, _: &$name) -> Result<usize, Error> { Ok($size) }
            fn write(&self, value: &$name, writer: &mut Writer<'_>) -> Result<(), Error> {
                $($codec.write(&value.$field, writer)?;)+
                Ok(())
            }
        }
        impl Decoder<$name> for SysvarCodec<$name> {
            fn read(&self, reader: &mut Reader<'_>) -> Result<$name, Error> {
                Ok($name { $($field: $codec.read(reader)?,)+ })
            }
        }
    };
}
pub(super) use layout;
