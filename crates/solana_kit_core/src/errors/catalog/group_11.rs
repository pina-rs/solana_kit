//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `codecsExpectedFixedLength` code.
    pub const CODECS_EXPECTED_FIXED_LENGTH: Self = Self(8078002);
    /// Pinned `codecsExpectedVariableLength` code.
    pub const CODECS_EXPECTED_VARIABLE_LENGTH: Self = Self(8078003);
    /// Pinned `codecsEncoderDecoderSizeCompatibilityMismatch` code.
    pub const CODECS_ENCODER_DECODER_SIZE_COMPATIBILITY_MISMATCH: Self = Self(8078004);
    /// Pinned `codecsEncoderDecoderFixedSizeMismatch` code.
    pub const CODECS_ENCODER_DECODER_FIXED_SIZE_MISMATCH: Self = Self(8078005);
    /// Pinned `codecsEncoderDecoderMaxSizeMismatch` code.
    pub const CODECS_ENCODER_DECODER_MAX_SIZE_MISMATCH: Self = Self(8078006);
    /// Pinned `codecsInvalidNumberOfItems` code.
    pub const CODECS_INVALID_NUMBER_OF_ITEMS: Self = Self(8078007);
    /// Pinned `codecsEnumDiscriminatorOutOfRange` code.
    pub const CODECS_ENUM_DISCRIMINATOR_OUT_OF_RANGE: Self = Self(8078008);
    /// Pinned `codecsInvalidDiscriminatedUnionVariant` code.
    pub const CODECS_INVALID_DISCRIMINATED_UNION_VARIANT: Self = Self(8078009);
    /// Pinned `codecsInvalidEnumVariant` code.
    pub const CODECS_INVALID_ENUM_VARIANT: Self = Self(8078010);
    /// Pinned `codecsNumberOutOfRange` code.
    pub const CODECS_NUMBER_OUT_OF_RANGE: Self = Self(8078011);
    /// Pinned `codecsInvalidStringForBase` code.
    pub const CODECS_INVALID_STRING_FOR_BASE: Self = Self(8078012);
    /// Pinned `codecsExpectedPositiveByteLength` code.
    pub const CODECS_EXPECTED_POSITIVE_BYTE_LENGTH: Self = Self(8078013);
    /// Pinned `codecsOffsetOutOfRange` code.
    pub const CODECS_OFFSET_OUT_OF_RANGE: Self = Self(8078014);
    /// Pinned `codecsInvalidLiteralUnionVariant` code.
    pub const CODECS_INVALID_LITERAL_UNION_VARIANT: Self = Self(8078015);
    /// Pinned `codecsLiteralUnionDiscriminatorOutOfRange` code.
    pub const CODECS_LITERAL_UNION_DISCRIMINATOR_OUT_OF_RANGE: Self = Self(8078016);
    /// Pinned `codecsUnionVariantOutOfRange` code.
    pub const CODECS_UNION_VARIANT_OUT_OF_RANGE: Self = Self(8078017);
    /// Pinned `codecsInvalidConstant` code.
    pub const CODECS_INVALID_CONSTANT: Self = Self(8078018);
    /// Pinned `codecsExpectedZeroValueToMatchItemFixedSize` code.
    pub const CODECS_EXPECTED_ZERO_VALUE_TO_MATCH_ITEM_FIXED_SIZE: Self = Self(8078019);
    /// Pinned `codecsEncodedBytesMustNotIncludeSentinel` code.
    pub const CODECS_ENCODED_BYTES_MUST_NOT_INCLUDE_SENTINEL: Self = Self(8078020);
    /// Pinned `codecsSentinelMissingInDecodedBytes` code.
    pub const CODECS_SENTINEL_MISSING_IN_DECODED_BYTES: Self = Self(8078021);
    /// Pinned `codecsCannotUseLexicalValuesAsEnumDiscriminators` code.
    pub const CODECS_CANNOT_USE_LEXICAL_VALUES_AS_ENUM_DISCRIMINATORS: Self = Self(8078022);
    /// Pinned `codecsExpectedDecoderToConsumeEntireByteArray` code.
    pub const CODECS_EXPECTED_DECODER_TO_CONSUME_ENTIRE_BYTE_ARRAY: Self = Self(8078023);
    /// Pinned `codecsInvalidPatternMatchValue` code.
    pub const CODECS_INVALID_PATTERN_MATCH_VALUE: Self = Self(8078024);
    /// Pinned `codecsInvalidPatternMatchBytes` code.
    pub const CODECS_INVALID_PATTERN_MATCH_BYTES: Self = Self(8078025);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode::CODECS_EXPECTED_FIXED_LENGTH,
        name: "codecsExpectedFixedLength",
        template: "Expected a fixed-size codec, got a variable-size one.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_EXPECTED_VARIABLE_LENGTH,
        name: "codecsExpectedVariableLength",
        template: "Expected a variable-size codec, got a fixed-size one.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_ENCODER_DECODER_SIZE_COMPATIBILITY_MISMATCH,
        name: "codecsEncoderDecoderSizeCompatibilityMismatch",
        template: "Encoder and decoder must either both be fixed-size or variable-size.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_ENCODER_DECODER_FIXED_SIZE_MISMATCH,
        name: "codecsEncoderDecoderFixedSizeMismatch",
        template: "Encoder and decoder must have the same fixed size, got [$encoderFixedSize] and [$decoderFixedSize].",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_ENCODER_DECODER_MAX_SIZE_MISMATCH,
        name: "codecsEncoderDecoderMaxSizeMismatch",
        template: "Encoder and decoder must have the same max size, got [$encoderMaxSize] and [$decoderMaxSize].",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_INVALID_NUMBER_OF_ITEMS,
        name: "codecsInvalidNumberOfItems",
        template: "Expected [$codecDescription] to have $expected items, got $actual.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_ENUM_DISCRIMINATOR_OUT_OF_RANGE,
        name: "codecsEnumDiscriminatorOutOfRange",
        template: "Enum discriminator out of range. Expected a number in [$formattedValidDiscriminators], got $discriminator.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_INVALID_DISCRIMINATED_UNION_VARIANT,
        name: "codecsInvalidDiscriminatedUnionVariant",
        template: "Invalid discriminated union variant. Expected one of [$variants], got $value.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_INVALID_ENUM_VARIANT,
        name: "codecsInvalidEnumVariant",
        template: "Invalid enum variant. Expected one of [$stringValues] or a number in [$formattedNumericalValues], got $variant.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_NUMBER_OUT_OF_RANGE,
        name: "codecsNumberOutOfRange",
        template: "Codec [$codecDescription] expected number to be in the range [$min, $max], got $value.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_INVALID_STRING_FOR_BASE,
        name: "codecsInvalidStringForBase",
        template: "Invalid value $value for base $base with alphabet $alphabet.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_EXPECTED_POSITIVE_BYTE_LENGTH,
        name: "codecsExpectedPositiveByteLength",
        template: "Codec [$codecDescription] expected a positive byte length, got $bytesLength.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_OFFSET_OUT_OF_RANGE,
        name: "codecsOffsetOutOfRange",
        template: "Codec [$codecDescription] expected offset to be in the range [0, $bytesLength], got $offset.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_INVALID_LITERAL_UNION_VARIANT,
        name: "codecsInvalidLiteralUnionVariant",
        template: "Invalid literal union variant. Expected one of [$variants], got $value.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_LITERAL_UNION_DISCRIMINATOR_OUT_OF_RANGE,
        name: "codecsLiteralUnionDiscriminatorOutOfRange",
        template: "Literal union discriminator out of range. Expected a number between $minRange and $maxRange, got $discriminator.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_UNION_VARIANT_OUT_OF_RANGE,
        name: "codecsUnionVariantOutOfRange",
        template: "Union variant out of range. Expected an index between $minRange and $maxRange, got $variant.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_INVALID_CONSTANT,
        name: "codecsInvalidConstant",
        template: "Expected byte array constant [$hexConstant] to be present in data [$hexData] at offset [$offset].",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_EXPECTED_ZERO_VALUE_TO_MATCH_ITEM_FIXED_SIZE,
        name: "codecsExpectedZeroValueToMatchItemFixedSize",
        template: "Codec [$codecDescription] expected zero-value [$hexZeroValue] to have the same size as the provided fixed-size item [$expectedSize bytes].",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_ENCODED_BYTES_MUST_NOT_INCLUDE_SENTINEL,
        name: "codecsEncodedBytesMustNotIncludeSentinel",
        template: "Sentinel [$hexSentinel] must not be present in encoded bytes [$hexEncodedBytes].",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_SENTINEL_MISSING_IN_DECODED_BYTES,
        name: "codecsSentinelMissingInDecodedBytes",
        template: "Expected sentinel [$hexSentinel] to be present in decoded bytes [$hexDecodedBytes].",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_CANNOT_USE_LEXICAL_VALUES_AS_ENUM_DISCRIMINATORS,
        name: "codecsCannotUseLexicalValuesAsEnumDiscriminators",
        template: "Enum codec cannot use lexical values [$stringValues] as discriminators. Either remove all lexical values or set useValuesAsDiscriminators to false.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_EXPECTED_DECODER_TO_CONSUME_ENTIRE_BYTE_ARRAY,
        name: "codecsExpectedDecoderToConsumeEntireByteArray",
        template: "This decoder expected a byte array of exactly $expectedLength bytes, but $numExcessBytes unexpected excess bytes remained after decoding. Are you sure that you have chosen the correct decoder for this data?",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_INVALID_PATTERN_MATCH_VALUE,
        name: "codecsInvalidPatternMatchValue",
        template: "Invalid pattern match value. The provided value does not match any of the specified patterns.",
    },
    CodeDefinition {
        code: ErrorCode::CODECS_INVALID_PATTERN_MATCH_BYTES,
        name: "codecsInvalidPatternMatchBytes",
        template: "Invalid pattern match bytes. The provided byte array does not match any of the specified patterns.",
    },
];
