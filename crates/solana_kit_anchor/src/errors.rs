//! Structured codec failures and the pinned Anchor runtime error table.

use crate::Idl;
use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::fmt;

/// A bounded parse, encode, decode, or event operation failed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Caller-selected resource budget exhausted.
    Limit,
    /// Invalid IDL structure or unsupported type node.
    Idl(&'static str),
    /// Named IDL entry does not exist.
    Unknown(String),
    /// Dynamic value has the wrong variant or numeric range.
    Value,
    /// Underlying Kit binary codec failure.
    Codec(solana_kit_core::codecs::Error),
    /// Explicit IDL discriminator did not match, including short input.
    Discriminator {
        /// Expected prefix.
        expected: Vec<u8>,
        /// Available actual prefix.
        found: Vec<u8>,
    },
    /// Encoding failure attributed to the original field/account context.
    Field {
        /// Offending field or account name.
        name: String,
        /// Original structured error.
        source: Box<Error>,
    },
    /// An eligible program-data line contains invalid base64.
    Base64,
}

impl Error {
    pub(crate) fn context(self, name: &str) -> Self {
        Self::Field {
            name: name.to_string(),
            source: Box::new(self),
        }
    }
}

impl From<solana_kit_core::codecs::Error> for Error {
    fn from(error: solana_kit_core::codecs::Error) -> Self {
        Self::Codec(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Field { name, source } => write!(formatter, "failed to encode {name}: {source}"),
            _ => write!(formatter, "Anchor runtime error: {self:?}"),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Field { source, .. } => Some(source.as_ref()),
            Self::Codec(source) => Some(source),
            _ => None,
        }
    }
}

/// A standard Anchor 0.31 runtime error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StandardError {
    /// Exact runtime code.
    pub code: i64,
    /// Runtime error name.
    pub name: &'static str,
    /// Runtime error message.
    pub message: &'static str,
}

/// An error resolved against the supplied IDL or the standard table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgramError {
    /// Original code.
    pub code: i64,
    /// Resolved name, or `Unknown`.
    pub name: String,
    /// Resolved message, preserving an explicitly empty custom message.
    pub message: String,
}

impl fmt::Display for ProgramError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "AnchorProgramError({}, {}): {}",
            self.code, self.name, self.message
        )
    }
}

impl core::error::Error for ProgramError {}

/// Resolves any signed 64-bit code, preferring IDL entries even below the custom range.
pub fn program_error(code: i64, idl: Option<&Idl>) -> ProgramError {
    if let Some(custom) = idl.and_then(|idl| idl.errors.get(&code)) {
        return ProgramError {
            code,
            name: custom.name.clone(),
            message: custom
                .message
                .clone()
                .unwrap_or_else(|| format!("Custom program error: {code}")),
        };
    }

    if let Some(standard) = STANDARD_ERRORS.iter().find(|entry| entry.code == code) {
        return ProgramError {
            code,
            name: standard.name.to_string(),
            message: standard.message.to_string(),
        };
    }

    ProgramError {
        code,
        name: "Unknown".to_string(),
        message: format!("Error code {code} is not a known Anchor error"),
    }
}

/// Complete pinned standard error table, retaining the original code order.
pub const STANDARD_ERRORS: &[StandardError] = &[
    StandardError {
        code: 100,
        name: "InstructionMissing",
        message: "Instruction discriminator not provided",
    },
    StandardError {
        code: 101,
        name: "InstructionFallbackNotFound",
        message: "Fallback functions are not supported",
    },
    StandardError {
        code: 102,
        name: "InstructionDidNotDeserialize",
        message: "The program could not deserialize the given instruction",
    },
    StandardError {
        code: 103,
        name: "InstructionDidNotSerialize",
        message: "The program could not serialize the given instruction",
    },
    StandardError {
        code: 1000,
        name: "IdlInstructionStub",
        message: "The program was compiled without idl instructions",
    },
    StandardError {
        code: 1001,
        name: "IdlInstructionInvalidProgram",
        message: "Invalid program given to the IDL instruction",
    },
    StandardError {
        code: 1002,
        name: "IdlAccountNotEmpty",
        message: "IDL account must be empty in order to resize, try closing first",
    },
    StandardError {
        code: 1500,
        name: "EventInstructionStub",
        message: "The program was compiled without `event-cpi` feature",
    },
    StandardError {
        code: 2000,
        name: "ConstraintMut",
        message: "A mut constraint was violated",
    },
    StandardError {
        code: 2001,
        name: "ConstraintHasOne",
        message: "A has one constraint was violated",
    },
    StandardError {
        code: 2002,
        name: "ConstraintSigner",
        message: "A signer constraint was violated",
    },
    StandardError {
        code: 2003,
        name: "ConstraintRaw",
        message: "A raw constraint was violated",
    },
    StandardError {
        code: 2004,
        name: "ConstraintOwner",
        message: "An owner constraint was violated",
    },
    StandardError {
        code: 2005,
        name: "ConstraintRentExempt",
        message: "A rent exemption constraint was violated",
    },
    StandardError {
        code: 2006,
        name: "ConstraintSeeds",
        message: "A seeds constraint was violated",
    },
    StandardError {
        code: 2007,
        name: "ConstraintExecutable",
        message: "An executable constraint was violated",
    },
    StandardError {
        code: 2008,
        name: "ConstraintState",
        message: "Deprecated Error, feel free to replace with something else",
    },
    StandardError {
        code: 2009,
        name: "ConstraintAssociated",
        message: "An associated constraint was violated",
    },
    StandardError {
        code: 2010,
        name: "ConstraintAssociatedInit",
        message: "An associated init constraint was violated",
    },
    StandardError {
        code: 2011,
        name: "ConstraintClose",
        message: "A close constraint was violated",
    },
    StandardError {
        code: 2012,
        name: "ConstraintAddress",
        message: "An address constraint was violated",
    },
    StandardError {
        code: 2013,
        name: "ConstraintZero",
        message: "Expected zero account discriminant",
    },
    StandardError {
        code: 2014,
        name: "ConstraintTokenMint",
        message: "A token mint constraint was violated",
    },
    StandardError {
        code: 2015,
        name: "ConstraintTokenOwner",
        message: "A token owner constraint was violated",
    },
    StandardError {
        code: 2016,
        name: "ConstraintMintMintAuthority",
        message: "A mint mint authority constraint was violated",
    },
    StandardError {
        code: 2017,
        name: "ConstraintMintFreezeAuthority",
        message: "A mint freeze authority constraint was violated",
    },
    StandardError {
        code: 2018,
        name: "ConstraintMintDecimals",
        message: "A mint decimals constraint was violated",
    },
    StandardError {
        code: 2019,
        name: "ConstraintSpace",
        message: "A space constraint was violated",
    },
    StandardError {
        code: 2020,
        name: "ConstraintAccountIsNone",
        message: "A required account for the constraint is None",
    },
    StandardError {
        code: 2021,
        name: "ConstraintTokenTokenProgram",
        message: "A token account token program constraint was violated",
    },
    StandardError {
        code: 2022,
        name: "ConstraintMintTokenProgram",
        message: "A mint token program constraint was violated",
    },
    StandardError {
        code: 2023,
        name: "ConstraintAssociatedTokenTokenProgram",
        message: "An associated token account token program constraint was violated",
    },
    StandardError {
        code: 2024,
        name: "ConstraintMintGroupPointerExtension",
        message: "A group pointer extension constraint was violated",
    },
    StandardError {
        code: 2025,
        name: "ConstraintMintGroupPointerExtensionAuthority",
        message: "A group pointer extension authority constraint was violated",
    },
    StandardError {
        code: 2026,
        name: "ConstraintMintGroupPointerExtensionGroupAddress",
        message: "A group pointer extension group address constraint was violated",
    },
    StandardError {
        code: 2027,
        name: "ConstraintMintGroupMemberPointerExtension",
        message: "A group member pointer extension constraint was violated",
    },
    StandardError {
        code: 2028,
        name: "ConstraintMintGroupMemberPointerExtensionAuthority",
        message: "A group member pointer extension authority constraint was violated",
    },
    StandardError {
        code: 2029,
        name: "ConstraintMintGroupMemberPointerExtensionMemberAddress",
        message: "A group member pointer extension group address constraint was violated",
    },
    StandardError {
        code: 2030,
        name: "ConstraintMintMetadataPointerExtension",
        message: "A metadata pointer extension constraint was violated",
    },
    StandardError {
        code: 2031,
        name: "ConstraintMintMetadataPointerExtensionAuthority",
        message: "A metadata pointer extension authority constraint was violated",
    },
    StandardError {
        code: 2032,
        name: "ConstraintMintMetadataPointerExtensionMetadataAddress",
        message: "A metadata pointer extension metadata address constraint was violated",
    },
    StandardError {
        code: 2033,
        name: "ConstraintMintCloseAuthorityExtension",
        message: "A close authority constraint was violated",
    },
    StandardError {
        code: 2034,
        name: "ConstraintMintCloseAuthorityExtensionAuthority",
        message: "A close authority extension authority constraint was violated",
    },
    StandardError {
        code: 2035,
        name: "ConstraintMintPermanentDelegateExtension",
        message: "A permanent delegate extension constraint was violated",
    },
    StandardError {
        code: 2036,
        name: "ConstraintMintPermanentDelegateExtensionDelegate",
        message: "A permanent delegate extension delegate constraint was violated",
    },
    StandardError {
        code: 2037,
        name: "ConstraintMintTransferHookExtension",
        message: "A transfer hook extension constraint was violated",
    },
    StandardError {
        code: 2038,
        name: "ConstraintMintTransferHookExtensionAuthority",
        message: "A transfer hook extension authority constraint was violated",
    },
    StandardError {
        code: 2039,
        name: "ConstraintMintTransferHookExtensionProgramId",
        message: "A transfer hook extension transfer hook program id constraint was violated",
    },
    StandardError {
        code: 2500,
        name: "RequireViolated",
        message: "A require expression was violated",
    },
    StandardError {
        code: 2501,
        name: "RequireEqViolated",
        message: "A require_eq expression was violated",
    },
    StandardError {
        code: 2502,
        name: "RequireKeysEqViolated",
        message: "A require_keys_eq expression was violated",
    },
    StandardError {
        code: 2503,
        name: "RequireNeqViolated",
        message: "A require_neq expression was violated",
    },
    StandardError {
        code: 2504,
        name: "RequireKeysNeqViolated",
        message: "A require_keys_neq expression was violated",
    },
    StandardError {
        code: 2505,
        name: "RequireGtViolated",
        message: "A require_gt expression was violated",
    },
    StandardError {
        code: 2506,
        name: "RequireGteViolated",
        message: "A require_gte expression was violated",
    },
    StandardError {
        code: 3000,
        name: "AccountDiscriminatorAlreadySet",
        message: "The account discriminator was already set on this account",
    },
    StandardError {
        code: 3001,
        name: "AccountDiscriminatorNotFound",
        message: "No discriminator was found on the account",
    },
    StandardError {
        code: 3002,
        name: "AccountDiscriminatorMismatch",
        message: "Account discriminator did not match what was expected",
    },
    StandardError {
        code: 3003,
        name: "AccountDidNotDeserialize",
        message: "Failed to deserialize the account",
    },
    StandardError {
        code: 3004,
        name: "AccountDidNotSerialize",
        message: "Failed to serialize the account",
    },
    StandardError {
        code: 3005,
        name: "AccountNotEnoughKeys",
        message: "Not enough account keys given to the instruction",
    },
    StandardError {
        code: 3006,
        name: "AccountNotMutable",
        message: "The given account is not mutable",
    },
    StandardError {
        code: 3007,
        name: "AccountOwnedByWrongProgram",
        message: "The given account is owned by a different program than expected",
    },
    StandardError {
        code: 3008,
        name: "InvalidProgramId",
        message: "Program ID was not as expected",
    },
    StandardError {
        code: 3009,
        name: "InvalidProgramExecutable",
        message: "Program account is not executable",
    },
    StandardError {
        code: 3010,
        name: "AccountNotSigner",
        message: "The given account did not sign",
    },
    StandardError {
        code: 3011,
        name: "AccountNotSystemOwned",
        message: "The given account is not owned by the system program",
    },
    StandardError {
        code: 3012,
        name: "AccountNotInitialized",
        message: "The program expected this account to be already initialized",
    },
    StandardError {
        code: 3013,
        name: "AccountNotProgramData",
        message: "The given account is not a program data account",
    },
    StandardError {
        code: 3014,
        name: "AccountNotAssociatedTokenAccount",
        message: "The given account is not the associated token account",
    },
    StandardError {
        code: 3015,
        name: "AccountSysvarMismatch",
        message: "The given public key does not match the required sysvar",
    },
    StandardError {
        code: 3016,
        name: "AccountReallocExceedsLimit",
        message: "The account reallocation exceeds the MAX_PERMITTED_DATA_INCREASE limit",
    },
    StandardError {
        code: 3017,
        name: "AccountDuplicateReallocs",
        message: "The account was duplicated for more than one reallocation",
    },
    StandardError {
        code: 4100,
        name: "DeclaredProgramIdMismatch",
        message: "The declared program id does not match the actual program id",
    },
    StandardError {
        code: 4101,
        name: "TryingToInitPayerAsProgramAccount",
        message: "You cannot/should not initialize the payer account as a program account",
    },
    StandardError {
        code: 4102,
        name: "InvalidNumericConversion",
        message: "Error during numeric conversion",
    },
    StandardError {
        code: 5000,
        name: "Deprecated",
        message: "The API being used is deprecated and should no longer be used",
    },
];
