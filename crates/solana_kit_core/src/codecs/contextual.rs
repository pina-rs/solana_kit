//! Output conversions with the original enclosing byte context.

use core::marker::PhantomData;

use super::{Decoder, Error, Reader, Size};

/// Converts a decoded value with the original input and starting offset.
///
/// The callback runs only after successful reading. Its failure retains consumed
/// bytes and claims, just like the inner decoder. It must bound allocations it
/// performs. In an explicit frame, the context is that frame's input and offset.
#[derive(Clone, Copy, Debug)]
pub struct Contextual<D, F, T> {
    decoder: D,
    map: F,
    value: PhantomData<fn(T)>,
}

impl<D, F, T> Contextual<D, F, T> {
    /// Wraps one concrete decoder without changing its size declaration.
    #[must_use]
    pub const fn new(decoder: D, map: F) -> Self {
        Self {
            decoder,
            map,
            value: PhantomData,
        }
    }
}

impl<D, F, T, U> Decoder<U> for Contextual<D, F, T>
where
    D: Decoder<T>,
    F: Fn(T, &[u8], usize) -> Result<U, Error>,
{
    fn decode_size(&self) -> Result<Size, Error> {
        self.decoder.decode_size()
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<U, Error> {
        let offset = reader.position();
        let value = self.decoder.read(reader)?;

        (self.map)(value, reader.bytes(), offset)
    }
}
