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
        code: ErrorCode::BLOCK_HEIGHT_EXCEEDED,
        name: "blockHeightExceeded",
        template: "The network has progressed past the last block for which this transaction could have been committed.",
    },
    CodeDefinition {
        code: ErrorCode::INVALID_NONCE,
        name: "invalidNonce",
        template: "The nonce $expectedNonceValue is no longer valid. It has advanced to $actualNonceValue",
    },
    CodeDefinition {
        code: ErrorCode::NONCE_ACCOUNT_NOT_FOUND,
        name: "nonceAccountNotFound",
        template: "No nonce account could be found at address $nonceAccountAddress",
    },
    CodeDefinition {
        code: ErrorCode::BLOCKHASH_STRING_LENGTH_OUT_OF_RANGE,
        name: "blockhashStringLengthOutOfRange",
        template: "Expected base58-encoded blockhash string of length in the range [32, 44]. Actual length: $actualLength.",
    },
    CodeDefinition {
        code: ErrorCode::INVALID_BLOCKHASH_BYTE_LENGTH,
        name: "invalidBlockhashByteLength",
        template: "Expected base58 encoded blockhash to decode to a byte array of length 32. Actual length: $actualLength.",
    },
    CodeDefinition {
        code: ErrorCode::LAMPORTS_OUT_OF_RANGE,
        name: "lamportsOutOfRange",
        template: "Lamports value must be in the range [0, 2e64-1]",
    },
    CodeDefinition {
        code: ErrorCode::MALFORMED_BIGINT_STRING,
        name: "malformedBigintString",
        template: "$value cannot be parsed as a BigInt",
    },
    CodeDefinition {
        code: ErrorCode::MALFORMED_NUMBER_STRING,
        name: "malformedNumberString",
        template: "$value cannot be parsed as a Number",
    },
    CodeDefinition {
        code: ErrorCode::TIMESTAMP_OUT_OF_RANGE,
        name: "timestampOutOfRange",
        template: "Timestamp value must be in the range [-(2^63), (2^63) - 1]. $value given",
    },
    CodeDefinition {
        code: ErrorCode::MALFORMED_JSON_RPC_ERROR,
        name: "malformedJsonRpcError",
        template: "$message",
    },
    CodeDefinition {
        code: ErrorCode::FAILED_TO_SEND_TRANSACTION,
        name: "failedToSendTransaction",
        template: "Failed to send transaction$causeMessage",
    },
    CodeDefinition {
        code: ErrorCode::FAILED_TO_SEND_TRANSACTIONS,
        name: "failedToSendTransactions",
        template: "Failed to send transactions$causeMessages",
    },
    CodeDefinition {
        code: ErrorCode::FAILED_TO_SIGN_TRANSACTION,
        name: "failedToSignTransaction",
        template: "Failed to sign transaction$causeMessage",
    },
    CodeDefinition {
        code: ErrorCode::FAILED_TO_SIGN_TRANSACTIONS,
        name: "failedToSignTransactions",
        template: "Failed to sign transactions$causeMessages",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_PARSE_ERROR,
        name: "jsonRpcParseError",
        template: "JSON-RPC error: An error occurred on the server while parsing the JSON text ($__serverMessage)",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_INTERNAL_ERROR,
        name: "jsonRpcInternalError",
        template: "JSON-RPC error: Internal JSON-RPC error ($__serverMessage)",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_INVALID_PARAMS,
        name: "jsonRpcInvalidParams",
        template: "JSON-RPC error: Invalid method parameter(s) ($__serverMessage)",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_METHOD_NOT_FOUND,
        name: "jsonRpcMethodNotFound",
        template: "JSON-RPC error: The method does not exist / is not available ($__serverMessage)",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_INVALID_REQUEST,
        name: "jsonRpcInvalidRequest",
        template: "JSON-RPC error: The JSON sent is not a valid Request object ($__serverMessage)",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_NO_SLOT_HISTORY,
        name: "jsonRpcServerErrorNoSlotHistory",
        template: "$__serverMessage",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_FILTER_TRANSACTION_NOT_FOUND,
        name: "jsonRpcServerErrorFilterTransactionNotFound",
        template: "$__serverMessage",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_LONG_TERM_STORAGE_UNREACHABLE,
        name: "jsonRpcServerErrorLongTermStorageUnreachable",
        template: "Failed to query long-term storage; please try again",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_SLOT_NOT_EPOCH_BOUNDARY,
        name: "jsonRpcServerErrorSlotNotEpochBoundary",
        template: "Rewards cannot be found because slot $slot is not the epoch boundary.",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_EPOCH_REWARDS_PERIOD_ACTIVE,
        name: "jsonRpcServerErrorEpochRewardsPeriodActive",
        template: "Epoch rewards period still active at slot $slot",
    },
];
