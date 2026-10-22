//! Strict framing and bounded parsing shared by both executable designs.

use std::fmt;

use serde::de::{DeserializeOwned, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::value::RawValue;

use crate::methods::Method;
use crate::{Reply, Request, TransportError};

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub bytes: usize,
    pub depth: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            bytes: 8192,
            depth: 32,
        }
    }
}

/// Unknown error data is retained as exact JSON, not called an exhaustive schema.
#[derive(Debug, Deserialize)]
pub struct RemoteError {
    pub code: i64,
    pub message: String,
    #[serde(default)]
    pub data: Presence<Box<RawValue>>,
}

/// Missing and explicit JSON null differ, including for a nullable success result.
#[derive(Debug, Default, PartialEq, Eq)]
pub enum Presence<T> {
    #[default]
    Missing,
    Present(T),
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Presence<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        T::deserialize(deserializer).map(Self::Present)
    }
}

#[derive(Debug)]
pub enum Error {
    IdExhausted,
    Transport(TransportError),
    Status(u16),
    BodyLimit,
    DepthLimit,
    Json(serde_json::Error),
    Schema(serde_json::Error),
    Envelope,
    Correlation,
    Remote(RemoteError),
}

impl From<TransportError> for Error {
    fn from(error: TransportError) -> Self {
        Self::Transport(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

#[derive(Serialize)]
struct Frame<'a, P> {
    jsonrpc: &'static str,
    id: u64,
    method: &'static str,
    params: &'a P,
}

pub fn request<R: Method>(id: u64, method: &R, limits: Limits) -> Result<Request, Error> {
    let body = serde_json::to_vec(&Frame {
        jsonrpc: "2.0",
        id,
        method: R::NAME,
        params: method,
    })?;
    check_limits(&body, limits)?;

    Ok(Request {
        body,
        max_reply_bytes: limits.bytes,
    })
}

/// This lexical guard bounds nested containers before allocating parsed values.
fn check_limits(body: &[u8], limits: Limits) -> Result<(), Error> {
    if body.len() > limits.bytes {
        return Err(Error::BodyLimit);
    }

    let mut depth = 0_usize;
    let mut state = ScanState::Outside;

    for byte in body {
        depth = match state.container_delta(*byte) {
            1 => depth + 1,
            -1 => depth.saturating_sub(1),
            _ => depth,
        };

        if depth > limits.depth {
            return Err(Error::DepthLimit);
        }
    }

    Ok(())
}

#[derive(Clone, Copy)]
enum ScanState {
    Outside,
    String,
    Escape,
}

impl ScanState {
    fn container_delta(&mut self, byte: u8) -> i8 {
        match (*self, byte) {
            (Self::Outside, b'{' | b'[') => return 1,
            (Self::Outside, b'}' | b']') => return -1,
            (Self::Outside, b'"') | (Self::Escape, _) => *self = Self::String,
            (Self::String, b'\\') => *self = Self::Escape,
            (Self::String, b'"') => *self = Self::Outside,
            _ => {}
        }

        0
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    jsonrpc: String,
    id: u64,
    #[serde(default)]
    result: Presence<Box<RawValue>>,
    #[serde(default)]
    error: Presence<Box<RawValue>>,
}

pub fn response<R: DeserializeOwned>(id: u64, reply: Reply, limits: Limits) -> Result<R, Error> {
    if !(200..300).contains(&reply.status) {
        return Err(Error::Status(reply.status));
    }

    check_limits(&reply.body, limits)?;
    let envelope: Envelope = serde_json::from_slice(&reply.body)?;

    if envelope.jsonrpc != "2.0" {
        return Err(Error::Envelope);
    }

    if envelope.id != id {
        return Err(Error::Correlation);
    }

    match (envelope.result, envelope.error) {
        (Presence::Present(raw), Presence::Missing) => {
            serde_json::from_str(raw.get()).map_err(Error::Schema)
        }
        (Presence::Missing, Presence::Present(raw)) => {
            Err(Error::Remote(serde_json::from_str(raw.get())?))
        }
        _ => Err(Error::Envelope),
    }
}

/// Required nullable field: serde's ordinary Option also accepts a missing field.
pub fn required_nullable<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct Nullable<T>(std::marker::PhantomData<T>);
    impl<'de, T: Deserialize<'de>> Visitor<'de> for Nullable<T> {
        type Value = Option<T>;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a present nullable value")
        }
        fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
        fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            T::deserialize(d).map(Some)
        }
    }
    deserializer.deserialize_option(Nullable(std::marker::PhantomData))
}
