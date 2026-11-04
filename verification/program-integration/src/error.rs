//! Harness failures retain their original causes and independent cleanup errors.

use crate::fixtures::FixtureError;
use crate::node::NodeError;
use core::fmt;
use solana_kit_client::confirmation::WaitError;
use solana_kit_client::methods::submission::{
    InstructionFailure, KnownTransactionFailure, TransactionFailure,
};

/// Harness or journey failure with its original cause.
#[derive(Debug)]
pub enum Error {
    /// Node location, version, readiness or cleanup failed.
    Node(NodeError),
    /// Cleartext loopback transport configuration failed.
    Native(solana_kit_native::ConfigError),
    /// Caller-owned operating-system entropy was unavailable.
    Entropy(solana_kit_native::keys::NativeEntropyError),
    /// Typed RPC exchange or schema validation failed.
    Rpc(solana_kit_client::Error),
    /// Bundled codec or instruction builder rejected caller input.
    Codec(solana_kit_core::codecs::Error),
    /// System client rejected caller input or a parsed invocation.
    System(solana_kit_system::ClientError),
    /// Program-derived address derivation failed.
    Pda(solana_kit_core::crypto::PdaError),
    /// Token ergonomic plan construction rejected caller input.
    TokenPlan(solana_kit_token::PlanError),
    /// Pinned fixture discovery, manifest or hash verification failed.
    Fixture(FixtureError),
    /// A required local file could not be read.
    Io(std::io::Error),
    /// Account decoding rejected owner, allocation or bytes.
    Account(solana_kit_core::accounts::AccountError),
    /// Transaction compilation, signing or encoding failed.
    Transaction(solana_kit_core::transaction::Error),
    /// Submission or the confirmation reducer failed.
    Wait(WaitError),
    /// Compilation, submission or confirmed execution was rejected.
    Submit(SubmitError),
    /// The harness rejected its own caller state.
    Client(&'static str),
    /// The faucet never credited the disposable payer.
    Airdrop(String),
    /// Independent cleanup failed after a journey.
    Cleanup(String),
    /// A deployed external program or its ProgramData did not match its fixture.
    ProgramImage(String),
    /// An external program codec or builder rejected caller input.
    Program(String),
    /// Startup failed and an independent cleanup failure was also retained.
    Startup {
        /// Original startup failure.
        error: Box<Error>,
        /// Cleanup failure from the same owned node, when any.
        cleanup: Option<NodeError>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Node(error) => write!(formatter, "{error}"),
            Self::Native(error) => write!(formatter, "native transport config: {error}"),
            Self::Entropy(error) => write!(formatter, "native entropy: {error}"),
            Self::Rpc(error) => write!(formatter, "RPC failure: {error}"),
            Self::Codec(error) => write!(formatter, "codec failure: {error}"),
            Self::System(error) => write!(formatter, "system client failure: {error}"),
            Self::Pda(error) => write!(formatter, "address derivation failure: {error}"),
            Self::TokenPlan(error) => write!(formatter, "token plan failure: {error}"),
            Self::Fixture(error) => write!(formatter, "{error}"),
            Self::Io(error) => write!(formatter, "local file failure: {error}"),
            Self::Account(error) => write!(formatter, "account failure: {error:?}"),
            Self::Transaction(error) => write!(formatter, "transaction failure: {error}"),
            Self::Wait(error) => write!(formatter, "confirmation failure: {error}"),
            Self::Submit(error) => write!(formatter, "submission failure: {error}"),
            Self::Client(message) => formatter.write_str(message),
            Self::Airdrop(message) => write!(formatter, "airdrop failure: {message}"),
            Self::Cleanup(message) => write!(formatter, "owned cleanup failure: {message}"),
            Self::ProgramImage(message) => write!(formatter, "program image: {message}"),
            Self::Program(message) => write!(formatter, "external program: {message}"),
            Self::Startup { error, cleanup } => write_startup(formatter, error, cleanup.as_ref()),
        }
    }
}

fn write_startup(
    formatter: &mut fmt::Formatter<'_>,
    error: &Error,
    cleanup: Option<&NodeError>,
) -> fmt::Result {
    write!(formatter, "{error}")?;

    if let Some(cleanup) = cleanup {
        write!(formatter, "; cleanup also failed: {cleanup}")?;
    }

    Ok(())
}

impl std::error::Error for Error {}

impl From<NodeError> for Error {
    fn from(error: NodeError) -> Self {
        Self::Node(error)
    }
}

impl From<solana_kit_native::ConfigError> for Error {
    fn from(error: solana_kit_native::ConfigError) -> Self {
        Self::Native(error)
    }
}

impl From<solana_kit_client::Error> for Error {
    fn from(error: solana_kit_client::Error) -> Self {
        Self::Rpc(error)
    }
}

impl From<solana_kit_core::codecs::Error> for Error {
    fn from(error: solana_kit_core::codecs::Error) -> Self {
        Self::Codec(error)
    }
}

impl From<solana_kit_system::ClientError> for Error {
    fn from(error: solana_kit_system::ClientError) -> Self {
        Self::System(error)
    }
}

impl From<solana_kit_core::crypto::PdaError> for Error {
    fn from(error: solana_kit_core::crypto::PdaError) -> Self {
        Self::Pda(error)
    }
}

impl From<solana_kit_token::PlanError> for Error {
    fn from(error: solana_kit_token::PlanError) -> Self {
        Self::TokenPlan(error)
    }
}

impl From<FixtureError> for Error {
    fn from(error: FixtureError) -> Self {
        Self::Fixture(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<solana_kit_core::accounts::AccountError> for Error {
    fn from(error: solana_kit_core::accounts::AccountError) -> Self {
        Self::Account(error)
    }
}

impl From<SubmitError> for Error {
    fn from(error: SubmitError) -> Self {
        Self::Submit(error)
    }
}

/// A submission that did not produce a confirmed successful signature.
#[derive(Debug)]
pub enum SubmitError {
    /// The exact message could not be compiled, signed or encoded.
    Transaction(solana_kit_core::transaction::Error),
    /// An RPC exchange failed outside execution status.
    Rpc(solana_kit_client::Error),
    /// The node or reducer reported an execution or confirmation failure.
    Wait(WaitError),
}

impl SubmitError {
    /// Exact typed on-chain failure, when the node executed and rejected it.
    pub fn transaction_failure(&self) -> Option<&TransactionFailure> {
        match self {
            Self::Wait(WaitError::Outcome(
                solana_kit_client::confirmation::Failure::Transaction(failure),
            )) => Some(failure),
            _ => None,
        }
    }

    /// Known top-level transaction failure variant.
    pub fn known_failure(&self) -> Option<&KnownTransactionFailure> {
        match self.transaction_failure()? {
            TransactionFailure::Known(reason) | TransactionFailure::WithContext(reason, _) => {
                Some(reason)
            }
            TransactionFailure::Unknown(_) => None,
        }
    }

    /// Exact failing instruction index and its typed instruction failure.
    pub fn instruction_failure(&self) -> Option<(u8, &InstructionFailure)> {
        match self.known_failure()? {
            KnownTransactionFailure::InstructionError(index, failure) => Some((*index, failure)),
            _ => None,
        }
    }
}

impl fmt::Display for SubmitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transaction(error) => write!(formatter, "local transaction failure: {error}"),
            Self::Rpc(error) => write!(formatter, "RPC failure: {error}"),
            Self::Wait(error) => write!(formatter, "confirmation failure: {error}"),
        }
    }
}

impl std::error::Error for SubmitError {}
