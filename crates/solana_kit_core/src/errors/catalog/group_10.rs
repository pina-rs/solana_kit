//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `transactionErrorWouldExceedMaxVoteCostLimit` code.
    pub const TRANSACTION_ERROR_WOULD_EXCEED_MAX_VOTE_COST_LIMIT: Self = Self(7050028);
    /// Pinned `transactionErrorWouldExceedAccountDataTotalLimit` code.
    pub const TRANSACTION_ERROR_WOULD_EXCEED_ACCOUNT_DATA_TOTAL_LIMIT: Self = Self(7050029);
    /// Pinned `transactionErrorDuplicateInstruction` code.
    pub const TRANSACTION_ERROR_DUPLICATE_INSTRUCTION: Self = Self(7050030);
    /// Pinned `transactionErrorInsufficientFundsForRent` code.
    pub const TRANSACTION_ERROR_INSUFFICIENT_FUNDS_FOR_RENT: Self = Self(7050031);
    /// Pinned `transactionErrorMaxLoadedAccountsDataSizeExceeded` code.
    pub const TRANSACTION_ERROR_MAX_LOADED_ACCOUNTS_DATA_SIZE_EXCEEDED: Self = Self(7050032);
    /// Pinned `transactionErrorInvalidLoadedAccountsDataSizeLimit` code.
    pub const TRANSACTION_ERROR_INVALID_LOADED_ACCOUNTS_DATA_SIZE_LIMIT: Self = Self(7050033);
    /// Pinned `transactionErrorResanitizationNeeded` code.
    pub const TRANSACTION_ERROR_RESANITIZATION_NEEDED: Self = Self(7050034);
    /// Pinned `transactionErrorProgramExecutionTemporarilyRestricted` code.
    pub const TRANSACTION_ERROR_PROGRAM_EXECUTION_TEMPORARILY_RESTRICTED: Self = Self(7050035);
    /// Pinned `transactionErrorUnbalancedTransaction` code.
    pub const TRANSACTION_ERROR_UNBALANCED_TRANSACTION: Self = Self(7050036);
    /// Pinned `instructionPlansMessageCannotAccommodatePlan` code.
    pub const INSTRUCTION_PLANS_MESSAGE_CANNOT_ACCOMMODATE_PLAN: Self = Self(7618000);
    /// Pinned `instructionPlansMessagePackerAlreadyComplete` code.
    pub const INSTRUCTION_PLANS_MESSAGE_PACKER_ALREADY_COMPLETE: Self = Self(7618001);
    /// Pinned `instructionPlansEmptyInstructionPlan` code.
    pub const INSTRUCTION_PLANS_EMPTY_INSTRUCTION_PLAN: Self = Self(7618002);
    /// Pinned `instructionPlansFailedToExecuteTransactionPlan` code.
    pub const INSTRUCTION_PLANS_FAILED_TO_EXECUTE_TRANSACTION_PLAN: Self = Self(7618003);
    /// Pinned `instructionPlansNonDivisibleTransactionPlansNotSupported` code.
    pub const INSTRUCTION_PLANS_NON_DIVISIBLE_TRANSACTION_PLANS_NOT_SUPPORTED: Self = Self(7618004);
    /// Pinned `instructionPlansFailedSingleTransactionPlanResultNotFound` code.
    pub const INSTRUCTION_PLANS_FAILED_SINGLE_TRANSACTION_PLAN_RESULT_NOT_FOUND: Self =
        Self(7618005);
    /// Pinned `instructionPlansUnexpectedInstructionPlan` code.
    pub const INSTRUCTION_PLANS_UNEXPECTED_INSTRUCTION_PLAN: Self = Self(7618006);
    /// Pinned `instructionPlansUnexpectedTransactionPlan` code.
    pub const INSTRUCTION_PLANS_UNEXPECTED_TRANSACTION_PLAN: Self = Self(7618007);
    /// Pinned `instructionPlansUnexpectedTransactionPlanResult` code.
    pub const INSTRUCTION_PLANS_UNEXPECTED_TRANSACTION_PLAN_RESULT: Self = Self(7618008);
    /// Pinned `instructionPlansExpectedSuccessfulTransactionPlanResult` code.
    pub const INSTRUCTION_PLANS_EXPECTED_SUCCESSFUL_TRANSACTION_PLAN_RESULT: Self = Self(7618009);
    /// Pinned `instructionPlansMaxInstructionsPerTransactionExceeded` code.
    pub const INSTRUCTION_PLANS_MAX_INSTRUCTIONS_PER_TRANSACTION_EXCEEDED: Self = Self(7618010);
    /// Pinned `instructionPlansInvalidMaxInstructionsPerTransaction` code.
    pub const INSTRUCTION_PLANS_INVALID_MAX_INSTRUCTIONS_PER_TRANSACTION: Self = Self(7618011);
    /// Pinned `instructionPlansMessageRejectedByPacker` code.
    pub const INSTRUCTION_PLANS_MESSAGE_REJECTED_BY_PACKER: Self = Self(7618012);
    /// Pinned `codecsCannotDecodeEmptyByteArray` code.
    pub const CODECS_CANNOT_DECODE_EMPTY_BYTE_ARRAY: Self = Self(8078000);
    /// Pinned `codecsInvalidByteLength` code.
    pub const CODECS_INVALID_BYTE_LENGTH: Self = Self(8078001);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode(7050028),
        name: "transactionErrorWouldExceedMaxVoteCostLimit",
        template: "Transaction would exceed max Vote Cost Limit",
    },
    CodeDefinition {
        code: ErrorCode(7050029),
        name: "transactionErrorWouldExceedAccountDataTotalLimit",
        template: "Transaction would exceed total account data limit",
    },
    CodeDefinition {
        code: ErrorCode(7050030),
        name: "transactionErrorDuplicateInstruction",
        template: "Transaction contains a duplicate instruction ($index) that is not allowed",
    },
    CodeDefinition {
        code: ErrorCode(7050031),
        name: "transactionErrorInsufficientFundsForRent",
        template: "Transaction results in an account ($accountIndex) with insufficient funds for rent",
    },
    CodeDefinition {
        code: ErrorCode(7050032),
        name: "transactionErrorMaxLoadedAccountsDataSizeExceeded",
        template: "Transaction exceeded max loaded accounts data size cap",
    },
    CodeDefinition {
        code: ErrorCode(7050033),
        name: "transactionErrorInvalidLoadedAccountsDataSizeLimit",
        template: "LoadedAccountsDataSizeLimit set for transaction must be greater than 0.",
    },
    CodeDefinition {
        code: ErrorCode(7050034),
        name: "transactionErrorResanitizationNeeded",
        template: "ResanitizationNeeded",
    },
    CodeDefinition {
        code: ErrorCode(7050035),
        name: "transactionErrorProgramExecutionTemporarilyRestricted",
        template: "Execution of the program referenced by account at index $accountIndex is temporarily restricted.",
    },
    CodeDefinition {
        code: ErrorCode(7050036),
        name: "transactionErrorUnbalancedTransaction",
        template: "Sum of account balances before and after transaction do not match",
    },
    CodeDefinition {
        code: ErrorCode(7618000),
        name: "instructionPlansMessageCannotAccommodatePlan",
        template: "The provided message has insufficient capacity to accommodate the next instruction(s) in this plan. Expected at least $numBytesRequired free byte(s), got $numFreeBytes byte(s).",
    },
    CodeDefinition {
        code: ErrorCode(7618001),
        name: "instructionPlansMessagePackerAlreadyComplete",
        template: "No more instructions to pack; the message packer has completed the instruction plan.",
    },
    CodeDefinition {
        code: ErrorCode(7618002),
        name: "instructionPlansEmptyInstructionPlan",
        template: "The provided instruction plan is empty.",
    },
    CodeDefinition {
        code: ErrorCode(7618003),
        name: "instructionPlansFailedToExecuteTransactionPlan",
        template: "The provided transaction plan failed to execute. See the transactionPlanResult attribute for more details.",
    },
    CodeDefinition {
        code: ErrorCode(7618004),
        name: "instructionPlansNonDivisibleTransactionPlansNotSupported",
        template: "This transaction plan executor does not support non-divisible sequential plans.",
    },
    CodeDefinition {
        code: ErrorCode(7618005),
        name: "instructionPlansFailedSingleTransactionPlanResultNotFound",
        template: "No failed transaction plan result was found in the provided transaction plan result.",
    },
    CodeDefinition {
        code: ErrorCode(7618006),
        name: "instructionPlansUnexpectedInstructionPlan",
        template: "Unexpected instruction plan. Expected $expectedKind plan, got $actualKind plan.",
    },
    CodeDefinition {
        code: ErrorCode(7618007),
        name: "instructionPlansUnexpectedTransactionPlan",
        template: "Unexpected transaction plan. Expected $expectedKind plan, got $actualKind plan.",
    },
    CodeDefinition {
        code: ErrorCode(7618008),
        name: "instructionPlansUnexpectedTransactionPlanResult",
        template: "Unexpected transaction plan result. Expected $expectedKind plan, got $actualKind plan.",
    },
    CodeDefinition {
        code: ErrorCode(7618009),
        name: "instructionPlansExpectedSuccessfulTransactionPlanResult",
        template: "Expected a successful transaction plan result. I.e. there is at least one failed or cancelled transaction in the plan.",
    },
    CodeDefinition {
        code: ErrorCode(7618010),
        name: "instructionPlansMaxInstructionsPerTransactionExceeded",
        template: "Planning this transaction message would require $numInstructions instructions, which exceeds the configured maximum of $maxInstructions instructions per transaction. This limit is configurable, and intended to leave headroom for inner instructions which are included in the maximum instruction limit for transactions. Increase `maxInstructionsPerTransaction` on the transaction planner (or `maxInstructions` on the message packer) to allow more instructions per transaction.",
    },
    CodeDefinition {
        code: ErrorCode(7618011),
        name: "instructionPlansInvalidMaxInstructionsPerTransaction",
        template: "The configured maximum of $maxInstructions instructions per transaction is invalid. It must be a positive integer no greater than the transaction format limit of $transactionInstructionLimit instructions per transaction. Provide a `maxInstructionsPerTransaction` (on the transaction planner) or `maxInstructions` (on the message packer) value between 1 and $transactionInstructionLimit.",
    },
    CodeDefinition {
        code: ErrorCode(7618012),
        name: "instructionPlansMessageRejectedByPacker",
        template: "The message packer rejected the provided transaction message: $reason.",
    },
    CodeDefinition {
        code: ErrorCode(8078000),
        name: "codecsCannotDecodeEmptyByteArray",
        template: "Codec [$codecDescription] cannot decode empty byte arrays.",
    },
    CodeDefinition {
        code: ErrorCode(8078001),
        name: "codecsInvalidByteLength",
        template: "Codec [$codecDescription] expected $expected bytes, got $bytesLength.",
    },
];
