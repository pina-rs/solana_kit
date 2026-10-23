use super::{Error, ProtocolVersion};
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::future::Future;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// MWA wallet methods. Method strings cannot be supplied by an untrusted peer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Method {
    Authorize,
    Reauthorize,
    Deauthorize,
    GetCapabilities,
    SignTransactions,
    SignMessages,
    SignAndSendTransactions,
    CloneAuthorization,
}

impl Method {
    /// Returns the JSON-RPC method name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Authorize => "authorize",
            Self::Reauthorize => "reauthorize",
            Self::Deauthorize => "deauthorize",
            Self::GetCapabilities => "get_capabilities",
            Self::SignTransactions => "sign_transactions",
            Self::SignMessages => "sign_messages",
            Self::SignAndSendTransactions => "sign_and_send_transactions",
            Self::CloneAuthorization => "clone_authorization",
        }
    }
}

/// A correlated JSON-RPC request. Payloads deliberately have no Debug output.
#[derive(Serialize)]
pub struct Request {
    id: u32,
    jsonrpc: &'static str,
    method: &'static str,
    params: Map<String, Value>,
}

impl Request {
    /// Applies the pinned legacy/v1 method and chain compatibility mapping.
    pub fn new(
        id: u32,
        method: Method,
        mut params: Map<String, Value>,
        version: ProtocolVersion,
    ) -> Result<Self, Error> {
        if method == Method::GetCapabilities {
            params.clear();
        }

        if method == Method::Authorize {
            map_chain(&mut params, version)?;
        }

        let method = authorization_method(method, version, &params);

        Ok(Self {
            id,
            jsonrpc: "2.0",
            method: method.as_str(),
            params,
        })
    }

    /// Encodes the complete request to UTF-8 JSON.
    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|_| Error::Json)
    }
    /// Returns the correlation ID expected in the response.
    pub fn id(&self) -> u32 {
        self.id
    }
}

fn authorization_method(
    method: Method,
    version: ProtocolVersion,
    params: &Map<String, Value>,
) -> Method {
    if !matches!(method, Method::Authorize | Method::Reauthorize) {
        return method;
    }

    let has_token = params
        .get("auth_token")
        .is_some_and(|value| !value.is_null());

    match (version, has_token) {
        (ProtocolVersion::Legacy, true) => Method::Reauthorize,
        _ => Method::Authorize,
    }
}

fn map_chain(params: &mut Map<String, Value>, version: ProtocolVersion) -> Result<(), Error> {
    let Some(chain) = params.get("chain") else {
        return Ok(());
    };

    if chain.is_null() {
        return Ok(());
    }

    let chain = chain.as_str().ok_or(Error::Json)?;

    let mapped = match (version, chain) {
        (ProtocolVersion::Legacy, "solana:mainnet") => "mainnet-beta",
        (ProtocolVersion::Legacy, "solana:devnet") => "devnet",
        (ProtocolVersion::Legacy, "solana:testnet") => "testnet",
        (ProtocolVersion::V1, "mainnet-beta") => "solana:mainnet",
        (ProtocolVersion::V1, "devnet") => "solana:devnet",
        (ProtocolVersion::V1, "testnet") => "solana:testnet",
        _ => chain,
    }
    .to_string();
    let key = if version == ProtocolVersion::Legacy {
        "cluster"
    } else {
        "chain"
    };

    params.insert(key.into(), Value::String(mapped));
    Ok(())
}

/// Wallet-reported JSON-RPC failure. Peer text is available only by explicit access.
#[derive(Deserialize, Serialize)]
pub struct ProtocolError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}

impl ProtocolError {
    /// Authorization denied.
    pub const AUTHORIZATION_FAILED: i32 = -1;
    /// Invalid signing payloads.
    pub const INVALID_PAYLOADS: i32 = -2;
    /// Signing declined.
    pub const NOT_SIGNED: i32 = -3;
    /// Submission failed.
    pub const NOT_SUBMITTED: i32 = -4;
    /// Request exceeds signing limits.
    pub const TOO_MANY_PAYLOADS: i32 = -5;
    /// The platform must perform Android origin attestation.
    pub const ATTEST_ORIGIN_ANDROID: i32 = -100;
}

/// Exactly one validated response outcome, associated with the expected request.
pub enum Response {
    Result(Map<String, Value>),
    Error(ProtocolError),
}

impl Response {
    /// Rejects wrong IDs, malformed envelopes, and simultaneous result and error.
    pub fn from_json(input: &str, expected_id: u32) -> Result<Self, Error> {
        let mut object: Map<String, Value> =
            serde_json::from_str(input).map_err(|_| Error::Json)?;

        if object.get("jsonrpc").and_then(Value::as_str) != Some("2.0")
            || object.get("id").and_then(Value::as_u64) != Some(u64::from(expected_id))
        {
            return Err(Error::Json);
        }

        match (object.remove("result"), object.remove("error")) {
            (Some(Value::Object(result)), None) => Ok(Self::Result(result)),
            (None, Some(Value::Object(error))) => serde_json::from_value(Value::Object(error))
                .map(Self::Error)
                .map_err(|_| Error::Json),
            _ => Err(Error::Json),
        }
    }
}

/// Converts capability flags with the pinned backwards compatibility behavior.
pub fn map_capabilities(
    mut result: Map<String, Value>,
    version: ProtocolVersion,
) -> Result<Map<String, Value>, Error> {
    if version == ProtocolVersion::Legacy {
        let mut features = vec![Value::String("solana:signTransactions".into())];

        if result.get("supports_clone_authorization") == Some(&Value::Bool(true)) {
            features.push(Value::String("solana:cloneAuthorization".into()));
        }

        result.insert("features".into(), Value::Array(features));

        return Ok(result);
    }

    let features: Vec<String> = match result.get("features") {
        None | Some(Value::Null) => Vec::new(),
        Some(value) => serde_json::from_value(value.clone()).map_err(|_| Error::Json)?,
    };

    result.insert(
        "supports_sign_and_send_transactions".into(),
        Value::Bool(true),
    );
    result.insert(
        "supports_clone_authorization".into(),
        Value::Bool(
            features
                .iter()
                .any(|feature| feature == "solana:cloneAuthorization"),
        ),
    );
    Ok(result)
}

/// A caller-owned request transport. Dropping the returned future cancels locally.
/// Implementations own actual transport teardown and must document remote effects.
pub trait MobileWallet {
    /// Concrete future permits local executors without a Send requirement.
    type Call<'a>: Future<Output = Result<Response, Error>> + 'a
    where
        Self: 'a;
    /// Sends one already-mapped request without starting detached tasks.
    fn call<'a>(&'a mut self, request: &'a Request) -> Self::Call<'a>;
}
