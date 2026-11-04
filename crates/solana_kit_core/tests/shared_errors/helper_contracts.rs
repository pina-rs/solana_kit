//! Executable proof for the pinned error-context helper conventions.
//!
//! Key literals, null dropping, cause retention and the wrap/create constructors
//! are exercised through the public core diagnostic API.

use solana_kit_core::errors::{Context, ContextError, ContextValue, Diagnostic, ErrorCode};
use std::{collections::BTreeMap, error::Error as _};

/// The source helper's `helper.json` facts, kept as the same literal.
pub const HELPER_FACTS: &str = r#"{"operation":"testOperation","hasPath":false,"causeCode":1,"causeContext":{"slot":55},"causeType":"SolanaError"}"#;

const KEYS: usize = 10;

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

/// Requires each source key literal to be accepted and preserved exactly.
pub fn exact(keys: &str) -> Result<(), Mismatch> {
    check_keys(keys)?;
    check_normalization()?;
    check_constructors()
}

/// The ten `SolanaErrorContextKeys` values are exact public string facts.
fn check_keys(keys: &str) -> Result<(), Mismatch> {
    let mut count = 0;

    for line in keys.lines() {
        if line.is_empty() || line.contains('\t') {
            return Err(Mismatch::new(
                "SolanaErrorContextKeys",
                "context key fixture is malformed",
            ));
        }

        let mut context = Context::default();
        context
            .insert(line, ContextValue::Text("value".into()))
            .map_err(|_| Mismatch::new("SolanaErrorContextKeys", "key literal was rejected"))?;

        if context.get(line) != Some(&ContextValue::Text("value".into())) {
            return Err(Mismatch::new(
                "SolanaErrorContextKeys",
                "key literal was not preserved",
            ));
        }

        count += 1;
    }

    if count != KEYS {
        return Err(Mismatch::new(
            "SolanaErrorContextKeys",
            "context key fixture does not have ten values",
        ));
    }

    Ok(())
}

/// Null top-level facts disappear; non-null facts and typed causes remain.
fn check_normalization() -> Result<(), Mismatch> {
    let wrapped = wrap_example()?;
    check_null_dropped(&wrapped)?;
    check_typed_cause(&wrapped)?;
    check_helper_literal()?;
    check_preset_cause()
}

/// Builds the independent Dart helper example.
fn wrap_example() -> Result<Diagnostic, Mismatch> {
    let mut context = Context::default();
    context
        .insert("operation", ContextValue::Text("testOperation".into()))
        .map_err(|_| Mismatch::new("createSolanaErrorContext", "operation rejected"))?;
    context
        .insert("path", ContextValue::Null)
        .map_err(|_| Mismatch::new("createSolanaErrorContext", "null rejected"))?;
    let mut cause_context = Context::default();
    cause_context
        .insert("slot", ContextValue::Integer(55))
        .map_err(|_| Mismatch::new("createSolanaErrorContext", "cause fact rejected"))?;
    let cause = Diagnostic::new(ErrorCode::BLOCK_HEIGHT_EXCEEDED, cause_context);

    Ok(Diagnostic::wrap(
        ErrorCode::ACCOUNTS_FAILED_TO_DECODE_ACCOUNT,
        context,
        cause,
    ))
}

/// Only the null fact disappears from the outer context.
fn check_null_dropped(wrapped: &Diagnostic) -> Result<(), Mismatch> {
    if wrapped.context().get("path").is_some() {
        return Err(Mismatch::new(
            "createSolanaErrorContext",
            "null fact was retained",
        ));
    }

    if wrapped.context().get("operation") != Some(&ContextValue::Text("testOperation".into())) {
        return Err(Mismatch::new(
            "createSolanaErrorContext",
            "non-null fact was dropped",
        ));
    }

    Ok(())
}

/// The typed cause keeps its exact code and nested context.
fn check_typed_cause(wrapped: &Diagnostic) -> Result<(), Mismatch> {
    let source = wrapped
        .source()
        .and_then(|error| error.downcast_ref::<Diagnostic>())
        .ok_or(Mismatch::new(
            "createSolanaErrorContext",
            "typed cause is not retained",
        ))?;

    if source.code().value() != 1 {
        return Err(Mismatch::new(
            "createSolanaErrorContext",
            "cause code differs",
        ));
    }

    if source.context().get("slot") != Some(&ContextValue::Integer(55)) {
        return Err(Mismatch::new(
            "createSolanaErrorContext",
            "cause context differs",
        ));
    }

    Ok(())
}

/// The independent Dart fixture must remain the reviewed literal.
fn check_helper_literal() -> Result<(), Mismatch> {
    if HELPER_FACTS
        == r#"{"operation":"testOperation","hasPath":false,"causeCode":1,"causeContext":{"slot":55},"causeType":"SolanaError"}"#
    {
        return Ok(());
    }

    Err(Mismatch::new(
        "createSolanaErrorContext",
        "independent source fixture differs",
    ))
}

/// A caller's existing cause fact is not overwritten by a typed cause.
fn check_preset_cause() -> Result<(), Mismatch> {
    let mut preset = Context::default();
    preset
        .insert("cause", ContextValue::Text("caller".into()))
        .map_err(|_| Mismatch::new("createSolanaErrorContext", "preset cause rejected"))?;
    let preserved = Diagnostic::wrap(
        ErrorCode::BLOCK_HEIGHT_EXCEEDED,
        preset,
        Diagnostic::new(ErrorCode::BLOCK_HEIGHT_EXCEEDED, Context::default()),
    );

    if preserved.context().get("cause") != Some(&ContextValue::Text("caller".into())) {
        return Err(Mismatch::new(
            "createSolanaErrorContext",
            "caller cause fact was overwritten",
        ));
    }

    Ok(())
}

/// `createSolanaError` and `wrapSolanaError` retain exact code and context.
fn check_constructors() -> Result<(), Mismatch> {
    let mut raw = Context::default();
    raw.insert("path", ContextValue::Null)
        .map_err(|_| Mismatch::new("createSolanaError", "null rejected"))?;
    raw.insert("operation", ContextValue::Text("testOperation".into()))
        .map_err(|_| Mismatch::new("createSolanaError", "operation rejected"))?;
    let created = Diagnostic::from_context(ErrorCode::ACCOUNTS_FAILED_TO_DECODE_ACCOUNT, raw);

    if created.code() != ErrorCode::ACCOUNTS_FAILED_TO_DECODE_ACCOUNT {
        return Err(Mismatch::new("createSolanaError", "code differs"));
    }

    if created.context().get("path").is_some() {
        return Err(Mismatch::new("createSolanaError", "null fact was retained"));
    }

    let mut nested = Context::default();
    nested
        .insert(
            "object",
            ContextValue::Object(BTreeMap::from([("a".into(), ContextValue::Null)])),
        )
        .map_err(|_| Mismatch::new("createSolanaError", "nested null rejected"))?;

    if created.context().get("operation") != Some(&ContextValue::Text("testOperation".into())) {
        return Err(Mismatch::new(
            "createSolanaError",
            "operation fact was dropped",
        ));
    }

    let cause = Diagnostic::new(ErrorCode::BLOCK_HEIGHT_EXCEEDED, nested);
    let wrapped = Diagnostic::wrap(created.code(), Context::default(), cause);

    if wrapped.source().is_none() {
        return Err(Mismatch::new(
            "wrapSolanaError",
            "required cause is not retained",
        ));
    }

    if Context::default().insert("bad key", ContextValue::Null) != Err(ContextError::Key) {
        return Err(Mismatch::new(
            "createSolanaError",
            "invalid key is not rejected",
        ));
    }

    Ok(())
}
