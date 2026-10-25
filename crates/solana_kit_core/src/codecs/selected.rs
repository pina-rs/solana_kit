//! Untagged alternatives selected before reading or writing any payload.

use super::{Decoder, Encoder, Error, Reader, Size, Writer};

/// Selects one encoder without adding a wire discriminator.
///
/// The selector must be deterministic across sizing and writing. A Rust enum
/// can retain heterogeneous payloads while each arm implements `Encoder<T>`.
/// Invalid indexes fail before the selected encoder runs.
pub struct UnionEncoder<'a, T: ?Sized, F> {
    /// Ordered alternatives sharing the caller's explicit value domain.
    pub variants: &'a [&'a dyn Encoder<T>],
    /// Returns the zero-based alternative for the supplied value.
    pub select: F,
}

impl<T: ?Sized, F: Fn(&T) -> Result<usize, Error>> Encoder<T> for UnionEncoder<'_, T, F> {
    fn encode_size(&self) -> Result<Size, Error> {
        choice_size(self.variants.iter().map(|item| item.encode_size()))
    }

    fn encoded_len(&self, value: &T) -> Result<usize, Error> {
        self.selected(value)?.encoded_len(value)
    }

    fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.selected(value)?.write(value, writer)
    }
}

impl<T: ?Sized, F: Fn(&T) -> Result<usize, Error>> UnionEncoder<'_, T, F> {
    fn selected(&self, value: &T) -> Result<&dyn Encoder<T>, Error> {
        self.variants
            .get((self.select)(value)?)
            .copied()
            .ok_or(Error::InvalidDiscriminator)
    }
}

/// Selects one decoder using the enclosing bytes and initial absolute offset.
///
/// Selection does not consume bytes, trial-decode, or refund allocation claims.
/// Only the chosen decoder runs. Selectors own and must bound their allocations.
pub struct UnionDecoder<'a, T, F> {
    /// Ordered alternatives producing the caller's explicit value domain.
    pub variants: &'a [&'a dyn Decoder<T>],
    /// Inspects bytes at the original offset and returns a zero-based index.
    pub select: F,
}

impl<T, F: Fn(&[u8], usize) -> Result<usize, Error>> Decoder<T> for UnionDecoder<'_, T, F> {
    fn decode_size(&self) -> Result<Size, Error> {
        choice_size(self.variants.iter().map(|item| item.decode_size()))
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        let index = (self.select)(reader.bytes(), reader.position())?;
        let selected = self
            .variants
            .get(index)
            .ok_or(Error::InvalidDiscriminator)?;

        selected.read(reader)
    }
}

fn choice_size(mut sizes: impl Iterator<Item = Result<Size, Error>>) -> Result<Size, Error> {
    let first = sizes.next().transpose()?.unwrap_or(Size::Fixed(0));
    sizes.try_fold(first, |size, next| Ok(Size::choice(&[size, next?])))
}
