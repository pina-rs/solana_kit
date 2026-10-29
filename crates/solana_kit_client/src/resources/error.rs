//! Stage-specific resource failures retain typed original causes.

use core::fmt;
use solana_kit_core::{codecs, transaction};

use crate::methods::submission::TransactionFailure;

/// Resource preparation or simulation failure, retaining its original cause.
#[derive(Debug)]
pub enum ResourceError {
    /// A compute limit exceeds the supported setter range.
    ComputeUnitLimitOutOfRange(u32),
    /// Compute Budget instruction encoding failed.
    Instruction(codecs::Error),
    /// The caller draft cannot compile or encode as a transaction.
    Compilation(transaction::Error),
    /// Transport, HTTP, RPC framing, or typed response decoding failed.
    Rpc(crate::Error),
    /// The response omitted compute usage or returned explicit null.
    MissingComputeUsage,
    /// Version one requires loaded usage, but it was absent or null.
    MissingLoadedAccountsDataSize,
    /// Simulation reached a transaction-level failure after validating usage.
    Transaction(TransactionFailure),
}

/// Tuning preserves failures from a caller-supplied estimator without erasing types.
#[derive(Debug)]
pub enum TuningError<E> {
    /// The original estimator failure.
    Estimation(E),
    /// Applying an estimate violates a message construction contract.
    Message(ResourceError),
}

impl fmt::Display for ResourceError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ComputeUnitLimitOutOfRange(limit) => {
                write!(output, "compute unit limit exceeds 1,400,000: {limit}")
            }
            Self::Instruction(error) => write!(output, "resource instruction failed: {error}"),
            Self::Compilation(error) => write!(output, "resource transaction failed: {error}"),
            Self::Rpc(error) => write!(output, "resource simulation RPC failed: {error}"),
            Self::MissingComputeUsage => output.write_str("simulation omitted compute usage"),
            Self::MissingLoadedAccountsDataSize => {
                output.write_str("version-one simulation omitted loaded account usage")
            }
            Self::Transaction(error) => write!(output, "resource simulation failed: {error:?}"),
        }
    }
}

impl std::error::Error for ResourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Instruction(error) => Some(error),
            Self::Compilation(error) => Some(error),
            Self::Rpc(error) => Some(error),
            _ => None,
        }
    }
}

impl<E: fmt::Display> fmt::Display for TuningError<E> {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Estimation(error) => write!(output, "resource estimation failed: {error}"),
            Self::Message(error) => write!(output, "resource tuning failed: {error}"),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for TuningError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Estimation(error) => Some(error),
            Self::Message(error) => Some(error),
        }
    }
}
