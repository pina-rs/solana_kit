//! Surfpool result wrappers retain context while exposing the upstream value.

use serde::de::DeserializeOwned;
use serde_json::Value;

/// A decoded result and the optional Surfpool context envelope.
#[derive(Clone, Debug, PartialEq)]
pub struct Response<T> {
    /// Plain result, or the `value` inside a contextual result.
    pub value: T,
    /// Complete contextual object except `value`, including future metadata.
    ///
    /// Present only when `context` is an object and `value` exists, matching Dart.
    pub envelope: Option<crate::Object>,
}

impl<T> Response<T> {
    /// Extracts the value using the pinned cheatcode convenience semantics.
    pub fn into_value(self) -> T {
        self.value
    }

    /// Borrows context without discarding other retained envelope metadata.
    pub fn context(&self) -> Option<&crate::Object> {
        self.envelope.as_ref()?.get("context")?.as_object()
    }
}

/// Unwraps only the contextual shape recognized by the pinned cheatcode surface.
pub(crate) fn decode<T: DeserializeOwned>(text: &str) -> Result<Response<T>, serde_json::Error> {
    let raw: Value = serde_json::from_str(text)?;
    let contextual = raw.get("context").is_some_and(Value::is_object)
        && raw
            .as_object()
            .is_some_and(|object| object.contains_key("value"));

    if let Value::Object(mut envelope) = raw {
        if contextual && let Some(value) = envelope.remove("value") {
            return Ok(Response {
                value: typed(value)?,
                envelope: Some(envelope),
            });
        }

        return Ok(Response {
            value: typed(Value::Object(envelope))?,
            envelope: None,
        });
    }

    Ok(Response {
        value: typed(raw)?,
        envelope: None,
    })
}

/// Re-parsing exact JSON keeps integer visitors and raw-value object guards intact.
fn typed<T: DeserializeOwned>(value: Value) -> Result<T, serde_json::Error> {
    serde_json::from_str(&serde_json::to_string(&value)?)
}
