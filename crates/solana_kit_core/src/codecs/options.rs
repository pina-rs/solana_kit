//! Explicit tagged, C-layout, and trailing optional values.

use super::{Decoder, Encoder, Error, Number, Reader, Writer};

/// Distinct wire contracts must be selected explicitly.
#[derive(Clone, Copy, Debug)]
enum Layout {
    ByteTag,
    WordTag,
    COption(usize),
    Remainder,
}

/// Encodes Rust [`Option`] with an explicit layout. Every tag is strictly zero
/// or one. C-layout absent payloads occupy their complete fixed width; like the
/// reference, their contents are ignored on decode and written as zeroes.
#[derive(Clone, Copy, Debug)]
pub struct OptionCodec<C> {
    item: C,
    layout: Layout,
}

impl<C> OptionCodec<C> {
    /// One-byte zero/one tag; absent payloads have no bytes (Borsh option).
    pub const fn u8(item: C) -> Self {
        Self {
            item,
            layout: Layout::ByteTag,
        }
    }

    /// Four-byte little-endian zero/one tag; absent payloads have no bytes.
    pub const fn u32(item: C) -> Self {
        Self {
            item,
            layout: Layout::WordTag,
        }
    }

    /// Four-byte tag plus exactly `item_size` payload bytes in both variants.
    /// The inner codec must consume/write exactly that many bytes when present.
    pub const fn c_option(item: C, item_size: usize) -> Self {
        Self {
            item,
            layout: Layout::COption(item_size),
        }
    }

    /// No tag: absent at the enclosing reader's end, otherwise present.
    /// Present values must consume at least one byte; use only as a final field.
    pub const fn remainder(item: C) -> Self {
        Self {
            item,
            layout: Layout::Remainder,
        }
    }
}

impl<C, T> Encoder<Option<T>> for OptionCodec<C>
where
    C: Encoder<T>,
{
    fn encoded_len(&self, value: &Option<T>) -> Result<usize, Error> {
        let size = value
            .as_ref()
            .map(|value| self.item.encoded_len(value))
            .transpose()?;

        let (prefix, payload) = match self.layout {
            Layout::ByteTag => (1usize, size.unwrap_or(0)),
            Layout::WordTag => (4, size.unwrap_or(0)),

            Layout::COption(width) if size.is_some_and(|size| size != width) => {
                return Err(Error::InvalidValue);
            }

            Layout::COption(width) => (4, width),

            Layout::Remainder if size == Some(0) => return Err(Error::InvalidValue),
            Layout::Remainder => (0, size.unwrap_or(0)),
        };

        prefix.checked_add(payload).ok_or(Error::Overflow)
    }

    fn write(&self, value: &Option<T>, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(value)?;
        let tag = u8::from(value.is_some());

        match self.layout {
            Layout::ByteTag => Number::<u8>::LE.write(&tag, writer)?,
            Layout::WordTag | Layout::COption(_) => {
                Number::<u32>::LE.write(&u32::from(tag), writer)?
            }
            Layout::Remainder => {}
        }

        match (value, self.layout) {
            (Some(value), Layout::COption(size)) => {
                writer.frame(size, |inner| self.item.write(value, inner))
            }
            (Some(value), _) => self.item.write(value, writer),
            (None, Layout::COption(size)) => writer.fill(size, 0),
            (None, _) => Ok(()),
        }
    }
}

impl<C, T> Decoder<Option<T>> for OptionCodec<C>
where
    C: Decoder<T>,
{
    /// Validates the tag before decoding and rejects zero-width remainder payloads.
    fn read(&self, reader: &mut Reader<'_>) -> Result<Option<T>, Error> {
        let tag = match self.layout {
            Layout::ByteTag => u32::from(Number::<u8>::LE.read(reader)?),
            Layout::WordTag | Layout::COption(_) => Number::<u32>::LE.read(reader)?,
            Layout::Remainder => u32::from(!reader.remaining().is_empty()),
        };

        if tag > 1 {
            return Err(Error::InvalidDiscriminator);
        }

        if tag == 0 {
            return self.read_none(reader);
        }

        let before = reader.position();

        let value = match self.layout {
            Layout::COption(size) => reader.frame(size, |inner| self.item.read(inner))?,
            _ => self.item.read(reader)?,
        };

        if matches!(self.layout, Layout::Remainder) && before == reader.position() {
            return Err(Error::InvalidValue);
        }

        Ok(Some(value))
    }
}

impl<C> OptionCodec<C> {
    /// Consumes absent C-layout storage without interpreting stale payload data.
    fn read_none<T>(&self, reader: &mut Reader<'_>) -> Result<Option<T>, Error> {
        if let Layout::COption(size) = self.layout {
            reader.take(size)?;
        }

        Ok(None)
    }
}
