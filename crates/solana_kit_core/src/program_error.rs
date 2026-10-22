//! Typed deterministic program-error matching for off-chain clients.

use crate::Address;
use alloc::string::{String, ToString};

/// Known failure facts retained independently of an RPC error representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProgramError {
    /// A program-defined numeric error code.
    Custom(u32),
    /// A standard runtime error recognized by its exact log text.
    Known(KnownProgramError),
    /// An unrecognized failure description, preserved verbatim.
    Unknown(String),
}

/// Common runtime failures with exact, stable log descriptions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KnownProgramError {
    /// Invalid caller argument.
    InvalidArgument,
    /// Instruction bytes were invalid.
    InvalidInstructionData,
    /// Account bytes were invalid.
    InvalidAccountData,
    /// Account allocation was too small.
    AccountDataTooSmall,
    /// Insufficient lamports or token funds.
    InsufficientFunds,
    /// Incorrect target program.
    IncorrectProgramId,
    /// A required signature was absent.
    MissingRequiredSignature,
    /// Account was already initialized.
    AccountAlreadyInitialized,
    /// Account was not initialized.
    UninitializedAccount,
}

/// A transaction-level instruction error, with its original instruction index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstructionFailure {
    /// Failed instruction position, never inferred from log order.
    pub index: usize,
    /// Typed error facts.
    pub error: ProgramError,
}

impl InstructionFailure {
    /// Matches only a custom error at an existing instruction for this program.
    #[must_use]
    pub fn is_program_error(
        &self,
        programs: &[Address],
        program: Address,
        code: Option<u32>,
    ) -> bool {
        let ProgramError::Custom(actual) = self.error else {
            return false;
        };

        programs.get(self.index) == Some(&program) && code.is_none_or(|expected| expected == actual)
    }
}

/// One failure log with the original text and program address preserved.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProgramFailure {
    /// Program reported by this log, including inner program failures.
    pub program: Address,
    /// Exact source line.
    pub log: String,
    /// Extracted error, including unrecognized descriptions.
    pub error: ProgramError,
}

/// A line resembled a failure log but its address or custom code was malformed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogParseError {
    /// Invalid base58 program address.
    InvalidAddress,
    /// Invalid, empty, or overflowing hexadecimal custom error code.
    InvalidCustomCode,
}

/// Parses exactly `Program <address> failed: <description>`.
///
/// Unmatched lines return `Ok(None)`. Unknown failure descriptions remain
/// `ProgramError::Unknown`; a success line never becomes a fabricated error.
/// No transaction index or outer-program attribution is guessed from logs.
pub fn parse_failure_log(line: &str) -> Result<Option<ProgramFailure>, LogParseError> {
    let Some(body) = line.strip_prefix("Program ") else {
        return Ok(None);
    };
    let Some((address, description)) = body.split_once(" failed: ") else {
        return Ok(None);
    };
    let program = address.parse().map_err(|_| LogParseError::InvalidAddress)?;
    let error = parse_description(description)?;

    Ok(Some(ProgramFailure {
        program,
        log: line.to_string(),
        error,
    }))
}

/// Classifies only exact known descriptions and canonical hexadecimal custom codes.
fn parse_description(description: &str) -> Result<ProgramError, LogParseError> {
    if let Some(code) = description.strip_prefix("custom program error: 0x") {
        if code.is_empty() || !code.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(LogParseError::InvalidCustomCode);
        }

        return u32::from_str_radix(code, 16)
            .map(ProgramError::Custom)
            .map_err(|_| LogParseError::InvalidCustomCode);
    }

    let known = match description {
        "invalid program argument" => KnownProgramError::InvalidArgument,
        "invalid instruction data" => KnownProgramError::InvalidInstructionData,
        "invalid account data for instruction" => KnownProgramError::InvalidAccountData,
        "account data too small for instruction" => KnownProgramError::AccountDataTooSmall,
        "insufficient funds for instruction" => KnownProgramError::InsufficientFunds,
        "incorrect program id for instruction" => KnownProgramError::IncorrectProgramId,
        "missing required signature for instruction" => KnownProgramError::MissingRequiredSignature,
        "instruction requires an uninitialized account" => {
            KnownProgramError::AccountAlreadyInitialized
        }
        "instruction requires an initialized account" => KnownProgramError::UninitializedAccount,
        _ => return Ok(ProgramError::Unknown(description.to_string())),
    };

    Ok(ProgramError::Known(known))
}
