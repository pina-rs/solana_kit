//! Constructor failures stay separate from ordinary byte-operation errors.

use core::fmt;

use super::{Error, Size};

/// Independent encoder and decoder declarations could not establish a pair.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PairError {
    /// A selected metadata query failed, retaining its original error.
    Metadata(Error),
    /// At least one selected implementation has no size declaration.
    UnknownSize,
    /// One declaration is fixed and the other is variable.
    KindMismatch,
    /// The two fixed widths differ.
    FixedSizeMismatch {
        /// Encoder's declared width.
        encoder: usize,
        /// Decoder's declared width.
        decoder: usize,
    },
    /// The complete optional variable maxima differ.
    MaxSizeMismatch {
        /// Encoder's declared maximum, including an explicit unknown maximum.
        encoder: Option<usize>,
        /// Decoder's declared maximum, including an explicit unknown maximum.
        decoder: Option<usize>,
    },
}

impl PairError {
    /// Checks declarations before the parts become a validated owner.
    pub(super) fn check(encoder: Size, decoder: Size) -> Result<(), Self> {
        match (encoder, decoder) {
            (Size::Unknown, _) | (_, Size::Unknown) => Err(Self::UnknownSize),

            (Size::Fixed(encoder), Size::Fixed(decoder)) if encoder != decoder => {
                Err(Self::FixedSizeMismatch { encoder, decoder })
            }

            (Size::Variable { max: encoder }, Size::Variable { max: decoder })
                if encoder != decoder =>
            {
                Err(Self::MaxSizeMismatch { encoder, decoder })
            }

            (Size::Fixed(_), Size::Variable { .. }) | (Size::Variable { .. }, Size::Fixed(_)) => {
                Err(Self::KindMismatch)
            }

            _ => Ok(()),
        }
    }
}

impl From<Error> for PairError {
    fn from(error: Error) -> Self {
        Self::Metadata(error)
    }
}

impl fmt::Display for PairError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "codec pairing error: {self:?}")
    }
}

impl core::error::Error for PairError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Metadata(error) => Some(error),
            _ => None,
        }
    }
}
