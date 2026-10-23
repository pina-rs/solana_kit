//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `codecsInvalidUtf8Bytes` code.
    pub const CODECS_INVALID_UTF8_BYTES: Self = Self(8078026);
    /// Pinned `codecsInvalidUtf8String` code.
    pub const CODECS_INVALID_UTF8_STRING: Self = Self(8078027);
    /// Pinned `codecsSentinelMissingAtEndOfBytes` code.
    pub const CODECS_SENTINEL_MISSING_AT_END_OF_BYTES: Self = Self(8078028);
    /// Pinned `codecsSentinelMustNotBeEmpty` code.
    pub const CODECS_SENTINEL_MUST_NOT_BE_EMPTY: Self = Self(8078029);
    /// Pinned `codecsUnexpectedZeroFixedSizeItemForArrayLikeSizeStrategy` code.
    pub const CODECS_UNEXPECTED_ZERO_FIXED_SIZE_ITEM_FOR_ARRAY_LIKE_SIZE_STRATEGY: Self =
        Self(8078030);
    /// Pinned `codecsInvalidBoolean` code.
    pub const CODECS_INVALID_BOOLEAN: Self = Self(8078999);
    /// Pinned `fixedPointsInvalidTotalBits` code.
    pub const FIXED_POINTS_INVALID_TOTAL_BITS: Self = Self(8090000);
    /// Pinned `fixedPointsInvalidFractionalBits` code.
    pub const FIXED_POINTS_INVALID_FRACTIONAL_BITS: Self = Self(8090001);
    /// Pinned `fixedPointsInvalidDecimals` code.
    pub const FIXED_POINTS_INVALID_DECIMALS: Self = Self(8090002);
    /// Pinned `fixedPointsFractionalBitsExceedTotalBits` code.
    pub const FIXED_POINTS_FRACTIONAL_BITS_EXCEED_TOTAL_BITS: Self = Self(8090003);
    /// Pinned `fixedPointsValueOutOfRange` code.
    pub const FIXED_POINTS_VALUE_OUT_OF_RANGE: Self = Self(8090004);
    /// Pinned `fixedPointsInvalidString` code.
    pub const FIXED_POINTS_INVALID_STRING: Self = Self(8090005);
    /// Pinned `fixedPointsInvalidZeroDenominatorRatio` code.
    pub const FIXED_POINTS_INVALID_ZERO_DENOMINATOR_RATIO: Self = Self(8090006);
    /// Pinned `fixedPointsArithmeticOverflow` code.
    pub const FIXED_POINTS_ARITHMETIC_OVERFLOW: Self = Self(8090007);
    /// Pinned `fixedPointsShapeMismatch` code.
    pub const FIXED_POINTS_SHAPE_MISMATCH: Self = Self(8090008);
    /// Pinned `fixedPointsDivisionByZero` code.
    pub const FIXED_POINTS_DIVISION_BY_ZERO: Self = Self(8090009);
    /// Pinned `fixedPointsStrictModePrecisionLoss` code.
    pub const FIXED_POINTS_STRICT_MODE_PRECISION_LOSS: Self = Self(8090010);
    /// Pinned `fixedPointsMalformedRawValue` code.
    pub const FIXED_POINTS_MALFORMED_RAW_VALUE: Self = Self(8090011);
    /// Pinned `fixedPointsTotalBitsNotByteAligned` code.
    pub const FIXED_POINTS_TOTAL_BITS_NOT_BYTE_ALIGNED: Self = Self(8090012);
    /// Pinned `rpcIntegerOverflow` code.
    pub const RPC_INTEGER_OVERFLOW: Self = Self(8100000);
    /// Pinned `rpcTransportHttpHeaderForbidden` code.
    pub const RPC_TRANSPORT_HTTP_HEADER_FORBIDDEN: Self = Self(8100001);
    /// Pinned `rpcTransportHttpError` code.
    pub const RPC_TRANSPORT_HTTP_ERROR: Self = Self(8100002);
    /// Pinned `rpcApiPlanMissingForRpcMethod` code.
    pub const RPC_API_PLAN_MISSING_FOR_RPC_METHOD: Self = Self(8100003);
    /// Pinned `rpcSubscriptionsCannotCreateSubscriptionPlan` code.
    pub const RPC_SUBSCRIPTIONS_CANNOT_CREATE_SUBSCRIPTION_PLAN: Self = Self(8190000);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode(8078026),
        name: "codecsInvalidUtf8Bytes",
        template: "Invalid UTF-8 byte sequence at offset $offset.",
    },
    CodeDefinition {
        code: ErrorCode(8078027),
        name: "codecsInvalidUtf8String",
        template: "Invalid UTF-8 string. Found a lone surrogate at index $index.",
    },
    CodeDefinition {
        code: ErrorCode(8078028),
        name: "codecsSentinelMissingAtEndOfBytes",
        template: "Codec [$codecDescription] expected sentinel [$hexSentinel] to terminate the collection, but reached the end of the byte array without it.",
    },
    CodeDefinition {
        code: ErrorCode(8078029),
        name: "codecsSentinelMustNotBeEmpty",
        template: "The sentinel must not be empty.",
    },
    CodeDefinition {
        code: ErrorCode(8078030),
        name: "codecsUnexpectedZeroFixedSizeItemForArrayLikeSizeStrategy",
        template: "Codec [$codecDescription] cannot use the [$sizeStrategy] size strategy: the item codec has a fixed size of zero and consumes no bytes.",
    },
    CodeDefinition {
        code: ErrorCode(8078999),
        name: "codecsInvalidBoolean",
        template: "Invalid boolean value. Expected 0 or 1, got $value.",
    },
    CodeDefinition {
        code: ErrorCode(8090000),
        name: "fixedPointsInvalidTotalBits",
        template: "Invalid `totalBits`. Expected a positive integer, got $totalBits.",
    },
    CodeDefinition {
        code: ErrorCode(8090001),
        name: "fixedPointsInvalidFractionalBits",
        template: "Invalid `fractionalBits`. Expected a non-negative integer, got $fractionalBits.",
    },
    CodeDefinition {
        code: ErrorCode(8090002),
        name: "fixedPointsInvalidDecimals",
        template: "Invalid `decimals`. Expected a non-negative integer, got $decimals.",
    },
    CodeDefinition {
        code: ErrorCode(8090003),
        name: "fixedPointsFractionalBitsExceedTotalBits",
        template: "`fractionalBits` ($fractionalBits) must not exceed `totalBits` ($totalBits).",
    },
    CodeDefinition {
        code: ErrorCode(8090004),
        name: "fixedPointsValueOutOfRange",
        template: "Fixed-point value of kind `$kind` is out of range for $signedness $totalBits-bit storage. Expected a raw bigint in [$min, $max], got $raw.",
    },
    CodeDefinition {
        code: ErrorCode(8090005),
        name: "fixedPointsInvalidString",
        template: "Invalid string `$input` for fixed-point value of kind `$kind`.",
    },
    CodeDefinition {
        code: ErrorCode(8090006),
        name: "fixedPointsInvalidZeroDenominatorRatio",
        template: "Invalid ratio $numerator/$denominator for fixed-point value of kind `$kind`. Denominator must be non-zero.",
    },
    CodeDefinition {
        code: ErrorCode(8090007),
        name: "fixedPointsArithmeticOverflow",
        template: "Fixed-point operation `$operation` of kind `$kind` overflowed. Expected a raw bigint in [$min, $max], got $result.",
    },
    CodeDefinition {
        code: ErrorCode(8090008),
        name: "fixedPointsShapeMismatch",
        template: "Fixed-point `$operation` operation expected $expectedKind ($expectedSignedness, $expectedTotalBits bits, $expectedScale $expectedScaleLabel); got $actualKind ($actualSignedness, $actualTotalBits bits, $actualScale $actualScaleLabel).",
    },
    CodeDefinition {
        code: ErrorCode(8090009),
        name: "fixedPointsDivisionByZero",
        template: "Fixed-point division by zero for value of kind `$kind` ($signedness, $totalBits bits).",
    },
    CodeDefinition {
        code: ErrorCode(8090010),
        name: "fixedPointsStrictModePrecisionLoss",
        template: "Fixed-point operation `$operation` of kind `$kind` cannot be performed exactly; pass a rounding mode other than `strict` to allow a rounded result.",
    },
    CodeDefinition {
        code: ErrorCode(8090011),
        name: "fixedPointsMalformedRawValue",
        template: "Fixed-point value of kind `$kind` has a malformed `raw` field. Expected a bigint, got `$raw`.",
    },
    CodeDefinition {
        code: ErrorCode(8090012),
        name: "fixedPointsTotalBitsNotByteAligned",
        template: "Fixed-point codec of kind `$kind` requires `totalBits` to be a multiple of 8; got $totalBits.",
    },
    CodeDefinition {
        code: ErrorCode(8100000),
        name: "rpcIntegerOverflow",
        template: "The $argumentLabel argument to the $methodName RPC method was $value. This number is unsafe because it exceeds the maximum safe integer.",
    },
    CodeDefinition {
        code: ErrorCode(8100001),
        name: "rpcTransportHttpHeaderForbidden",
        template: "HTTP header(s) forbidden: $headers.",
    },
    CodeDefinition {
        code: ErrorCode(8100002),
        name: "rpcTransportHttpError",
        template: "HTTP error ($statusCode): $message",
    },
    CodeDefinition {
        code: ErrorCode(8100003),
        name: "rpcApiPlanMissingForRpcMethod",
        template: "Could not find an API plan for RPC method: $method",
    },
    CodeDefinition {
        code: ErrorCode(8190000),
        name: "rpcSubscriptionsCannotCreateSubscriptionPlan",
        template: "The notification name must end in 'Notifications' and the API must supply a subscription plan creator function for the notification '$notificationName'.",
    },
];
