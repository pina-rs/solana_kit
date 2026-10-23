//! Shared diagnostics preserve the client's existing typed error owners.
mod context_wire;
mod execution;
mod rpc;
mod rpc_enum;
use crate::{
    Error, RemoteError,
    methods::submission::{KnownTransactionFailure, TransactionFailure},
};
pub use context_wire::{decode_context, encode_context};
pub use rpc_enum::{RpcEnumError, RpcEnumErrorConfig, instruction_error, rpc_enum_error};
use solana_kit_core::errors::{
    Context, ContextError, ContextValue, Diagnostic, ErrorCode, ErrorFacts,
};
use std::{error::Error as StdError, fmt};

impl fmt::Display for RemoteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "JSON-RPC error #{}", self.code)
    }
}
impl StdError for RemoteError {}
impl ErrorFacts for RemoteError {
    fn error_code(&self) -> Option<ErrorCode> {
        Some(ErrorCode::from_value(self.code))
    }
}
impl ErrorFacts for Error {
    fn error_code(&self) -> Option<ErrorCode> {
        match self {
            Self::Remote(error) => error.error_code(),
            Self::Status(_) => Some(ErrorCode::RPC_TRANSPORT_HTTP_ERROR),
            _ => None,
        }
    }
    fn error_context(&self) -> Context {
        let mut context = Context::default();

        if let Self::Status(status) = self {
            context
                .insert("statusCode", ContextValue::Unsigned((*status).into()))
                .expect("one bounded numeric fact");
        }

        context
    }
}

impl TransactionFailure {
    /// Converts the already validated execution value into shared code and context.
    /// Unknown variants retain their original raw JSON as the underlying cause.
    pub fn into_diagnostic(self) -> Result<Diagnostic, ContextError> {
        let mut context = Context::default();

        let code = match &self {
            Self::Known(error) | Self::WithContext(error, _) => {
                execution_context(error, &mut context)?;

                error
                    .error_code()
                    .unwrap_or(ErrorCode::TRANSACTION_ERROR_UNKNOWN)
            }
            Self::Unknown(raw) => rpc_enum::unknown_context(raw.get(), &mut context)?,
        };

        Ok(Diagnostic::new(code, context).with_source(self))
    }
}
impl fmt::Display for TransactionFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("transaction execution failed")
    }
}
impl StdError for TransactionFailure {}

/// Validates a JSON-RPC error object using the existing strict integer owner.
/// Fractional, missing, overflowing and wrongly typed code/message values fail.
pub fn json_rpc_error(json: &str) -> Result<Diagnostic, Error> {
    serde_json::from_str::<RemoteError>(json)?.into_diagnostic()
}

/// Validates an execution error using the existing known/future variant parser.
pub fn transaction_error(json: &str) -> Result<Diagnostic, Error> {
    serde_json::from_str::<TransactionFailure>(json)?
        .into_diagnostic()
        .map_err(Error::DiagnosticContext)
}

fn execution_context(
    error: &KnownTransactionFailure,
    context: &mut Context,
) -> Result<(), ContextError> {
    match error {
        KnownTransactionFailure::InstructionError(index, error) => {
            context.insert("index", ContextValue::Unsigned((*index).into()))?;
            if let crate::methods::submission::InstructionFailure::Custom(code) = error {
                context.insert("code", ContextValue::Unsigned((*code).into()))?;
            }
        }
        KnownTransactionFailure::DuplicateInstruction(index) => {
            context.insert("index", ContextValue::Unsigned((*index).into()))?;
        }
        KnownTransactionFailure::InsufficientFundsForRent { account_index }
        | KnownTransactionFailure::ProgramExecutionTemporarilyRestricted { account_index } => {
            context.insert(
                "accountIndex",
                ContextValue::Unsigned((*account_index).into()),
            )?;
        }
        KnownTransactionFailure::ProgramCacheHitMaxLimit
        | KnownTransactionFailure::CommitCancelled => {
            let name = match error {
                KnownTransactionFailure::ProgramCacheHitMaxLimit => "ProgramCacheHitMaxLimit",
                _ => "CommitCancelled",
            };

            context.insert("errorName", ContextValue::Text(name.into()))?;
        }
        _ => {}
    }

    Ok(())
}
