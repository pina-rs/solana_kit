//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `signerWalletMultisignUnimplemented` code.
    pub const SIGNER_WALLET_MULTISIGN_UNIMPLEMENTED: Self = Self(5508011);
    /// Pinned `signerWalletAccountCannotSignTransaction` code.
    pub const SIGNER_WALLET_ACCOUNT_CANNOT_SIGN_TRANSACTION: Self = Self(5508012);
    /// Pinned `offchainMessageMaximumLengthExceeded` code.
    pub const OFFCHAIN_MESSAGE_MAXIMUM_LENGTH_EXCEEDED: Self = Self(5607000);
    /// Pinned `offchainMessageRestrictedAsciiBodyCharacterOutOfRange` code.
    pub const OFFCHAIN_MESSAGE_RESTRICTED_ASCII_BODY_CHARACTER_OUT_OF_RANGE: Self = Self(5607001);
    /// Pinned `offchainMessageApplicationDomainStringLengthOutOfRange` code.
    pub const OFFCHAIN_MESSAGE_APPLICATION_DOMAIN_STRING_LENGTH_OUT_OF_RANGE: Self = Self(5607002);
    /// Pinned `offchainMessageInvalidApplicationDomainByteLength` code.
    pub const OFFCHAIN_MESSAGE_INVALID_APPLICATION_DOMAIN_BYTE_LENGTH: Self = Self(5607003);
    /// Pinned `offchainMessageNumSignaturesMismatch` code.
    pub const OFFCHAIN_MESSAGE_NUM_SIGNATURES_MISMATCH: Self = Self(5607004);
    /// Pinned `offchainMessageNumRequiredSignersCannotBeZero` code.
    pub const OFFCHAIN_MESSAGE_NUM_REQUIRED_SIGNERS_CANNOT_BE_ZERO: Self = Self(5607005);
    /// Pinned `offchainMessageVersionNumberNotSupported` code.
    pub const OFFCHAIN_MESSAGE_VERSION_NUMBER_NOT_SUPPORTED: Self = Self(5607006);
    /// Pinned `offchainMessageMessageFormatMismatch` code.
    pub const OFFCHAIN_MESSAGE_MESSAGE_FORMAT_MISMATCH: Self = Self(5607007);
    /// Pinned `offchainMessageMessageLengthMismatch` code.
    pub const OFFCHAIN_MESSAGE_MESSAGE_LENGTH_MISMATCH: Self = Self(5607008);
    /// Pinned `offchainMessageMessageMustBeNonEmpty` code.
    pub const OFFCHAIN_MESSAGE_MESSAGE_MUST_BE_NON_EMPTY: Self = Self(5607009);
    /// Pinned `offchainMessageNumEnvelopeSignaturesCannotBeZero` code.
    pub const OFFCHAIN_MESSAGE_NUM_ENVELOPE_SIGNATURES_CANNOT_BE_ZERO: Self = Self(5607010);
    /// Pinned `offchainMessageSignaturesMissing` code.
    pub const OFFCHAIN_MESSAGE_SIGNATURES_MISSING: Self = Self(5607011);
    /// Pinned `offchainMessageEnvelopeSignersMismatch` code.
    pub const OFFCHAIN_MESSAGE_ENVELOPE_SIGNERS_MISMATCH: Self = Self(5607012);
    /// Pinned `offchainMessageAddressesCannotSignOffchainMessage` code.
    pub const OFFCHAIN_MESSAGE_ADDRESSES_CANNOT_SIGN_OFFCHAIN_MESSAGE: Self = Self(5607013);
    /// Pinned `offchainMessageUnexpectedVersion` code.
    pub const OFFCHAIN_MESSAGE_UNEXPECTED_VERSION: Self = Self(5607014);
    /// Pinned `offchainMessageSignatoriesMustBeSorted` code.
    pub const OFFCHAIN_MESSAGE_SIGNATORIES_MUST_BE_SORTED: Self = Self(5607015);
    /// Pinned `offchainMessageSignatoriesMustBeUnique` code.
    pub const OFFCHAIN_MESSAGE_SIGNATORIES_MUST_BE_UNIQUE: Self = Self(5607016);
    /// Pinned `offchainMessageSignatureVerificationFailure` code.
    pub const OFFCHAIN_MESSAGE_SIGNATURE_VERIFICATION_FAILURE: Self = Self(5607017);
    /// Pinned `offchainMessageContentDoesNotMatchExpected` code.
    pub const OFFCHAIN_MESSAGE_CONTENT_DOES_NOT_MATCH_EXPECTED: Self = Self(5607018);
    /// Pinned `offchainMessageRequiredSignatoriesDoNotMatchExpected` code.
    pub const OFFCHAIN_MESSAGE_REQUIRED_SIGNATORIES_DO_NOT_MATCH_EXPECTED: Self = Self(5607019);
    /// Pinned `transactionInvokedProgramsCannotPayFees` code.
    pub const TRANSACTION_INVOKED_PROGRAMS_CANNOT_PAY_FEES: Self = Self(5663000);
    /// Pinned `transactionInvokedProgramsMustNotBeWritable` code.
    pub const TRANSACTION_INVOKED_PROGRAMS_MUST_NOT_BE_WRITABLE: Self = Self(5663001);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode(5508011),
        name: "signerWalletMultisignUnimplemented",
        template: "Wallet account signers do not support signing multiple messages/transactions in a single operation",
    },
    CodeDefinition {
        code: ErrorCode(5508012),
        name: "signerWalletAccountCannotSignTransaction",
        template: "The wallet account $address cannot be used to create a transaction signer because it does not implement either the `solana:signTransaction` or `solana:signAndSendTransaction` feature. At least one of these features is required. The account supports the following features: $supportedFeatures.",
    },
    CodeDefinition {
        code: ErrorCode(5607000),
        name: "offchainMessageMaximumLengthExceeded",
        template: "The message body provided has a byte-length of $actualBytes. The maximum allowable byte-length is $maxBytes",
    },
    CodeDefinition {
        code: ErrorCode(5607001),
        name: "offchainMessageRestrictedAsciiBodyCharacterOutOfRange",
        template: "The message body provided contains characters whose codes fall outside the allowed range.",
    },
    CodeDefinition {
        code: ErrorCode(5607002),
        name: "offchainMessageApplicationDomainStringLengthOutOfRange",
        template: "Expected base58-encoded application domain string of length in the range [32, 44]. Actual length: $actualLength.",
    },
    CodeDefinition {
        code: ErrorCode(5607003),
        name: "offchainMessageInvalidApplicationDomainByteLength",
        template: "Expected base58 encoded application domain to decode to a byte array of length 32. Actual length: $actualLength.",
    },
    CodeDefinition {
        code: ErrorCode(5607004),
        name: "offchainMessageNumSignaturesMismatch",
        template: "The offchain message preamble specifies $numRequiredSignatures required signature(s), got $signaturesLength.",
    },
    CodeDefinition {
        code: ErrorCode(5607005),
        name: "offchainMessageNumRequiredSignersCannotBeZero",
        template: "Offchain message must specify the address of at least one required signer",
    },
    CodeDefinition {
        code: ErrorCode(5607006),
        name: "offchainMessageVersionNumberNotSupported",
        template: "This version of Kit does not support decoding offchain messages with version $unsupportedVersion.",
    },
    CodeDefinition {
        code: ErrorCode(5607007),
        name: "offchainMessageMessageFormatMismatch",
        template: "Expected message format $expectedMessageFormat, got $actualMessageFormat",
    },
    CodeDefinition {
        code: ErrorCode(5607008),
        name: "offchainMessageMessageLengthMismatch",
        template: "The message length specified in the message preamble is $specifiedLength bytes. The actual length of the message is $actualLength bytes.",
    },
    CodeDefinition {
        code: ErrorCode(5607009),
        name: "offchainMessageMessageMustBeNonEmpty",
        template: "Offchain message content must be non-empty",
    },
    CodeDefinition {
        code: ErrorCode(5607010),
        name: "offchainMessageNumEnvelopeSignaturesCannotBeZero",
        template: "Offchain message envelope must reserve space for at least one signature",
    },
    CodeDefinition {
        code: ErrorCode(5607011),
        name: "offchainMessageSignaturesMissing",
        template: "Offchain message is missing signatures for addresses: $addresses.",
    },
    CodeDefinition {
        code: ErrorCode(5607012),
        name: "offchainMessageEnvelopeSignersMismatch",
        template: "The signer addresses in this offchain message envelope do not match the list of required signers in the message preamble.",
    },
    CodeDefinition {
        code: ErrorCode(5607013),
        name: "offchainMessageAddressesCannotSignOffchainMessage",
        template: "Attempted to sign an offchain message with an address that is not a signer for it",
    },
    CodeDefinition {
        code: ErrorCode(5607014),
        name: "offchainMessageUnexpectedVersion",
        template: "Expected offchain message version $expectedVersion. Got $actualVersion.",
    },
    CodeDefinition {
        code: ErrorCode(5607015),
        name: "offchainMessageSignatoriesMustBeSorted",
        template: "The signatories of this offchain message must be listed in lexicographical order",
    },
    CodeDefinition {
        code: ErrorCode(5607016),
        name: "offchainMessageSignatoriesMustBeUnique",
        template: "An address must be listed no more than once among the signatories of an offchain message",
    },
    CodeDefinition {
        code: ErrorCode(5607017),
        name: "offchainMessageSignatureVerificationFailure",
        template: "Offchain message signature verification failed.",
    },
    CodeDefinition {
        code: ErrorCode(5607018),
        name: "offchainMessageContentDoesNotMatchExpected",
        template: "The content of the offchain message does not match the content that was expected. Expected content with a byte-length of $expectedBytes; got content with a byte-length of $actualBytes. The signer may have signed different data than was requested; do not trust its signature.",
    },
    CodeDefinition {
        code: ErrorCode(5607019),
        name: "offchainMessageRequiredSignatoriesDoNotMatchExpected",
        template: "The offchain message lists different required signatories than was expected. Expected [$expectedAddresses]. Got [$actualAddresses]. The signer may have signed different data than was requested; do not trust its signature.",
    },
    CodeDefinition {
        code: ErrorCode(5663000),
        name: "transactionInvokedProgramsCannotPayFees",
        template: "This transaction includes an address ($programAddress) which is both invoked and set as the fee payer. Program addresses may not pay fees",
    },
    CodeDefinition {
        code: ErrorCode(5663001),
        name: "transactionInvokedProgramsMustNotBeWritable",
        template: "This transaction includes an address ($programAddress) which is both invoked and marked writable. Program addresses may not be writable",
    },
];
