//! Configurable optional layouts using ordinary Rust Option values.

use super::{Decoder, Encoder, Error, Reader, Writer};

/// Storage written for an absent value.
#[derive(Clone, Copy, Debug)]
pub enum Absence<'a> {
    /// No payload bytes. Without a prefix, only exhausted input means absence.
    Omit,
    /// Exactly this many zero bytes. Tagged reads ignore their contents.
    Zeroes(usize),
    /// Exactly these bytes, validated on tagged reads too.
    Constant(&'a [u8]),
}

impl Absence<'_> {
    fn len(self) -> usize {
        match self {
            Self::Omit => 0,
            Self::Zeroes(size) => size,
            Self::Constant(bytes) => bytes.len(),
        }
    }

    fn matches(self, bytes: &[u8]) -> bool {
        match self {
            Self::Omit => bytes.is_empty(),
            Self::Zeroes(size) => bytes
                .get(..size)
                .is_some_and(|bytes| bytes.iter().all(|byte| *byte == 0)),
            Self::Constant(pattern) => bytes.starts_with(pattern),
        }
    }
}

/// A strict zero/one boolean stored through a caller-selected u32 codec.
/// Mapped numeric codecs support narrow, wide, signed and floating storage with
/// checked conversions. Invalid decoded numbers must be rejected by that mapping.
#[derive(Clone, Copy, Debug)]
pub struct Boolean<P>(pub P);

impl<P: Encoder<u32>> Encoder<bool> for Boolean<P> {
    fn encoded_len(&self, value: &bool) -> Result<usize, Error> {
        self.0.encoded_len(&u32::from(*value))
    }

    fn write(&self, value: &bool, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.0.write(&u32::from(*value), writer)
    }
}

impl<P: Decoder<u32>> Decoder<bool> for Boolean<P> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<bool, Error> {
        match self.0.read(reader)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::InvalidDiscriminator),
        }
    }
}

/// Configurable nullable/option wire bytes without a second value hierarchy.
/// A prefix codec stores strict zero/one values. None disables the prefix.
/// Zero storage requires present values to occupy the same caller-declared width.
#[derive(Clone, Copy, Debug)]
pub struct Nullable<'a, C, P> {
    /// Codec for present values.
    pub item: C,
    /// Optional strict boolean prefix codec.
    pub prefix: Option<P>,
    /// Wire representation of absence.
    pub absence: Absence<'a>,
}

impl<C: Encoder<T>, P: Encoder<bool>, T> Encoder<Option<T>> for Nullable<'_, C, P> {
    fn encoded_len(&self, value: &Option<T>) -> Result<usize, Error> {
        let prefix = self
            .prefix
            .as_ref()
            .map(|prefix| prefix.encoded_len(&value.is_some()))
            .transpose()?
            .unwrap_or(0);

        let payload = match value {
            Some(value) => self.item.encoded_len(value)?,
            None => self.absence.len(),
        };

        if value.is_some() && matches!(self.absence, Absence::Zeroes(size) if size != payload) {
            return Err(Error::InvalidValue);
        }

        prefix.checked_add(payload).ok_or(Error::Overflow)
    }

    fn write(&self, value: &Option<T>, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(value)?;

        if let Some(prefix) = &self.prefix {
            prefix.write(&value.is_some(), writer)?;
        }

        match value {
            Some(value) => self.item.write(value, writer),

            None => match self.absence {
                Absence::Omit => Ok(()),
                Absence::Zeroes(size) => writer.fill(size, 0),
                Absence::Constant(bytes) => writer.put(bytes),
            },
        }
    }
}

impl<C: Decoder<T>, P: Decoder<bool>, T> Decoder<Option<T>> for Nullable<'_, C, P> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Option<T>, Error> {
        let present = match &self.prefix {
            Some(prefix) => prefix.read(reader)?,
            None => !self.absence.matches(reader.remaining()),
        };

        if present {
            return match self.absence {
                Absence::Zeroes(size) => reader
                    .frame(size, |reader| self.item.read(reader))
                    .map(Some),
                _ => self.item.read(reader).map(Some),
            };
        }

        let bytes = reader.take(self.absence.len())?;

        if matches!(self.absence, Absence::Constant(pattern) if bytes != pattern) {
            return Err(Error::InvalidDiscriminator);
        }

        Ok(None)
    }
}
