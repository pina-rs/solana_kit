//! Typed preparation failures preserve the original RPC, codec, compiler and signer causes.

use core::fmt;

/// A deterministic construction or serialization failure.
#[derive(Debug)]
pub enum StakingBuildError {
    /// Stake amount and rent cannot be added as a wire u64.
    LamportOverflow,
    /// An instruction or text codec rejected its inputs or budgets.
    Codec(solana_kit_core::codecs::Error),
    /// The reused System instruction builder rejected its inputs.
    System(solana_kit_system::ClientError),
    /// Message compilation, signature custody or wire encoding failed.
    Transaction(solana_kit_core::transaction::Error),
}

impl From<solana_kit_core::codecs::Error> for StakingBuildError {
    fn from(error: solana_kit_core::codecs::Error) -> Self {
        Self::Codec(error)
    }
}
impl From<solana_kit_system::ClientError> for StakingBuildError {
    fn from(error: solana_kit_system::ClientError) -> Self {
        Self::System(error)
    }
}
impl From<solana_kit_core::transaction::Error> for StakingBuildError {
    fn from(error: solana_kit_core::transaction::Error) -> Self {
        Self::Transaction(error)
    }
}
impl fmt::Display for StakingBuildError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "staking construction failed: {self:?}")
    }
}
impl std::error::Error for StakingBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::LamportOverflow => None,
            Self::Codec(error) => Some(error),
            Self::System(error) => Some(error),
            Self::Transaction(error) => Some(error),
        }
    }
}

/// An RPC preparation, deterministic construction or caller-signer failure.
#[derive(Debug)]
pub enum PreparationError<SignerFailure> {
    /// The shared RPC client rejected an exchange without retrying.
    Rpc(solana_kit_client::Error),
    /// Deterministic instruction, message or signature validation failed.
    Build(StakingBuildError),
    /// The original caller-owned signer failure.
    Signer(SignerFailure),
}

impl<E> From<solana_kit_client::Error> for PreparationError<E> {
    fn from(error: solana_kit_client::Error) -> Self {
        Self::Rpc(error)
    }
}
impl<E> From<StakingBuildError> for PreparationError<E> {
    fn from(error: StakingBuildError) -> Self {
        Self::Build(error)
    }
}
impl<E: fmt::Display> fmt::Display for PreparationError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rpc(error) => write!(formatter, "staking RPC preparation failed: {error}"),
            Self::Build(error) => error.fmt(formatter),
            Self::Signer(error) => write!(formatter, "staking signer failed: {error}"),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for PreparationError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Rpc(error) => Some(error),
            Self::Build(error) => Some(error),
            Self::Signer(error) => Some(error),
        }
    }
}
