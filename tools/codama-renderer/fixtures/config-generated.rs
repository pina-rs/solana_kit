// Generated from the exact pinned Codama input; see upstream notices.
use alloc::vec::Vec;
use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Reader, Writer},
};

struct CheckedCounted<P, C> {
    prefix: P,
    item: C,
    minimum: usize,
}
impl<P: Encoder<u32>, C: Encoder<T>, T> Encoder<[T]> for CheckedCounted<P, C> {
    fn encoded_len(&self, value: &[T]) -> Result<usize, Error> {
        Counted {
            prefix: &self.prefix,
            item: &self.item,
        }
        .encoded_len(value)
    }
    fn write(&self, value: &[T], writer: &mut Writer<'_>) -> Result<(), Error> {
        Counted {
            prefix: &self.prefix,
            item: &self.item,
        }
        .write(value, writer)
    }
}
impl<P: Decoder<u32>, C: Decoder<T>, T> Decoder<Vec<T>> for CheckedCounted<P, C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<T>, Error> {
        let count = usize::try_from(self.prefix.read(reader)?).map_err(|_| Error::Overflow)?;
        let minimum = count.checked_mul(self.minimum).ok_or(Error::Overflow)?;

        if minimum > reader.remaining().len() {
            return Err(Error::Truncated);
        }

        solana_kit_core::codecs::Sequence {
            item: &self.item,
            count: solana_kit_core::codecs::Count::Fixed(count),
        }
        .read(reader)
    }
}
/// Validated data for `ConfigKeys`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigKeys {
    /// The `value` wire field.
    pub value: Vec<(Address, bool)>,
}
/// Bounded codec for [`ConfigKeys`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigKeysCodec;
impl Encoder<ConfigKeys> for ConfigKeysCodec {
    fn encoded_len(&self, value: &ConfigKeys) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Mapped::new(
                        ShortU16,
                        |value: &u32| u16::try_from(*value).map_err(|_| Error::Overflow),
                        |value: u16| Ok::<_, Error>(u32::from(value)),
                    ),
                    item: Tuple(
                        Mapped::new(
                            FixedBytes::<32>,
                            |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                            |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                        ),
                        Bool,
                    ),
                    minimum: 33,
                }
                .encoded_len(value.value.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ConfigKeys, writer: &mut Writer<'_>) -> Result<(), Error> {
        CheckedCounted {
            prefix: Mapped::new(
                ShortU16,
                |value: &u32| u16::try_from(*value).map_err(|_| Error::Overflow),
                |value: u16| Ok::<_, Error>(u32::from(value)),
            ),
            item: Tuple(
                Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                Bool,
            ),
            minimum: 33,
        }
        .write(value.value.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<ConfigKeys> for ConfigKeysCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfigKeys, Error> {
        Ok(ConfigKeys {
            value: CheckedCounted {
                prefix: Mapped::new(
                    ShortU16,
                    |value: &u32| u16::try_from(*value).map_err(|_| Error::Overflow),
                    |value: u16| Ok::<_, Error>(u32::from(value)),
                ),
                item: Tuple(
                    Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    Bool,
                ),
                minimum: 33,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `Config`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    /// The `keys` wire field.
    pub keys: ConfigKeys,
    /// The `data` wire field.
    pub data: Vec<u8>,
}
/// Bounded codec for [`Config`].
#[derive(Clone, Copy, Debug)]
pub struct ConfigCodec;
impl Encoder<Config> for ConfigCodec {
    fn encoded_len(&self, value: &Config) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(ConfigKeysCodec.encoded_len(&value.keys)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bytes.encoded_len(value.data.as_slice())?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Config, writer: &mut Writer<'_>) -> Result<(), Error> {
        ConfigKeysCodec.write(&value.keys, writer)?;
        Bytes.write(value.data.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<Config> for ConfigCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Config, Error> {
        Ok(Config {
            keys: ConfigKeysCodec.read(reader)?,
            data: Bytes.read(reader)?,
        })
    }
}
/// Validated data for `Store`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Store {
    /// The `keys` wire field.
    pub keys: ConfigKeys,
    /// The `data` wire field.
    pub data: Vec<u8>,
}
/// Bounded codec for [`Store`].
#[derive(Clone, Copy, Debug)]
pub struct StoreCodec;
impl Encoder<Store> for StoreCodec {
    fn encoded_len(&self, value: &Store) -> Result<usize, Error> {
        let size = 0usize;
        let size = size
            .checked_add(ConfigKeysCodec.encoded_len(&value.keys)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Bytes.encoded_len(value.data.as_slice())?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &Store, writer: &mut Writer<'_>) -> Result<(), Error> {
        ConfigKeysCodec.write(&value.keys, writer)?;
        Bytes.write(value.data.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<Store> for StoreCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Store, Error> {
        Ok(Store {
            keys: ConfigKeysCodec.read(reader)?,
            data: Bytes.read(reader)?,
        })
    }
}
use solana_kit_core::codecs::Bool;
use solana_kit_core::codecs::Bytes;
use solana_kit_core::codecs::Counted;
use solana_kit_core::codecs::FixedBytes;
use solana_kit_core::codecs::Mapped;
use solana_kit_core::codecs::ShortU16;
use solana_kit_core::codecs::Tuple;
