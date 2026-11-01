//! tRPC envelope selection precedes typed object decoding and never triggers a mutation.

use super::{Error, object};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::value::RawValue;
use solana_kit_client::{Presence, Reply, rest::Budgets};

#[derive(Deserialize)]
struct Envelope {
    #[serde(default, rename = "0")]
    batch: Presence<Box<RawValue>>,
    #[serde(default)]
    result: Presence<Box<RawValue>>,
    #[serde(default)]
    error: Presence<Box<RawValue>>,
}

#[derive(Deserialize)]
struct ResultData {
    #[serde(default)]
    data: Presence<Box<RawValue>>,
}

#[derive(Deserialize)]
struct Either {
    #[serde(default, rename = "_tag")]
    tag: Presence<String>,
    #[serde(default)]
    left: Presence<Box<RawValue>>,
    #[serde(default)]
    right: Presence<Box<RawValue>>,
}

#[derive(Deserialize)]
struct RemoteError {
    #[serde(default)]
    message: Option<String>,
}

pub(super) fn decode<T: DeserializeOwned>(reply: Reply, budgets: Budgets) -> Result<T, Error> {
    budgets.check_bytes(&reply.body).map_err(Error::Json)?;

    if !(200..300).contains(&reply.status) {
        return Err(Error::Http {
            status: reply.status,
            body: reply.body,
        });
    }

    budgets.validate_json(&reply.body).map_err(Error::Json)?;
    let text = std::str::from_utf8(&reply.body).map_err(|_| Error::Envelope("invalid UTF-8"))?;
    let envelope = object::parse::<Envelope>(text).map_err(Error::Schema)?;
    result(unbatch(envelope)?, reply.body)
}

/// Only one batch envelope is supported; explicit null is not an absent batch.
fn unbatch(mut envelope: Envelope) -> Result<Envelope, Error> {
    let Presence::Present(batch) = std::mem::take(&mut envelope.batch) else {
        return Ok(envelope);
    };

    if !matches!(
        (&envelope.result, &envelope.error),
        (Presence::Missing, Presence::Missing)
    ) {
        return Err(Error::Envelope("mixed batch and direct outcome"));
    }

    let envelope: Envelope = object::parse(batch.get()).map_err(Error::Schema)?;

    if matches!(envelope.batch, Presence::Present(_)) {
        return Err(Error::Envelope("nested batch"));
    }

    Ok(envelope)
}

/// Exactly one result/error outcome is chosen before inspecting source-specific result shapes.
fn result<T: DeserializeOwned>(envelope: Envelope, body: Vec<u8>) -> Result<T, Error> {
    let result = match (envelope.result, envelope.error) {
        (Presence::Present(result), Presence::Missing) => result,
        (Presence::Missing, Presence::Present(error)) => {
            let error: RemoteError = object::parse(error.get()).map_err(Error::Schema)?;

            return Err(Error::Provider {
                message: error.message,
                body,
            });
        }
        _ => return Err(Error::Envelope("expected exactly one result or error")),
    };

    let data = object::parse::<ResultData>(result.get()).map_err(Error::Schema)?;
    let data = match &data.data {
        Presence::Missing => result.as_ref(),
        Presence::Present(data) => data.as_ref(),
    };

    either(data, body)
}

/// A tagged Either result must carry its matching value and no conflicting branch.
fn either<T: DeserializeOwned>(data: &RawValue, body: Vec<u8>) -> Result<T, Error> {
    let either: Either = object::parse(data.get()).map_err(Error::Schema)?;
    let Presence::Present(tag) = either.tag else {
        return object::parse(data.get()).map_err(Error::Schema);
    };

    match (tag.as_str(), either.left, either.right) {
        ("Right", Presence::Missing, Presence::Present(value)) => {
            object::parse(value.get()).map_err(Error::Schema)
        }
        ("Left", Presence::Present(value), Presence::Missing) => {
            let error: RemoteError = object::parse(value.get()).map_err(Error::Schema)?;

            Err(Error::Provider {
                message: error.message,
                body,
            })
        }
        _ => Err(Error::Envelope(
            "unknown, missing or conflicting result tag",
        )),
    }
}
