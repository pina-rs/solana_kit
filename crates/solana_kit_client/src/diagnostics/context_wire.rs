//! Pinned context URI/base64 representation at the optional JSON boundary.
use base64::{Engine, engine::general_purpose::STANDARD};
use solana_kit_core::errors::{Context, ContextError, ContextValue};
use std::collections::BTreeMap;
const MAX_QUERY: usize = 16384;
const MAX_ENCODING: usize = 32768;
const MAX_DEPTH: usize = 8;
const HEX_RADIX: u32 = 16;

/// Encodes public context using the source's percent-encoded query and base64 form.
/// List items retain the source's nested percent escaping rather than round-trip typing.
pub fn encode_context(context: &Context) -> Result<String, ContextError> {
    let mut parts = Vec::new();

    for (key, value) in context.iter() {
        parts.push(format!("{}={}", percent(key), encode_value(value)?));
    }

    let query = parts.join("&");

    if query.len() > MAX_QUERY {
        return Err(ContextError::Limit);
    }

    Ok(STANDARD.encode(query))
}

/// Decodes bounded context; malformed encodings and overflowing integers are errors.
pub fn decode_context(encoded: &str) -> Result<Context, ContextError> {
    if encoded.len() > MAX_ENCODING {
        return Err(ContextError::Limit);
    }

    let bytes = STANDARD
        .decode(encoded)
        .map_err(|_| ContextError::Encoding)?;
    let query = String::from_utf8(bytes).map_err(|_| ContextError::Encoding)?;
    let mut context = Context::default();

    if query.is_empty() {
        return Ok(context);
    }

    for pair in query.split('&') {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        let key = unpercent(key)?;
        let value = unpercent(value)?;
        context.insert(&key, decode_value(&value)?)?;
    }

    Ok(context)
}

fn encode_value(value: &ContextValue) -> Result<String, ContextError> {
    match value {
        ContextValue::List(values) => {
            let parts: Result<Vec<_>, _> = values.iter().map(encode_value).collect();
            Ok(percent(&parts?.join(",")))
        }
        ContextValue::Object(_) => Ok(percent(&to_json(value)?.to_string())),
        _ => Ok(percent(&value.to_string())),
    }
}
/// Pinned decoding tries null, exact integers, structured JSON, then public text.
fn decode_value(value: &str) -> Result<ContextValue, ContextError> {
    if value == "null" {
        return Ok(ContextValue::Null);
    }

    if !value.is_empty()
        && value
            .strip_prefix('-')
            .unwrap_or(value)
            .bytes()
            .all(|byte| byte.is_ascii_digit())
    {
        return value
            .parse::<i128>()
            .map(ContextValue::Integer)
            .map_err(|_| ContextError::Number);
    }

    if (value.starts_with('{') || value.starts_with('['))
        && let Ok(json) = serde_json::from_str::<serde_json::Value>(value)
    {
        return from_json(json, 0);
    }

    Ok(ContextValue::Text(value.into()))
}
fn to_json(value: &ContextValue) -> Result<serde_json::Value, ContextError> {
    use serde_json::Value;
    Ok(match value {
        ContextValue::Null => Value::Null,
        ContextValue::Boolean(value) => Value::Bool(*value),
        ContextValue::Integer(value) => {
            serde_json::from_str(&value.to_string()).map_err(|_| ContextError::Number)?
        }
        ContextValue::Unsigned(value) => {
            serde_json::from_str(&value.to_string()).map_err(|_| ContextError::Number)?
        }
        ContextValue::Text(value) => Value::String(value.clone()),
        ContextValue::List(values) => {
            Value::Array(values.iter().map(to_json).collect::<Result<_, _>>()?)
        }
        ContextValue::Object(values) => Value::Object(
            values
                .iter()
                .map(|(key, value)| Ok((key.clone(), to_json(value)?)))
                .collect::<Result<_, ContextError>>()?,
        ),
    })
}
pub(super) fn from_json(
    value: serde_json::Value,
    depth: usize,
) -> Result<ContextValue, ContextError> {
    use serde_json::Value;

    if depth > MAX_DEPTH {
        return Err(ContextError::Limit);
    }
    Ok(match value {
        Value::Null => ContextValue::Null,
        Value::Bool(value) => ContextValue::Boolean(value),
        Value::Number(value) => ContextValue::Integer(
            value
                .to_string()
                .parse()
                .map_err(|_| ContextError::Number)?,
        ),
        Value::String(value) => ContextValue::Text(value),
        Value::Array(values) => ContextValue::List(
            values
                .into_iter()
                .map(|value| from_json(value, depth + 1))
                .collect::<Result<_, _>>()?,
        ),
        Value::Object(values) => ContextValue::Object(
            values
                .into_iter()
                .map(|(key, value)| Ok((key, from_json(value, depth + 1)?)))
                .collect::<Result<BTreeMap<_, _>, ContextError>>()?,
        ),
    })
}
fn percent(text: &str) -> String {
    let mut encoded = String::new();

    for byte in text.bytes() {
        if byte.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&byte) {
            encoded.push(char::from(byte));
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }

    encoded
}
fn unpercent(text: &str) -> Result<String, ContextError> {
    let mut bytes = Vec::new();
    let mut input = text.bytes();

    while let Some(byte) = input.next() {
        match byte {
            b'%' => {
                let high = char::from(input.next().ok_or(ContextError::Encoding)?)
                    .to_digit(HEX_RADIX)
                    .ok_or(ContextError::Encoding)?;
                let low = char::from(input.next().ok_or(ContextError::Encoding)?)
                    .to_digit(HEX_RADIX)
                    .ok_or(ContextError::Encoding)?;
                bytes.push(
                    u8::try_from(high * HEX_RADIX + low).map_err(|_| ContextError::Encoding)?,
                );
            }
            b'+' => bytes.push(b' '),
            _ => bytes.push(byte),
        }
    }

    String::from_utf8(bytes).map_err(|_| ContextError::Encoding)
}
