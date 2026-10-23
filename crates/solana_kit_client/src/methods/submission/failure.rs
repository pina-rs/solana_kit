// SPDX-License-Identifier: Apache-2.0
// Adapted variant declarations from solana-transaction-error 3.4.0 and
// solana-instruction-error 2.5.0; see APACHE-2.0 and the scoped parity receipt.
//! Typed execution failures; RPC transport failures remain separate.
use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;

/// Typed JSON variants specified by `solana-instruction-error-2.5.0`; no official runtime dependency.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum InstructionFailure {
    /// Node-reported `GenericError` execution failure.
    GenericError,
    /// Node-reported `InvalidArgument` execution failure.
    InvalidArgument,
    /// Node-reported `InvalidInstructionData` execution failure.
    InvalidInstructionData,
    /// Node-reported `InvalidAccountData` execution failure.
    InvalidAccountData,
    /// Node-reported `AccountDataTooSmall` execution failure.
    AccountDataTooSmall,
    /// Node-reported `InsufficientFunds` execution failure.
    InsufficientFunds,
    /// Node-reported `IncorrectProgramId` execution failure.
    IncorrectProgramId,
    /// Node-reported `MissingRequiredSignature` execution failure.
    MissingRequiredSignature,
    /// Node-reported `AccountAlreadyInitialized` execution failure.
    AccountAlreadyInitialized,
    /// Node-reported `UninitializedAccount` execution failure.
    UninitializedAccount,
    /// Node-reported `UnbalancedInstruction` execution failure.
    UnbalancedInstruction,
    /// Node-reported `ModifiedProgramId` execution failure.
    ModifiedProgramId,
    /// Node-reported `ExternalAccountLamportSpend` execution failure.
    ExternalAccountLamportSpend,
    /// Node-reported `ExternalAccountDataModified` execution failure.
    ExternalAccountDataModified,
    /// Node-reported `ReadonlyLamportChange` execution failure.
    ReadonlyLamportChange,
    /// Node-reported `ReadonlyDataModified` execution failure.
    ReadonlyDataModified,
    /// Node-reported `DuplicateAccountIndex` execution failure.
    DuplicateAccountIndex,
    /// Node-reported `ExecutableModified` execution failure.
    ExecutableModified,
    /// Node-reported `RentEpochModified` execution failure.
    RentEpochModified,
    /// Node-reported `NotEnoughAccountKeys` execution failure.
    NotEnoughAccountKeys,
    /// Node-reported `AccountDataSizeChanged` execution failure.
    AccountDataSizeChanged,
    /// Node-reported `AccountNotExecutable` execution failure.
    AccountNotExecutable,
    /// Node-reported `AccountBorrowFailed` execution failure.
    AccountBorrowFailed,
    /// Node-reported `AccountBorrowOutstanding` execution failure.
    AccountBorrowOutstanding,
    /// Node-reported `DuplicateAccountOutOfSync` execution failure.
    DuplicateAccountOutOfSync,
    /// Node-reported `Custom` execution failure.
    Custom(u32),
    /// Node-reported `InvalidError` execution failure.
    InvalidError,
    /// Node-reported `ExecutableDataModified` execution failure.
    ExecutableDataModified,
    /// Node-reported `ExecutableLamportChange` execution failure.
    ExecutableLamportChange,
    /// Node-reported `ExecutableAccountNotRentExempt` execution failure.
    ExecutableAccountNotRentExempt,
    /// Node-reported `UnsupportedProgramId` execution failure.
    UnsupportedProgramId,
    /// Node-reported `CallDepth` execution failure.
    CallDepth,
    /// Node-reported `MissingAccount` execution failure.
    MissingAccount,
    /// Node-reported `ReentrancyNotAllowed` execution failure.
    ReentrancyNotAllowed,
    /// Node-reported `MaxSeedLengthExceeded` execution failure.
    MaxSeedLengthExceeded,
    /// Node-reported `InvalidSeeds` execution failure.
    InvalidSeeds,
    /// Node-reported `InvalidRealloc` execution failure.
    InvalidRealloc,
    /// Node-reported `ComputationalBudgetExceeded` execution failure.
    ComputationalBudgetExceeded,
    /// Node-reported `PrivilegeEscalation` execution failure.
    PrivilegeEscalation,
    /// Node-reported `ProgramEnvironmentSetupFailure` execution failure.
    ProgramEnvironmentSetupFailure,
    /// Node-reported `ProgramFailedToComplete` execution failure.
    ProgramFailedToComplete,
    /// Node-reported `ProgramFailedToCompile` execution failure.
    ProgramFailedToCompile,
    /// Node-reported `Immutable` execution failure.
    Immutable,
    /// Node-reported `IncorrectAuthority` execution failure.
    IncorrectAuthority,
    /// Node-reported `BorshIoError` execution failure.
    BorshIoError,
    /// Node-reported `AccountNotRentExempt` execution failure.
    AccountNotRentExempt,
    /// Node-reported `InvalidAccountOwner` execution failure.
    InvalidAccountOwner,
    /// Node-reported `ArithmeticOverflow` execution failure.
    ArithmeticOverflow,
    /// Node-reported `UnsupportedSysvar` execution failure.
    UnsupportedSysvar,
    /// Node-reported `IllegalOwner` execution failure.
    IllegalOwner,
    /// Node-reported `MaxAccountsDataAllocationsExceeded` execution failure.
    MaxAccountsDataAllocationsExceeded,
    /// Node-reported `MaxAccountsExceeded` execution failure.
    MaxAccountsExceeded,
    /// Node-reported `MaxInstructionTraceLengthExceeded` execution failure.
    MaxInstructionTraceLengthExceeded,
    /// Node-reported `BuiltinProgramsMustConsumeComputeUnits` execution failure.
    BuiltinProgramsMustConsumeComputeUnits,
}

/// Typed JSON variants specified by `solana-transaction-error-3.4.0`; no official runtime dependency.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum KnownTransactionFailure {
    /// Node-reported `AccountInUse` execution failure.
    AccountInUse,
    /// Node-reported `AccountLoadedTwice` execution failure.
    AccountLoadedTwice,
    /// Node-reported `AccountNotFound` execution failure.
    AccountNotFound,
    /// Node-reported `ProgramAccountNotFound` execution failure.
    ProgramAccountNotFound,
    /// Node-reported `InsufficientFundsForFee` execution failure.
    InsufficientFundsForFee,
    /// Node-reported `InvalidAccountForFee` execution failure.
    InvalidAccountForFee,
    /// Node-reported `AlreadyProcessed` execution failure.
    AlreadyProcessed,
    /// Node-reported `BlockhashNotFound` execution failure.
    BlockhashNotFound,
    /// Node-reported `InstructionError` execution failure.
    InstructionError(u8, InstructionFailure),
    /// Node-reported `CallChainTooDeep` execution failure.
    CallChainTooDeep,
    /// Node-reported `MissingSignatureForFee` execution failure.
    MissingSignatureForFee,
    /// Node-reported `InvalidAccountIndex` execution failure.
    InvalidAccountIndex,
    /// Node-reported `SignatureFailure` execution failure.
    SignatureFailure,
    /// Node-reported `InvalidProgramForExecution` execution failure.
    InvalidProgramForExecution,
    /// Node-reported `SanitizeFailure` execution failure.
    SanitizeFailure,
    /// Node-reported `ClusterMaintenance` execution failure.
    ClusterMaintenance,
    /// Node-reported `AccountBorrowOutstanding` execution failure.
    AccountBorrowOutstanding,
    /// Node-reported `WouldExceedMaxBlockCostLimit` execution failure.
    WouldExceedMaxBlockCostLimit,
    /// Node-reported `UnsupportedVersion` execution failure.
    UnsupportedVersion,
    /// Node-reported `InvalidWritableAccount` execution failure.
    InvalidWritableAccount,
    /// Node-reported `WouldExceedMaxAccountCostLimit` execution failure.
    WouldExceedMaxAccountCostLimit,
    /// Node-reported `WouldExceedAccountDataBlockLimit` execution failure.
    WouldExceedAccountDataBlockLimit,
    /// Node-reported `TooManyAccountLocks` execution failure.
    TooManyAccountLocks,
    /// Node-reported `AddressLookupTableNotFound` execution failure.
    AddressLookupTableNotFound,
    /// Node-reported `InvalidAddressLookupTableOwner` execution failure.
    InvalidAddressLookupTableOwner,
    /// Node-reported `InvalidAddressLookupTableData` execution failure.
    InvalidAddressLookupTableData,
    /// Node-reported `InvalidAddressLookupTableIndex` execution failure.
    InvalidAddressLookupTableIndex,
    /// Node-reported `InvalidRentPayingAccount` execution failure.
    InvalidRentPayingAccount,
    /// Node-reported `WouldExceedMaxVoteCostLimit` execution failure.
    WouldExceedMaxVoteCostLimit,
    /// Node-reported `WouldExceedAccountDataTotalLimit` execution failure.
    WouldExceedAccountDataTotalLimit,
    /// Node-reported `DuplicateInstruction` execution failure.
    DuplicateInstruction(u8),
    /// Node-reported `InsufficientFundsForRent` execution failure.
    InsufficientFundsForRent {
        #[doc = "Index of the affected account."]
        account_index: u8,
    },
    /// Node-reported `MaxLoadedAccountsDataSizeExceeded` execution failure.
    MaxLoadedAccountsDataSizeExceeded,
    /// Node-reported `InvalidLoadedAccountsDataSizeLimit` execution failure.
    InvalidLoadedAccountsDataSizeLimit,
    /// Node-reported `ResanitizationNeeded` execution failure.
    ResanitizationNeeded,
    /// Node-reported `ProgramExecutionTemporarilyRestricted` execution failure.
    ProgramExecutionTemporarilyRestricted {
        #[doc = "Index of the affected account."]
        account_index: u8,
    },
    /// Node-reported `UnbalancedTransaction` execution failure.
    UnbalancedTransaction,
    /// Node-reported `ProgramCacheHitMaxLimit` execution failure.
    ProgramCacheHitMaxLimit,
    /// Node-reported `CommitCancelled` execution failure.
    CommitCancelled,
}

fn known(name: &str) -> bool {
    matches!(name, |"AccountInUse"| "AccountLoadedTwice"
        | "AccountNotFound"
        | "ProgramAccountNotFound"
        | "InsufficientFundsForFee"
        | "InvalidAccountForFee"
        | "AlreadyProcessed"
        | "BlockhashNotFound"
        | "InstructionError"
        | "CallChainTooDeep"
        | "MissingSignatureForFee"
        | "InvalidAccountIndex"
        | "SignatureFailure"
        | "InvalidProgramForExecution"
        | "SanitizeFailure"
        | "ClusterMaintenance"
        | "AccountBorrowOutstanding"
        | "WouldExceedMaxBlockCostLimit"
        | "UnsupportedVersion"
        | "InvalidWritableAccount"
        | "WouldExceedMaxAccountCostLimit"
        | "WouldExceedAccountDataBlockLimit"
        | "TooManyAccountLocks"
        | "AddressLookupTableNotFound"
        | "InvalidAddressLookupTableOwner"
        | "InvalidAddressLookupTableData"
        | "InvalidAddressLookupTableIndex"
        | "InvalidRentPayingAccount"
        | "WouldExceedMaxVoteCostLimit"
        | "WouldExceedAccountDataTotalLimit"
        | "DuplicateInstruction"
        | "InsufficientFundsForRent"
        | "MaxLoadedAccountsDataSizeExceeded"
        | "InvalidLoadedAccountsDataSizeLimit"
        | "ResanitizationNeeded"
        | "ProgramExecutionTemporarilyRestricted"
        | "UnbalancedTransaction"
        | "ProgramCacheHitMaxLimit"
        | "CommitCancelled")
}

/// A typed known execution failure or a preserved future variant.
pub enum TransactionFailure {
    /// A fully validated known variant.
    Known(KnownTransactionFailure),
    /// A known execution reason with additional original node context.
    WithContext(KnownTransactionFailure, Box<RawValue>),
    /// Future string or single-key object variant, retained exactly.
    Unknown(Box<RawValue>),
}

impl std::fmt::Debug for TransactionFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known(reason) | Self::WithContext(reason, _) => formatter
                .debug_struct("TransactionFailure")
                .field("reason", reason)
                .finish_non_exhaustive(),
            Self::Unknown(_) => formatter.write_str("TransactionFailure::Unknown(<redacted>)"),
        }
    }
}
impl<'de> Deserialize<'de> for TransactionFailure {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(d)?;
        let name = variant_name(raw.get()).map_err(serde::de::Error::custom)?;

        if name == "InstructionError" {
            let fields: std::collections::BTreeMap<String, Box<RawValue>> =
                serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
            let (index, instruction): (u8, Box<RawValue>) =
                serde_json::from_str(fields["InstructionError"].get())
                    .map_err(serde::de::Error::custom)?;
            let name = variant_name(instruction.get()).map_err(serde::de::Error::custom)?;

            if name == "BorshIoError" && instruction.get().starts_with('{') {
                return Ok(Self::WithContext(
                    KnownTransactionFailure::InstructionError(
                        index,
                        InstructionFailure::BorshIoError,
                    ),
                    raw,
                ));
            }

            let known = name == "Custom"
                || serde_json::from_value::<InstructionFailure>(serde_json::Value::String(name))
                    .is_ok();

            if !known {
                return Ok(Self::Unknown(raw));
            }
        }

        if known(&name) {
            return serde_json::from_str(raw.get())
                .map(Self::Known)
                .map_err(serde::de::Error::custom);
        }

        Ok(Self::Unknown(raw))
    }
}
fn variant_name(json: &str) -> Result<String, serde_json::Error> {
    if json.starts_with('"') {
        return serde_json::from_str(json);
    }

    let fields: std::collections::BTreeMap<String, Box<RawValue>> = serde_json::from_str(json)?;

    if fields.len() != 1 {
        return Err(<serde_json::Error as serde::de::Error>::custom(
            "execution error requires one variant",
        ));
    }

    Ok(fields.into_keys().next().expect("one validated variant"))
}
