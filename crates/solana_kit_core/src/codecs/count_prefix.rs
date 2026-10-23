//! Explicit exhaustion leniency for existing count-prefixed sequences.

use super::{Decoder, Encoder, Error, Reader, Writer};

/// A count prefix that reads zero only at the exact end of its enclosing input.
/// Partial prefixes still fail through the underlying codec. Use with Counted
/// and Mapped numeric codecs to retain checked count-domain conversions.
#[derive(Clone, Copy, Debug)]
pub struct OptionalCount<P>(pub P);

impl<P: Encoder<u32>> Encoder<u32> for OptionalCount<P> {
    fn encoded_len(&self, value: &u32) -> Result<usize, Error> {
        self.0.encoded_len(value)
    }

    fn write(&self, value: &u32, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.0.write(value, writer)
    }
}

impl<P: Decoder<u32>> Decoder<u32> for OptionalCount<P> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<u32, Error> {
        if reader.remaining().is_empty() {
            return Ok(0);
        }

        self.0.read(reader)
    }
}
