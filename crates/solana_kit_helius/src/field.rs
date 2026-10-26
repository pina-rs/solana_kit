//! Exact JSON optionality and decimal values.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A response field that distinguishes absence, explicit null and a typed value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field<T> {
    /// The object did not contain this field.
    Missing,
    /// The object contained JSON null.
    Null,
    /// The object contained a valid non-null value.
    Value(T),
}

impl<T> Default for Field<T> {
    fn default() -> Self {
        Self::Missing
    }
}

impl<T> Field<T> {
    /// Returns whether serialization should omit this field entirely.
    pub const fn is_missing(&self) -> bool {
        matches!(self, Self::Missing)
    }

    /// Borrows a present non-null value without conflating malformed data.
    pub const fn value(&self) -> Option<&T> {
        match self {
            Self::Value(value) => Some(value),
            Self::Missing | Self::Null => None,
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Field<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Option::<T>::deserialize(deserializer).map(|value| match value {
            Some(value) => Self::Value(value),
            None => Self::Null,
        })
    }
}

impl<T: Serialize> Serialize for Field<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Value(value) => value.serialize(serializer),
            Self::Missing | Self::Null => serializer.serialize_none(),
        }
    }
}

/// An exact JSON decimal, avoiding implicit conversion through a binary float.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct Decimal(serde_json::Number);

impl<'de> Deserialize<'de> for Decimal {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        let raw = Box::<serde_json::value::RawValue>::deserialize(decoder)?;

        if !matches!(raw.get().as_bytes().first(), Some(b'-' | b'0'..=b'9')) {
            return Err(serde::de::Error::custom("expected a JSON number"));
        }

        serde_json::from_str(raw.get())
            .map(Self)
            .map_err(serde::de::Error::custom)
    }
}

impl Decimal {
    /// Borrows the exact JSON number for deliberate caller conversion.
    pub const fn number(&self) -> &serde_json::Number {
        &self.0
    }
}

impl core::str::FromStr for Decimal {
    type Err = serde_json::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(value)
    }
}
