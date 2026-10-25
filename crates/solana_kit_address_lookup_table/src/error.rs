//! Exact custom codes from the pinned upstream program error enum.
use solana_kit_core::{
    Address,
    program_error::{InstructionFailure, ProgramError},
};

const MAX_SEED_LENGTH: u32 = 0;
const INVALID_SEEDS: u32 = 1;
const ILLEGAL_OWNER: u32 = 2;
const READONLY_DATA: u32 = 10;
const READONLY_LAMPORTS: u32 = 11;

/// Known ALT custom errors and lossless unknown numeric codes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AddressLookupTableError {
    /// Seed length exceeds the address derivation limit (code 0).
    PubkeyErrorMaxSeedLengthExceeded,
    /// Seeds do not derive a viable address (code 1).
    PubkeyErrorInvalidSeeds,
    /// Owner is not permitted (code 2).
    PubkeyErrorIllegalOwner,
    /// Readonly account data changed (code 10).
    ReadonlyDataModified,
    /// Readonly account lamports changed (code 11).
    ReadonlyLamportsChanged,
    /// An unrecognized code, retained exactly.
    Unknown(u32),
}

impl AddressLookupTableError {
    /// Classifies the exact pinned numeric codes.
    #[must_use]
    pub const fn from_code(code: u32) -> Self {
        match code {
            MAX_SEED_LENGTH => Self::PubkeyErrorMaxSeedLengthExceeded,
            INVALID_SEEDS => Self::PubkeyErrorInvalidSeeds,
            ILLEGAL_OWNER => Self::PubkeyErrorIllegalOwner,
            READONLY_DATA => Self::ReadonlyDataModified,
            READONLY_LAMPORTS => Self::ReadonlyLamportsChanged,
            code => Self::Unknown(code),
        }
    }

    /// Returns the original custom code.
    #[must_use]
    pub const fn code(self) -> u32 {
        match self {
            Self::PubkeyErrorMaxSeedLengthExceeded => MAX_SEED_LENGTH,
            Self::PubkeyErrorInvalidSeeds => INVALID_SEEDS,
            Self::PubkeyErrorIllegalOwner => ILLEGAL_OWNER,
            Self::ReadonlyDataModified => READONLY_DATA,
            Self::ReadonlyLamportsChanged => READONLY_LAMPORTS,
            Self::Unknown(code) => code,
        }
    }

    /// Classifies a failure only when its actual index targets the supplied program.
    #[must_use]
    pub fn from_failure(
        failure: &InstructionFailure,
        programs: &[Address],
        program: Address,
    ) -> Option<Self> {
        if !failure.is_program_error(programs, program, None) {
            return None;
        }

        match failure.error {
            ProgramError::Custom(code) => Some(Self::from_code(code)),
            _ => None,
        }
    }
}
