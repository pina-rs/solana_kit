//! Executable proof for the pinned message templates and interpolation.
//!
//! With an empty context the source formatter reproduces every template byte
//! for byte. Interpolation, unknown variables, structured values and the
//! bounded rendering limit are exercised separately.

use solana_kit_core::errors::{Context, ContextError, ContextValue, ErrorCode};
use std::{collections::BTreeMap, format, string::String, vec, vec::Vec};

const HEX_RADIX: u32 = 16;
const CODES: usize = 365;
const FALLBACK_VALUE: i64 = 9_901_007;
const FALLBACK_MESSAGE: &str = "Solana error #9901007";
const MAX_TEXT: usize = 1024;
const LIST_ITEMS: usize = 4;

/// The first failed comparison.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mismatch {
    /// Pinned constant name or `<messages>`.
    pub name: String,
    /// Stable explanation.
    pub field: &'static str,
}

impl Mismatch {
    fn new(name: impl Into<String>, field: &'static str) -> Self {
        Self {
            name: name.into(),
            field,
        }
    }
}

/// Requires every empty-context rendering to equal its pinned template.
pub fn exact(fixture: &str) -> Result<(), Mismatch> {
    check_templates(fixture)?;
    check_unknown_fallback()?;
    check_interpolation()
}

/// Renders each fixture row and enforces the exact 365-row totals.
fn check_templates(fixture: &str) -> Result<(), Mismatch> {
    let mut rendered = 0;

    for line in fixture.lines() {
        check_template(line)?;
        rendered += 1;
    }

    if rendered != CODES || rendered != ErrorCode::definitions().count() {
        return Err(Mismatch::new(
            "<messages>",
            "rendered template count differs from 365",
        ));
    }

    Ok(())
}

/// Requires one row's empty-context rendering to equal its template bytes.
fn check_template(line: &str) -> Result<(), Mismatch> {
    let fields: Vec<_> = line.split('\t').collect();

    if fields.len() != 4 {
        return Err(Mismatch::new(
            "<messages>",
            "message row does not have four fields",
        ));
    }

    let value: i64 = fields[1]
        .parse()
        .map_err(|_| Mismatch::new(fields[0], "message code is not an integer"))?;
    let expected = decode_template(fields[0], fields[3])?;
    let message = ErrorCode::from_value(value)
        .message(&Context::default())
        .map_err(|_| Mismatch::new(fields[0], "message template does not render"))?;

    if message.as_bytes() != expected.as_slice() {
        return Err(Mismatch::new(fields[0], "message template differs"));
    }

    if message.is_empty() {
        return Err(Mismatch::new(fields[0], "message template is empty"));
    }

    Ok(())
}

/// Decodes the fixture's exact message bytes.
fn decode_template(name: &str, hex: &str) -> Result<Vec<u8>, Mismatch> {
    if !hex.len().is_multiple_of(2) {
        return Err(Mismatch::new(
            name,
            "message template is not even-length hex",
        ));
    }

    let mut template = Vec::new();

    for pair in hex.as_bytes().chunks(2) {
        template.push(decode_byte(name, pair)?);
    }

    Ok(template)
}

/// Decodes one ASCII hex pair into its original byte.
fn decode_byte(name: &str, pair: &[u8]) -> Result<u8, Mismatch> {
    let pair = std::str::from_utf8(pair)
        .map_err(|_| Mismatch::new(name, "message template is not ASCII hex"))?;

    u8::from_str_radix(pair, HEX_RADIX)
        .map_err(|_| Mismatch::new(name, "message template is not hex"))
}

/// The source fallback string is used only for codes with no definition.
fn check_unknown_fallback() -> Result<(), Mismatch> {
    let code = ErrorCode::from_value(FALLBACK_VALUE);

    if code.definition().is_some() {
        return Err(Mismatch::new(
            "<messages>",
            "fallback probe unexpectedly has a definition",
        ));
    }

    let message = code
        .message(&Context::default())
        .map_err(|_| Mismatch::new("<messages>", "fallback message does not render"))?;

    if message != FALLBACK_MESSAGE {
        return Err(Mismatch::new("<messages>", "fallback message differs"));
    }

    Ok(())
}

/// Interpolates every supported public fact shape exactly.
fn check_interpolation() -> Result<(), Mismatch> {
    check_text_template()?;
    check_list_template()?;
    check_object_template()?;
    check_scalars()?;
    check_numeric_template()?;
    check_missing_variables()?;
    check_render_limit()
}

/// Interpolates a text fact into the pinned account message.
fn check_text_template() -> Result<(), Mismatch> {
    let context = context_with("address", ContextValue::Text("A1".into()))?;
    let message = ErrorCode::ACCOUNTS_ACCOUNT_NOT_FOUND
        .message(&context)
        .map_err(|_| Mismatch::new("accountsAccountNotFound", "address message failed"))?;

    if message != "Account not found at address: A1" {
        return Err(Mismatch::new(
            "accountsAccountNotFound",
            "text interpolation differs",
        ));
    }

    Ok(())
}

/// Interpolates a typed list with the source's bracketed display.
fn check_list_template() -> Result<(), Mismatch> {
    let value = ContextValue::List(vec![
        ContextValue::Text("a".into()),
        ContextValue::Text("b".into()),
    ]);
    let context = context_with("addresses", value)?;
    let message = ErrorCode::ACCOUNTS_ONE_OR_MORE_ACCOUNTS_NOT_FOUND
        .message(&context)
        .map_err(|_| Mismatch::new("accountsOneOrMoreAccountsNotFound", "list failed"))?;

    if message != "Accounts not found at addresses: [a, b]" {
        return Err(Mismatch::new(
            "accountsOneOrMoreAccountsNotFound",
            "list interpolation differs",
        ));
    }

    Ok(())
}

/// Interpolates a typed object with deterministic key ordering.
fn check_object_template() -> Result<(), Mismatch> {
    let value = ContextValue::Object(BTreeMap::from([("a".into(), ContextValue::Integer(1))]));
    let context = context_with("value", value)?;
    let message = ErrorCode::CODECS_INVALID_BOOLEAN
        .message(&context)
        .map_err(|_| Mismatch::new("codecsInvalidBoolean", "object message failed"))?;

    if message != "Invalid boolean value. Expected 0 or 1, got {a: 1}." {
        return Err(Mismatch::new(
            "codecsInvalidBoolean",
            "object interpolation differs",
        ));
    }

    Ok(())
}

/// Interpolates null, boolean and integer facts exactly.
fn check_scalars() -> Result<(), Mismatch> {
    for (value, expected) in [
        (
            ContextValue::Null,
            "Invalid boolean value. Expected 0 or 1, got null.",
        ),
        (
            ContextValue::Boolean(true),
            "Invalid boolean value. Expected 0 or 1, got true.",
        ),
        (
            ContextValue::Integer(7),
            "Invalid boolean value. Expected 0 or 1, got 7.",
        ),
    ] {
        let context = context_with("value", value)?;
        let message = ErrorCode::CODECS_INVALID_BOOLEAN
            .message(&context)
            .map_err(|_| Mismatch::new("codecsInvalidBoolean", "scalar message failed"))?;

        if message != expected {
            return Err(Mismatch::new(
                "codecsInvalidBoolean",
                "scalar interpolation differs",
            ));
        }
    }

    Ok(())
}

/// Interpolates the pinned numeric placeholder exactly.
fn check_numeric_template() -> Result<(), Mismatch> {
    let context = context_with("code", ContextValue::Integer(7))?;
    let message = ErrorCode::INSTRUCTION_ERROR_CUSTOM
        .message(&context)
        .map_err(|_| Mismatch::new("instructionErrorCustom", "custom message failed"))?;

    if message != "Custom program error: #7" {
        return Err(Mismatch::new(
            "instructionErrorCustom",
            "numeric interpolation differs",
        ));
    }

    Ok(())
}

/// Missing placeholders stay literal and unrelated facts are ignored.
fn check_missing_variables() -> Result<(), Mismatch> {
    let missing = ErrorCode::INSTRUCTION_ERROR_CUSTOM
        .message(&Context::default())
        .map_err(|_| Mismatch::new("getErrorMessage", "missing variable failed"))?;

    if missing != "Custom program error: #$code" {
        return Err(Mismatch::new(
            "getErrorMessage",
            "missing variable is not retained",
        ));
    }

    let mut context = context_with("code", ContextValue::Integer(7))?;
    context
        .insert("unused", ContextValue::Text("ignored".into()))
        .map_err(|_| Mismatch::new("getErrorMessage", "unused fact rejected"))?;
    let message = ErrorCode::INSTRUCTION_ERROR_CUSTOM
        .message(&context)
        .map_err(|_| Mismatch::new("getErrorMessage", "extra fact failed"))?;

    if message != "Custom program error: #7" {
        return Err(Mismatch::new(
            "getErrorMessage",
            "extra facts changed the message",
        ));
    }

    Ok(())
}

/// The bounded renderer rejects output beyond its public limit.
fn check_render_limit() -> Result<(), Mismatch> {
    let value = ContextValue::List(vec![ContextValue::Text("x".repeat(MAX_TEXT)); LIST_ITEMS]);
    let context = context_with("address", value)?;

    if ErrorCode::ACCOUNTS_ACCOUNT_NOT_FOUND.message(&context) != Err(ContextError::Limit) {
        return Err(Mismatch::new(
            "getErrorMessage",
            "rendering limit is not enforced",
        ));
    }

    Ok(())
}

/// Builds a one-fact context or reports the rejected key.
fn context_with(key: &str, value: ContextValue) -> Result<Context, Mismatch> {
    let mut context = Context::default();
    context
        .insert(key, value)
        .map_err(|_| Mismatch::new("getErrorMessage", "context fact rejected"))?;

    Ok(context)
}

/// Proves a one-byte template corruption is rejected at the intended name.
pub fn rejects_wrong_template(fixture: &str) -> Result<(), Mismatch> {
    exact(fixture)?;
    let corrupted = corrupt_last_row(fixture)?;

    match exact(&corrupted) {
        Ok(()) => Err(Mismatch::new(
            "<messages>",
            "wrong message template was accepted",
        )),
        Err(mismatch)
            if mismatch.field == "message template differs" && mismatch.name != "<messages>" =>
        {
            Ok(())
        }
        Err(_) => Err(Mismatch::new(
            "<messages>",
            "wrong message template failed for the wrong reason",
        )),
    }
}

/// Replaces the final template byte with a different hex digit.
fn corrupt_last_row(fixture: &str) -> Result<String, Mismatch> {
    let rows: Vec<_> = fixture.lines().collect();
    let last = *rows
        .last()
        .ok_or(Mismatch::new("<messages>", "message fixture is empty"))?;
    let fields: Vec<_> = last.split('\t').collect();

    if fields.len() != 4 {
        return Err(Mismatch::new(
            "<messages>",
            "message row does not have four fields",
        ));
    }

    let mut hex = fields[3].as_bytes().to_vec();
    let last_byte = hex
        .last_mut()
        .ok_or(Mismatch::new(fields[0], "message template is empty"))?;
    *last_byte = if *last_byte == b'0' { b'1' } else { b'0' };
    let wrong = format!(
        "{}\t{}\t{}\t{}",
        fields[0],
        fields[1],
        fields[2],
        std::str::from_utf8(&hex).expect("ASCII hex")
    );
    let mut corrupted = rows[..rows.len() - 1].join("\n");
    corrupted.push('\n');
    corrupted.push_str(&wrong);

    Ok(corrupted)
}
