//! Typed causes remain inspectable while ordinary formatting omits external values.

use crate::transactions::{ComputeUnitBufferError, ComputeUnitsError, PriorityFeeError};
use core::fmt;
use solana_kit_client::methods::Context;

/// RPC stage following the simulation observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmartTransactionStage {
    /// Recommended fee sampled by account keys.
    FeeEstimate,
    /// Fresh blockhash and real expiry requested after fee resolution.
    Blockhash,
}

/// Deterministic instruction or message construction failed.
#[derive(Debug)]
pub enum SmartTransactionBuildError {
    /// The maintained Compute Budget instruction codec rejected its contract.
    Codec(solana_kit_core::codecs::Error),
    /// Core compilation or unsigned transaction encoding failed.
    Transaction(solana_kit_core::transaction::Error),
}

impl fmt::Display for SmartTransactionBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("smart transaction construction failed")
    }
}

impl std::error::Error for SmartTransactionBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Codec(error) => Some(error),
            Self::Transaction(error) => Some(error),
        }
    }
}

/// Unsigned smart preparation failure, with no automatic retry or fallback.
pub enum SmartTransactionError {
    /// Neither an explicit payer nor a caller signer address was supplied.
    MissingFeePayer,
    /// An explicit compute limit was zero.
    NonPositiveComputeUnitLimit,
    /// The provider omitted its recommended fee or returned null.
    MissingFeeEstimate,
    /// Shared simulation admission or execution failed.
    Simulation(ComputeUnitsError),
    /// Deterministic instruction/message preparation failed.
    Build(SmartTransactionBuildError),
    /// The selected compute buffer could not be evaluated.
    Buffer(ComputeUnitBufferError),
    /// The exact resolved fee did not fit its wire range.
    Fee(PriorityFeeError),
    /// The fee or refreshed-blockhash RPC failed.
    Rpc {
        /// Operation whose original failure is retained.
        stage: SmartTransactionStage,
        /// Original shared client error, available through explicit inspection.
        error: solana_kit_client::Error,
    },
    /// The final blockhash context did not meet the caller's minimum slot.
    StaleBlockhashContext {
        /// Rejected node evaluation context.
        context: Context,
        /// Caller-requested minimum evaluation slot.
        minimum_slot: u64,
    },
}

impl fmt::Debug for SmartTransactionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SmartTransactionError(<redacted>)")
    }
}

impl fmt::Display for SmartTransactionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::MissingFeePayer => "smart preparation requires a payer identity",
            Self::NonPositiveComputeUnitLimit => {
                "smart preparation requires positive compute units"
            }
            Self::MissingFeeEstimate => "smart preparation did not receive a fee estimate",
            Self::Simulation(_) => "smart preparation simulation failed",
            Self::Build(_) => "smart preparation construction failed",
            Self::Buffer(_) => "smart preparation compute buffer failed",
            Self::Fee(_) => "smart preparation fee resolution failed",
            Self::Rpc { .. } => "smart preparation RPC failed",
            Self::StaleBlockhashContext { .. } => "smart preparation blockhash context is stale",
        })
    }
}

impl std::error::Error for SmartTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Simulation(error) => Some(error),
            Self::Build(error) => Some(error),
            Self::Buffer(error) => Some(error),
            Self::Fee(error) => Some(error),
            Self::Rpc { error, .. } => Some(error),
            Self::MissingFeePayer
            | Self::NonPositiveComputeUnitLimit
            | Self::MissingFeeEstimate
            | Self::StaleBlockhashContext { .. } => None,
        }
    }
}

impl From<solana_kit_core::transaction::Error> for SmartTransactionError {
    fn from(error: solana_kit_core::transaction::Error) -> Self {
        Self::Build(SmartTransactionBuildError::Transaction(error))
    }
}
impl From<solana_kit_core::codecs::Error> for SmartTransactionError {
    fn from(error: solana_kit_core::codecs::Error) -> Self {
        Self::Build(SmartTransactionBuildError::Codec(error))
    }
}
impl From<ComputeUnitsError> for SmartTransactionError {
    fn from(error: ComputeUnitsError) -> Self {
        Self::Simulation(error)
    }
}
impl From<ComputeUnitBufferError> for SmartTransactionError {
    fn from(error: ComputeUnitBufferError) -> Self {
        Self::Buffer(error)
    }
}
impl From<PriorityFeeError> for SmartTransactionError {
    fn from(error: PriorityFeeError) -> Self {
        Self::Fee(error)
    }
}
