//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `jsonRpcServerErrorMinContextSlotNotReached` code.
    pub const JSON_RPC_SERVER_ERROR_MIN_CONTEXT_SLOT_NOT_REACHED: Self = Self(-32016);
    /// Pinned `jsonRpcServerErrorUnsupportedTransactionVersion` code.
    pub const JSON_RPC_SERVER_ERROR_UNSUPPORTED_TRANSACTION_VERSION: Self = Self(-32015);
    /// Pinned `jsonRpcServerErrorBlockStatusNotAvailableYet` code.
    pub const JSON_RPC_SERVER_ERROR_BLOCK_STATUS_NOT_AVAILABLE_YET: Self = Self(-32014);
    /// Pinned `jsonRpcServerErrorTransactionSignatureLenMismatch` code.
    pub const JSON_RPC_SERVER_ERROR_TRANSACTION_SIGNATURE_LEN_MISMATCH: Self = Self(-32013);
    /// Pinned `jsonRpcScanError` code.
    pub const JSON_RPC_SCAN_ERROR: Self = Self(-32012);
    /// Pinned `jsonRpcServerErrorTransactionHistoryNotAvailable` code.
    pub const JSON_RPC_SERVER_ERROR_TRANSACTION_HISTORY_NOT_AVAILABLE: Self = Self(-32011);
    /// Pinned `jsonRpcServerErrorKeyExcludedFromSecondaryIndex` code.
    pub const JSON_RPC_SERVER_ERROR_KEY_EXCLUDED_FROM_SECONDARY_INDEX: Self = Self(-32010);
    /// Pinned `jsonRpcServerErrorLongTermStorageSlotSkipped` code.
    pub const JSON_RPC_SERVER_ERROR_LONG_TERM_STORAGE_SLOT_SKIPPED: Self = Self(-32009);
    /// Pinned `jsonRpcServerErrorNoSnapshot` code.
    pub const JSON_RPC_SERVER_ERROR_NO_SNAPSHOT: Self = Self(-32008);
    /// Pinned `jsonRpcServerErrorSlotSkipped` code.
    pub const JSON_RPC_SERVER_ERROR_SLOT_SKIPPED: Self = Self(-32007);
    /// Pinned `jsonRpcServerErrorTransactionPrecompileVerificationFailure` code.
    pub const JSON_RPC_SERVER_ERROR_TRANSACTION_PRECOMPILE_VERIFICATION_FAILURE: Self =
        Self(-32006);
    /// Pinned `jsonRpcServerErrorNodeUnhealthy` code.
    pub const JSON_RPC_SERVER_ERROR_NODE_UNHEALTHY: Self = Self(-32005);
    /// Pinned `jsonRpcServerErrorBlockNotAvailable` code.
    pub const JSON_RPC_SERVER_ERROR_BLOCK_NOT_AVAILABLE: Self = Self(-32004);
    /// Pinned `jsonRpcServerErrorTransactionSignatureVerificationFailure` code.
    pub const JSON_RPC_SERVER_ERROR_TRANSACTION_SIGNATURE_VERIFICATION_FAILURE: Self = Self(-32003);
    /// Pinned `jsonRpcServerErrorSendTransactionPreflightFailure` code.
    pub const JSON_RPC_SERVER_ERROR_SEND_TRANSACTION_PREFLIGHT_FAILURE: Self = Self(-32002);
    /// Pinned `jsonRpcServerErrorBlockCleanedUp` code.
    pub const JSON_RPC_SERVER_ERROR_BLOCK_CLEANED_UP: Self = Self(-32001);
    /// Pinned `addressesInvalidByteLength` code.
    pub const ADDRESSES_INVALID_BYTE_LENGTH: Self = Self(2800000);
    /// Pinned `addressesStringLengthOutOfRange` code.
    pub const ADDRESSES_STRING_LENGTH_OUT_OF_RANGE: Self = Self(2800001);
    /// Pinned `addressesInvalidBase58EncodedAddress` code.
    pub const ADDRESSES_INVALID_BASE58_ENCODED_ADDRESS: Self = Self(2800002);
    /// Pinned `addressesInvalidEd25519PublicKey` code.
    pub const ADDRESSES_INVALID_ED25519_PUBLIC_KEY: Self = Self(2800003);
    /// Pinned `addressesMalformedPda` code.
    pub const ADDRESSES_MALFORMED_PDA: Self = Self(2800004);
    /// Pinned `addressesPdaBumpSeedOutOfRange` code.
    pub const ADDRESSES_PDA_BUMP_SEED_OUT_OF_RANGE: Self = Self(2800005);
    /// Pinned `addressesMaxNumberOfPdaSeedsExceeded` code.
    pub const ADDRESSES_MAX_NUMBER_OF_PDA_SEEDS_EXCEEDED: Self = Self(2800006);
    /// Pinned `addressesMaxPdaSeedLengthExceeded` code.
    pub const ADDRESSES_MAX_PDA_SEED_LENGTH_EXCEEDED: Self = Self(2800007);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_MIN_CONTEXT_SLOT_NOT_REACHED,
        name: "jsonRpcServerErrorMinContextSlotNotReached",
        template: "Minimum context slot has not been reached",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_UNSUPPORTED_TRANSACTION_VERSION,
        name: "jsonRpcServerErrorUnsupportedTransactionVersion",
        template: "$__serverMessage",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_BLOCK_STATUS_NOT_AVAILABLE_YET,
        name: "jsonRpcServerErrorBlockStatusNotAvailableYet",
        template: "$__serverMessage",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_TRANSACTION_SIGNATURE_LEN_MISMATCH,
        name: "jsonRpcServerErrorTransactionSignatureLenMismatch",
        template: "Transaction signature length mismatch",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SCAN_ERROR,
        name: "jsonRpcScanError",
        template: "$__serverMessage",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_TRANSACTION_HISTORY_NOT_AVAILABLE,
        name: "jsonRpcServerErrorTransactionHistoryNotAvailable",
        template: "Transaction history is not available from this node",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_KEY_EXCLUDED_FROM_SECONDARY_INDEX,
        name: "jsonRpcServerErrorKeyExcludedFromSecondaryIndex",
        template: "$__serverMessage",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_LONG_TERM_STORAGE_SLOT_SKIPPED,
        name: "jsonRpcServerErrorLongTermStorageSlotSkipped",
        template: "$__serverMessage",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_NO_SNAPSHOT,
        name: "jsonRpcServerErrorNoSnapshot",
        template: "No snapshot",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_SLOT_SKIPPED,
        name: "jsonRpcServerErrorSlotSkipped",
        template: "$__serverMessage",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_TRANSACTION_PRECOMPILE_VERIFICATION_FAILURE,
        name: "jsonRpcServerErrorTransactionPrecompileVerificationFailure",
        template: "$__serverMessage",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_NODE_UNHEALTHY,
        name: "jsonRpcServerErrorNodeUnhealthy",
        template: "Node is unhealthy; behind by $numSlotsBehind slots",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_BLOCK_NOT_AVAILABLE,
        name: "jsonRpcServerErrorBlockNotAvailable",
        template: "$__serverMessage",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_TRANSACTION_SIGNATURE_VERIFICATION_FAILURE,
        name: "jsonRpcServerErrorTransactionSignatureVerificationFailure",
        template: "Transaction signature verification failure",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_SEND_TRANSACTION_PREFLIGHT_FAILURE,
        name: "jsonRpcServerErrorSendTransactionPreflightFailure",
        template: "Transaction simulation failed",
    },
    CodeDefinition {
        code: ErrorCode::JSON_RPC_SERVER_ERROR_BLOCK_CLEANED_UP,
        name: "jsonRpcServerErrorBlockCleanedUp",
        template: "$__serverMessage",
    },
    CodeDefinition {
        code: ErrorCode::ADDRESSES_INVALID_BYTE_LENGTH,
        name: "addressesInvalidByteLength",
        template: "Expected base58 encoded address to decode to a byte array of length 32. Actual length: $actualLength.",
    },
    CodeDefinition {
        code: ErrorCode::ADDRESSES_STRING_LENGTH_OUT_OF_RANGE,
        name: "addressesStringLengthOutOfRange",
        template: "Expected base58-encoded address string of length in the range [32, 44]. Actual length: $actualLength.",
    },
    CodeDefinition {
        code: ErrorCode::ADDRESSES_INVALID_BASE58_ENCODED_ADDRESS,
        name: "addressesInvalidBase58EncodedAddress",
        template: "$putativeAddress is not a base58-encoded address.",
    },
    CodeDefinition {
        code: ErrorCode::ADDRESSES_INVALID_ED25519_PUBLIC_KEY,
        name: "addressesInvalidEd25519PublicKey",
        template: "The CryptoKey must be an Ed25519 public key.",
    },
    CodeDefinition {
        code: ErrorCode::ADDRESSES_MALFORMED_PDA,
        name: "addressesMalformedPda",
        template: "Expected given program derived address to have the following format: [Address, ProgramDerivedAddressBump].",
    },
    CodeDefinition {
        code: ErrorCode::ADDRESSES_PDA_BUMP_SEED_OUT_OF_RANGE,
        name: "addressesPdaBumpSeedOutOfRange",
        template: "Expected program derived address bump to be in the range [0, 255], got: $bump.",
    },
    CodeDefinition {
        code: ErrorCode::ADDRESSES_MAX_NUMBER_OF_PDA_SEEDS_EXCEEDED,
        name: "addressesMaxNumberOfPdaSeedsExceeded",
        template: "A maximum of $maxSeeds seeds, including the bump seed, may be supplied when creating an address. Received: $actual.",
    },
    CodeDefinition {
        code: ErrorCode::ADDRESSES_MAX_PDA_SEED_LENGTH_EXCEEDED,
        name: "addressesMaxPdaSeedLengthExceeded",
        template: "The seed at index $index with length $actual exceeds the maximum length of $maxSeedLength bytes.",
    },
];
