//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `instructionErrorProgramFailedToCompile` code.
    pub const INSTRUCTION_ERROR_PROGRAM_FAILED_TO_COMPILE: Self = Self(4615042);
    /// Pinned `instructionErrorImmutable` code.
    pub const INSTRUCTION_ERROR_IMMUTABLE: Self = Self(4615043);
    /// Pinned `instructionErrorIncorrectAuthority` code.
    pub const INSTRUCTION_ERROR_INCORRECT_AUTHORITY: Self = Self(4615044);
    /// Pinned `instructionErrorBorshIoError` code.
    pub const INSTRUCTION_ERROR_BORSH_IO_ERROR: Self = Self(4615045);
    /// Pinned `instructionErrorAccountNotRentExempt` code.
    pub const INSTRUCTION_ERROR_ACCOUNT_NOT_RENT_EXEMPT: Self = Self(4615046);
    /// Pinned `instructionErrorInvalidAccountOwner` code.
    pub const INSTRUCTION_ERROR_INVALID_ACCOUNT_OWNER: Self = Self(4615047);
    /// Pinned `instructionErrorArithmeticOverflow` code.
    pub const INSTRUCTION_ERROR_ARITHMETIC_OVERFLOW: Self = Self(4615048);
    /// Pinned `instructionErrorUnsupportedSysvar` code.
    pub const INSTRUCTION_ERROR_UNSUPPORTED_SYSVAR: Self = Self(4615049);
    /// Pinned `instructionErrorIllegalOwner` code.
    pub const INSTRUCTION_ERROR_ILLEGAL_OWNER: Self = Self(4615050);
    /// Pinned `instructionErrorMaxAccountsDataAllocationsExceeded` code.
    pub const INSTRUCTION_ERROR_MAX_ACCOUNTS_DATA_ALLOCATIONS_EXCEEDED: Self = Self(4615051);
    /// Pinned `instructionErrorMaxAccountsExceeded` code.
    pub const INSTRUCTION_ERROR_MAX_ACCOUNTS_EXCEEDED: Self = Self(4615052);
    /// Pinned `instructionErrorMaxInstructionTraceLengthExceeded` code.
    pub const INSTRUCTION_ERROR_MAX_INSTRUCTION_TRACE_LENGTH_EXCEEDED: Self = Self(4615053);
    /// Pinned `instructionErrorBuiltinProgramsMustConsumeComputeUnits` code.
    pub const INSTRUCTION_ERROR_BUILTIN_PROGRAMS_MUST_CONSUME_COMPUTE_UNITS: Self = Self(4615054);
    /// Pinned `signerAddressCannotHaveMultipleSigners` code.
    pub const SIGNER_ADDRESS_CANNOT_HAVE_MULTIPLE_SIGNERS: Self = Self(5508000);
    /// Pinned `signerExpectedKeyPairSigner` code.
    pub const SIGNER_EXPECTED_KEY_PAIR_SIGNER: Self = Self(5508001);
    /// Pinned `signerExpectedMessageSigner` code.
    pub const SIGNER_EXPECTED_MESSAGE_SIGNER: Self = Self(5508002);
    /// Pinned `signerExpectedMessageModifyingSigner` code.
    pub const SIGNER_EXPECTED_MESSAGE_MODIFYING_SIGNER: Self = Self(5508003);
    /// Pinned `signerExpectedMessagePartialSigner` code.
    pub const SIGNER_EXPECTED_MESSAGE_PARTIAL_SIGNER: Self = Self(5508004);
    /// Pinned `signerExpectedTransactionSigner` code.
    pub const SIGNER_EXPECTED_TRANSACTION_SIGNER: Self = Self(5508005);
    /// Pinned `signerExpectedTransactionModifyingSigner` code.
    pub const SIGNER_EXPECTED_TRANSACTION_MODIFYING_SIGNER: Self = Self(5508006);
    /// Pinned `signerExpectedTransactionPartialSigner` code.
    pub const SIGNER_EXPECTED_TRANSACTION_PARTIAL_SIGNER: Self = Self(5508007);
    /// Pinned `signerExpectedTransactionSendingSigner` code.
    pub const SIGNER_EXPECTED_TRANSACTION_SENDING_SIGNER: Self = Self(5508008);
    /// Pinned `signerTransactionCannotHaveMultipleSendingSigners` code.
    pub const SIGNER_TRANSACTION_CANNOT_HAVE_MULTIPLE_SENDING_SIGNERS: Self = Self(5508009);
    /// Pinned `signerTransactionSendingSignerMissing` code.
    pub const SIGNER_TRANSACTION_SENDING_SIGNER_MISSING: Self = Self(5508010);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode::INSTRUCTION_ERROR_PROGRAM_FAILED_TO_COMPILE,
        name: "instructionErrorProgramFailedToCompile",
        template: "Program failed to compile",
    },
    CodeDefinition {
        code: ErrorCode::INSTRUCTION_ERROR_IMMUTABLE,
        name: "instructionErrorImmutable",
        template: "Account is immutable",
    },
    CodeDefinition {
        code: ErrorCode::INSTRUCTION_ERROR_INCORRECT_AUTHORITY,
        name: "instructionErrorIncorrectAuthority",
        template: "Incorrect authority provided",
    },
    CodeDefinition {
        code: ErrorCode::INSTRUCTION_ERROR_BORSH_IO_ERROR,
        name: "instructionErrorBorshIoError",
        template: "Failed to serialize or deserialize account data",
    },
    CodeDefinition {
        code: ErrorCode::INSTRUCTION_ERROR_ACCOUNT_NOT_RENT_EXEMPT,
        name: "instructionErrorAccountNotRentExempt",
        template: "An account does not have enough lamports to be rent-exempt",
    },
    CodeDefinition {
        code: ErrorCode::INSTRUCTION_ERROR_INVALID_ACCOUNT_OWNER,
        name: "instructionErrorInvalidAccountOwner",
        template: "Invalid account owner",
    },
    CodeDefinition {
        code: ErrorCode::INSTRUCTION_ERROR_ARITHMETIC_OVERFLOW,
        name: "instructionErrorArithmeticOverflow",
        template: "Program arithmetic overflowed",
    },
    CodeDefinition {
        code: ErrorCode::INSTRUCTION_ERROR_UNSUPPORTED_SYSVAR,
        name: "instructionErrorUnsupportedSysvar",
        template: "Unsupported sysvar",
    },
    CodeDefinition {
        code: ErrorCode::INSTRUCTION_ERROR_ILLEGAL_OWNER,
        name: "instructionErrorIllegalOwner",
        template: "Provided owner is not allowed",
    },
    CodeDefinition {
        code: ErrorCode::INSTRUCTION_ERROR_MAX_ACCOUNTS_DATA_ALLOCATIONS_EXCEEDED,
        name: "instructionErrorMaxAccountsDataAllocationsExceeded",
        template: "Accounts data allocations exceeded the maximum allowed per transaction",
    },
    CodeDefinition {
        code: ErrorCode::INSTRUCTION_ERROR_MAX_ACCOUNTS_EXCEEDED,
        name: "instructionErrorMaxAccountsExceeded",
        template: "Max accounts exceeded",
    },
    CodeDefinition {
        code: ErrorCode::INSTRUCTION_ERROR_MAX_INSTRUCTION_TRACE_LENGTH_EXCEEDED,
        name: "instructionErrorMaxInstructionTraceLengthExceeded",
        template: "Max instruction trace length exceeded",
    },
    CodeDefinition {
        code: ErrorCode::INSTRUCTION_ERROR_BUILTIN_PROGRAMS_MUST_CONSUME_COMPUTE_UNITS,
        name: "instructionErrorBuiltinProgramsMustConsumeComputeUnits",
        template: "Builtin programs must consume compute units",
    },
    CodeDefinition {
        code: ErrorCode::SIGNER_ADDRESS_CANNOT_HAVE_MULTIPLE_SIGNERS,
        name: "signerAddressCannotHaveMultipleSigners",
        template: "Multiple distinct signers were identified for address $address. Please ensure that you are using the same signer instance for each address.",
    },
    CodeDefinition {
        code: ErrorCode::SIGNER_EXPECTED_KEY_PAIR_SIGNER,
        name: "signerExpectedKeyPairSigner",
        template: "The provided value does not implement the KeyPairSigner interface",
    },
    CodeDefinition {
        code: ErrorCode::SIGNER_EXPECTED_MESSAGE_SIGNER,
        name: "signerExpectedMessageSigner",
        template: "The provided value does not implement any of the MessageSigner interfaces",
    },
    CodeDefinition {
        code: ErrorCode::SIGNER_EXPECTED_MESSAGE_MODIFYING_SIGNER,
        name: "signerExpectedMessageModifyingSigner",
        template: "The provided value does not implement the MessageModifyingSigner interface",
    },
    CodeDefinition {
        code: ErrorCode::SIGNER_EXPECTED_MESSAGE_PARTIAL_SIGNER,
        name: "signerExpectedMessagePartialSigner",
        template: "The provided value does not implement the MessagePartialSigner interface",
    },
    CodeDefinition {
        code: ErrorCode::SIGNER_EXPECTED_TRANSACTION_SIGNER,
        name: "signerExpectedTransactionSigner",
        template: "The provided value does not implement any of the TransactionSigner interfaces",
    },
    CodeDefinition {
        code: ErrorCode::SIGNER_EXPECTED_TRANSACTION_MODIFYING_SIGNER,
        name: "signerExpectedTransactionModifyingSigner",
        template: "The provided value does not implement the TransactionModifyingSigner interface",
    },
    CodeDefinition {
        code: ErrorCode::SIGNER_EXPECTED_TRANSACTION_PARTIAL_SIGNER,
        name: "signerExpectedTransactionPartialSigner",
        template: "The provided value does not implement the TransactionPartialSigner interface",
    },
    CodeDefinition {
        code: ErrorCode::SIGNER_EXPECTED_TRANSACTION_SENDING_SIGNER,
        name: "signerExpectedTransactionSendingSigner",
        template: "The provided value does not implement the TransactionSendingSigner interface",
    },
    CodeDefinition {
        code: ErrorCode::SIGNER_TRANSACTION_CANNOT_HAVE_MULTIPLE_SENDING_SIGNERS,
        name: "signerTransactionCannotHaveMultipleSendingSigners",
        template: "More than one TransactionSendingSigner was identified.",
    },
    CodeDefinition {
        code: ErrorCode::SIGNER_TRANSACTION_SENDING_SIGNER_MISSING,
        name: "signerTransactionSendingSignerMissing",
        template: "No TransactionSendingSigner was identified.",
    },
];
