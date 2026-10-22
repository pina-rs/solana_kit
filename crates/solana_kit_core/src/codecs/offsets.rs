//! Checked absolute cursor transforms with borrowed buffer context.

use alloc::vec::Vec;

use super::{Decoder, Encoder, Error, Reader, Writer, framing::allocate};

/// Read-only context for an offset callback. Before the nested operation,
/// `start` and `end` both equal `original`. Afterwards they describe its actual
/// entry and exit positions. Post-write bytes include nested modifications.
#[derive(Clone, Copy, Debug)]
pub struct OffsetScope<'a> {
    /// Entire enclosing buffer, including bytes preceding the cursor.
    pub bytes: &'a [u8],
    /// Position before applying the pre-offset callback.
    pub original: usize,
    /// Position selected by the pre-offset callback.
    pub start: usize,
    /// Position returned by the nested operation.
    pub end: usize,
}

impl OffsetScope<'_> {
    /// Wraps a signed absolute offset modulo the enclosing byte length.
    /// Empty buffers return zero, matching Dart. The end position wraps to zero.
    #[must_use]
    pub fn wrap(&self, offset: isize) -> usize {
        let length = self.bytes.len();

        if length == 0 {
            return 0;
        }

        if offset >= 0 {
            return offset.unsigned_abs() % length;
        }

        let remainder = offset.unsigned_abs() % length;

        if remainder == 0 {
            0
        } else {
            length - remainder
        }
    }
}

/// Changes cursor positions independently of encoded allocation extent.
/// Each callback returns an absolute position in the enclosing buffer.
/// Use `|scope| Ok(scope.original)` or `|scope| Ok(scope.end)` for identity.
/// A failed pre-offset leaves the cursor unchanged. Nested failures retain
/// nested cursor movement and claims. A failed post-offset retains nested exit.
/// Rewinding never refunds allocation claims or restores overwritten bytes.
#[derive(Clone, Copy, Debug)]
pub struct Offset<C, P, Q> {
    /// Nested codec.
    pub item: C,
    /// Selects the nested operation's starting position.
    pub pre: P,
    /// Selects the next operation's starting position.
    pub post: Q,
}

impl<C: Encoder<T>, P, Q, T: ?Sized> Encoder<T> for Offset<C, P, Q>
where
    P: Fn(OffsetScope<'_>) -> Result<usize, Error>,
    Q: Fn(OffsetScope<'_>) -> Result<usize, Error>,
{
    fn encoded_len(&self, value: &T) -> Result<usize, Error> {
        self.item.encoded_len(value)
    }

    fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error> {
        let original = writer.position();
        let start = (self.pre)(OffsetScope {
            bytes: writer.bytes(),
            original,
            start: original,
            end: original,
        })?;
        writer.seek(start)?;
        self.item.write(value, writer)?;
        let end = (self.post)(OffsetScope {
            bytes: writer.bytes(),
            original,
            start,
            end: writer.position(),
        })?;
        writer.seek(end)
    }

    fn encode(&self, value: &T, max_bytes: usize) -> Result<Vec<u8>, Error> {
        let mut bytes = allocate(self.encoded_len(value)?, max_bytes)?;
        self.write(value, &mut Writer::new(&mut bytes))?;
        Ok(bytes)
    }
}

impl<C: Decoder<T>, P, Q, T> Decoder<T> for Offset<C, P, Q>
where
    P: Fn(OffsetScope<'_>) -> Result<usize, Error>,
    Q: Fn(OffsetScope<'_>) -> Result<usize, Error>,
{
    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        let original = reader.position();
        let start = (self.pre)(OffsetScope {
            bytes: reader.bytes(),
            original,
            start: original,
            end: original,
        })?;
        reader.seek(start)?;
        let value = self.item.read(reader)?;
        let end = (self.post)(OffsetScope {
            bytes: reader.bytes(),
            original,
            start,
            end: reader.position(),
        })?;
        reader.seek(end)?;
        Ok(value)
    }
}
