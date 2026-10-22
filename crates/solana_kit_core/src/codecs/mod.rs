//! Bounded binary codecs with explicit full and prefix decoding.
//!
//! [`Decoder::decode`] rejects trailing bytes. [`Decoder::decode_prefix`] returns
//! the value and next offset. Cursors never index outside their slices; a failed
//! operation may have advanced its cursor. Allocating decoders must claim their
//! output elements against the shared [`Limits::max_items`] budget. Byte and
//! element limits apply to an entire operation, including nested codecs.
//!
//! # Errors and composition
//!
//! Reads and writes return [`Error::Truncated`] for insufficient storage and
//! [`Error::Overflow`] for lengths that cannot be represented or added. Numeric
//! ranges are enforced by the Rust input type. [`Error::LimitExceeded`] rejects
//! oversized input or element claims before allocation; allocation failures
//! return [`Error::AllocationFailed`]. Boolean, option, and constant validation
//! returns [`Error::InvalidDiscriminator`], and UTF-8 validation returns
//! [`Error::InvalidUtf8`]. Compact integers reject redundant bytes as
//! [`Error::NonCanonical`]. Exact frames and full decoding reject unread bytes
//! as [`Error::TrailingBytes`]. Size mismatches and non-progressing remainder
//! codecs return [`Error::InvalidValue`].
//!
//! The element budget counts each allocated sequence entry and each allocated
//! byte of bytes/text, including nested allocations. It is not a heap-byte quota
//! for arbitrary caller-defined types. Custom allocating decoders must call
//! [`Reader::claim`]. No built-in codec recurses internally; nesting is selected
//! by the caller's concrete codec types. Remainder codecs belong at the end of
//! their enclosing frame. UTF-8 preserves both nulls and a leading BOM.

mod cursor;
mod numbers;
mod options;
mod sequences;
mod transforms;

pub use cursor::{Limits, Reader, Writer};
pub use numbers::{Endian, Int256, Number, ShortU16, Uint256};
pub use options::OptionCodec;
pub use sequences::{Bool, Bytes, Count, FixedBytes, Sequence, Tuple, Utf8};
pub use transforms::{Affix, Constant, SizePrefix, SizePrefix64};

use alloc::vec::Vec;
use core::fmt;

/// A binary operation failed without accepting malformed input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Input or output ended before the requested bytes.
    Truncated,
    /// A checked length or numeric operation overflowed.
    Overflow,
    /// An operation exceeded its caller-supplied byte or allocation budget.
    LimitExceeded,
    /// Full decoding left unread data.
    TrailingBytes,
    /// A compact integer used more bytes than necessary.
    NonCanonical,
    /// A tag or constant did not match the expected layout.
    InvalidDiscriminator,
    /// Text was not valid UTF-8.
    InvalidUtf8,
    /// The value did not satisfy the codec's contract.
    InvalidValue,
    /// An allocation could not be reserved.
    AllocationFailed,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "binary codec error: {self:?}")
    }
}

impl core::error::Error for Error {}

/// An encoder writes into a caller-owned, bounded buffer.
pub trait Encoder<T: ?Sized> {
    /// Returns the exact encoded size, rejecting overflow or invalid values.
    fn encoded_len(&self, value: &T) -> Result<usize, Error>;

    /// Writes at the cursor and advances it; errors may leave a partial write.
    fn write(&self, value: &T, writer: &mut Writer<'_>) -> Result<(), Error>;

    /// Encodes into an owned buffer bounded by `max_bytes`.
    fn encode(&self, value: &T, max_bytes: usize) -> Result<Vec<u8>, Error> {
        let size = self.encoded_len(value)?;

        if size > max_bytes {
            return Err(Error::LimitExceeded);
        }

        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(size)
            .map_err(|_| Error::AllocationFailed)?;
        bytes.resize(size, 0);
        let mut writer = Writer::new(&mut bytes);
        self.write(value, &mut writer)?;

        if writer.position() != size {
            return Err(Error::InvalidValue);
        }

        Ok(bytes)
    }
}

/// A decoder shares a single allocation budget with all nested decoders.
pub trait Decoder<T> {
    /// Reads at the cursor and advances it; errors may leave a partial read.
    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error>;

    /// Decodes exactly one value, rejecting trailing input bytes.
    fn decode(&self, bytes: &[u8], limits: Limits) -> Result<T, Error> {
        let mut reader = Reader::new(bytes, limits)?;
        let value = self.read(&mut reader)?;
        reader.finish()?;
        Ok(value)
    }

    /// Decodes from `offset`, returning the value and next absolute offset.
    fn decode_prefix(
        &self,
        bytes: &[u8],
        offset: usize,
        limits: Limits,
    ) -> Result<(T, usize), Error> {
        let mut reader = Reader::new(bytes, limits)?;
        reader.take(offset)?;
        let value = self.read(&mut reader)?;
        Ok((value, reader.position()))
    }
}
