//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `transactionExpectedBlockhashLifetime` code.
    pub const TRANSACTION_EXPECTED_BLOCKHASH_LIFETIME: Self = Self(5663002);
    /// Pinned `transactionExpectedNonceLifetime` code.
    pub const TRANSACTION_EXPECTED_NONCE_LIFETIME: Self = Self(5663003);
    /// Pinned `transactionVersionNumberOutOfRange` code.
    pub const TRANSACTION_VERSION_NUMBER_OUT_OF_RANGE: Self = Self(5663004);
    /// Pinned `transactionFailedToDecompileAddressLookupTableContentsMissing` code.
    pub const TRANSACTION_FAILED_TO_DECOMPILE_ADDRESS_LOOKUP_TABLE_CONTENTS_MISSING: Self =
        Self(5663005);
    /// Pinned `transactionFailedToDecompileAddressLookupTableIndexOutOfRange` code.
    pub const TRANSACTION_FAILED_TO_DECOMPILE_ADDRESS_LOOKUP_TABLE_INDEX_OUT_OF_RANGE: Self =
        Self(5663006);
    /// Pinned `transactionFailedToDecompileInstructionProgramAddressNotFound` code.
    pub const TRANSACTION_FAILED_TO_DECOMPILE_INSTRUCTION_PROGRAM_ADDRESS_NOT_FOUND: Self =
        Self(5663007);
    /// Pinned `transactionFailedToDecompileFeePayerMissing` code.
    pub const TRANSACTION_FAILED_TO_DECOMPILE_FEE_PAYER_MISSING: Self = Self(5663008);
    /// Pinned `transactionSignaturesMissing` code.
    pub const TRANSACTION_SIGNATURES_MISSING: Self = Self(5663009);
    /// Pinned `transactionAddressMissing` code.
    pub const TRANSACTION_ADDRESS_MISSING: Self = Self(5663010);
    /// Pinned `transactionFeePayerMissing` code.
    pub const TRANSACTION_FEE_PAYER_MISSING: Self = Self(5663011);
    /// Pinned `transactionFeePayerSignatureMissing` code.
    pub const TRANSACTION_FEE_PAYER_SIGNATURE_MISSING: Self = Self(5663012);
    /// Pinned `transactionInvalidNonceTransactionInstructionsMissing` code.
    pub const TRANSACTION_INVALID_NONCE_TRANSACTION_INSTRUCTIONS_MISSING: Self = Self(5663013);
    /// Pinned `transactionInvalidNonceTransactionFirstInstructionMustBeAdvanceNonce` code.
    pub const TRANSACTION_INVALID_NONCE_TRANSACTION_FIRST_INSTRUCTION_MUST_BE_ADVANCE_NONCE: Self =
        Self(5663014);
    /// Pinned `transactionAddressesCannotSignTransaction` code.
    pub const TRANSACTION_ADDRESSES_CANNOT_SIGN_TRANSACTION: Self = Self(5663015);
    /// Pinned `transactionCannotEncodeWithEmptySignatures` code.
    pub const TRANSACTION_CANNOT_ENCODE_WITH_EMPTY_SIGNATURES: Self = Self(5663016);
    /// Pinned `transactionMessageSignaturesMismatch` code.
    pub const TRANSACTION_MESSAGE_SIGNATURES_MISMATCH: Self = Self(5663017);
    /// Pinned `transactionFailedToEstimateComputeLimit` code.
    pub const TRANSACTION_FAILED_TO_ESTIMATE_COMPUTE_LIMIT: Self = Self(5663018);
    /// Pinned `transactionFailedWhenSimulatingToEstimateComputeLimit` code.
    pub const TRANSACTION_FAILED_WHEN_SIMULATING_TO_ESTIMATE_COMPUTE_LIMIT: Self = Self(5663019);
    /// Pinned `transactionExceedsSizeLimit` code.
    pub const TRANSACTION_EXCEEDS_SIZE_LIMIT: Self = Self(5663020);
    /// Pinned `transactionVersionNumberNotSupported` code.
    pub const TRANSACTION_VERSION_NUMBER_NOT_SUPPORTED: Self = Self(5663021);
    /// Pinned `transactionNonceAccountCannotBeInLookupTable` code.
    pub const TRANSACTION_NONCE_ACCOUNT_CANNOT_BE_IN_LOOKUP_TABLE: Self = Self(5663022);
    /// Pinned `transactionMalformedMessageBytes` code.
    pub const TRANSACTION_MALFORMED_MESSAGE_BYTES: Self = Self(5663023);
    /// Pinned `transactionCannotEncodeWithEmptyMessageBytes` code.
    pub const TRANSACTION_CANNOT_ENCODE_WITH_EMPTY_MESSAGE_BYTES: Self = Self(5663024);
    /// Pinned `transactionCannotDecodeEmptyTransactionBytes` code.
    pub const TRANSACTION_CANNOT_DECODE_EMPTY_TRANSACTION_BYTES: Self = Self(5663025);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode(5663002),
        name: "transactionExpectedBlockhashLifetime",
        template: "Transaction does not have a blockhash lifetime",
    },
    CodeDefinition {
        code: ErrorCode(5663003),
        name: "transactionExpectedNonceLifetime",
        template: "Transaction is not a durable nonce transaction",
    },
    CodeDefinition {
        code: ErrorCode(5663004),
        name: "transactionVersionNumberOutOfRange",
        template: "Transaction version must be in the range [0, 127]. $actualVersion given",
    },
    CodeDefinition {
        code: ErrorCode(5663005),
        name: "transactionFailedToDecompileAddressLookupTableContentsMissing",
        template: "Contents of these address lookup tables unknown: $lookupTableAddresses",
    },
    CodeDefinition {
        code: ErrorCode(5663006),
        name: "transactionFailedToDecompileAddressLookupTableIndexOutOfRange",
        template: "Lookup of address at index $highestRequestedIndex failed for lookup table $lookupTableAddress. Highest known index is $highestKnownIndex.",
    },
    CodeDefinition {
        code: ErrorCode(5663007),
        name: "transactionFailedToDecompileInstructionProgramAddressNotFound",
        template: "Could not find program address at index $index",
    },
    CodeDefinition {
        code: ErrorCode(5663008),
        name: "transactionFailedToDecompileFeePayerMissing",
        template: "No fee payer set in CompiledTransaction",
    },
    CodeDefinition {
        code: ErrorCode(5663009),
        name: "transactionSignaturesMissing",
        template: "Transaction is missing signatures for addresses: $addresses.",
    },
    CodeDefinition {
        code: ErrorCode(5663010),
        name: "transactionAddressMissing",
        template: "Transaction is missing an address at index: $index.",
    },
    CodeDefinition {
        code: ErrorCode(5663011),
        name: "transactionFeePayerMissing",
        template: "Transaction is missing a fee payer.",
    },
    CodeDefinition {
        code: ErrorCode(5663012),
        name: "transactionFeePayerSignatureMissing",
        template: "Could not determine this transaction's signature. Make sure that the transaction has been signed by its fee payer.",
    },
    CodeDefinition {
        code: ErrorCode(5663013),
        name: "transactionInvalidNonceTransactionInstructionsMissing",
        template: "Transaction with no instructions cannot be durable nonce transaction.",
    },
    CodeDefinition {
        code: ErrorCode(5663014),
        name: "transactionInvalidNonceTransactionFirstInstructionMustBeAdvanceNonce",
        template: "Transaction first instruction is not advance nonce account instruction.",
    },
    CodeDefinition {
        code: ErrorCode(5663015),
        name: "transactionAddressesCannotSignTransaction",
        template: "Attempted to sign a transaction with an address that is not a signer for it",
    },
    CodeDefinition {
        code: ErrorCode(5663016),
        name: "transactionCannotEncodeWithEmptySignatures",
        template: "Transaction has no expected signers therefore it cannot be encoded",
    },
    CodeDefinition {
        code: ErrorCode(5663017),
        name: "transactionMessageSignaturesMismatch",
        template: "The transaction message expected the transaction to have $numRequiredSignatures signatures, got $signaturesLength.",
    },
    CodeDefinition {
        code: ErrorCode(5663018),
        name: "transactionFailedToEstimateComputeLimit",
        template: "Failed to estimate the compute unit consumption for this transaction message.",
    },
    CodeDefinition {
        code: ErrorCode(5663019),
        name: "transactionFailedWhenSimulatingToEstimateComputeLimit",
        template: "Transaction failed when it was simulated in order to estimate the compute unit consumption.",
    },
    CodeDefinition {
        code: ErrorCode(5663020),
        name: "transactionExceedsSizeLimit",
        template: "Transaction size $transactionSize exceeds limit of $transactionSizeLimit bytes",
    },
    CodeDefinition {
        code: ErrorCode(5663021),
        name: "transactionVersionNumberNotSupported",
        template: "This version of Kit does not support decoding transactions with version $unsupportedVersion. The current max supported version is 1.",
    },
    CodeDefinition {
        code: ErrorCode(5663022),
        name: "transactionNonceAccountCannotBeInLookupTable",
        template: "The transaction has a durable nonce lifetime, but the nonce account address is in a lookup table.",
    },
    CodeDefinition {
        code: ErrorCode(5663023),
        name: "transactionMalformedMessageBytes",
        template: "Transaction message bytes are malformed: $messageBytes",
    },
    CodeDefinition {
        code: ErrorCode(5663024),
        name: "transactionCannotEncodeWithEmptyMessageBytes",
        template: "Transaction message bytes are empty, so the transaction cannot be encoded",
    },
    CodeDefinition {
        code: ErrorCode(5663025),
        name: "transactionCannotDecodeEmptyTransactionBytes",
        template: "Transaction bytes are empty, so no transaction can be decoded",
    },
];
