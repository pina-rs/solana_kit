//! Portable proof for the pinned instruction and transaction execution mapping.

use solana_kit_client::{
    diagnostics::{instruction_error, transaction_error},
    methods::submission::TransactionFailure,
};
use solana_kit_core::errors::{ContextValue, ErrorCode};
use std::{
    borrow::ToOwned,
    collections::BTreeMap,
    error::Error as _,
    format,
    string::{String, ToString},
    vec::Vec,
};

const EXECUTION_ROWS: usize = 93;
const MAX_U32: i64 = 4_294_967_295;
const MAX_INDEX: u8 = 255;

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

/// Every independent execution literal still maps to the exact code and facts.
pub fn execution(fixture: &str) -> Result<(), Mismatch> {
    check_execution_rows(fixture)?;
    instruction_boundaries()?;
    transaction_boundaries()
}

/// Compares each row's code and stringified context with the fixture.
fn check_execution_rows(fixture: &str) -> Result<(), Mismatch> {
    let mut rows = 0;

    for row in fixture.lines() {
        check_execution_row(row)?;
        rows += 1;
    }

    if rows != EXECUTION_ROWS {
        return Err(Mismatch::new(
            "transaction_error",
            "execution oracle row count is not 93",
        ));
    }

    Ok(())
}

/// Compares one three-field execution row.
fn check_execution_row(row: &str) -> Result<(), Mismatch> {
    let fields: Vec<_> = row.split('\t').collect();

    if fields.len() != 3 {
        return Err(Mismatch::new(
            "transaction_error",
            "execution row does not have three fields",
        ));
    }

    let diagnostic = transaction_error(fields[0])
        .map_err(|_| Mismatch::new("transaction_error", "execution input was rejected"))?;
    let expected: i64 = fields[1]
        .parse()
        .map_err(|_| Mismatch::new("transaction_error", "execution code is not an integer"))?;

    if diagnostic.code().value() != expected {
        return Err(Mismatch::new("transaction_error", "execution code differs"));
    }

    let expected: BTreeMap<String, String> = serde_json::from_str(fields[2])
        .map_err(|_| Mismatch::new("transaction_error", "execution facts are not JSON"))?;
    let actual: BTreeMap<_, _> = diagnostic
        .context()
        .iter()
        .map(|(key, value)| (key.to_owned(), value.to_string()))
        .collect();

    if actual != expected {
        return Err(Mismatch::new("transaction_error", "execution facts differ"));
    }

    Ok(())
}

/// The instruction helper keeps source names, payloads and typed limits.
fn instruction_boundaries() -> Result<(), Mismatch> {
    check_instruction_scalar()?;
    check_instruction_payloads()?;
    check_instruction_custom()?;
    check_instruction_rejections()
}

/// A scalar input becomes the unknown instruction name with its index.
fn check_instruction_scalar() -> Result<(), Mismatch> {
    let future = instruction_error(0, "42")
        .map_err(|_| Mismatch::new("getSolanaErrorFromInstructionError", "scalar failed"))?;

    if future.code() != ErrorCode::INSTRUCTION_ERROR_UNKNOWN
        || future.context().get("errorName") != Some(&ContextValue::Text("42".into()))
    {
        return Err(Mismatch::new(
            "getSolanaErrorFromInstructionError",
            "scalar future name differs",
        ));
    }

    Ok(())
}

/// A future payload is retained and an explicit null is omitted.
fn check_instruction_payloads() -> Result<(), Mismatch> {
    let payload = instruction_error(3, r#"{"Future":5}"#)
        .map_err(|_| Mismatch::new("getSolanaErrorFromInstructionError", "payload failed"))?;

    if payload.context().get("instructionErrorContext") != Some(&ContextValue::Integer(5)) {
        return Err(Mismatch::new(
            "getSolanaErrorFromInstructionError",
            "future instruction payload differs",
        ));
    }

    let null_payload = instruction_error(0, r#"{"Future":null}"#)
        .map_err(|_| Mismatch::new("getSolanaErrorFromInstructionError", "null failed"))?;

    if null_payload
        .context()
        .get("instructionErrorContext")
        .is_some()
    {
        return Err(Mismatch::new(
            "getSolanaErrorFromInstructionError",
            "null instruction payload was retained",
        ));
    }

    Ok(())
}

/// A custom payload becomes the unsigned code with the instruction index.
fn check_instruction_custom() -> Result<(), Mismatch> {
    let json = format!(r#"{{"Custom":{MAX_U32}}}"#);
    let custom = instruction_error(0, &json)
        .map_err(|_| Mismatch::new("getSolanaErrorFromInstructionError", "custom failed"))?;

    if custom.code() != ErrorCode::INSTRUCTION_ERROR_CUSTOM {
        return Err(Mismatch::new(
            "getSolanaErrorFromInstructionError",
            "custom code projection differs",
        ));
    }

    if custom.context().get("code") != Some(&ContextValue::Unsigned(u32::MAX.into())) {
        return Err(Mismatch::new(
            "getSolanaErrorFromInstructionError",
            "custom payload differs",
        ));
    }

    check_instruction_index(custom.context().get("index"))
}

/// The custom instruction index is retained as an unsigned fact.
fn check_instruction_index(index: Option<&ContextValue>) -> Result<(), Mismatch> {
    if index != Some(&ContextValue::Unsigned(0)) {
        return Err(Mismatch::new(
            "getSolanaErrorFromInstructionError",
            "custom instruction index differs",
        ));
    }

    Ok(())
}

/// Non-u32 custom payloads and a non-u8 index are explicit failures.
fn check_instruction_rejections() -> Result<(), Mismatch> {
    for json in [
        r#"{"Custom":4294967296}"#,
        r#"{"Custom":"seven"}"#,
        r#"{"Custom":null}"#,
    ] {
        if instruction_error(0, json).is_ok() {
            return Err(Mismatch::new(
                "getSolanaErrorFromInstructionError",
                "non-u32 custom payload was accepted",
            ));
        }
    }

    if instruction_error(3, r#"{"BorshIoError":"opaque"}"#)
        .map_err(|_| Mismatch::new("getSolanaErrorFromInstructionError", "borsh failed"))?
        .context()
        .get("index")
        != Some(&ContextValue::Unsigned(3))
    {
        return Err(Mismatch::new(
            "getSolanaErrorFromInstructionError",
            "known instruction index differs",
        ));
    }

    Ok(())
}

/// The transaction helper keeps nested indices and strict protocol integers.
fn transaction_boundaries() -> Result<(), Mismatch> {
    check_nested_instruction()?;
    check_transaction_indices()?;
    check_transaction_future()?;
    check_transaction_rejections()
}

/// A nested instruction error maps to the custom code and unsigned index.
fn check_nested_instruction() -> Result<(), Mismatch> {
    let json = format!(r#"{{"InstructionError":[{MAX_INDEX},{{"Custom":{MAX_U32}}}]}}"#);
    let nested = transaction_error(&json)
        .map_err(|_| Mismatch::new("getSolanaErrorFromTransactionError", "nested failed"))?;

    if nested.code() != ErrorCode::INSTRUCTION_ERROR_CUSTOM {
        return Err(Mismatch::new(
            "getSolanaErrorFromTransactionError",
            "nested code differs",
        ));
    }

    if nested.context().get("index") != Some(&ContextValue::Unsigned(MAX_INDEX.into())) {
        return Err(Mismatch::new(
            "getSolanaErrorFromTransactionError",
            "nested instruction index differs",
        ));
    }

    Ok(())
}

/// Duplicate-instruction and rent account indices keep their exact widths.
fn check_transaction_indices() -> Result<(), Mismatch> {
    let duplicate = transaction_error(r#"{"DuplicateInstruction":255}"#)
        .map_err(|_| Mismatch::new("getSolanaErrorFromTransactionError", "duplicate failed"))?;

    if duplicate.context().get("index") != Some(&ContextValue::Unsigned(MAX_INDEX.into())) {
        return Err(Mismatch::new(
            "getSolanaErrorFromTransactionError",
            "duplicate instruction index differs",
        ));
    }

    let rent = transaction_error(r#"{"InsufficientFundsForRent":{"account_index":255}}"#)
        .map_err(|_| Mismatch::new("getSolanaErrorFromTransactionError", "rent failed"))?;

    if rent.context().get("accountIndex") != Some(&ContextValue::Unsigned(MAX_INDEX.into())) {
        return Err(Mismatch::new(
            "getSolanaErrorFromTransactionError",
            "rent account index differs",
        ));
    }

    Ok(())
}

/// A future transaction payload is retained with its raw source.
fn check_transaction_future() -> Result<(), Mismatch> {
    let future = transaction_error(r#"{"Future":9223372036854775807}"#)
        .map_err(|_| Mismatch::new("getSolanaErrorFromTransactionError", "future failed"))?;

    if future.code() != ErrorCode::TRANSACTION_ERROR_UNKNOWN {
        return Err(Mismatch::new(
            "getSolanaErrorFromTransactionError",
            "future code differs",
        ));
    }

    if future.context().get("transactionErrorContext")
        != Some(&ContextValue::Integer(i64::MAX.into()))
    {
        return Err(Mismatch::new(
            "getSolanaErrorFromTransactionError",
            "future payload differs",
        ));
    }

    let raw = future
        .source()
        .and_then(|error| error.downcast_ref::<TransactionFailure>())
        .and_then(|failure| match failure {
            TransactionFailure::Unknown(raw) => Some(raw.get()),
            _ => None,
        });

    if raw != Some(r#"{"Future":9223372036854775807}"#) {
        return Err(Mismatch::new(
            "getSolanaErrorFromTransactionError",
            "future raw payload differs",
        ));
    }

    Ok(())
}

/// Invalid protocol shapes and out-of-range indices are explicit failures.
fn check_transaction_rejections() -> Result<(), Mismatch> {
    for json in [
        r#"{"InstructionError":[256,"GenericError"]}"#,
        r#"{"InstructionError":[0,{"Custom":4294967296}]}"#,
        r#"{"DuplicateInstruction":-1}"#,
        r#"{"AccountInUse":1,"AccountNotFound":2}"#,
        "{}",
    ] {
        if transaction_error(json).is_ok() {
            return Err(Mismatch::new(
                "getSolanaErrorFromTransactionError",
                "invalid execution shape was accepted",
            ));
        }
    }

    Ok(())
}

/// Proves one corrupted expected code fails before the oracle is accepted.
pub fn rejects_wrong_execution(fixture: &str) -> Result<(), Mismatch> {
    execution(fixture)?;
    let corrupted = corrupt_last_row(fixture)?;

    match execution(&corrupted) {
        Ok(()) => Err(Mismatch::new(
            "transaction_error",
            "wrong execution code was accepted",
        )),
        Err(mismatch) if mismatch.field == "execution code differs" => {
            execution(fixture)?;

            Ok(())
        }
        Err(_) => Err(Mismatch::new(
            "transaction_error",
            "wrong execution code failed for the wrong reason",
        )),
    }
}

/// Replaces the final expected code with a different integer.
fn corrupt_last_row(fixture: &str) -> Result<String, Mismatch> {
    let rows: Vec<_> = fixture.lines().collect();
    let last = *rows.last().ok_or(Mismatch::new(
        "transaction_error",
        "execution fixture is empty",
    ))?;
    let fields: Vec<_> = last.split('\t').collect();

    if fields.len() != 3 {
        return Err(Mismatch::new(
            "transaction_error",
            "execution row does not have three fields",
        ));
    }

    let value: i64 = fields[1]
        .parse()
        .map_err(|_| Mismatch::new("transaction_error", "execution code is not an integer"))?;
    let wrong = format!("{}\t{}\t{}", fields[0], value ^ 1, fields[2]);
    let mut corrupted = rows[..rows.len() - 1].join("\n");
    corrupted.push('\n');
    corrupted.push_str(&wrong);

    Ok(corrupted)
}
