//! Struct sequence decoding is not accepted at JSON object boundaries.

use super::Optional;
use serde::{
    Deserialize, Deserializer,
    de::{DeserializeOwned, Error},
};
use serde_json::value::RawValue;
use solana_kit_client::Presence;

pub(super) fn parse<T: DeserializeOwned>(raw: &str) -> Result<T, serde_json::Error> {
    if !raw.trim_start().starts_with('{') {
        return Err(serde_json::Error::custom("expected a portal JSON object"));
    }

    serde_json::from_str(raw)
}

pub(super) fn optional<'de, D, T>(decoder: D) -> Result<Optional<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let raw = Option::<Box<RawValue>>::deserialize(decoder)?;
    raw.map(|raw| parse(raw.get()).map_err(D::Error::custom))
        .transpose()
        .map(Presence::Present)
}

pub(super) fn number<'de, D: Deserializer<'de>>(
    decoder: D,
) -> Result<Optional<serde_json::Number>, D::Error> {
    let raw = Option::<Box<RawValue>>::deserialize(decoder)?;
    raw.map(|raw| {
        let text = raw.get().trim_start();

        if !matches!(text.as_bytes().first(), Some(b'-' | b'0'..=b'9')) {
            return Err(D::Error::custom("expected a portal JSON number"));
        }

        serde_json::from_str(text).map_err(D::Error::custom)
    })
    .transpose()
    .map(Presence::Present)
}
