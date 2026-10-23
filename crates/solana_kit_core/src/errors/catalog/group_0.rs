//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `blockHeightExceeded` code.
    pub const BLOCK_HEIGHT_EXCEEDED: Self = Self(1);
    /// Pinned `invalidNonce` code.
    pub const INVALID_NONCE: Self = Self(2);
    /// Pinned `nonceAccountNotFound` code.
    pub const NONCE_ACCOUNT_NOT_FOUND: Self = Self(3);
    /// Pinned `blockhashStringLengthOutOfRange` code.
    pub const BLOCKHASH_STRING_LENGTH_OUT_OF_RANGE: Self = Self(4);
    /// Pinned `invalidBlockhashByteLength` code.
    pub const INVALID_BLOCKHASH_BYTE_LENGTH: Self = Self(5);
    /// Pinned `lamportsOutOfRange` code.
    pub const LAMPORTS_OUT_OF_RANGE: Self = Self(6);
    /// Pinned `malformedBigintString` code.
    pub const MALFORMED_BIGINT_STRING: Self = Self(7);
    /// Pinned `malformedNumberString` code.
    pub const MALFORMED_NUMBER_STRING: Self = Self(8);
    /// Pinned `timestampOutOfRange` code.
    pub const TIMESTAMP_OUT_OF_RANGE: Self = Self(9);
    /// Pinned `malformedJsonRpcError` code.
    pub const MALFORMED_JSON_RPC_ERROR: Self = Self(10);
    /// Pinned `failedToSendTransaction` code.
    pub const FAILED_TO_SEND_TRANSACTION: Self = Self(11);
    /// Pinned `failedToSendTransactions` code.
    pub const FAILED_TO_SEND_TRANSACTIONS: Self = Self(12);
    /// Pinned `failedToSignTransaction` code.
    pub const FAILED_TO_SIGN_TRANSACTION: Self = Self(13);
    /// Pinned `failedToSignTransactions` code.
    pub const FAILED_TO_SIGN_TRANSACTIONS: Self = Self(14);
    /// Pinned `jsonRpcParseError` code.
    pub const JSON_RPC_PARSE_ERROR: Self = Self(-32700);
    /// Pinned `jsonRpcInternalError` code.
    pub const JSON_RPC_INTERNAL_ERROR: Self = Self(-32603);
    /// Pinned `jsonRpcInvalidParams` code.
    pub const JSON_RPC_INVALID_PARAMS: Self = Self(-32602);
    /// Pinned `jsonRpcMethodNotFound` code.
    pub const JSON_RPC_METHOD_NOT_FOUND: Self = Self(-32601);
    /// Pinned `jsonRpcInvalidRequest` code.
    pub const JSON_RPC_INVALID_REQUEST: Self = Self(-32600);
    /// Pinned `jsonRpcServerErrorNoSlotHistory` code.
    pub const JSON_RPC_SERVER_ERROR_NO_SLOT_HISTORY: Self = Self(-32021);
    /// Pinned `jsonRpcServerErrorFilterTransactionNotFound` code.
    pub const JSON_RPC_SERVER_ERROR_FILTER_TRANSACTION_NOT_FOUND: Self = Self(-32020);
    /// Pinned `jsonRpcServerErrorLongTermStorageUnreachable` code.
    pub const JSON_RPC_SERVER_ERROR_LONG_TERM_STORAGE_UNREACHABLE: Self = Self(-32019);
    /// Pinned `jsonRpcServerErrorSlotNotEpochBoundary` code.
    pub const JSON_RPC_SERVER_ERROR_SLOT_NOT_EPOCH_BOUNDARY: Self = Self(-32018);
    /// Pinned `jsonRpcServerErrorEpochRewardsPeriodActive` code.
    pub const JSON_RPC_SERVER_ERROR_EPOCH_REWARDS_PERIOD_ACTIVE: Self = Self(-32017);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode(1),
        name: "blockHeightExceeded",
        template: "The network has progressed past the last block for which this transaction could have been committed.",
    },
    CodeDefinition {
        code: ErrorCode(2),
        name: "invalidNonce",
        template: "The nonce $expectedNonceValue is no longer valid. It has advanced to $actualNonceValue",
    },
    CodeDefinition {
        code: ErrorCode(3),
        name: "nonceAccountNotFound",
        template: "No nonce account could be found at address $nonceAccountAddress",
    },
    CodeDefinition {
        code: ErrorCode(4),
        name: "blockhashStringLengthOutOfRange",
        template: "Expected base58-encoded blockhash string of length in the range [32, 44]. Actual length: $actualLength.",
    },
    CodeDefinition {
        code: ErrorCode(5),
        name: "invalidBlockhashByteLength",
        template: "Expected base58 encoded blockhash to decode to a byte array of length 32. Actual length: $actualLength.",
    },
    CodeDefinition {
        code: ErrorCode(6),
        name: "lamportsOutOfRange",
        template: "Lamports value must be in the range [0, 2e64-1]",
    },
    CodeDefinition {
        code: ErrorCode(7),
        name: "malformedBigintString",
        template: "$value cannot be parsed as a BigInt",
    },
    CodeDefinition {
        code: ErrorCode(8),
        name: "malformedNumberString",
        template: "$value cannot be parsed as a Number",
    },
    CodeDefinition {
        code: ErrorCode(9),
        name: "timestampOutOfRange",
        template: "Timestamp value must be in the range [-(2^63), (2^63) - 1]. $value given",
    },
    CodeDefinition {
        code: ErrorCode(10),
        name: "malformedJsonRpcError",
        template: "$message",
    },
    CodeDefinition {
        code: ErrorCode(11),
        name: "failedToSendTransaction",
        template: "Failed to send transaction$causeMessage",
    },
    CodeDefinition {
        code: ErrorCode(12),
        name: "failedToSendTransactions",
        template: "Failed to send transactions$causeMessages",
    },
    CodeDefinition {
        code: ErrorCode(13),
        name: "failedToSignTransaction",
        template: "Failed to sign transaction$causeMessage",
    },
    CodeDefinition {
        code: ErrorCode(14),
        name: "failedToSignTransactions",
        template: "Failed to sign transactions$causeMessages",
    },
    CodeDefinition {
        code: ErrorCode(-32700),
        name: "jsonRpcParseError",
        template: "JSON-RPC error: An error occurred on the server while parsing the JSON text ($__serverMessage)",
    },
    CodeDefinition {
        code: ErrorCode(-32603),
        name: "jsonRpcInternalError",
        template: "JSON-RPC error: Internal JSON-RPC error ($__serverMessage)",
    },
    CodeDefinition {
        code: ErrorCode(-32602),
        name: "jsonRpcInvalidParams",
        template: "JSON-RPC error: Invalid method parameter(s) ($__serverMessage)",
    },
    CodeDefinition {
        code: ErrorCode(-32601),
        name: "jsonRpcMethodNotFound",
        template: "JSON-RPC error: The method does not exist / is not available ($__serverMessage)",
    },
    CodeDefinition {
        code: ErrorCode(-32600),
        name: "jsonRpcInvalidRequest",
        template: "JSON-RPC error: The JSON sent is not a valid Request object ($__serverMessage)",
    },
    CodeDefinition {
        code: ErrorCode(-32021),
        name: "jsonRpcServerErrorNoSlotHistory",
        template: "$__serverMessage",
    },
    CodeDefinition {
        code: ErrorCode(-32020),
        name: "jsonRpcServerErrorFilterTransactionNotFound",
        template: "$__serverMessage",
    },
    CodeDefinition {
        code: ErrorCode(-32019),
        name: "jsonRpcServerErrorLongTermStorageUnreachable",
        template: "Failed to query long-term storage; please try again",
    },
    CodeDefinition {
        code: ErrorCode(-32018),
        name: "jsonRpcServerErrorSlotNotEpochBoundary",
        template: "Rewards cannot be found because slot $slot is not the epoch boundary.",
    },
    CodeDefinition {
        code: ErrorCode(-32017),
        name: "jsonRpcServerErrorEpochRewardsPeriodActive",
        template: "Epoch rewards period still active at slot $slot",
    },
];
