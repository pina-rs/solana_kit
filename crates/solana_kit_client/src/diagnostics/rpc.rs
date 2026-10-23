//! JSON-RPC context projection at the existing strict client boundary.
use super::{context_wire::from_json, transaction_error};
use crate::{Error, Presence, RemoteError};
use solana_kit_core::errors::{Context, ContextValue, Diagnostic, ErrorCode};

const PREFLIGHT_KEYS: &[&str] = &[
    "accounts",
    "fee",
    "innerInstructions",
    "loadedAccountsDataSize",
    "loadedAddresses",
    "logs",
    "postBalances",
    "postTokenBalances",
    "preBalances",
    "preTokenBalances",
    "replacementBlockhash",
    "returnData",
    "unitsConsumed",
];
const SERVER_MESSAGES: &[ErrorCode] = &[
    ErrorCode::JSON_RPC_INTERNAL_ERROR,
    ErrorCode::JSON_RPC_INVALID_PARAMS,
    ErrorCode::JSON_RPC_INVALID_REQUEST,
    ErrorCode::JSON_RPC_METHOD_NOT_FOUND,
    ErrorCode::JSON_RPC_PARSE_ERROR,
    ErrorCode::JSON_RPC_SCAN_ERROR,
    ErrorCode::JSON_RPC_SERVER_ERROR_BLOCK_CLEANED_UP,
    ErrorCode::JSON_RPC_SERVER_ERROR_BLOCK_NOT_AVAILABLE,
    ErrorCode::JSON_RPC_SERVER_ERROR_BLOCK_STATUS_NOT_AVAILABLE_YET,
    ErrorCode::JSON_RPC_SERVER_ERROR_KEY_EXCLUDED_FROM_SECONDARY_INDEX,
    ErrorCode::JSON_RPC_SERVER_ERROR_LONG_TERM_STORAGE_SLOT_SKIPPED,
    ErrorCode::JSON_RPC_SERVER_ERROR_SLOT_SKIPPED,
    ErrorCode::JSON_RPC_SERVER_ERROR_TRANSACTION_PRECOMPILE_VERIFICATION_FAILURE,
    ErrorCode::JSON_RPC_SERVER_ERROR_UNSUPPORTED_TRANSACTION_VERSION,
];

impl RemoteError {
    /// Projects bounded public metadata and preserves exact signed unknown codes.
    /// Preflight `err` becomes a typed execution cause; absent result fields are null.
    pub fn into_diagnostic(self) -> Result<Diagnostic, Error> {
        let code = ErrorCode::from_value(self.code);
        let mut context = Context::default();
        let mut cause = None;

        if SERVER_MESSAGES.contains(&code) {
            context
                .insert("__serverMessage", ContextValue::Text(self.message.clone()))
                .map_err(Error::DiagnosticContext)?;
        } else if let Presence::Present(data) = &self.data {
            let value: serde_json::Value = serde_json::from_str(data.get())?;
            project_data(value, code, &mut context, &mut cause)?;
        }

        if code == ErrorCode::JSON_RPC_SERVER_ERROR_SEND_TRANSACTION_PREFLIGHT_FAILURE {
            add_preflight_defaults(&mut context)?;
        }

        let diagnostic = Diagnostic::new(code, context);
        Ok(match cause {
            Some(cause) => diagnostic.with_source(cause),
            None => diagnostic.with_origin(self),
        })
    }
}

fn add_preflight_defaults(context: &mut Context) -> Result<(), Error> {
    for key in PREFLIGHT_KEYS {
        if context.get(key).is_none() {
            context
                .insert(key, ContextValue::Null)
                .map_err(Error::DiagnosticContext)?;
        }
    }

    Ok(())
}

/// RPC data is normally opaque; preflight alone extracts `err` as an execution cause.
fn project_data(
    value: serde_json::Value,
    code: ErrorCode,
    context: &mut Context,
    cause: &mut Option<Diagnostic>,
) -> Result<(), Error> {
    let preflight = code == ErrorCode::JSON_RPC_SERVER_ERROR_SEND_TRANSACTION_PREFLIGHT_FAILURE;
    let serde_json::Value::Object(values) = value else {
        if preflight && !value.is_null() {
            return Err(Error::Envelope);
        }

        return Ok(());
    };

    for (key, value) in values {
        if preflight && key == "err" {
            *cause = execution_cause(value)?;
            continue;
        }

        context
            .insert(&key, from_json(value, 0).map_err(Error::DiagnosticContext)?)
            .map_err(Error::DiagnosticContext)?;
    }

    Ok(())
}

fn execution_cause(value: serde_json::Value) -> Result<Option<Diagnostic>, Error> {
    if value.is_null() {
        return Ok(None);
    }

    transaction_error(&value.to_string()).map(Some)
}
