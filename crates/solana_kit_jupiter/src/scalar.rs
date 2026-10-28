//! Accept the reference scalar alternatives without lossy numeric conversions.

use serde::{Deserialize, Deserializer, de::Error};
use serde_json::{Number, value::RawValue};
use std::str::FromStr;

pub(crate) fn number<'de, D: Deserializer<'de>>(decoder: D) -> Result<Option<Number>, D::Error> {
    let raw = Option::<Box<RawValue>>::deserialize(decoder)?;
    raw.map(|value| parse_number(value.get()).map_err(D::Error::custom))
        .transpose()
}

pub(crate) fn number_or_string<'de, D: Deserializer<'de>>(
    decoder: D,
) -> Result<Option<Number>, D::Error> {
    let raw = Option::<Box<RawValue>>::deserialize(decoder)?;
    raw.map(|value| scalar_text(value.get()).and_then(|text| parse_number(&text)))
        .transpose()
        .map_err(D::Error::custom)
}

pub(crate) fn integer<'de, D, T>(decoder: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
{
    let raw = Option::<Box<RawValue>>::deserialize(decoder)?;
    raw.map(|value| {
        let text = scalar_text(value.get()).map_err(D::Error::custom)?;
        text.parse()
            .map_err(|_| D::Error::custom("invalid unsigned integer"))
    })
    .transpose()
}

pub(crate) fn text_or_integer<'de, D: Deserializer<'de>>(
    decoder: D,
) -> Result<Option<String>, D::Error> {
    let raw = Option::<Box<RawValue>>::deserialize(decoder)?;
    raw.map(|value| {
        let text = scalar_text(value.get()).map_err(D::Error::custom)?;

        if !value.get().starts_with('"') && !text.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(D::Error::custom("expected a string or unsigned integer"));
        }

        Ok(text)
    })
    .transpose()
}

fn scalar_text(raw: &str) -> Result<String, &'static str> {
    if raw.starts_with('"') {
        return serde_json::from_str(raw).map_err(|_| "invalid scalar string");
    }

    Ok(raw.to_owned())
}

fn parse_number(raw: &str) -> Result<Number, &'static str> {
    // Reject serde's internal arbitrary-precision object representation as a wire number.
    if !matches!(raw.as_bytes().first(), Some(b'-' | b'0'..=b'9')) {
        return Err("expected a JSON number");
    }

    raw.parse().map_err(|_| "invalid JSON number")
}
