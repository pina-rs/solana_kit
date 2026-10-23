//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `instructionErrorExecutableModified` code.
    pub const INSTRUCTION_ERROR_EXECUTABLE_MODIFIED: Self = Self(4615018);
    /// Pinned `instructionErrorRentEpochModified` code.
    pub const INSTRUCTION_ERROR_RENT_EPOCH_MODIFIED: Self = Self(4615019);
    /// Pinned `instructionErrorNotEnoughAccountKeys` code.
    pub const INSTRUCTION_ERROR_NOT_ENOUGH_ACCOUNT_KEYS: Self = Self(4615020);
    /// Pinned `instructionErrorAccountDataSizeChanged` code.
    pub const INSTRUCTION_ERROR_ACCOUNT_DATA_SIZE_CHANGED: Self = Self(4615021);
    /// Pinned `instructionErrorAccountNotExecutable` code.
    pub const INSTRUCTION_ERROR_ACCOUNT_NOT_EXECUTABLE: Self = Self(4615022);
    /// Pinned `instructionErrorAccountBorrowFailed` code.
    pub const INSTRUCTION_ERROR_ACCOUNT_BORROW_FAILED: Self = Self(4615023);
    /// Pinned `instructionErrorAccountBorrowOutstanding` code.
    pub const INSTRUCTION_ERROR_ACCOUNT_BORROW_OUTSTANDING: Self = Self(4615024);
    /// Pinned `instructionErrorDuplicateAccountOutOfSync` code.
    pub const INSTRUCTION_ERROR_DUPLICATE_ACCOUNT_OUT_OF_SYNC: Self = Self(4615025);
    /// Pinned `instructionErrorCustom` code.
    pub const INSTRUCTION_ERROR_CUSTOM: Self = Self(4615026);
    /// Pinned `instructionErrorInvalidError` code.
    pub const INSTRUCTION_ERROR_INVALID_ERROR: Self = Self(4615027);
    /// Pinned `instructionErrorExecutableDataModified` code.
    pub const INSTRUCTION_ERROR_EXECUTABLE_DATA_MODIFIED: Self = Self(4615028);
    /// Pinned `instructionErrorExecutableLamportChange` code.
    pub const INSTRUCTION_ERROR_EXECUTABLE_LAMPORT_CHANGE: Self = Self(4615029);
    /// Pinned `instructionErrorExecutableAccountNotRentExempt` code.
    pub const INSTRUCTION_ERROR_EXECUTABLE_ACCOUNT_NOT_RENT_EXEMPT: Self = Self(4615030);
    /// Pinned `instructionErrorUnsupportedProgramId` code.
    pub const INSTRUCTION_ERROR_UNSUPPORTED_PROGRAM_ID: Self = Self(4615031);
    /// Pinned `instructionErrorCallDepth` code.
    pub const INSTRUCTION_ERROR_CALL_DEPTH: Self = Self(4615032);
    /// Pinned `instructionErrorMissingAccount` code.
    pub const INSTRUCTION_ERROR_MISSING_ACCOUNT: Self = Self(4615033);
    /// Pinned `instructionErrorReentrancyNotAllowed` code.
    pub const INSTRUCTION_ERROR_REENTRANCY_NOT_ALLOWED: Self = Self(4615034);
    /// Pinned `instructionErrorMaxSeedLengthExceeded` code.
    pub const INSTRUCTION_ERROR_MAX_SEED_LENGTH_EXCEEDED: Self = Self(4615035);
    /// Pinned `instructionErrorInvalidSeeds` code.
    pub const INSTRUCTION_ERROR_INVALID_SEEDS: Self = Self(4615036);
    /// Pinned `instructionErrorInvalidRealloc` code.
    pub const INSTRUCTION_ERROR_INVALID_REALLOC: Self = Self(4615037);
    /// Pinned `instructionErrorComputationalBudgetExceeded` code.
    pub const INSTRUCTION_ERROR_COMPUTATIONAL_BUDGET_EXCEEDED: Self = Self(4615038);
    /// Pinned `instructionErrorPrivilegeEscalation` code.
    pub const INSTRUCTION_ERROR_PRIVILEGE_ESCALATION: Self = Self(4615039);
    /// Pinned `instructionErrorProgramEnvironmentSetupFailure` code.
    pub const INSTRUCTION_ERROR_PROGRAM_ENVIRONMENT_SETUP_FAILURE: Self = Self(4615040);
    /// Pinned `instructionErrorProgramFailedToComplete` code.
    pub const INSTRUCTION_ERROR_PROGRAM_FAILED_TO_COMPLETE: Self = Self(4615041);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode(4615018),
        name: "instructionErrorExecutableModified",
        template: "Instruction changed executable bit of an account",
    },
    CodeDefinition {
        code: ErrorCode(4615019),
        name: "instructionErrorRentEpochModified",
        template: "Instruction modified rent epoch of an account",
    },
    CodeDefinition {
        code: ErrorCode(4615020),
        name: "instructionErrorNotEnoughAccountKeys",
        template: "Insufficient account keys for instruction",
    },
    CodeDefinition {
        code: ErrorCode(4615021),
        name: "instructionErrorAccountDataSizeChanged",
        template: "Program other than the account's owner changed the size of the account data",
    },
    CodeDefinition {
        code: ErrorCode(4615022),
        name: "instructionErrorAccountNotExecutable",
        template: "Instruction expected an executable account",
    },
    CodeDefinition {
        code: ErrorCode(4615023),
        name: "instructionErrorAccountBorrowFailed",
        template: "Instruction tries to borrow reference for an account which is already borrowed",
    },
    CodeDefinition {
        code: ErrorCode(4615024),
        name: "instructionErrorAccountBorrowOutstanding",
        template: "Instruction left account with an outstanding borrowed reference",
    },
    CodeDefinition {
        code: ErrorCode(4615025),
        name: "instructionErrorDuplicateAccountOutOfSync",
        template: "Instruction modifications of multiply-passed account differ",
    },
    CodeDefinition {
        code: ErrorCode(4615026),
        name: "instructionErrorCustom",
        template: "Custom program error: #$code",
    },
    CodeDefinition {
        code: ErrorCode(4615027),
        name: "instructionErrorInvalidError",
        template: "Program returned invalid error code",
    },
    CodeDefinition {
        code: ErrorCode(4615028),
        name: "instructionErrorExecutableDataModified",
        template: "Instruction changed executable accounts data",
    },
    CodeDefinition {
        code: ErrorCode(4615029),
        name: "instructionErrorExecutableLamportChange",
        template: "Instruction changed the balance of an executable account",
    },
    CodeDefinition {
        code: ErrorCode(4615030),
        name: "instructionErrorExecutableAccountNotRentExempt",
        template: "Executable accounts must be rent exempt",
    },
    CodeDefinition {
        code: ErrorCode(4615031),
        name: "instructionErrorUnsupportedProgramId",
        template: "Unsupported program id",
    },
    CodeDefinition {
        code: ErrorCode(4615032),
        name: "instructionErrorCallDepth",
        template: "Cross-program invocation call depth too deep",
    },
    CodeDefinition {
        code: ErrorCode(4615033),
        name: "instructionErrorMissingAccount",
        template: "An account required by the instruction is missing",
    },
    CodeDefinition {
        code: ErrorCode(4615034),
        name: "instructionErrorReentrancyNotAllowed",
        template: "Cross-program invocation reentrancy not allowed for this instruction",
    },
    CodeDefinition {
        code: ErrorCode(4615035),
        name: "instructionErrorMaxSeedLengthExceeded",
        template: "Length of the seed is too long for address generation",
    },
    CodeDefinition {
        code: ErrorCode(4615036),
        name: "instructionErrorInvalidSeeds",
        template: "Provided seeds do not result in a valid address",
    },
    CodeDefinition {
        code: ErrorCode(4615037),
        name: "instructionErrorInvalidRealloc",
        template: "Failed to reallocate account data",
    },
    CodeDefinition {
        code: ErrorCode(4615038),
        name: "instructionErrorComputationalBudgetExceeded",
        template: "Computational budget exceeded",
    },
    CodeDefinition {
        code: ErrorCode(4615039),
        name: "instructionErrorPrivilegeEscalation",
        template: "Cross-program invocation with unauthorized signer or writable account",
    },
    CodeDefinition {
        code: ErrorCode(4615040),
        name: "instructionErrorProgramEnvironmentSetupFailure",
        template: "Failed to create program execution environment",
    },
    CodeDefinition {
        code: ErrorCode(4615041),
        name: "instructionErrorProgramFailedToComplete",
        template: "Program failed to complete",
    },
];
