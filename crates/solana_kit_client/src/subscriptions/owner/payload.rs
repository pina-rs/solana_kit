//! Owned payload kinds retain exact byte accounting for both provider representations.

use crate::{Error, subscriptions::SubscriptionMethod};
use serde_json::value::RawValue;

pub(super) enum Payload {
    Json(Box<RawValue>),
    Binary(Vec<u8>),
}

impl Payload {
    pub(super) fn len(&self) -> usize {
        match self {
            Self::Json(raw) => raw.get().len(),
            Self::Binary(bytes) => bytes.len(),
        }
    }

    pub(super) fn decode<R: SubscriptionMethod>(&self, request: &R) -> Result<R::Response, Error> {
        match self {
            Self::Json(raw) => request.decode(raw.get()),
            Self::Binary(bytes) => request.decode_binary(bytes),
        }
    }
}
