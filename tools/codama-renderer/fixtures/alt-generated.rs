// Generated from the exact pinned Codama input; see upstream notices.
use alloc::vec::Vec;
use solana_kit_core::codecs::Constant;
use solana_kit_core::codecs::Number;
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
/// Validated data for `AddressLookupTable`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddressLookupTable {
    /// The `deactivation_slot` wire field.
    pub deactivation_slot: u64,
    /// The `last_extended_slot` wire field.
    pub last_extended_slot: u64,
    /// The `last_extended_slot_start_index` wire field.
    pub last_extended_slot_start_index: u8,
    /// The `authority` wire field.
    pub authority: Option<Address>,
    /// The `padding` wire field.
    pub padding: u16,
    /// The `addresses` wire field.
    pub addresses: Vec<Address>,
}
/// Bounded codec for [`AddressLookupTable`].
#[derive(Clone, Copy, Debug)]
pub struct AddressLookupTableCodec;
impl Encoder<AddressLookupTable> for AddressLookupTableCodec {
    fn encoded_len(&self, value: &AddressLookupTable) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.deactivation_slot)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.last_extended_slot)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.last_extended_slot_start_index)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                FixedSize {
                    item: OptionCodec::u8(Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    )),
                    size: 33,
                    decoded_size: None,
                    allow_truncation: false,
                    max_encoded_size: 33,
                }
                .encoded_len(&value.authority)?,
            )
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u16>::LE.encoded_len(&value.padding)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(
                OwnedSequence(Sequence {
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    count: Count::Remainder,
                })
                .encoded_len(&value.addresses)?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &AddressLookupTable, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[1, 0, 0, 0]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.deactivation_slot, writer)?;
        Number::<u64>::LE.write(&value.last_extended_slot, writer)?;
        Number::<u8>::LE.write(&value.last_extended_slot_start_index, writer)?;
        FixedSize {
            item: OptionCodec::u8(Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            )),
            size: 33,
            decoded_size: None,
            allow_truncation: false,
            max_encoded_size: 33,
        }
        .write(&value.authority, writer)?;
        Number::<u16>::LE.write(&value.padding, writer)?;
        OwnedSequence(Sequence {
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            count: Count::Remainder,
        })
        .write(&value.addresses, writer)?;
        Ok(())
    }
}
impl Decoder<AddressLookupTable> for AddressLookupTableCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AddressLookupTable, Error> {
        Constant(&[1, 0, 0, 0]).read(reader)?;
        Ok(AddressLookupTable {
            deactivation_slot: Number::<u64>::LE.read(reader)?,
            last_extended_slot: Number::<u64>::LE.read(reader)?,
            last_extended_slot_start_index: Number::<u8>::LE.read(reader)?,
            authority: FixedSize {
                item: OptionCodec::u8(Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                )),
                size: 33,
                decoded_size: None,
                allow_truncation: false,
                max_encoded_size: 33,
            }
            .read(reader)?,
            padding: Number::<u16>::LE.read(reader)?,
            addresses: OwnedSequence(Sequence {
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                count: Count::Remainder,
            })
            .read(reader)?,
        })
    }
}
/// Validated data for `CreateLookupTable`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateLookupTable {
    /// The `recent_slot` wire field.
    pub recent_slot: u64,
    /// The `bump` wire field.
    pub bump: u8,
}
/// Bounded codec for [`CreateLookupTable`].
#[derive(Clone, Copy, Debug)]
pub struct CreateLookupTableCodec;
impl Encoder<CreateLookupTable> for CreateLookupTableCodec {
    fn encoded_len(&self, value: &CreateLookupTable) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(Number::<u64>::LE.encoded_len(&value.recent_slot)?)
            .ok_or(Error::Overflow)?;
        let size = size
            .checked_add(Number::<u8>::LE.encoded_len(&value.bump)?)
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &CreateLookupTable, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[0, 0, 0, 0]).write(&(), writer)?;
        Number::<u64>::LE.write(&value.recent_slot, writer)?;
        Number::<u8>::LE.write(&value.bump, writer)?;
        Ok(())
    }
}
impl Decoder<CreateLookupTable> for CreateLookupTableCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CreateLookupTable, Error> {
        Constant(&[0, 0, 0, 0]).read(reader)?;
        Ok(CreateLookupTable {
            recent_slot: Number::<u64>::LE.read(reader)?,
            bump: Number::<u8>::LE.read(reader)?,
        })
    }
}
/// Validated data for `FreezeLookupTable`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreezeLookupTable {}
/// Bounded codec for [`FreezeLookupTable`].
#[derive(Clone, Copy, Debug)]
pub struct FreezeLookupTableCodec;
impl Encoder<FreezeLookupTable> for FreezeLookupTableCodec {
    fn encoded_len(&self, value: &FreezeLookupTable) -> Result<usize, Error> {
        let size = 4usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &FreezeLookupTable, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[1, 0, 0, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<FreezeLookupTable> for FreezeLookupTableCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<FreezeLookupTable, Error> {
        Constant(&[1, 0, 0, 0]).read(reader)?;
        let _ = reader;
        Ok(FreezeLookupTable {})
    }
}
/// Validated data for `ExtendLookupTable`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtendLookupTable {
    /// The `addresses` wire field.
    pub addresses: Vec<Address>,
}
/// Bounded codec for [`ExtendLookupTable`].
#[derive(Clone, Copy, Debug)]
pub struct ExtendLookupTableCodec;
impl Encoder<ExtendLookupTable> for ExtendLookupTableCodec {
    fn encoded_len(&self, value: &ExtendLookupTable) -> Result<usize, Error> {
        let size = 4usize;
        let size = size
            .checked_add(
                CheckedCounted {
                    prefix: Mapped::new(
                        Number::<u64>::LE,
                        |value: &u32| Ok::<_, Error>(u64::from(*value)),
                        |value: u64| u32::try_from(value).map_err(|_| Error::Overflow),
                    ),
                    item: Mapped::new(
                        FixedBytes::<32>,
                        |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                        |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                    ),
                    minimum: 32,
                }
                .encoded_len(value.addresses.as_slice())?,
            )
            .ok_or(Error::Overflow)?;
        Ok(size)
    }
    fn write(&self, value: &ExtendLookupTable, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[2, 0, 0, 0]).write(&(), writer)?;
        CheckedCounted {
            prefix: Mapped::new(
                Number::<u64>::LE,
                |value: &u32| Ok::<_, Error>(u64::from(*value)),
                |value: u64| u32::try_from(value).map_err(|_| Error::Overflow),
            ),
            item: Mapped::new(
                FixedBytes::<32>,
                |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
            ),
            minimum: 32,
        }
        .write(value.addresses.as_slice(), writer)?;
        Ok(())
    }
}
impl Decoder<ExtendLookupTable> for ExtendLookupTableCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ExtendLookupTable, Error> {
        Constant(&[2, 0, 0, 0]).read(reader)?;
        Ok(ExtendLookupTable {
            addresses: CheckedCounted {
                prefix: Mapped::new(
                    Number::<u64>::LE,
                    |value: &u32| Ok::<_, Error>(u64::from(*value)),
                    |value: u64| u32::try_from(value).map_err(|_| Error::Overflow),
                ),
                item: Mapped::new(
                    FixedBytes::<32>,
                    |value: &Address| Ok::<_, Error>(*value.as_bytes()),
                    |bytes| Ok::<_, Error>(Address::from_bytes(bytes)),
                ),
                minimum: 32,
            }
            .read(reader)?,
        })
    }
}
/// Validated data for `DeactivateLookupTable`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeactivateLookupTable {}
/// Bounded codec for [`DeactivateLookupTable`].
#[derive(Clone, Copy, Debug)]
pub struct DeactivateLookupTableCodec;
impl Encoder<DeactivateLookupTable> for DeactivateLookupTableCodec {
    fn encoded_len(&self, value: &DeactivateLookupTable) -> Result<usize, Error> {
        let size = 4usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &DeactivateLookupTable, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[3, 0, 0, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<DeactivateLookupTable> for DeactivateLookupTableCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<DeactivateLookupTable, Error> {
        Constant(&[3, 0, 0, 0]).read(reader)?;
        let _ = reader;
        Ok(DeactivateLookupTable {})
    }
}
/// Validated data for `CloseLookupTable`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CloseLookupTable {}
/// Bounded codec for [`CloseLookupTable`].
#[derive(Clone, Copy, Debug)]
pub struct CloseLookupTableCodec;
impl Encoder<CloseLookupTable> for CloseLookupTableCodec {
    fn encoded_len(&self, value: &CloseLookupTable) -> Result<usize, Error> {
        let size = 4usize;
        let _ = value;
        Ok(size)
    }
    fn write(&self, value: &CloseLookupTable, writer: &mut Writer<'_>) -> Result<(), Error> {
        Constant(&[4, 0, 0, 0]).write(&(), writer)?;
        let _ = value;
        let _ = writer;
        Ok(())
    }
}
impl Decoder<CloseLookupTable> for CloseLookupTableCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<CloseLookupTable, Error> {
        Constant(&[4, 0, 0, 0]).read(reader)?;
        let _ = reader;
        Ok(CloseLookupTable {})
    }
}
use solana_kit_core::codecs::Counted;
use solana_kit_core::codecs::FixedBytes;
use solana_kit_core::codecs::FixedSize;
use solana_kit_core::codecs::Mapped;
use solana_kit_core::codecs::OptionCodec;
use solana_kit_core::codecs::{Count, OwnedSequence, Sequence};
