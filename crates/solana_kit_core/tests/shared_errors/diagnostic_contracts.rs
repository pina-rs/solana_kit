//! Executable proof for the pinned `SolanaError` diagnostic contract.
//!
//! Storage, accessors, display identity, source retention, equality and the
//! simulation unwrap boundary are exercised through the public core API.

use solana_kit_core::{
    codecs,
    errors::{Context, ContextError, ContextValue, Diagnostic, ErrorCode, ErrorFacts},
};
use std::{collections::BTreeMap, error::Error as _, format, vec};

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

/// Exercises the class, constructor and both field accessors.
pub fn exact() -> Result<(), Mismatch> {
    check_fields()?;
    check_context_values()?;
    check_display_and_debug()?;
    check_source_and_facts()?;
    check_simulation_boundary()
}

/// The constructor stores the exact code and an immutable context map.
fn check_fields() -> Result<(), Mismatch> {
    let mut context = Context::default();
    context
        .insert("address", ContextValue::Text("A1".into()))
        .map_err(|_| Mismatch::new("SolanaError", "field context was rejected"))?;
    let original = context.clone();
    let diagnostic = Diagnostic::new(ErrorCode::ACCOUNTS_ACCOUNT_NOT_FOUND, context);

    if diagnostic.code() != ErrorCode::ACCOUNTS_ACCOUNT_NOT_FOUND {
        return Err(Mismatch::new("SolanaError.code", "code differs"));
    }

    if diagnostic.context() != &original {
        return Err(Mismatch::new("SolanaError.context", "context differs"));
    }

    if Diagnostic::new(ErrorCode::BLOCK_HEIGHT_EXCEEDED, Context::default())
        .context()
        .iter()
        .count()
        != 0
    {
        return Err(Mismatch::new(
            "SolanaError.new",
            "default context is not empty",
        ));
    }

    Ok(())
}

/// Every public scalar and structured fact survives construction unchanged.
fn check_context_values() -> Result<(), Mismatch> {
    let object = ContextValue::Object(BTreeMap::from([
        ("a".into(), ContextValue::Integer(1)),
        ("b".into(), ContextValue::Null),
    ]));
    let values = [
        ContextValue::Null,
        ContextValue::Integer(i64::MIN.into()),
        ContextValue::Unsigned(u64::MAX.into()),
        ContextValue::Boolean(true),
        ContextValue::Text("text".into()),
        ContextValue::List(vec![ContextValue::Integer(1), ContextValue::Null]),
        object,
    ];
    let mut context = Context::default();

    for (index, value) in values.into_iter().enumerate() {
        let key = format!("field{index}");
        context
            .insert(&key, value.clone())
            .map_err(|_| Mismatch::new("SolanaError.context", "fact was rejected"))?;

        if context.get(&key) != Some(&value) {
            return Err(Mismatch::new(
                "SolanaError.context",
                "structured fact changed",
            ));
        }
    }

    let diagnostic = Diagnostic::new(ErrorCode::BLOCK_HEIGHT_EXCEEDED, context.clone());

    if diagnostic.context() != &context {
        return Err(Mismatch::new(
            "SolanaError.context",
            "constructor does not retain every fact",
        ));
    }

    if context.normalized().get("field0").is_some() {
        return Err(Mismatch::new(
            "SolanaError.context",
            "null fact was not dropped",
        ));
    }

    Ok(())
}

/// `toString` exposes the same `SolanaError#<value>` prefix for known codes.
fn check_display_and_debug() -> Result<(), Mismatch> {
    let mut context = Context::default();
    context
        .insert("address", ContextValue::Text("A1".into()))
        .map_err(|_| Mismatch::new("SolanaError.toString", "context rejected"))?;
    let diagnostic = Diagnostic::new(ErrorCode::ACCOUNTS_ACCOUNT_NOT_FOUND, context);
    let text = format!("{diagnostic}");

    if text != "SolanaError#3230000" {
        return Err(Mismatch::new(
            "SolanaError.toString",
            "display identity differs",
        ));
    }

    let unknown = format!(
        "{}",
        Diagnostic::new(ErrorCode::from_value(i64::MAX), Context::default())
    );

    if unknown != "SolanaError#9223372036854775807" {
        return Err(Mismatch::new(
            "SolanaError.toString",
            "unknown display identity differs",
        ));
    }

    if format!("{diagnostic:?}").contains("A1") {
        return Err(Mismatch::new(
            "SolanaError.toString",
            "debug output exposes context",
        ));
    }

    Ok(())
}

/// The original error remains available through the standard source chain.
fn check_source_and_facts() -> Result<(), Mismatch> {
    let cause = Diagnostic::new(ErrorCode::CODECS_INVALID_BOOLEAN, Context::default())
        .with_source(codecs::Error::InvalidBoolean { value: 255 });
    let outer = Diagnostic::new(
        ErrorCode::JSON_RPC_SERVER_ERROR_SEND_TRANSACTION_PREFLIGHT_FAILURE,
        Context::default(),
    )
    .with_source(cause);

    if outer.error_code()
        != Some(ErrorCode::JSON_RPC_SERVER_ERROR_SEND_TRANSACTION_PREFLIGHT_FAILURE)
    {
        return Err(Mismatch::new(
            "isSolanaError",
            "diagnostic facts do not expose its code",
        ));
    }

    let source = outer
        .source()
        .and_then(|error| error.downcast_ref::<Diagnostic>())
        .ok_or(Mismatch::new(
            "isSolanaError",
            "typed source is not retained",
        ))?;

    if source.error_code() != Some(ErrorCode::CODECS_INVALID_BOOLEAN) {
        return Err(Mismatch::new(
            "isSolanaError",
            "nested diagnostic code differs",
        ));
    }

    if source
        .source()
        .and_then(|error| error.downcast_ref::<codecs::Error>())
        != Some(&codecs::Error::InvalidBoolean { value: 255 })
    {
        return Err(Mismatch::new(
            "isSolanaError",
            "leaf source is not retained",
        ));
    }

    let origin = Diagnostic::new(ErrorCode::BLOCK_HEIGHT_EXCEEDED, Context::default())
        .with_origin(ContextError::Encoding);

    if origin.source().is_none() {
        return Err(Mismatch::new("isSolanaError", "origin is not retained"));
    }

    Ok(())
}

/// Only the three pinned simulation wrappers expose one typed cause level.
fn check_simulation_boundary() -> Result<(), Mismatch> {
    for value in [-32002, 5663019, 5663037] {
        let cause = Diagnostic::new(ErrorCode::ACCOUNTS_ACCOUNT_NOT_FOUND, Context::default());
        let outer =
            Diagnostic::new(ErrorCode::from_value(value), Context::default()).with_source(cause);
        let unwrapped = outer
            .simulation_cause()
            .downcast_ref::<Diagnostic>()
            .ok_or(Mismatch::new(
                "unwrapSimulationError",
                "simulation cause is not exposed",
            ))?;

        if unwrapped.code() != ErrorCode::ACCOUNTS_ACCOUNT_NOT_FOUND {
            return Err(Mismatch::new(
                "unwrapSimulationError",
                "simulation cause code differs",
            ));
        }

        let none = Diagnostic::new(ErrorCode::from_value(value), Context::default());

        if !same(none.simulation_cause(), &none) {
            return Err(Mismatch::new(
                "unwrapSimulationError",
                "absent cause was replaced",
            ));
        }
    }

    let non_simulation = Diagnostic::new(ErrorCode::BLOCK_HEIGHT_EXCEEDED, Context::default())
        .with_source(ContextError::Encoding);

    if !same(non_simulation.simulation_cause(), &non_simulation) {
        return Err(Mismatch::new(
            "unwrapSimulationError",
            "non-simulation error was unwrapped",
        ));
    }

    let origin = Diagnostic::new(
        ErrorCode::JSON_RPC_SERVER_ERROR_SEND_TRANSACTION_PREFLIGHT_FAILURE,
        Context::default(),
    )
    .with_origin(ContextError::Encoding);

    if !same(origin.simulation_cause(), &origin) {
        return Err(Mismatch::new(
            "unwrapSimulationError",
            "boundary origin was treated as a cause",
        ));
    }

    Ok(())
}

/// Compares two trait-object references by address.
fn same(
    left: &(dyn core::error::Error + 'static),
    right: &(dyn core::error::Error + 'static),
) -> bool {
    std::ptr::addr_eq(left, right)
}
