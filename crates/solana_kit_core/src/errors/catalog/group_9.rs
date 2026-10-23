//! Generated from the pinned Dart error oracle.
use super::{CodeDefinition, ErrorCode};
impl ErrorCode {
    /// Pinned `transactionErrorProgramAccountNotFound` code.
    pub const TRANSACTION_ERROR_PROGRAM_ACCOUNT_NOT_FOUND: Self = Self(7050004);
    /// Pinned `transactionErrorInsufficientFundsForFee` code.
    pub const TRANSACTION_ERROR_INSUFFICIENT_FUNDS_FOR_FEE: Self = Self(7050005);
    /// Pinned `transactionErrorInvalidAccountForFee` code.
    pub const TRANSACTION_ERROR_INVALID_ACCOUNT_FOR_FEE: Self = Self(7050006);
    /// Pinned `transactionErrorAlreadyProcessed` code.
    pub const TRANSACTION_ERROR_ALREADY_PROCESSED: Self = Self(7050007);
    /// Pinned `transactionErrorBlockhashNotFound` code.
    pub const TRANSACTION_ERROR_BLOCKHASH_NOT_FOUND: Self = Self(7050008);
    /// Pinned `transactionErrorCallChainTooDeep` code.
    pub const TRANSACTION_ERROR_CALL_CHAIN_TOO_DEEP: Self = Self(7050009);
    /// Pinned `transactionErrorMissingSignatureForFee` code.
    pub const TRANSACTION_ERROR_MISSING_SIGNATURE_FOR_FEE: Self = Self(7050010);
    /// Pinned `transactionErrorInvalidAccountIndex` code.
    pub const TRANSACTION_ERROR_INVALID_ACCOUNT_INDEX: Self = Self(7050011);
    /// Pinned `transactionErrorSignatureFailure` code.
    pub const TRANSACTION_ERROR_SIGNATURE_FAILURE: Self = Self(7050012);
    /// Pinned `transactionErrorInvalidProgramForExecution` code.
    pub const TRANSACTION_ERROR_INVALID_PROGRAM_FOR_EXECUTION: Self = Self(7050013);
    /// Pinned `transactionErrorSanitizeFailure` code.
    pub const TRANSACTION_ERROR_SANITIZE_FAILURE: Self = Self(7050014);
    /// Pinned `transactionErrorClusterMaintenance` code.
    pub const TRANSACTION_ERROR_CLUSTER_MAINTENANCE: Self = Self(7050015);
    /// Pinned `transactionErrorAccountBorrowOutstanding` code.
    pub const TRANSACTION_ERROR_ACCOUNT_BORROW_OUTSTANDING: Self = Self(7050016);
    /// Pinned `transactionErrorWouldExceedMaxBlockCostLimit` code.
    pub const TRANSACTION_ERROR_WOULD_EXCEED_MAX_BLOCK_COST_LIMIT: Self = Self(7050017);
    /// Pinned `transactionErrorUnsupportedVersion` code.
    pub const TRANSACTION_ERROR_UNSUPPORTED_VERSION: Self = Self(7050018);
    /// Pinned `transactionErrorInvalidWritableAccount` code.
    pub const TRANSACTION_ERROR_INVALID_WRITABLE_ACCOUNT: Self = Self(7050019);
    /// Pinned `transactionErrorWouldExceedMaxAccountCostLimit` code.
    pub const TRANSACTION_ERROR_WOULD_EXCEED_MAX_ACCOUNT_COST_LIMIT: Self = Self(7050020);
    /// Pinned `transactionErrorWouldExceedAccountDataBlockLimit` code.
    pub const TRANSACTION_ERROR_WOULD_EXCEED_ACCOUNT_DATA_BLOCK_LIMIT: Self = Self(7050021);
    /// Pinned `transactionErrorTooManyAccountLocks` code.
    pub const TRANSACTION_ERROR_TOO_MANY_ACCOUNT_LOCKS: Self = Self(7050022);
    /// Pinned `transactionErrorAddressLookupTableNotFound` code.
    pub const TRANSACTION_ERROR_ADDRESS_LOOKUP_TABLE_NOT_FOUND: Self = Self(7050023);
    /// Pinned `transactionErrorInvalidAddressLookupTableOwner` code.
    pub const TRANSACTION_ERROR_INVALID_ADDRESS_LOOKUP_TABLE_OWNER: Self = Self(7050024);
    /// Pinned `transactionErrorInvalidAddressLookupTableData` code.
    pub const TRANSACTION_ERROR_INVALID_ADDRESS_LOOKUP_TABLE_DATA: Self = Self(7050025);
    /// Pinned `transactionErrorInvalidAddressLookupTableIndex` code.
    pub const TRANSACTION_ERROR_INVALID_ADDRESS_LOOKUP_TABLE_INDEX: Self = Self(7050026);
    /// Pinned `transactionErrorInvalidRentPayingAccount` code.
    pub const TRANSACTION_ERROR_INVALID_RENT_PAYING_ACCOUNT: Self = Self(7050027);
}
pub(super) const DEFINITIONS: &[CodeDefinition] = &[
    CodeDefinition {
        code: ErrorCode(7050004),
        name: "transactionErrorProgramAccountNotFound",
        template: "Attempt to load a program that does not exist",
    },
    CodeDefinition {
        code: ErrorCode(7050005),
        name: "transactionErrorInsufficientFundsForFee",
        template: "Insufficient funds for fee",
    },
    CodeDefinition {
        code: ErrorCode(7050006),
        name: "transactionErrorInvalidAccountForFee",
        template: "This account may not be used to pay transaction fees",
    },
    CodeDefinition {
        code: ErrorCode(7050007),
        name: "transactionErrorAlreadyProcessed",
        template: "This transaction has already been processed",
    },
    CodeDefinition {
        code: ErrorCode(7050008),
        name: "transactionErrorBlockhashNotFound",
        template: "Blockhash not found",
    },
    CodeDefinition {
        code: ErrorCode(7050009),
        name: "transactionErrorCallChainTooDeep",
        template: "Loader call chain is too deep",
    },
    CodeDefinition {
        code: ErrorCode(7050010),
        name: "transactionErrorMissingSignatureForFee",
        template: "Transaction requires a fee but has no signature present",
    },
    CodeDefinition {
        code: ErrorCode(7050011),
        name: "transactionErrorInvalidAccountIndex",
        template: "Transaction contains an invalid account reference",
    },
    CodeDefinition {
        code: ErrorCode(7050012),
        name: "transactionErrorSignatureFailure",
        template: "Transaction did not pass signature verification",
    },
    CodeDefinition {
        code: ErrorCode(7050013),
        name: "transactionErrorInvalidProgramForExecution",
        template: "This program may not be used for executing instructions",
    },
    CodeDefinition {
        code: ErrorCode(7050014),
        name: "transactionErrorSanitizeFailure",
        template: "Transaction failed to sanitize accounts offsets correctly",
    },
    CodeDefinition {
        code: ErrorCode(7050015),
        name: "transactionErrorClusterMaintenance",
        template: "Transactions are currently disabled due to cluster maintenance",
    },
    CodeDefinition {
        code: ErrorCode(7050016),
        name: "transactionErrorAccountBorrowOutstanding",
        template: "Transaction processing left an account with an outstanding borrowed reference",
    },
    CodeDefinition {
        code: ErrorCode(7050017),
        name: "transactionErrorWouldExceedMaxBlockCostLimit",
        template: "Transaction would exceed max Block Cost Limit",
    },
    CodeDefinition {
        code: ErrorCode(7050018),
        name: "transactionErrorUnsupportedVersion",
        template: "Transaction version is unsupported",
    },
    CodeDefinition {
        code: ErrorCode(7050019),
        name: "transactionErrorInvalidWritableAccount",
        template: "Transaction loads a writable account that cannot be written",
    },
    CodeDefinition {
        code: ErrorCode(7050020),
        name: "transactionErrorWouldExceedMaxAccountCostLimit",
        template: "Transaction would exceed max account limit within the block",
    },
    CodeDefinition {
        code: ErrorCode(7050021),
        name: "transactionErrorWouldExceedAccountDataBlockLimit",
        template: "Transaction would exceed account data limit within the block",
    },
    CodeDefinition {
        code: ErrorCode(7050022),
        name: "transactionErrorTooManyAccountLocks",
        template: "Transaction locked too many accounts",
    },
    CodeDefinition {
        code: ErrorCode(7050023),
        name: "transactionErrorAddressLookupTableNotFound",
        template: "Transaction loads an address table account that doesn't exist",
    },
    CodeDefinition {
        code: ErrorCode(7050024),
        name: "transactionErrorInvalidAddressLookupTableOwner",
        template: "Transaction loads an address table account with an invalid owner",
    },
    CodeDefinition {
        code: ErrorCode(7050025),
        name: "transactionErrorInvalidAddressLookupTableData",
        template: "Transaction loads an address table account with invalid data",
    },
    CodeDefinition {
        code: ErrorCode(7050026),
        name: "transactionErrorInvalidAddressLookupTableIndex",
        template: "Transaction address table lookup uses an invalid index",
    },
    CodeDefinition {
        code: ErrorCode(7050027),
        name: "transactionErrorInvalidRentPayingAccount",
        template: "Transaction leaves an account with a lower balance than rent-exempt minimum",
    },
];
