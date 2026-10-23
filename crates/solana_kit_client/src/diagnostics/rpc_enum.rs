//! Source-equivalent ordered enum mapping with an explicit typed context callback.
use super::context_wire::from_json;
use crate::Error;
use serde_json::{Value, value::RawValue};
use solana_kit_core::errors::{Context, ContextError, ContextValue, Diagnostic, ErrorCode};
use std::{error::Error as StdError, fmt};

/// Ordered source names and the first known code; unknown names use base minus one.
pub struct RpcEnumErrorConfig<'a> {
    /// Stable code for the first ordered name.
    pub base: ErrorCode,
    /// Source order determines each code offset.
    pub names: &'a [&'a str],
}

/// Preserved enum input, available by downcasting a diagnostic's source.
pub struct RpcEnumError {
    raw: Box<RawValue>,
}

impl RpcEnumError {
    /// Exact original JSON, without canonicalizing unknown numeric payloads.
    #[must_use]
    pub fn raw(&self) -> &str {
        self.raw.get()
    }
}
impl fmt::Debug for RpcEnumError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("RpcEnumError(<redacted>)")
    }
}
impl fmt::Display for RpcEnumError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("RPC enum error")
    }
}
impl StdError for RpcEnumError {}

/// Maps a string or single-key object and invokes the callback exactly once.
/// Invalid shapes, unregistered offsets, integer overflow and context limits fail.
pub fn rpc_enum_error(
    config: RpcEnumErrorConfig<'_>,
    json: &str,
    callback: impl FnOnce(ErrorCode, &str, Option<ContextValue>) -> Result<Context, ContextError>,
) -> Result<Diagnostic, Error> {
    let raw = serde_json::from_str::<Box<RawValue>>(json)?;
    let (name, value) = enum_parts(json)?;
    let offset = config
        .names
        .iter()
        .position(|candidate| *candidate == name)
        .map(i64::try_from)
        .transpose()
        .map_err(|_| Error::DiagnosticContext(ContextError::Number))?
        .unwrap_or(-1);
    let code = config
        .base
        .value()
        .checked_add(offset)
        .map(ErrorCode::from_value)
        .filter(|code| code.definition().is_some())
        .ok_or(Error::DiagnosticContext(ContextError::Number))?;
    let context = value
        .map(|value| from_json(value, 0))
        .transpose()
        .map_err(Error::DiagnosticContext)?;
    let context = callback(code, &name, context).map_err(Error::DiagnosticContext)?;

    Ok(Diagnostic::new(code, context).with_source(RpcEnumError { raw }))
}

pub(super) fn enum_parts(json: &str) -> Result<(String, Option<Value>), Error> {
    match serde_json::from_str(json)? {
        Value::String(name) => Ok((name, None)),

        Value::Object(values) if values.len() == 1 => {
            let (name, value) = values.into_iter().next().expect("one validated variant");
            Ok((name, Some(value)))
        }

        Value::Object(_) => Err(Error::Envelope),
        value => Ok((
            from_json(value, 0)
                .map_err(Error::DiagnosticContext)?
                .to_string(),
            None,
        )),
    }
}

pub(super) fn unknown_context(
    json: &str,
    context: &mut Context,
) -> Result<ErrorCode, ContextError> {
    let (mut name, mut value) = enum_parts(json).map_err(|_| ContextError::Encoding)?;
    let mut code = ErrorCode::TRANSACTION_ERROR_UNKNOWN;
    let mut key = "transactionErrorContext";

    if name == "InstructionError" {
        let (index, instruction): (u8, Box<RawValue>) =
            serde_json::from_value(value.take().ok_or(ContextError::Encoding)?)
                .map_err(|_| ContextError::Encoding)?;
        context.insert("index", ContextValue::Unsigned(index.into()))?;
        (name, value) = enum_parts(instruction.get()).map_err(|_| ContextError::Encoding)?;
        code = ErrorCode::INSTRUCTION_ERROR_UNKNOWN;
        key = "instructionErrorContext";
    }

    context.insert("errorName", ContextValue::Text(name))?;

    if let Some(value) = value.filter(|value| !value.is_null()) {
        context.insert(key, from_json(value, 0)?)?;
    }

    Ok(code)
}

/// Adapts a source instruction helper input with a validated protocol index.
/// The generic source callback contract accepts scalar future names as well.
pub fn instruction_error(index: u8, json: &str) -> Result<Diagnostic, Error> {
    let names: Vec<_> = ErrorCode::definitions()
        .filter(|definition| definition.code != ErrorCode::INSTRUCTION_ERROR_UNKNOWN)
        .filter_map(|definition| definition.name.strip_prefix("instructionError"))
        .collect();
    let config = RpcEnumErrorConfig {
        base: ErrorCode::INSTRUCTION_ERROR_GENERIC_ERROR,
        names: &names,
    };

    rpc_enum_error(config, json, |code, name, payload| {
        let mut context = Context::default();
        context.insert("index", ContextValue::Unsigned(index.into()))?;

        if code == ErrorCode::INSTRUCTION_ERROR_UNKNOWN {
            context.insert("errorName", ContextValue::Text(name.into()))?;

            if let Some(payload) = payload.filter(|value| *value != ContextValue::Null) {
                context.insert("instructionErrorContext", payload)?;
            }
        } else if code == ErrorCode::INSTRUCTION_ERROR_CUSTOM {
            let Some(ContextValue::Integer(value)) = payload else {
                return Err(ContextError::Number);
            };
            let value = u32::try_from(value).map_err(|_| ContextError::Number)?;
            context.insert("code", ContextValue::Unsigned(value.into()))?;
        }

        Ok(context)
    })
}
