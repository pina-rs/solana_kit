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
        code: ErrorCode(8078002),
        name: "codecsExpectedFixedLength",
        template: "Expected a fixed-size codec, got a variable-size one.",
    },
    CodeDefinition {
        code: ErrorCode(8078003),
        name: "codecsExpectedVariableLength",
        template: "Expected a variable-size codec, got a fixed-size one.",
    },
    CodeDefinition {
        code: ErrorCode(8078004),
        name: "codecsEncoderDecoderSizeCompatibilityMismatch",
        template: "Encoder and decoder must either both be fixed-size or variable-size.",
    },
    CodeDefinition {
        code: ErrorCode(8078005),
        name: "codecsEncoderDecoderFixedSizeMismatch",
        template: "Encoder and decoder must have the same fixed size, got [$encoderFixedSize] and [$decoderFixedSize].",
    },
    CodeDefinition {
        code: ErrorCode(8078006),
        name: "codecsEncoderDecoderMaxSizeMismatch",
        template: "Encoder and decoder must have the same max size, got [$encoderMaxSize] and [$decoderMaxSize].",
    },
    CodeDefinition {
        code: ErrorCode(8078007),
        name: "codecsInvalidNumberOfItems",
        template: "Expected [$codecDescription] to have $expected items, got $actual.",
    },
    CodeDefinition {
        code: ErrorCode(8078008),
        name: "codecsEnumDiscriminatorOutOfRange",
        template: "Enum discriminator out of range. Expected a number in [$formattedValidDiscriminators], got $discriminator.",
    },
    CodeDefinition {
        code: ErrorCode(8078009),
        name: "codecsInvalidDiscriminatedUnionVariant",
        template: "Invalid discriminated union variant. Expected one of [$variants], got $value.",
    },
    CodeDefinition {
        code: ErrorCode(8078010),
        name: "codecsInvalidEnumVariant",
        template: "Invalid enum variant. Expected one of [$stringValues] or a number in [$formattedNumericalValues], got $variant.",
    },
    CodeDefinition {
        code: ErrorCode(8078011),
        name: "codecsNumberOutOfRange",
        template: "Codec [$codecDescription] expected number to be in the range [$min, $max], got $value.",
    },
    CodeDefinition {
        code: ErrorCode(8078012),
        name: "codecsInvalidStringForBase",
        template: "Invalid value $value for base $base with alphabet $alphabet.",
    },
    CodeDefinition {
        code: ErrorCode(8078013),
        name: "codecsExpectedPositiveByteLength",
        template: "Codec [$codecDescription] expected a positive byte length, got $bytesLength.",
    },
    CodeDefinition {
        code: ErrorCode(8078014),
        name: "codecsOffsetOutOfRange",
        template: "Codec [$codecDescription] expected offset to be in the range [0, $bytesLength], got $offset.",
    },
    CodeDefinition {
        code: ErrorCode(8078015),
        name: "codecsInvalidLiteralUnionVariant",
        template: "Invalid literal union variant. Expected one of [$variants], got $value.",
    },
    CodeDefinition {
        code: ErrorCode(8078016),
        name: "codecsLiteralUnionDiscriminatorOutOfRange",
        template: "Literal union discriminator out of range. Expected a number between $minRange and $maxRange, got $discriminator.",
    },
    CodeDefinition {
        code: ErrorCode(8078017),
        name: "codecsUnionVariantOutOfRange",
        template: "Union variant out of range. Expected an index between $minRange and $maxRange, got $variant.",
    },
    CodeDefinition {
        code: ErrorCode(8078018),
        name: "codecsInvalidConstant",
        template: "Expected byte array constant [$hexConstant] to be present in data [$hexData] at offset [$offset].",
    },
    CodeDefinition {
        code: ErrorCode(8078019),
        name: "codecsExpectedZeroValueToMatchItemFixedSize",
        template: "Codec [$codecDescription] expected zero-value [$hexZeroValue] to have the same size as the provided fixed-size item [$expectedSize bytes].",
    },
    CodeDefinition {
        code: ErrorCode(8078020),
        name: "codecsEncodedBytesMustNotIncludeSentinel",
        template: "Sentinel [$hexSentinel] must not be present in encoded bytes [$hexEncodedBytes].",
    },
    CodeDefinition {
        code: ErrorCode(8078021),
        name: "codecsSentinelMissingInDecodedBytes",
        template: "Expected sentinel [$hexSentinel] to be present in decoded bytes [$hexDecodedBytes].",
    },
    CodeDefinition {
        code: ErrorCode(8078022),
        name: "codecsCannotUseLexicalValuesAsEnumDiscriminators",
        template: "Enum codec cannot use lexical values [$stringValues] as discriminators. Either remove all lexical values or set useValuesAsDiscriminators to false.",
    },
    CodeDefinition {
        code: ErrorCode(8078023),
        name: "codecsExpectedDecoderToConsumeEntireByteArray",
        template: "This decoder expected a byte array of exactly $expectedLength bytes, but $numExcessBytes unexpected excess bytes remained after decoding. Are you sure that you have chosen the correct decoder for this data?",
    },
    CodeDefinition {
        code: ErrorCode(8078024),
        name: "codecsInvalidPatternMatchValue",
        template: "Invalid pattern match value. The provided value does not match any of the specified patterns.",
    },
    CodeDefinition {
        code: ErrorCode(8078025),
        name: "codecsInvalidPatternMatchBytes",
        template: "Invalid pattern match bytes. The provided byte array does not match any of the specified patterns.",
    },
];
