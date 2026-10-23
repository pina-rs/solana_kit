//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `keysWriteKeyPairUnsupportedEnvironment` code.
    pub const KEYS_WRITE_KEY_PAIR_UNSUPPORTED_ENVIRONMENT: Self = Self(3704006);
    /// Pinned `keysVerificationRequiresExtractablePublicKey` code.
    pub const KEYS_VERIFICATION_REQUIRES_EXTRACTABLE_PUBLIC_KEY: Self = Self(3704007);
    /// Pinned `fsUnsupportedEnvironment` code.
    pub const FS_UNSUPPORTED_ENVIRONMENT: Self = Self(3712000);
    /// Pinned `instructionExpectedToHaveAccounts` code.
    pub const INSTRUCTION_EXPECTED_TO_HAVE_ACCOUNTS: Self = Self(4128000);
    /// Pinned `instructionExpectedToHaveData` code.
    pub const INSTRUCTION_EXPECTED_TO_HAVE_DATA: Self = Self(4128001);
    /// Pinned `instructionProgramIdMismatch` code.
    pub const INSTRUCTION_PROGRAM_ID_MISMATCH: Self = Self(4128002);
    /// Pinned `instructionErrorUnknown` code.
    pub const INSTRUCTION_ERROR_UNKNOWN: Self = Self(4615000);
    /// Pinned `instructionErrorGenericError` code.
    pub const INSTRUCTION_ERROR_GENERIC_ERROR: Self = Self(4615001);
    /// Pinned `instructionErrorInvalidArgument` code.
    pub const INSTRUCTION_ERROR_INVALID_ARGUMENT: Self = Self(4615002);
    /// Pinned `instructionErrorInvalidInstructionData` code.
    pub const INSTRUCTION_ERROR_INVALID_INSTRUCTION_DATA: Self = Self(4615003);
    /// Pinned `instructionErrorInvalidAccountData` code.
    pub const INSTRUCTION_ERROR_INVALID_ACCOUNT_DATA: Self = Self(4615004);
    /// Pinned `instructionErrorAccountDataTooSmall` code.
    pub const INSTRUCTION_ERROR_ACCOUNT_DATA_TOO_SMALL: Self = Self(4615005);
    /// Pinned `instructionErrorInsufficientFunds` code.
    pub const INSTRUCTION_ERROR_INSUFFICIENT_FUNDS: Self = Self(4615006);
    /// Pinned `instructionErrorIncorrectProgramId` code.
    pub const INSTRUCTION_ERROR_INCORRECT_PROGRAM_ID: Self = Self(4615007);
    /// Pinned `instructionErrorMissingRequiredSignature` code.
    pub const INSTRUCTION_ERROR_MISSING_REQUIRED_SIGNATURE: Self = Self(4615008);
    /// Pinned `instructionErrorAccountAlreadyInitialized` code.
    pub const INSTRUCTION_ERROR_ACCOUNT_ALREADY_INITIALIZED: Self = Self(4615009);
    /// Pinned `instructionErrorUninitializedAccount` code.
    pub const INSTRUCTION_ERROR_UNINITIALIZED_ACCOUNT: Self = Self(4615010);
    /// Pinned `instructionErrorUnbalancedInstruction` code.
    pub const INSTRUCTION_ERROR_UNBALANCED_INSTRUCTION: Self = Self(4615011);
    /// Pinned `instructionErrorModifiedProgramId` code.
    pub const INSTRUCTION_ERROR_MODIFIED_PROGRAM_ID: Self = Self(4615012);
    /// Pinned `instructionErrorExternalAccountLamportSpend` code.
    pub const INSTRUCTION_ERROR_EXTERNAL_ACCOUNT_LAMPORT_SPEND: Self = Self(4615013);
    /// Pinned `instructionErrorExternalAccountDataModified` code.
    pub const INSTRUCTION_ERROR_EXTERNAL_ACCOUNT_DATA_MODIFIED: Self = Self(4615014);
    /// Pinned `instructionErrorReadonlyLamportChange` code.
    pub const INSTRUCTION_ERROR_READONLY_LAMPORT_CHANGE: Self = Self(4615015);
    /// Pinned `instructionErrorReadonlyDataModified` code.
    pub const INSTRUCTION_ERROR_READONLY_DATA_MODIFIED: Self = Self(4615016);
    /// Pinned `instructionErrorDuplicateAccountIndex` code.
    pub const INSTRUCTION_ERROR_DUPLICATE_ACCOUNT_INDEX: Self = Self(4615017);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode(3704006),
        name: "keysWriteKeyPairUnsupportedEnvironment",
        template: "Writing a key pair to disk is not supported in this environment.",
    },
    CodeDefinition {
        code: ErrorCode(3704007),
        name: "keysVerificationRequiresExtractablePublicKey",
        template: "Signatures can only be verified using an extractable public key. The bytes of the public key must be readable. If you imported the public key yourself, set `extractable` to `true`.",
    },
    CodeDefinition {
        code: ErrorCode(3712000),
        name: "fsUnsupportedEnvironment",
        template: "Filesystem operation `$operation` is not supported in this environment.",
    },
    CodeDefinition {
        code: ErrorCode(4128000),
        name: "instructionExpectedToHaveAccounts",
        template: "The instruction does not have any accounts.",
    },
    CodeDefinition {
        code: ErrorCode(4128001),
        name: "instructionExpectedToHaveData",
        template: "The instruction does not have any data.",
    },
    CodeDefinition {
        code: ErrorCode(4128002),
        name: "instructionProgramIdMismatch",
        template: "Expected instruction to have program address $expectedProgramAddress, got $actualProgramAddress.",
    },
    CodeDefinition {
        code: ErrorCode(4615000),
        name: "instructionErrorUnknown",
        template: "The instruction failed with the error: $errorName",
    },
    CodeDefinition {
        code: ErrorCode(4615001),
        name: "instructionErrorGenericError",
        template: "Generic instruction error",
    },
    CodeDefinition {
        code: ErrorCode(4615002),
        name: "instructionErrorInvalidArgument",
        template: "Invalid program argument",
    },
    CodeDefinition {
        code: ErrorCode(4615003),
        name: "instructionErrorInvalidInstructionData",
        template: "Invalid instruction data",
    },
    CodeDefinition {
        code: ErrorCode(4615004),
        name: "instructionErrorInvalidAccountData",
        template: "Invalid account data for instruction",
    },
    CodeDefinition {
        code: ErrorCode(4615005),
        name: "instructionErrorAccountDataTooSmall",
        template: "Account data too small for instruction",
    },
    CodeDefinition {
        code: ErrorCode(4615006),
        name: "instructionErrorInsufficientFunds",
        template: "Insufficient funds for instruction",
    },
    CodeDefinition {
        code: ErrorCode(4615007),
        name: "instructionErrorIncorrectProgramId",
        template: "Incorrect program id for instruction",
    },
    CodeDefinition {
        code: ErrorCode(4615008),
        name: "instructionErrorMissingRequiredSignature",
        template: "Missing required signature for instruction",
    },
    CodeDefinition {
        code: ErrorCode(4615009),
        name: "instructionErrorAccountAlreadyInitialized",
        template: "Instruction requires an uninitialized account",
    },
    CodeDefinition {
        code: ErrorCode(4615010),
        name: "instructionErrorUninitializedAccount",
        template: "Instruction requires an initialized account",
    },
    CodeDefinition {
        code: ErrorCode(4615011),
        name: "instructionErrorUnbalancedInstruction",
        template: "Sum of account balances before and after instruction do not match",
    },
    CodeDefinition {
        code: ErrorCode(4615012),
        name: "instructionErrorModifiedProgramId",
        template: "Instruction illegally modified the program id of an account",
    },
    CodeDefinition {
        code: ErrorCode(4615013),
        name: "instructionErrorExternalAccountLamportSpend",
        template: "Instruction spent from the balance of an account it does not own",
    },
    CodeDefinition {
        code: ErrorCode(4615014),
        name: "instructionErrorExternalAccountDataModified",
        template: "Instruction modified data of an account it does not own",
    },
    CodeDefinition {
        code: ErrorCode(4615015),
        name: "instructionErrorReadonlyLamportChange",
        template: "Instruction changed the balance of a read-only account",
    },
    CodeDefinition {
        code: ErrorCode(4615016),
        name: "instructionErrorReadonlyDataModified",
        template: "Instruction modified data of a read-only account",
    },
    CodeDefinition {
        code: ErrorCode(4615017),
        name: "instructionErrorDuplicateAccountIndex",
        template: "Instruction contains duplicate accounts",
    },
];
