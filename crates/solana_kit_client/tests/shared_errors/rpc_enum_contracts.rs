//! Portable proof for the pinned ordered RPC enum mapping.

use solana_kit_client::diagnostics::{RpcEnumError, RpcEnumErrorConfig, rpc_enum_error};
use solana_kit_core::errors::{Context, ContextError, ContextValue, ErrorCode};
use std::{borrow::ToOwned, error::Error as _};

const GENERIC_BASE: ErrorCode = ErrorCode::INSTRUCTION_ERROR_GENERIC_ERROR;
const UNDEFINED_BASE: i64 = 9_901_007;
const PAYLOAD: i64 = 255;

/// The first failed comparison.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Mismatch {
    /// Contract section that failed.
    pub name: &'static str,
    /// Stable explanation.
    pub field: &'static str,
}

impl Mismatch {
    const fn new(name: &'static str, field: &'static str) -> Self {
        Self { name, field }
    }
}

/// The ordered RPC enum config maps offsets and invokes its callback once.
pub fn rpc_enum() -> Result<(), Mismatch> {
    check_string_mapping()?;
    check_map_payload()?;
    check_null_payload()?;
    check_fallback_names()?;
    check_invalid_shapes()
}

/// A string name maps to its ordered offset and retains the raw input.
fn check_string_mapping() -> Result<(), Mismatch> {
    let mut calls = 0;
    let diagnostic = rpc_enum_error(
        config(),
        "\"AccountLoadedTwice\"",
        |code, _name, payload| {
            calls += 1;
            expected(code, ErrorCode::TRANSACTION_ERROR_ACCOUNT_LOADED_TWICE)?;

            if payload.is_some() {
                return Err(ContextError::Number);
            }

            Ok(Context::default())
        },
    )
    .map_err(|_| Mismatch::new("getSolanaErrorFromRpcError", "string mapping failed"))?;

    if calls != 1 {
        return Err(Mismatch::new(
            "RpcEnumErrorConfig.getErrorContext",
            "callback was not invoked once",
        ));
    }

    if diagnostic.code() != ErrorCode::TRANSACTION_ERROR_ACCOUNT_LOADED_TWICE {
        return Err(Mismatch::new(
            "RpcEnumErrorConfig.errorCodeBaseOffset",
            "ordered offset differs",
        ));
    }

    let raw = diagnostic
        .source()
        .and_then(|error| error.downcast_ref::<RpcEnumError>())
        .ok_or(Mismatch::new(
            "getSolanaErrorFromRpcError",
            "raw enum input is not retained",
        ))?;

    if raw.raw() != "\"AccountLoadedTwice\"" {
        return Err(Mismatch::new(
            "getSolanaErrorFromRpcError",
            "raw enum input differs",
        ));
    }

    Ok(())
}

/// A one-key map's integer payload reaches the callback as a typed fact.
fn check_map_payload() -> Result<(), Mismatch> {
    let mut observed = None;
    let mapped = rpc_enum_error(config(), r#"{"AccountInUse":255}"#, |_, name, payload| {
        observed = Some((name.to_owned(), payload));

        Ok(Context::default())
    })
    .map_err(|_| Mismatch::new("getSolanaErrorFromRpcError", "map mapping failed"))?;

    if mapped.code() != ErrorCode::TRANSACTION_ERROR_ACCOUNT_IN_USE {
        return Err(Mismatch::new(
            "RpcEnumErrorConfig.orderedErrorNames",
            "first ordered name differs",
        ));
    }

    let expected = (
        "AccountInUse".to_owned(),
        Some(ContextValue::Integer(PAYLOAD.into())),
    );

    if observed != Some(expected) {
        return Err(Mismatch::new(
            "RpcEnumErrorConfig.getErrorContext",
            "typed payload differs",
        ));
    }

    Ok(())
}

/// An explicit JSON null is absent for the callback, as in the source.
fn check_null_payload() -> Result<(), Mismatch> {
    let mut observed = None;
    rpc_enum_error(config(), r#"{"AccountInUse":null}"#, |_, _, payload| {
        observed = Some(payload);

        Ok(Context::default())
    })
    .map_err(|_| Mismatch::new("getSolanaErrorFromRpcError", "null payload failed"))?;

    if !matches!(observed, Some(None)) {
        return Err(Mismatch::new(
            "RpcEnumErrorConfig.getErrorContext",
            "explicit null payload is not absent",
        ));
    }

    Ok(())
}

/// Unknown names and scalar inputs use the base-minus-one fallback.
fn check_fallback_names() -> Result<(), Mismatch> {
    check_fallback("\"Future\"", "Future")?;
    check_fallback("42", "42")
}

/// Maps one fallback input and requires the unknown code and name.
fn check_fallback(json: &str, name: &str) -> Result<(), Mismatch> {
    let mapped = rpc_enum_error(config(), json, |code, observed, _| {
        expected(code, ErrorCode::TRANSACTION_ERROR_UNKNOWN)?;

        if observed != name {
            return Err(ContextError::Number);
        }

        Ok(Context::default())
    })
    .map_err(|_| Mismatch::new("getSolanaErrorFromRpcError", "fallback failed"))?;

    if mapped.code() != ErrorCode::TRANSACTION_ERROR_UNKNOWN {
        return Err(Mismatch::new(
            "getSolanaErrorFromRpcError",
            "fallback code differs",
        ));
    }

    Ok(())
}

/// Multiple-key maps and undefined offsets are explicit failures.
fn check_invalid_shapes() -> Result<(), Mismatch> {
    let multikey = RpcEnumErrorConfig {
        base: GENERIC_BASE,
        names: &["GenericError"],
    };

    if rpc_enum_error(multikey, r#"{"GenericError":1,"Other":2}"#, |_, _, _| {
        Ok(Context::default())
    })
    .is_ok()
    {
        return Err(Mismatch::new(
            "getSolanaErrorFromRpcError",
            "multiple keys were accepted",
        ));
    }

    let undefined = RpcEnumErrorConfig {
        base: ErrorCode::from_value(UNDEFINED_BASE),
        names: &["Undefined"],
    };

    if rpc_enum_error(undefined, "\"Undefined\"", |_, _, _| Ok(Context::default())).is_ok() {
        return Err(Mismatch::new(
            "getSolanaErrorFromRpcError",
            "undefined offset was accepted",
        ));
    }

    Ok(())
}

/// The committed two-name transaction config.
fn config() -> RpcEnumErrorConfig<'static> {
    RpcEnumErrorConfig {
        base: ErrorCode::TRANSACTION_ERROR_ACCOUNT_IN_USE,
        names: &["AccountInUse", "AccountLoadedTwice"],
    }
}

/// Requires the callback's code to match the pinned mapping.
fn expected(code: ErrorCode, wanted: ErrorCode) -> Result<(), ContextError> {
    if code != wanted {
        return Err(ContextError::Number);
    }

    Ok(())
}
