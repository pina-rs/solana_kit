//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `transactionVersionZeroMustBeEncodedWithSignaturesFirst` code.
    pub const TRANSACTION_VERSION_ZERO_MUST_BE_ENCODED_WITH_SIGNATURES_FIRST: Self = Self(5663026);
    /// Pinned `transactionSignatureCountTooHighForTransactionBytes` code.
    pub const TRANSACTION_SIGNATURE_COUNT_TOO_HIGH_FOR_TRANSACTION_BYTES: Self = Self(5663027);
    /// Pinned `transactionInvalidConfigMaskPriorityFeeBits` code.
    pub const TRANSACTION_INVALID_CONFIG_MASK_PRIORITY_FEE_BITS: Self = Self(5663028);
    /// Pinned `transactionInvalidNonceAccountIndex` code.
    pub const TRANSACTION_INVALID_NONCE_ACCOUNT_INDEX: Self = Self(5663029);
    /// Pinned `transactionInvalidConfigValueKind` code.
    pub const TRANSACTION_INVALID_CONFIG_VALUE_KIND: Self = Self(5663030);
    /// Pinned `transactionInstructionHeadersPayloadsMismatch` code.
    pub const TRANSACTION_INSTRUCTION_HEADERS_PAYLOADS_MISMATCH: Self = Self(5663031);
    /// Pinned `transactionTooManySignerAddresses` code.
    pub const TRANSACTION_TOO_MANY_SIGNER_ADDRESSES: Self = Self(5663032);
    /// Pinned `transactionTooManyAccountAddresses` code.
    pub const TRANSACTION_TOO_MANY_ACCOUNT_ADDRESSES: Self = Self(5663033);
    /// Pinned `transactionTooManyInstructions` code.
    pub const TRANSACTION_TOO_MANY_INSTRUCTIONS: Self = Self(5663034);
    /// Pinned `transactionTooManyAccountsInInstruction` code.
    pub const TRANSACTION_TOO_MANY_ACCOUNTS_IN_INSTRUCTION: Self = Self(5663035);
    /// Pinned `transactionFailedToEstimateLoadedAccountsDataSizeLimit` code.
    pub const TRANSACTION_FAILED_TO_ESTIMATE_LOADED_ACCOUNTS_DATA_SIZE_LIMIT: Self = Self(5663036);
    /// Pinned `transactionFailedWhenSimulatingToEstimateResourceLimits` code.
    pub const TRANSACTION_FAILED_WHEN_SIMULATING_TO_ESTIMATE_RESOURCE_LIMITS: Self = Self(5663037);
    /// Pinned `transactionFailedToDecompileInstructionAccountIndexOutOfRange` code.
    pub const TRANSACTION_FAILED_TO_DECOMPILE_INSTRUCTION_ACCOUNT_INDEX_OUT_OF_RANGE: Self =
        Self(5663038);
    /// Pinned `transactionComputeUnitLimitOutOfRange` code.
    pub const TRANSACTION_COMPUTE_UNIT_LIMIT_OUT_OF_RANGE: Self = Self(5663039);
    /// Pinned `transactionInvalidHeapSize` code.
    pub const TRANSACTION_INVALID_HEAP_SIZE: Self = Self(5663040);
    /// Pinned `transactionInvalidNonceFormat` code.
    pub const TRANSACTION_INVALID_NONCE_FORMAT: Self = Self(5663041);
    /// Pinned `transactionReportedSignatureMismatch` code.
    pub const TRANSACTION_REPORTED_SIGNATURE_MISMATCH: Self = Self(5663042);
    /// Pinned `transactionIntrospectionCannotDecodeJsonParsedTransaction` code.
    pub const TRANSACTION_INTROSPECTION_CANNOT_DECODE_JSON_PARSED_TRANSACTION: Self = Self(5664000);
    /// Pinned `transactionIntrospectionUnrecognizedGetTransactionResponse` code.
    pub const TRANSACTION_INTROSPECTION_UNRECOGNIZED_GET_TRANSACTION_RESPONSE: Self = Self(5664001);
    /// Pinned `transactionIntrospectionHeaderAccountsMismatch` code.
    pub const TRANSACTION_INTROSPECTION_HEADER_ACCOUNTS_MISMATCH: Self = Self(5664002);
    /// Pinned `transactionErrorUnknown` code.
    pub const TRANSACTION_ERROR_UNKNOWN: Self = Self(7050000);
    /// Pinned `transactionErrorAccountInUse` code.
    pub const TRANSACTION_ERROR_ACCOUNT_IN_USE: Self = Self(7050001);
    /// Pinned `transactionErrorAccountLoadedTwice` code.
    pub const TRANSACTION_ERROR_ACCOUNT_LOADED_TWICE: Self = Self(7050002);
    /// Pinned `transactionErrorAccountNotFound` code.
    pub const TRANSACTION_ERROR_ACCOUNT_NOT_FOUND: Self = Self(7050003);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode(5663026),
        name: "transactionVersionZeroMustBeEncodedWithSignaturesFirst",
        template: "Transaction version 0 must be encoded with signatures first. This transaction was encoded with first byte $firstByte, which is expected to be a signature count for v0 transactions.",
    },
    CodeDefinition {
        code: ErrorCode(5663027),
        name: "transactionSignatureCountTooHighForTransactionBytes",
        template: "The provided transaction bytes expect that there should be $numExpectedSignatures signatures, but the bytes are not long enough to contain a transaction message with this many signatures. The provided bytes are $transactionBytesLength bytes long.",
    },
    CodeDefinition {
        code: ErrorCode(5663028),
        name: "transactionInvalidConfigMaskPriorityFeeBits",
        template: "Invalid transaction config mask: $mask. Bits 0 and 1 must match (both set or both unset)",
    },
    CodeDefinition {
        code: ErrorCode(5663029),
        name: "transactionInvalidNonceAccountIndex",
        template: "The transaction has a durable nonce lifetime, but the nonce account index is invalid. Expected a nonce account index less than $numberOfStaticAccounts, got $nonceAccountIndex.",
    },
    CodeDefinition {
        code: ErrorCode(5663030),
        name: "transactionInvalidConfigValueKind",
        template: "The transaction config value for $configName has the incorrect kind. Expected $expectedKind, got $actualKind.",
    },
    CodeDefinition {
        code: ErrorCode(5663031),
        name: "transactionInstructionHeadersPayloadsMismatch",
        template: "The transaction does not have the same number of instruction headers and instruction payloads. Got $numInstructionHeaders instruction headers, and $numInstructionPayloads instruction payloads.",
    },
    CodeDefinition {
        code: ErrorCode(5663032),
        name: "transactionTooManySignerAddresses",
        template: "Transaction has $actualCount unique signer addresses but the maximum allowed is $maxAllowed",
    },
    CodeDefinition {
        code: ErrorCode(5663033),
        name: "transactionTooManyAccountAddresses",
        template: "Transaction has $actualCount unique account addresses but the maximum allowed is $maxAllowed",
    },
    CodeDefinition {
        code: ErrorCode(5663034),
        name: "transactionTooManyInstructions",
        template: "Transaction has $actualCount instructions but the maximum allowed is $maxAllowed",
    },
    CodeDefinition {
        code: ErrorCode(5663035),
        name: "transactionTooManyAccountsInInstruction",
        template: "The instruction at index $instructionIndex has $actualCount account references but the maximum allowed is $maxAllowed",
    },
    CodeDefinition {
        code: ErrorCode(5663036),
        name: "transactionFailedToEstimateLoadedAccountsDataSizeLimit",
        template: "Failed to estimate the loaded accounts data size limit for this transaction message.",
    },
    CodeDefinition {
        code: ErrorCode(5663037),
        name: "transactionFailedWhenSimulatingToEstimateResourceLimits",
        template: "Transaction failed when it was simulated in order to estimate resource limits.",
    },
    CodeDefinition {
        code: ErrorCode(5663038),
        name: "transactionFailedToDecompileInstructionAccountIndexOutOfRange",
        template: "Could not find an account address at index $index while decompiling an instruction",
    },
    CodeDefinition {
        code: ErrorCode(5663039),
        name: "transactionComputeUnitLimitOutOfRange",
        template: "Transaction compute unit limit must be an integer in the range [0, $maxComputeUnitLimit]. `$computeUnitLimit` given",
    },
    CodeDefinition {
        code: ErrorCode(5663040),
        name: "transactionInvalidHeapSize",
        template: "Transaction heap size must be an integer multiple of $multipleOf bytes in the range [$minHeapSize, $maxHeapSize]. `$heapSize` given",
    },
    CodeDefinition {
        code: ErrorCode(5663041),
        name: "transactionInvalidNonceFormat",
        template: "Expected base58 encoded nonce to decode to a byte array of length 32. Actual length: $actualLength.",
    },
    CodeDefinition {
        code: ErrorCode(5663042),
        name: "transactionReportedSignatureMismatch",
        template: "The signature returned for the submitted transaction does not match the transaction signature computed locally. Expected $expectedSignature, got $reportedSignature.",
    },
    CodeDefinition {
        code: ErrorCode(5664000),
        name: "transactionIntrospectionCannotDecodeJsonParsedTransaction",
        template: "`getTransaction` responses fetched with `encoding: 'jsonParsed'` cannot be decoded. Re-fetch the transaction with `encoding: 'base64'`, `'base58'`, or `'json'`",
    },
    CodeDefinition {
        code: ErrorCode(5664001),
        name: "transactionIntrospectionUnrecognizedGetTransactionResponse",
        template: "Could not recognize the shape of this `getTransaction` response. Expected a response fetched with `encoding: 'base64'`, `'base58'`, or `'json'`",
    },
    CodeDefinition {
        code: ErrorCode(5664002),
        name: "transactionIntrospectionHeaderAccountsMismatch",
        template: "The transaction message header counts do not fit within the static account list. numSignerAccounts: $numSignerAccounts, numReadonlyNonSignerAccounts: $numReadonlyNonSignerAccounts, numStaticAccounts: $numStaticAccounts.",
    },
    CodeDefinition {
        code: ErrorCode(7050000),
        name: "transactionErrorUnknown",
        template: "The transaction failed with the error $errorName",
    },
    CodeDefinition {
        code: ErrorCode(7050001),
        name: "transactionErrorAccountInUse",
        template: "Account in use",
    },
    CodeDefinition {
        code: ErrorCode(7050002),
        name: "transactionErrorAccountLoadedTwice",
        template: "Account loaded twice",
    },
    CodeDefinition {
        code: ErrorCode(7050003),
        name: "transactionErrorAccountNotFound",
        template: "Attempt to debit an account but found no record of a prior credit.",
    },
];
