//! Portable proof for the pinned JSON-RPC and simulation projections.

use solana_kit_client::{RemoteError, diagnostics::json_rpc_error};
use solana_kit_core::errors::{ContextValue, Diagnostic, ErrorCode};
use std::{error::Error as _, format};

const PREFLIGHT_CODE: i64 = -32_002;
const SERVER_CODE: i64 = -32_603;
const RPC_CODE: i64 = 8_100_000;
const PREFLIGHT_DEFAULTS: usize = 13;
const CUSTOM_PAYLOAD: u128 = 99;

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

/// JSON-RPC projection follows the pinned preflight, server and data rules.
pub fn json_rpc() -> Result<(), Mismatch> {
    check_preflight()?;
    check_empty_preflight()?;
    check_server_message()?;
    check_data_projection()?;
    check_unknown_codes()?;
    check_invalid_envelopes()?;
    check_origin()
}

/// Preflight projects 13 metadata fields and a typed execution cause.
fn check_preflight() -> Result<(), Mismatch> {
    let input = format!(
        r#"{{"code":{PREFLIGHT_CODE},"message":"simulation","data":{{"err":{{"InstructionError":[2,{{"Custom":{CUSTOM_PAYLOAD}}}]}},"fee":5000,"preTokenBalances":["public"],"logs":[]}}}}"#
    );
    let preflight = json_rpc_error(&input)
        .map_err(|_| Mismatch::new("getSolanaErrorFromJsonRpcError", "preflight failed"))?;

    if preflight.code() != ErrorCode::JSON_RPC_SERVER_ERROR_SEND_TRANSACTION_PREFLIGHT_FAILURE {
        return Err(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "preflight code differs",
        ));
    }

    if preflight.context().iter().count() != PREFLIGHT_DEFAULTS {
        return Err(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "preflight field count differs",
        ));
    }

    check_preflight_fields(&preflight)?;
    check_preflight_cause(&preflight)
}

/// Retained preflight fields and null defaults are exact.
fn check_preflight_fields(preflight: &Diagnostic) -> Result<(), Mismatch> {
    if preflight.context().get("fee") != Some(&ContextValue::Integer(5000)) {
        return Err(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "preflight fee differs",
        ));
    }

    if preflight.context().get("returnData") != Some(&ContextValue::Null) {
        return Err(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "preflight null default differs",
        ));
    }

    if preflight.context().get("err").is_some() {
        return Err(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "preflight err was not projected away",
        ));
    }

    Ok(())
}

/// The preflight `err` becomes a typed diagnostic and retains its source.
fn check_preflight_cause(preflight: &Diagnostic) -> Result<(), Mismatch> {
    let cause = preflight
        .simulation_cause()
        .downcast_ref::<Diagnostic>()
        .ok_or(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "preflight cause is not typed",
        ))?;

    if cause.code() != ErrorCode::INSTRUCTION_ERROR_CUSTOM {
        return Err(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "preflight cause code differs",
        ));
    }

    if cause.context().get("code") != Some(&ContextValue::Unsigned(CUSTOM_PAYLOAD)) {
        return Err(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "preflight cause payload differs",
        ));
    }

    if cause.source().is_none() {
        return Err(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "preflight cause lost its execution source",
        ));
    }

    Ok(())
}

/// A preflight error without data still carries every null default.
fn check_empty_preflight() -> Result<(), Mismatch> {
    let input = format!(r#"{{"code":{PREFLIGHT_CODE},"message":"simulation"}}"#);
    let empty = json_rpc_error(&input)
        .map_err(|_| Mismatch::new("getSolanaErrorFromJsonRpcError", "empty failed"))?;

    if empty.context().iter().count() != PREFLIGHT_DEFAULTS
        || !same(empty.simulation_cause(), &empty)
    {
        return Err(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "preflight defaults or absent cause differ",
        ));
    }

    Ok(())
}

/// Server-side message codes replace the whole data object with the message.
fn check_server_message() -> Result<(), Mismatch> {
    let input =
        format!(r#"{{"code":{SERVER_CODE},"message":"server detail","data":{{"discarded":1}}}}"#);
    let server = json_rpc_error(&input)
        .map_err(|_| Mismatch::new("getSolanaErrorFromJsonRpcError", "server failed"))?;

    if server.context().get("__serverMessage") != Some(&ContextValue::Text("server detail".into()))
    {
        return Err(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "server message projection differs",
        ));
    }

    if server.context().get("discarded").is_some() {
        return Err(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "server data was projected",
        ));
    }

    Ok(())
}

/// Ordinary object data is projected; non-object data becomes empty context.
fn check_data_projection() -> Result<(), Mismatch> {
    let input = format!(r#"{{"code":{RPC_CODE},"message":"rpc","data":{{"a":1}}}}"#);
    let data = json_rpc_error(&input)
        .map_err(|_| Mismatch::new("getSolanaErrorFromJsonRpcError", "data failed"))?;

    if data.context().get("a") != Some(&ContextValue::Integer(1)) {
        return Err(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "data map projection differs",
        ));
    }

    let input = format!(r#"{{"code":{RPC_CODE},"message":"rpc","data":[1]}}"#);
    let list = json_rpc_error(&input)
        .map_err(|_| Mismatch::new("getSolanaErrorFromJsonRpcError", "list failed"))?;

    if list.context().iter().next().is_some() {
        return Err(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "non-object data was projected",
        ));
    }

    Ok(())
}

/// Unknown signed codes and extreme integers are preserved exactly.
fn check_unknown_codes() -> Result<(), Mismatch> {
    for value in [i64::MIN, PREFLIGHT_CODE, 8_078_999, i64::MAX] {
        let json = format!(r#"{{"code":{value},"message":"opaque","data":{{"future":true}}}}"#);
        let diagnostic = json_rpc_error(&json)
            .map_err(|_| Mismatch::new("getSolanaErrorFromJsonRpcError", "opaque failed"))?;

        if diagnostic.code().value() != value {
            return Err(Mismatch::new(
                "getSolanaErrorFromJsonRpcError",
                "unknown signed code was not preserved",
            ));
        }

        if format!("{diagnostic:?}").contains("opaque") {
            return Err(Mismatch::new(
                "getSolanaErrorFromJsonRpcError",
                "debug output exposes the server message",
            ));
        }
    }

    Ok(())
}

/// Fractional, overflowing, missing and wrongly typed envelopes fail.
fn check_invalid_envelopes() -> Result<(), Mismatch> {
    for json in [
        r#"{"code":1.5,"message":"bad"}"#,
        r#"{"code":9223372036854775808,"message":"bad"}"#,
        r#"{"code":true,"message":"bad"}"#,
        r#"{"code":1}"#,
        r#"[]"#,
    ] {
        if json_rpc_error(json).is_ok() {
            return Err(Mismatch::new(
                "getSolanaErrorFromJsonRpcError",
                "invalid envelope was accepted",
            ));
        }
    }

    Ok(())
}

/// A non-preflight diagnostic retains the original boundary error.
fn check_origin() -> Result<(), Mismatch> {
    let input = format!(r#"{{"code":{RPC_CODE},"message":"rpc"}}"#);
    let origin = json_rpc_error(&input)
        .map_err(|_| Mismatch::new("getSolanaErrorFromJsonRpcError", "origin failed"))?;
    let remote = origin
        .source()
        .and_then(|error| error.downcast_ref::<RemoteError>())
        .ok_or(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "boundary origin is not retained",
        ))?;

    if remote.code != RPC_CODE || remote.message != "rpc" {
        return Err(Mismatch::new(
            "getSolanaErrorFromJsonRpcError",
            "boundary origin facts differ",
        ));
    }

    Ok(())
}

/// Compares two trait-object references by address.
fn same(
    left: &(dyn std::error::Error + 'static),
    right: &(dyn std::error::Error + 'static),
) -> bool {
    std::ptr::addr_eq(left, right)
}
