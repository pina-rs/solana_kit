//! Provider failures remain exact bounded JSON and never become successful deletions.

use crate::service::Error;
use serde_json::value::RawValue;

#[derive(serde::Deserialize)]
struct Envelope {
    #[serde(default)]
    error: Option<Box<RawValue>>,
}

pub(super) fn payload(bytes: &[u8]) -> Result<Box<RawValue>, Error> {
    let raw: Box<RawValue> = serde_json::from_slice(bytes).map_err(Error::Schema)?;

    if raw.get().starts_with('{') {
        let envelope: Envelope = serde_json::from_str(raw.get()).map_err(Error::Schema)?;

        if let Some(error) = envelope.error {
            return Err(Error::Provider(error));
        }
    }

    Ok(raw)
}

pub(super) fn decode<Output: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<Output, Error> {
    let raw = payload(bytes)?;
    serde_json::from_str(raw.get()).map_err(Error::Schema)
}
