//! Lossless facts from existing deterministic error owners.
use super::{Context, ContextValue, ErrorCode, ErrorFacts};
use crate::{ParseError, ValueKind, accounts::AccountError, codecs};

impl ErrorFacts for codecs::PairError {
    fn error_code(&self) -> Option<ErrorCode> {
        match self {
            Self::Metadata(error) => error.error_code(),
            Self::UnknownSize => None,
            Self::KindMismatch => {
                Some(ErrorCode::CODECS_ENCODER_DECODER_SIZE_COMPATIBILITY_MISMATCH)
            }
            Self::FixedSizeMismatch { .. } => {
                Some(ErrorCode::CODECS_ENCODER_DECODER_FIXED_SIZE_MISMATCH)
            }
            Self::MaxSizeMismatch { .. } => {
                Some(ErrorCode::CODECS_ENCODER_DECODER_MAX_SIZE_MISMATCH)
            }
        }
    }

    fn error_context(&self) -> Context {
        match self {
            Self::Metadata(error) => error.error_context(),
            Self::FixedSizeMismatch { encoder, decoder } => Context::default()
                .number("encoderFixedSize", *encoder)
                .number("decoderFixedSize", *decoder),
            Self::MaxSizeMismatch { encoder, decoder } => Context::default()
                .number("encoderMaxSize", optional_size(*encoder))
                .number("decoderMaxSize", optional_size(*decoder)),
            _ => Context::default(),
        }
    }
}

/// Retains null at the facts boundary instead of omitting an unknown maximum.
fn optional_size(size: Option<usize>) -> ContextValue {
    size.map_or(ContextValue::Null, ContextValue::from)
}

impl ErrorFacts for codecs::Error {
    fn error_code(&self) -> Option<ErrorCode> {
        match self {
            Self::InvalidBoolean { .. } => Some(ErrorCode::CODECS_INVALID_BOOLEAN),
            Self::InvalidConstant { .. } => Some(ErrorCode::CODECS_INVALID_CONSTANT),
            Self::Truncated => Some(ErrorCode::CODECS_INVALID_BYTE_LENGTH),
            Self::TrailingBytes => {
                Some(ErrorCode::CODECS_EXPECTED_DECODER_TO_CONSUME_ENTIRE_BYTE_ARRAY)
            }
            Self::NumberOutOfRange => Some(ErrorCode::CODECS_NUMBER_OUT_OF_RANGE),
            _ => None,
        }
    }
    fn error_context(&self) -> Context {
        match self {
            Self::InvalidBoolean { value } => Context::default().number("value", *value),
            Self::InvalidConstant { offset, length } => Context::default()
                .number("offset", *offset)
                .number("length", *length),
            _ => Context::default(),
        }
    }
}
impl ErrorFacts for ParseError {
    fn error_code(&self) -> Option<ErrorCode> {
        match self {
            Self::WrongLength {
                kind: ValueKind::Address,
                ..
            } => Some(ErrorCode::ADDRESSES_INVALID_BYTE_LENGTH),
            Self::WrongLength {
                kind: ValueKind::Hash,
                ..
            } => Some(ErrorCode::INVALID_BLOCKHASH_BYTE_LENGTH),
            Self::WrongLength {
                kind: ValueKind::Signature,
                ..
            } => Some(ErrorCode::KEYS_INVALID_SIGNATURE_BYTE_LENGTH),
            Self::EncodedLengthExceeded {
                kind: ValueKind::Address,
                ..
            } => Some(ErrorCode::ADDRESSES_STRING_LENGTH_OUT_OF_RANGE),
            Self::EncodedLengthExceeded {
                kind: ValueKind::Hash,
                ..
            } => Some(ErrorCode::BLOCKHASH_STRING_LENGTH_OUT_OF_RANGE),
            Self::EncodedLengthExceeded {
                kind: ValueKind::Signature,
                ..
            } => Some(ErrorCode::KEYS_SIGNATURE_STRING_LENGTH_OUT_OF_RANGE),
            Self::InvalidBase58 {
                kind: ValueKind::Address,
            } => Some(ErrorCode::ADDRESSES_INVALID_BASE58_ENCODED_ADDRESS),
            Self::InvalidBase58 { .. } => None,
        }
    }
    fn error_context(&self) -> Context {
        match self {
            Self::WrongLength { actual, .. } => Context::default().number("actualLength", *actual),
            Self::EncodedLengthExceeded {
                actual, maximum, ..
            } => Context::default()
                .number("actualLength", *actual)
                .number("maximum", *maximum),
            Self::InvalidBase58 { .. } => Context::default(),
        }
    }
}
impl ErrorFacts for AccountError {
    fn error_code(&self) -> Option<ErrorCode> {
        match self {
            Self::Missing(_) => Some(ErrorCode::ACCOUNTS_ACCOUNT_NOT_FOUND),
            Self::Decode { .. } => Some(ErrorCode::ACCOUNTS_FAILED_TO_DECODE_ACCOUNT),
            _ => None,
        }
    }
}
#[cfg(feature = "crypto")]
impl ErrorFacts for crate::crypto::PdaError {
    fn error_code(&self) -> Option<ErrorCode> {
        Some(match self {
            Self::TooManySeeds => ErrorCode::ADDRESSES_MAX_NUMBER_OF_PDA_SEEDS_EXCEEDED,
            Self::SeedTooLong => ErrorCode::ADDRESSES_MAX_PDA_SEED_LENGTH_EXCEEDED,
            Self::OnCurve => ErrorCode::ADDRESSES_INVALID_SEEDS_POINT_ON_CURVE,
            Self::NoViableBump => ErrorCode::ADDRESSES_FAILED_TO_FIND_VIABLE_PDA_BUMP_SEED,
            Self::IllegalOwner => ErrorCode::ADDRESSES_PDA_ENDS_WITH_PDA_MARKER,
        })
    }
}
