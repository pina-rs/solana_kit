//! Serde's struct sequence support must not admit arrays where Jupiter requires objects.

use serde::{
    Deserialize, Deserializer,
    de::{DeserializeOwned, Error},
};
use serde_json::value::RawValue;

pub(crate) fn parse<T: DeserializeOwned>(raw: &str) -> Result<T, serde_json::Error> {
    if !raw.trim_start().starts_with('{') {
        return Err(serde_json::Error::custom("expected a JSON object"));
    }

    serde_json::from_str(raw)
}

pub(crate) fn optional<'de, D, T>(decoder: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let raw = Option::<Box<RawValue>>::deserialize(decoder)?;
    raw.map(|value| parse(value.get()).map_err(D::Error::custom))
        .transpose()
}

pub(crate) fn array<'de, D, T>(decoder: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let raw = Option::<Vec<Box<RawValue>>>::deserialize(decoder)?;
    raw.map(|items| {
        items
            .iter()
            .map(|value| parse(value.get()).map_err(D::Error::custom))
            .collect()
    })
    .transpose()
}
