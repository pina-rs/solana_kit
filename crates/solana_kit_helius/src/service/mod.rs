//! Typed REST operations over caller-owned, bounded HTTP exchanges.

mod error;
mod form;

use serde::Serialize;
use solana_kit_client::rest;

pub use error::Error;
pub use rest::{Budgets, Method as Verb, Transport as ServiceTransport};

/// An independently approved provider origin and credential policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Service {
    /// Helius API origin, with an API key supplied by the adapter.
    Api,
    /// Webhook API at api-mainnet.helius-rpc.com; approved separately from other API origins.
    Webhooks,
    /// Wallet v1 beta origin; approved independently from the legacy Enhanced Transactions API.
    Wallet,
    /// Helius authentication API origin.
    Auth,
    /// Helius administration origin, with x-api-key supplied by the adapter.
    Admin,
    /// Helius transaction sender origin.
    Sender,
    /// Developer dashboard API origin with caller-owned bearer authentication.
    Developer,
}

/// Whether the request has a JSON document. Omission differs from JSON null.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Body {
    /// Send no body.
    Absent,
    /// Serialize the operation as JSON, including null if its schema is null.
    Json,
    /// Encode the operation's ordered form fields as application/x-www-form-urlencoded.
    Form,
}

/// One shared REST request addressed to an approved Helius service slot.
pub type ServiceRequest = rest::Request<Service>;

/// Typed REST schema, kept distinct from JSON-RPC framing.
pub trait Operation: Serialize {
    /// Typed response decoded only after status and budgets pass.
    type Response: serde::de::DeserializeOwned;
    /// Destination slot chosen by the operation, never a server-provided URL.
    const SERVICE: Service;
    /// HTTP verb.
    const VERB: Verb;
    /// Explicit body presence.
    const BODY: Body;

    /// Ordered path segments, relative to the adapter's configured service root.
    fn path(&self) -> Vec<String>;

    /// Ordered query values. Authentication is added by the application adapter.
    fn query(&self) -> Vec<(String, String)> {
        Vec::new()
    }

    /// Ordered form fields for Body::Form. No ambient credentials are resolved.
    fn form(&self) -> Vec<(String, String)> {
        Vec::new()
    }

    /// Decodes a bounded valid JSON success body. Endpoint-specific envelopes may override this.
    fn decode(&self, bytes: &[u8]) -> Result<Self::Response, Error> {
        serde_json::from_slice(bytes).map_err(Error::Schema)
    }
}

/// A REST protocol owner without hidden runtime, credentials or retry state.
pub struct ServiceClient<T> {
    transport: T,
    budgets: Budgets,
}

impl<T: ServiceTransport<Service>> ServiceClient<T> {
    /// Associates an adapter with explicit allocation budgets.
    pub const fn new(transport: T, budgets: Budgets) -> Self {
        Self { transport, budgets }
    }

    /// Executes one typed exchange. Dropping this future drops its owned exchange.
    pub async fn call<O: Operation>(&self, operation: O) -> Result<O::Response, Error> {
        let request = self.request(&operation)?;
        let reply = self
            .transport
            .exchange(request)
            .await
            .map_err(Error::Transport)?;
        self.budgets.check_bytes(&reply.body)?;

        if !(200..300).contains(&reply.status) {
            return Err(Error::Http {
                status: reply.status,
                body: reply.body,
            });
        }

        let body = if reply.body.is_empty() {
            b"null".as_slice()
        } else {
            &reply.body
        };

        self.budgets.validate_json(body)?;
        operation.decode(body)
    }

    fn request<O: Operation>(&self, operation: &O) -> Result<ServiceRequest, Error> {
        let body = match O::BODY {
            Body::Absent => rest::Body::Absent,
            Body::Json => rest::Body::Json(self.budgets.serialize(operation)?),
            Body::Form => rest::Body::Form(form::encode(operation.form(), self.budgets)?),
        };

        Ok(ServiceRequest {
            service: O::SERVICE,
            method: O::VERB,
            path_segments: operation.path(),
            query: operation.query(),
            body,
            max_reply_bytes: self.budgets.bytes,
        })
    }
}

pub(crate) fn pair<T: ToString>(query: &mut Vec<(String, String)>, key: &str, value: Option<T>) {
    if let Some(value) = value {
        query.push((key.to_owned(), value.to_string()));
    }
}
