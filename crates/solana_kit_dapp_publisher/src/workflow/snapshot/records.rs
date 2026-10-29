//! Local recovery records require JSON objects; serde tuple-shaped struct input is not admitted.

use super::{UploadIntent, UploadReceipt};
use crate::portal::object;
use serde::{Deserialize, Deserializer, de::Error};
use serde_json::value::RawValue;

pub(super) fn intent<'de, D: Deserializer<'de>>(
    decoder: D,
) -> Result<Option<UploadIntent>, D::Error> {
    Option::<Box<RawValue>>::deserialize(decoder)?
        .map(|raw| object::parse(raw.get()).map_err(D::Error::custom))
        .transpose()
}

pub(super) fn receipts<'de, D: Deserializer<'de>>(
    decoder: D,
) -> Result<Vec<UploadReceipt>, D::Error> {
    Vec::<Box<RawValue>>::deserialize(decoder)?
        .into_iter()
        .map(|raw| object::parse(raw.get()).map_err(D::Error::custom))
        .collect()
}
