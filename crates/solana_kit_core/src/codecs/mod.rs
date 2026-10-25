//! Bounded binary codecs with explicit full and prefix decoding.
//!
//! [`Decoder::decode`](crate::codecs::Decoder::decode) rejects trailing bytes.
//! [`Decoder::decode_prefix`](crate::codecs::Decoder::decode_prefix) returns
//! the value and next offset. Cursors never index outside their slices; a failed
//! operation may have advanced its cursor. Allocating decoders must claim their
//! output elements against the shared [`Limits::max_items`](crate::codecs::Limits::max_items)
//! budget. Byte and
//! element limits apply to an entire operation, including nested codecs.
//!
//! # Errors and composition
//!
//! Reads and writes return [`Error::Truncated`](crate::codecs::Error::Truncated)
//! for insufficient storage and [`Error::Overflow`](crate::codecs::Error::Overflow)
//! for lengths that cannot be represented or added. Numeric ranges are enforced
//! by the Rust input type. [`Error::LimitExceeded`](crate::codecs::Error::LimitExceeded) rejects
//! oversized input or element claims before allocation; allocation failures
//! return [`Error::AllocationFailed`](crate::codecs::Error::AllocationFailed).
//! Option and choice validation returns
//! [`Error::InvalidDiscriminator`](crate::codecs::Error::InvalidDiscriminator),
//! and UTF-8 validation returns [`Error::InvalidUtf8`](crate::codecs::Error::InvalidUtf8).
//! Compact integers reject redundant bytes as
//! [`Error::NonCanonical`](crate::codecs::Error::NonCanonical).
//! Exact frames and full decoding reject unread bytes as
//! [`Error::TrailingBytes`](crate::codecs::Error::TrailingBytes).
//! Size mismatches and non-progressing remainder codecs return
//! [`Error::InvalidValue`](crate::codecs::Error::InvalidValue).
//!
//! The element budget counts each allocated sequence entry and each allocated
//! byte of bytes/text, including nested allocations. It is not a heap-byte quota
//! for arbitrary caller-defined types. Custom allocating decoders must call
//! [`Reader::claim`](crate::codecs::Reader::claim). No built-in codec recurses internally;
//! nesting is selected
//! by the caller's concrete codec types. Remainder codecs belong at the end of
//! their enclosing frame. UTF-8 preserves both nulls and a leading BOM.

#[cfg(feature = "bigint-codecs")]
mod big_number;
mod collections;
mod contextual;
mod count_prefix;
mod cursor;
mod delimiters;
mod dynamic_tuple;
mod framing;
mod item_sentinel;
mod mapping;
mod nullable;
mod numbers;
mod offsets;
mod options;
mod pair_error;
mod radix;
mod record;
mod selected;
mod sequences;
mod size;
mod structured;
mod text;
mod transforms;
mod utf16_radix;
mod wide_text;

#[cfg(feature = "bigint-codecs")]
pub use big_number::BigNumber;
pub use collections::{OrderedMapCodec, OrderedSetCodec};
pub use contextual::Contextual;
pub use count_prefix::OptionalCount;
pub use cursor::{Limits, Reader, Writer};
pub use delimiters::{Padded, Sentinel};
pub use dynamic_tuple::{TupleDecoder, TupleEncoder};
pub use framing::{ByteSizePrefix, FixedSize, FrameConsumption, Resized, Reverse};
pub use item_sentinel::{ItemSentinel, Termination};
pub use mapping::{Checked, Mapped, Paired};
pub use nullable::{Absence, Boolean, Nullable};
#[cfg(feature = "bigint-codecs")]
pub use num_bigint::{BigInt, BigUint};
pub use numbers::{Endian, Int256, Number, ShortU16, Uint256, ensure_number_range};
pub use offsets::{Offset, OffsetScope};
pub use options::OptionCodec;
pub use pair_error::PairError;
pub use radix::{BaseX, BaseXReslice};
pub use record::{RecordDecoder, RecordEncoder};
pub use selected::{UnionDecoder, UnionEncoder};
pub use sequences::{Bool, Bytes, Count, FixedBytes, Sequence, Tuple, Utf8};
pub use size::Size;
pub use structured::{BitArray, Choice, Counted, Dependent, Either, Literal, OwnedSequence, Unit};
#[cfg(feature = "text-codecs")]
pub use text::Base64;
pub use text::{Base16, Utf8Config, pad_null_characters, remove_null_characters};
pub use transforms::{Affix, Constant, SizePrefix, SizePrefix64};
pub use utf16_radix::{Utf16BaseX, Utf16BaseXReslice};

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
    /// A boolean byte was neither zero nor one.
    InvalidBoolean {
        /// Invalid byte, retained without accepting it as a boolean.
        value: u8,
    },
    /// A fixed constant differed at the original cursor offset.
    InvalidConstant {
        /// Absolute cursor position before reading the constant.
        offset: usize,
        /// Expected constant length; the bytes themselves are not retained.
        length: usize,
    },
    /// A value was outside its explicitly supplied numeric interval.
    NumberOutOfRange,
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
    /// Declares this input type's format without inspecting a value.
    ///
    /// The declaration must stay stable for unchanged configuration, including
    /// through clones and shared callback state. It is not an allocation or
    /// cursor bound. Exact value sizing and operation limits remain authoritative.
    /// The default preserves ordinary operations without claiming a format class.
    ///
    /// # Errors
    /// Implementations return failures deriving their declaration, including
    /// [`Error::Overflow`] for checked composition outside the usize domain.
    fn encode_size(&self) -> Result<Size, Error> {
        Ok(Size::Unknown)
    }

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
    /// Declares this output type's format without reading bytes.
    ///
    /// The declaration must stay stable for unchanged configuration, including
    /// through clones and shared callback state. It does not impose a universal
    /// consumption check or replace byte limits and shared allocation claims.
    /// The default preserves ordinary operations without claiming a format class.
    ///
    /// # Errors
    /// Implementations return failures deriving their declaration, including
    /// [`Error::Overflow`] for checked composition outside the usize domain.
    fn decode_size(&self) -> Result<Size, Error> {
        Ok(Size::Unknown)
    }

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
