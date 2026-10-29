//! Literal keys and every duplicate value are decoded before last-wins insertion.

use super::DeployError;
use serde::{
    Deserialize, Deserializer,
    de::{self, DeserializeSeed, MapAccess, Visitor},
};
use serde_json::{Value, value::RawValue};
use solana_kit_surfpool::Object;
use std::{
    cell::{Cell, RefCell},
    fmt,
};

// serde_json starts at 128, decrements on each container entry, and rejects zero.
const JSON_RECURSION_LIMIT: u8 = 128;

/// The root map uses serde_json's own entry guard; nested raw values carry the rest.
pub(super) fn object(bytes: &[u8]) -> Result<Object, DeployError> {
    let failure = DecodeFailure::default();
    let seed = Literal {
        remaining_depth: JSON_RECURSION_LIMIT - 1,
        failure: &failure,
    };
    let mut parser = serde_json::Deserializer::from_slice(bytes);
    let result = parser.deserialize_map(seed).and_then(|object| {
        parser.end()?;

        Ok(object)
    });

    if failure.nesting_exceeded.get() {
        return Err(DeployError::IdlNesting);
    }

    if let Some(cause) = failure.cause.into_inner() {
        return Err(DeployError::IdlJson(cause));
    }

    result.map_err(DeployError::IdlJson)
}

#[derive(Default)]
struct DecodeFailure {
    nesting_exceeded: Cell<bool>,
    cause: RefCell<Option<serde_json::Error>>,
}

impl DecodeFailure {
    /// Serde's generic error conversion must not replace the original JSON cause.
    fn retain<E: de::Error>(&self, cause: serde_json::Error) -> E {
        let converted = E::custom(&cause);
        let mut original = self.cause.borrow_mut();

        if original.is_none() {
            *original = Some(cause);
        }

        converted
    }
}

#[derive(Clone, Copy)]
struct Literal<'a> {
    remaining_depth: u8,
    failure: &'a DecodeFailure,
}

impl Literal<'_> {
    /// RawValue skips nested containers, so retain the decoder's original guard.
    fn enter<E: de::Error>(self) -> Result<Self, E> {
        if self.remaining_depth <= 1 {
            self.failure.nesting_exceeded.set(true);

            return Err(E::custom("JSON recursion limit exceeded"));
        }

        Ok(Self {
            remaining_depth: self.remaining_depth - 1,
            ..self
        })
    }
}

impl<'de> DeserializeSeed<'de> for Literal<'_> {
    type Value = Value;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        let raw = <&RawValue>::deserialize(deserializer)?;

        let parsed = match raw.get().as_bytes().first() {
            Some(b'{') => serde_json::Deserializer::from_str(raw.get())
                .deserialize_map(self.enter::<D::Error>()?)
                .map(Value::Object),
            Some(b'[') => {
                let seed = self.enter::<D::Error>()?;
                let elements: Vec<&RawValue> = serde_json::from_str(raw.get())
                    .map_err(|cause| self.failure.retain::<D::Error>(cause))?;

                elements
                    .into_iter()
                    .map(|raw| seed.deserialize(raw))
                    .collect::<Result<Vec<_>, _>>()
                    .map(Value::Array)
            }
            _ => serde_json::from_str(raw.get()),
        };

        parsed.map_err(|cause| self.failure.retain::<D::Error>(cause))
    }
}

impl<'de> Visitor<'de> for Literal<'_> {
    type Value = Object;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an IDL JSON object")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Object, A::Error> {
        let mut object = Object::new();

        while let Some(key) = map.next_key::<String>()? {
            let value = map.next_value_seed(self)?;
            object.insert(key, value);
        }

        Ok(object)
    }
}
