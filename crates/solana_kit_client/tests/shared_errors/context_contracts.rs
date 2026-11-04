//! Portable proof for the pinned context wire encoding and decoding.

use solana_kit_client::diagnostics::{decode_context, encode_context};
use solana_kit_core::errors::{Context, ContextError, ContextValue};
use std::{borrow::ToOwned, collections::BTreeMap, string::String, vec, vec::Vec};

const BASE_LITERALS: usize = 5;
const EXTRA_LITERALS: usize = 3;

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

/// The five committed source encodings still reproduce exactly.
pub fn context_literals(literals: &str) -> Result<(), Mismatch> {
    let cases = literal_cases()?;
    check_encodings(&cases, literals, "context encoding differs")
}

/// Builds the five committed contexts in their original order.
fn literal_cases() -> Result<Vec<Context>, Mismatch> {
    let mut cases = vec![Context::default(); BASE_LITERALS];
    cases[1] = fact("count", ContextValue::Integer(42))?;
    cases[1] = insert(cases[1].clone(), "key", ContextValue::Null)?;
    cases[2] = fact(
        "items",
        ContextValue::List(vec![
            ContextValue::Text("a".into()),
            ContextValue::Text("b".into()),
        ]),
    )?;
    cases[3] = fact(
        "nested",
        ContextValue::Object(BTreeMap::from([("x".into(), ContextValue::Integer(1))])),
    )?;
    cases[4] = fact(
        "items",
        ContextValue::List(vec![
            ContextValue::Text("a b".into()),
            ContextValue::Text("%".into()),
        ]),
    )?;

    Ok(cases)
}

/// Additional source encodings cover booleans, unicode, nesting and empties.
pub fn context_extra(literals: &str) -> Result<(), Mismatch> {
    let cases = extra_cases()?;
    check_encodings(&cases, literals, "extra context encoding differs")?;
    check_extra_decodes(literals)
}

/// Builds the three additional public contexts.
fn extra_cases() -> Result<Vec<Context>, Mismatch> {
    let mut flag = fact("flag", ContextValue::Boolean(true))?;
    flag = insert(flag, "text", ContextValue::Text("a&b=c".into()))?;
    flag = insert(flag, "unicode", ContextValue::Text("é".into()))?;
    let nested = fact(
        "nested",
        ContextValue::Object(BTreeMap::from([(
            "z".into(),
            ContextValue::List(vec![ContextValue::Integer(1), ContextValue::Null]),
        )])),
    )?;
    let empty = fact("empty", ContextValue::Text(String::new()))?;

    Ok(vec![flag, nested, empty])
}

/// Compares each constructed context with its independent literal.
fn check_encodings(cases: &[Context], literals: &str, field: &'static str) -> Result<(), Mismatch> {
    let literals: Vec<_> = literals.lines().collect();

    if literals.len() != cases.len() {
        return Err(Mismatch::new(
            "encodeContextObject",
            "context literal count differs",
        ));
    }

    for (context, expected) in cases.iter().zip(literals) {
        let actual = encode_context(context)
            .map_err(|_| Mismatch::new("encodeContextObject", "encode failed"))?;

        if actual != expected {
            return Err(Mismatch::new("encodeContextObject", field));
        }
    }

    Ok(())
}

/// Mounted values decode with the source's documented lossy typing.
fn check_extra_decodes(literals: &str) -> Result<(), Mismatch> {
    let literals: Vec<_> = literals.lines().collect();

    if literals.len() != EXTRA_LITERALS {
        return Err(Mismatch::new(
            "decodeEncodedContext",
            "extra context literal count differs",
        ));
    }

    check_flag_decode(literals[0])?;
    check_nested_decode(literals[1])?;
    check_empty_decode(literals[2])?;
    check_decode_rules()
}

/// Boolean text and unicode survive the source representation.
fn check_flag_decode(literal: &str) -> Result<(), Mismatch> {
    let decoded = decode_context(literal)
        .map_err(|_| Mismatch::new("decodeEncodedContext", "extra decode failed"))?;

    if decoded.get("flag") != Some(&ContextValue::Text("true".to_owned())) {
        return Err(Mismatch::new(
            "decodeEncodedContext",
            "boolean text decode differs",
        ));
    }

    if decoded.get("unicode") != Some(&ContextValue::Text("é".to_owned())) {
        return Err(Mismatch::new(
            "decodeEncodedContext",
            "unicode decode differs",
        ));
    }

    Ok(())
}

/// A nested array of integer and null decodes back to typed facts.
fn check_nested_decode(literal: &str) -> Result<(), Mismatch> {
    let decoded = decode_context(literal)
        .map_err(|_| Mismatch::new("decodeEncodedContext", "nested decode failed"))?;
    let expected = ContextValue::Object(BTreeMap::from([(
        "z".into(),
        ContextValue::List(vec![ContextValue::Integer(1), ContextValue::Null]),
    )]));

    if decoded.get("nested") != Some(&expected) {
        return Err(Mismatch::new(
            "decodeEncodedContext",
            "nested decode differs",
        ));
    }

    Ok(())
}

/// An encoded empty string remains an explicit empty text fact.
fn check_empty_decode(literal: &str) -> Result<(), Mismatch> {
    let decoded = decode_context(literal)
        .map_err(|_| Mismatch::new("decodeEncodedContext", "empty decode failed"))?;

    if decoded.get("empty") != Some(&ContextValue::Text(String::new())) {
        return Err(Mismatch::new(
            "decodeEncodedContext",
            "empty text decode differs",
        ));
    }

    Ok(())
}

/// Strict decoding follows the source query parser and its typed failures.
fn check_decode_rules() -> Result<(), Mismatch> {
    check_empty_and_malformed()?;
    check_lossy_rules()?;
    check_numeric_rules()
}

/// Empty input is empty context; malformed percent escapes are errors.
fn check_empty_and_malformed() -> Result<(), Mismatch> {
    if decode_context("") != Ok(Context::default()) {
        return Err(Mismatch::new(
            "decodeEncodedContext",
            "empty encoding is not empty context",
        ));
    }

    for corrupted in ["%%%", "YT0l", "YT0lWlo="] {
        if decode_context(corrupted) != Err(ContextError::Encoding) {
            return Err(Mismatch::new(
                "decodeEncodedContext",
                "malformed percent encoding was accepted",
            ));
        }
    }

    Ok(())
}

/// Duplicate keys, bare keys and plus signs follow source outcomes.
fn check_lossy_rules() -> Result<(), Mismatch> {
    let duplicate = decoded("YT0xJmE9Mg==", "duplicate decode failed")?;

    if duplicate.get("a") != Some(&ContextValue::Integer(2)) {
        return Err(Mismatch::new(
            "decodeEncodedContext",
            "duplicate key does not keep the last value",
        ));
    }

    let bare = decoded("YQ==", "bare key decode failed")?;

    if bare.get("a") != Some(&ContextValue::Text(String::new())) {
        return Err(Mismatch::new(
            "decodeEncodedContext",
            "key without equals does not decode to empty text",
        ));
    }

    let plus = decoded("YT0r", "plus decode failed")?;

    if plus.get("a") != Some(&ContextValue::Text(" ".to_owned())) {
        return Err(Mismatch::new(
            "decodeEncodedContext",
            "plus sign does not decode to a space",
        ));
    }

    Ok(())
}

/// Padded, negative-zero and wide integers keep exact values.
fn check_numeric_rules() -> Result<(), Mismatch> {
    let padded = decoded("YT0wMDc=", "padded decode failed")?;

    if padded.get("a") != Some(&ContextValue::Integer(7)) {
        return Err(Mismatch::new(
            "decodeEncodedContext",
            "padded integer decode differs",
        ));
    }

    let negative_zero = decoded("YT0tMA==", "negative zero failed")?;

    if negative_zero.get("a") != Some(&ContextValue::Integer(0)) {
        return Err(Mismatch::new(
            "decodeEncodedContext",
            "negative zero decode differs",
        ));
    }

    let wide = decoded("YT05MjIzMzcyMDM2ODU0Nzc1ODA4", "wide integer failed")?;

    if wide.get("a") != Some(&ContextValue::Integer(9_223_372_036_854_775_808)) {
        return Err(Mismatch::new(
            "decodeEncodedContext",
            "wide integer is not preserved exactly",
        ));
    }

    if decode_context("YT17IngiOjEuNX0=") != Err(ContextError::Number) {
        return Err(Mismatch::new(
            "decodeEncodedContext",
            "fractional JSON number is not rejected",
        ));
    }

    Ok(())
}

/// Decodes one literal with a stable failure label.
fn decoded(literal: &str, field: &'static str) -> Result<Context, Mismatch> {
    decode_context(literal).map_err(|_| Mismatch::new("decodeEncodedContext", field))
}

/// Builds a one-fact context.
fn fact(key: &str, value: ContextValue) -> Result<Context, Mismatch> {
    let mut context = Context::default();
    context
        .insert(key, value)
        .map_err(|_| Mismatch::new("encodeContextObject", "context fact rejected"))?;

    Ok(context)
}

/// Inserts one more fact into an existing context.
fn insert(mut context: Context, key: &str, value: ContextValue) -> Result<Context, Mismatch> {
    context
        .insert(key, value)
        .map_err(|_| Mismatch::new("encodeContextObject", "context fact rejected"))?;

    Ok(context)
}
