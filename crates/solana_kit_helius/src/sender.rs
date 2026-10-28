//! Sender submission preserves explicit routing and exactly one exchange.

use crate::{
    Field,
    service::{Body, Error, Operation, Service, Verb},
};
use serde::{Deserialize, Serialize, Serializer};
use solana_kit_client::methods::{
    Method,
    submission::{SendConfig, SendTransaction, SignedWire, WireEncoding},
};
use solana_kit_core::Signature;

mod regions;
pub use regions::{
    LAMPORTS_PER_SOL, MIN_TIP_LAMPORTS_MAX, MIN_TIP_LAMPORTS_SWQOS, SenderRegion, TIP_ACCOUNTS,
};

/// Explicit path policy on the application's configured Sender service slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    /// POST the configured origin's root, as in broadcastTransaction.
    Root,
    /// POST /fast with optional SWQOS-only query policy.
    Fast {
        /// Include swqos_only=true only when explicitly selected.
        swqos_only: bool,
    },
}

/// Submit exact caller-signed wire bytes. This operation never signs or retries.
#[derive(Debug)]
pub struct Submission<'a> {
    /// Exact signed transaction bytes, encoded once as base64 by the Kit owner.
    pub wire: SignedWire<'a>,
    /// Correlation ID supplied by the caller. The reference broadcast uses one.
    pub id: u32,
    /// Caller-controlled preflight option. The reference default is true.
    pub skip_preflight: bool,
    /// Explicit root or fast route. The adapter owns any regional origin policy.
    pub route: Route,
}

impl Serialize for Submission<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Frame<'a> {
            jsonrpc: &'static str,
            id: u32,
            method: &'static str,
            params: SendTransaction<'a>,
        }

        Frame {
            jsonrpc: "2.0",
            id: self.id,
            method: "sendTransaction",
            params: SendTransaction {
                wire: self.wire,
                encoding: WireEncoding::Base64,
                config: SendConfig {
                    skip_preflight: Some(self.skip_preflight),
                    max_retries: Some(0),
                    ..SendConfig::default()
                },
            },
        }
        .serialize(serializer)
    }
}

/// A canonical Sender acknowledgement bound to the submitted primary signature.
///
/// Identity correlation does not prove broadcast, confirmation or finality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct Acknowledgement(pub Signature);

impl TryFrom<String> for Acknowledgement {
    type Error = solana_kit_core::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse().map(Self)
    }
}

#[derive(Deserialize)]
struct Envelope {
    #[serde(default)]
    jsonrpc: Field<String>,
    #[serde(default)]
    id: Field<u32>,
    #[serde(default)]
    result: Field<Box<serde_json::value::RawValue>>,
    #[serde(default)]
    error: Field<Box<serde_json::value::RawValue>>,
}

impl Operation for Submission<'_> {
    type Response = Acknowledgement;
    const SERVICE: Service = Service::Sender;
    const VERB: Verb = Verb::Post;
    const BODY: Body = Body::Json;

    fn path(&self) -> Vec<String> {
        match self.route {
            Route::Root => Vec::new(),
            Route::Fast { .. } => vec!["fast".into()],
        }
    }

    fn query(&self) -> Vec<(String, String)> {
        match self.route {
            Route::Fast { swqos_only: true } => vec![("swqos_only".into(), "true".into())],
            _ => Vec::new(),
        }
    }

    fn decode(&self, bytes: &[u8]) -> Result<Self::Response, Error> {
        let raw: Box<serde_json::value::RawValue> =
            serde_json::from_slice(bytes).map_err(Error::Schema)?;

        if raw.get().starts_with('"') {
            return self.acknowledgement(raw.get());
        }

        let envelope: Envelope = serde_json::from_str(raw.get()).map_err(Error::Schema)?;
        validate_identity(&envelope, self.id)?;

        if let Field::Value(error) = envelope.error {
            return Err(Error::Provider(error));
        }

        match envelope.result {
            Field::Value(signature) => self.acknowledgement(signature.get()),
            _ => Err(schema("Sender result must contain a signature string")),
        }
    }
}

impl Submission<'_> {
    /// Shares the RPC decoder so every Sender grammar enforces the same binding.
    fn acknowledgement(&self, json: &str) -> Result<Acknowledgement, Error> {
        SendTransaction {
            wire: self.wire,
            encoding: WireEncoding::Base64,
            config: SendConfig::default(),
        }
        .decode(json)
        .map(Acknowledgement)
        .map_err(Error::Submission)
    }
}

fn validate_identity(envelope: &Envelope, id: u32) -> Result<(), Error> {
    let version_matches = match &envelope.jsonrpc {
        Field::Missing => false,
        Field::Value(version) => version == "2.0",
        Field::Null => false,
    };

    if !version_matches {
        return Err(schema("invalid Sender JSON-RPC version"));
    }

    let id_matches = match &envelope.id {
        Field::Missing => false,
        Field::Value(actual) => *actual == id,
        Field::Null => false,
    };

    if !id_matches {
        return Err(schema("mismatched Sender request ID"));
    }

    Ok(())
}

fn schema(message: &str) -> Error {
    Error::Schema(<serde_json::Error as serde::de::Error>::custom(message))
}
