//! Exact pinned Token-2022 error vocabulary.
use solana_kit_core::{
    Address,
    program_error::{InstructionFailure, ProgramError},
};
/// Pinned contiguous source discriminants.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KnownToken2022Error {
    /// Pinned upstream `NotRentExempt` error.
    NotRentExempt,
    /// Pinned upstream `InsufficientFunds` error.
    InsufficientFunds,
    /// Pinned upstream `InvalidMint` error.
    InvalidMint,
    /// Pinned upstream `MintMismatch` error.
    MintMismatch,
    /// Pinned upstream `OwnerMismatch` error.
    OwnerMismatch,
    /// Pinned upstream `FixedSupply` error.
    FixedSupply,
    /// Pinned upstream `AlreadyInUse` error.
    AlreadyInUse,
    /// Pinned upstream `InvalidNumberOfProvidedSigners` error.
    InvalidNumberOfProvidedSigners,
    /// Pinned upstream `InvalidNumberOfRequiredSigners` error.
    InvalidNumberOfRequiredSigners,
    /// Pinned upstream `UninitializedState` error.
    UninitializedState,
    /// Pinned upstream `NativeNotSupported` error.
    NativeNotSupported,
    /// Pinned upstream `NonNativeHasBalance` error.
    NonNativeHasBalance,
    /// Pinned upstream `InvalidInstruction` error.
    InvalidInstruction,
    /// Pinned upstream `InvalidState` error.
    InvalidState,
    /// Pinned upstream `Overflow` error.
    Overflow,
    /// Pinned upstream `AuthorityTypeNotSupported` error.
    AuthorityTypeNotSupported,
    /// Pinned upstream `MintCannotFreeze` error.
    MintCannotFreeze,
    /// Pinned upstream `AccountFrozen` error.
    AccountFrozen,
    /// Pinned upstream `MintDecimalsMismatch` error.
    MintDecimalsMismatch,
    /// Pinned upstream `NonNativeNotSupported` error.
    NonNativeNotSupported,
    /// Pinned upstream `ExtensionTypeMismatch` error.
    ExtensionTypeMismatch,
    /// Pinned upstream `ExtensionBaseMismatch` error.
    ExtensionBaseMismatch,
    /// Pinned upstream `ExtensionAlreadyInitialized` error.
    ExtensionAlreadyInitialized,
    /// Pinned upstream `ConfidentialTransferAccountHasBalance` error.
    ConfidentialTransferAccountHasBalance,
    /// Pinned upstream `ConfidentialTransferAccountNotApproved` error.
    ConfidentialTransferAccountNotApproved,
    /// Pinned upstream `ConfidentialTransferDepositsAndTransfersDisabled` error.
    ConfidentialTransferDepositsAndTransfersDisabled,
    /// Pinned upstream `ConfidentialTransferElGamalPubkeyMismatch` error.
    ConfidentialTransferElGamalPubkeyMismatch,
    /// Pinned upstream `ConfidentialTransferBalanceMismatch` error.
    ConfidentialTransferBalanceMismatch,
    /// Pinned upstream `MintHasSupply` error.
    MintHasSupply,
    /// Pinned upstream `NoAuthorityExists` error.
    NoAuthorityExists,
    /// Pinned upstream `TransferFeeExceedsMaximum` error.
    TransferFeeExceedsMaximum,
    /// Pinned upstream `MintRequiredForTransfer` error.
    MintRequiredForTransfer,
    /// Pinned upstream `FeeMismatch` error.
    FeeMismatch,
    /// Pinned upstream `FeeParametersMismatch` error.
    FeeParametersMismatch,
    /// Pinned upstream `ImmutableOwner` error.
    ImmutableOwner,
    /// Pinned upstream `AccountHasWithheldTransferFees` error.
    AccountHasWithheldTransferFees,
    /// Pinned upstream `NoMemo` error.
    NoMemo,
    /// Pinned upstream `NonTransferable` error.
    NonTransferable,
    /// Pinned upstream `NonTransferableNeedsImmutableOwnership` error.
    NonTransferableNeedsImmutableOwnership,
    /// Pinned upstream `MaximumPendingBalanceCreditCounterExceeded` error.
    MaximumPendingBalanceCreditCounterExceeded,
    /// Pinned upstream `MaximumDepositAmountExceeded` error.
    MaximumDepositAmountExceeded,
    /// Pinned upstream `CpiGuardSettingsLocked` error.
    CpiGuardSettingsLocked,
    /// Pinned upstream `CpiGuardTransferBlocked` error.
    CpiGuardTransferBlocked,
    /// Pinned upstream `CpiGuardBurnBlocked` error.
    CpiGuardBurnBlocked,
    /// Pinned upstream `CpiGuardCloseAccountBlocked` error.
    CpiGuardCloseAccountBlocked,
    /// Pinned upstream `CpiGuardApproveBlocked` error.
    CpiGuardApproveBlocked,
    /// Pinned upstream `CpiGuardSetAuthorityBlocked` error.
    CpiGuardSetAuthorityBlocked,
    /// Pinned upstream `CpiGuardOwnerChangeBlocked` error.
    CpiGuardOwnerChangeBlocked,
    /// Pinned upstream `ExtensionNotFound` error.
    ExtensionNotFound,
    /// Pinned upstream `NonConfidentialTransfersDisabled` error.
    NonConfidentialTransfersDisabled,
    /// Pinned upstream `ConfidentialTransferFeeAccountHasWithheldFee` error.
    ConfidentialTransferFeeAccountHasWithheldFee,
    /// Pinned upstream `InvalidExtensionCombination` error.
    InvalidExtensionCombination,
    /// Pinned upstream `InvalidLengthForAlloc` error.
    InvalidLengthForAlloc,
    /// Pinned upstream `AccountDecryption` error.
    AccountDecryption,
    /// Pinned upstream `ProofGeneration` error.
    ProofGeneration,
    /// Pinned upstream `InvalidProofInstructionOffset` error.
    InvalidProofInstructionOffset,
    /// Pinned upstream `HarvestToMintDisabled` error.
    HarvestToMintDisabled,
    /// Pinned upstream `SplitProofContextStateAccountsNotSupported` error.
    SplitProofContextStateAccountsNotSupported,
    /// Pinned upstream `NotEnoughProofContextStateAccounts` error.
    NotEnoughProofContextStateAccounts,
    /// Pinned upstream `MalformedCiphertext` error.
    MalformedCiphertext,
    /// Pinned upstream `CiphertextArithmeticFailed` error.
    CiphertextArithmeticFailed,
    /// Pinned upstream `PedersenCommitmentMismatch` error.
    PedersenCommitmentMismatch,
    /// Pinned upstream `RangeProofLengthMismatch` error.
    RangeProofLengthMismatch,
    /// Pinned upstream `IllegalBitLength` error.
    IllegalBitLength,
    /// Pinned upstream `FeeCalculation` error.
    FeeCalculation,
    /// Pinned upstream `IllegalMintBurnConversion` error.
    IllegalMintBurnConversion,
    /// Pinned upstream `InvalidScale` error.
    InvalidScale,
    /// Pinned upstream `MintPaused` error.
    MintPaused,
    /// Pinned upstream `PendingBalanceNonZero` error.
    PendingBalanceNonZero,
}
const KNOWN: &[KnownToken2022Error] = &[
    KnownToken2022Error::NotRentExempt,
    KnownToken2022Error::InsufficientFunds,
    KnownToken2022Error::InvalidMint,
    KnownToken2022Error::MintMismatch,
    KnownToken2022Error::OwnerMismatch,
    KnownToken2022Error::FixedSupply,
    KnownToken2022Error::AlreadyInUse,
    KnownToken2022Error::InvalidNumberOfProvidedSigners,
    KnownToken2022Error::InvalidNumberOfRequiredSigners,
    KnownToken2022Error::UninitializedState,
    KnownToken2022Error::NativeNotSupported,
    KnownToken2022Error::NonNativeHasBalance,
    KnownToken2022Error::InvalidInstruction,
    KnownToken2022Error::InvalidState,
    KnownToken2022Error::Overflow,
    KnownToken2022Error::AuthorityTypeNotSupported,
    KnownToken2022Error::MintCannotFreeze,
    KnownToken2022Error::AccountFrozen,
    KnownToken2022Error::MintDecimalsMismatch,
    KnownToken2022Error::NonNativeNotSupported,
    KnownToken2022Error::ExtensionTypeMismatch,
    KnownToken2022Error::ExtensionBaseMismatch,
    KnownToken2022Error::ExtensionAlreadyInitialized,
    KnownToken2022Error::ConfidentialTransferAccountHasBalance,
    KnownToken2022Error::ConfidentialTransferAccountNotApproved,
    KnownToken2022Error::ConfidentialTransferDepositsAndTransfersDisabled,
    KnownToken2022Error::ConfidentialTransferElGamalPubkeyMismatch,
    KnownToken2022Error::ConfidentialTransferBalanceMismatch,
    KnownToken2022Error::MintHasSupply,
    KnownToken2022Error::NoAuthorityExists,
    KnownToken2022Error::TransferFeeExceedsMaximum,
    KnownToken2022Error::MintRequiredForTransfer,
    KnownToken2022Error::FeeMismatch,
    KnownToken2022Error::FeeParametersMismatch,
    KnownToken2022Error::ImmutableOwner,
    KnownToken2022Error::AccountHasWithheldTransferFees,
    KnownToken2022Error::NoMemo,
    KnownToken2022Error::NonTransferable,
    KnownToken2022Error::NonTransferableNeedsImmutableOwnership,
    KnownToken2022Error::MaximumPendingBalanceCreditCounterExceeded,
    KnownToken2022Error::MaximumDepositAmountExceeded,
    KnownToken2022Error::CpiGuardSettingsLocked,
    KnownToken2022Error::CpiGuardTransferBlocked,
    KnownToken2022Error::CpiGuardBurnBlocked,
    KnownToken2022Error::CpiGuardCloseAccountBlocked,
    KnownToken2022Error::CpiGuardApproveBlocked,
    KnownToken2022Error::CpiGuardSetAuthorityBlocked,
    KnownToken2022Error::CpiGuardOwnerChangeBlocked,
    KnownToken2022Error::ExtensionNotFound,
    KnownToken2022Error::NonConfidentialTransfersDisabled,
    KnownToken2022Error::ConfidentialTransferFeeAccountHasWithheldFee,
    KnownToken2022Error::InvalidExtensionCombination,
    KnownToken2022Error::InvalidLengthForAlloc,
    KnownToken2022Error::AccountDecryption,
    KnownToken2022Error::ProofGeneration,
    KnownToken2022Error::InvalidProofInstructionOffset,
    KnownToken2022Error::HarvestToMintDisabled,
    KnownToken2022Error::SplitProofContextStateAccountsNotSupported,
    KnownToken2022Error::NotEnoughProofContextStateAccounts,
    KnownToken2022Error::MalformedCiphertext,
    KnownToken2022Error::CiphertextArithmeticFailed,
    KnownToken2022Error::PedersenCommitmentMismatch,
    KnownToken2022Error::RangeProofLengthMismatch,
    KnownToken2022Error::IllegalBitLength,
    KnownToken2022Error::FeeCalculation,
    KnownToken2022Error::IllegalMintBurnConversion,
    KnownToken2022Error::InvalidScale,
    KnownToken2022Error::MintPaused,
    KnownToken2022Error::PendingBalanceNonZero,
];
const LABELS: &[&str] = &[
    "NotRentExempt",
    "InsufficientFunds",
    "InvalidMint",
    "MintMismatch",
    "OwnerMismatch",
    "FixedSupply",
    "AlreadyInUse",
    "InvalidNumberOfProvidedSigners",
    "InvalidNumberOfRequiredSigners",
    "UninitializedState",
    "NativeNotSupported",
    "NonNativeHasBalance",
    "InvalidInstruction",
    "InvalidState",
    "Overflow",
    "AuthorityTypeNotSupported",
    "MintCannotFreeze",
    "AccountFrozen",
    "MintDecimalsMismatch",
    "NonNativeNotSupported",
    "ExtensionTypeMismatch",
    "ExtensionBaseMismatch",
    "ExtensionAlreadyInitialized",
    "ConfidentialTransferAccountHasBalance",
    "ConfidentialTransferAccountNotApproved",
    "ConfidentialTransferDepositsAndTransfersDisabled",
    "ConfidentialTransferElGamalPubkeyMismatch",
    "ConfidentialTransferBalanceMismatch",
    "MintHasSupply",
    "NoAuthorityExists",
    "TransferFeeExceedsMaximum",
    "MintRequiredForTransfer",
    "FeeMismatch",
    "FeeParametersMismatch",
    "ImmutableOwner",
    "AccountHasWithheldTransferFees",
    "NoMemo",
    "NonTransferable",
    "NonTransferableNeedsImmutableOwnership",
    "MaximumPendingBalanceCreditCounterExceeded",
    "MaximumDepositAmountExceeded",
    "CpiGuardSettingsLocked",
    "CpiGuardTransferBlocked",
    "CpiGuardBurnBlocked",
    "CpiGuardCloseAccountBlocked",
    "CpiGuardApproveBlocked",
    "CpiGuardSetAuthorityBlocked",
    "CpiGuardOwnerChangeBlocked",
    "ExtensionNotFound",
    "NonConfidentialTransfersDisabled",
    "ConfidentialTransferFeeAccountHasWithheldFee",
    "InvalidExtensionCombination",
    "InvalidLengthForAlloc",
    "AccountDecryption",
    "ProofGeneration",
    "InvalidProofInstructionOffset",
    "HarvestToMintDisabled",
    "SplitProofContextStateAccountsNotSupported",
    "NotEnoughProofContextStateAccounts",
    "MalformedCiphertext",
    "CiphertextArithmeticFailed",
    "PedersenCommitmentMismatch",
    "RangeProofLengthMismatch",
    "IllegalBitLength",
    "FeeCalculation",
    "IllegalMintBurnConversion",
    "InvalidScale",
    "MintPaused",
    "PendingBalanceNonZero",
];
/// Program custom error with unknown numeric codes preserved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Token2022Error {
    /// A pinned upstream error.
    Known(KnownToken2022Error),
    /// An unrecognized future numeric code.
    Unknown(u32),
}
impl Token2022Error {
    /// Classifies the exact custom numeric code.
    pub fn from_code(code: u32) -> Self {
        match usize::try_from(code)
            .ok()
            .and_then(|index| KNOWN.get(index))
            .copied()
        {
            Some(known) => Self::Known(known),
            None => Self::Unknown(code),
        }
    }
    /// Returns the original custom numeric code.
    pub fn code(self) -> u32 {
        match self {
            Self::Known(known) => known as u32,
            Self::Unknown(code) => code,
        }
    }
    /// Returns the pinned Rust label for recognized errors.
    pub fn label(self) -> Option<&'static str> {
        match self {
            Self::Known(known) => usize::try_from(known as u32)
                .ok()
                .and_then(|index| LABELS.get(index))
                .copied(),
            Self::Unknown(_) => None,
        }
    }
}
/// Classifies only the actual instruction index for the explicit expected program.
pub fn classify_failure(
    failure: &InstructionFailure,
    programs: &[Address],
    program: Address,
) -> Option<Token2022Error> {
    if programs.get(failure.index) != Some(&program) {
        return None;
    }

    match failure.error {
        ProgramError::Custom(code) => Some(Token2022Error::from_code(code)),
        _ => None,
    }
}
