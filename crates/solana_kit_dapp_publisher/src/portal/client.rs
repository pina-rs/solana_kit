//! One independently correlated portal request, with no ambient configuration or tasks.

use super::{Error, Operation, response};
use solana_kit_client::rest::{self, Budgets, Transport};

/// Caller-approved portal origin and authentication namespace, separate from uploads.
///
/// The adapter maps this identity to an approved API root and adds the source
/// `accept: application/json` and caller-provided `x-api-key` headers. Neither
/// root nor credential comes from request or response fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PortalService;

/// Typed portal protocol. The adapter owns origin, credentials, TLS and cancellation.
pub struct Client<T> {
    transport: T,
    pub(super) budgets: Budgets,
}

impl<T: Transport<PortalService>> Client<T> {
    /// Creates no I/O, executor, connection or credential lookup.
    pub const fn new(transport: T, budgets: Budgets) -> Self {
        Self { transport, budgets }
    }

    /// Sends exactly one typed operation and decodes its bounded object response.
    ///
    /// Prepared transactions remain untrusted proposals for the caller's separate
    /// policy/review/signing boundary. Submission operations require explicit calls.
    pub async fn call<O: Operation>(&self, operation: &O) -> Result<O::Response, Error> {
        let input = self.budgets.serialize(operation).map_err(Error::Json)?;
        let (method, query, body) = if O::QUERY {
            let input =
                String::from_utf8(input).map_err(|_| Error::Envelope("serialized UTF-8"))?;
            (
                rest::Method::Get,
                vec![("input".into(), input)],
                rest::Body::Absent,
            )
        } else {
            (rest::Method::Post, Vec::new(), rest::Body::Json(input))
        };

        let request = rest::Request {
            service: PortalService,
            method,
            path_segments: vec!["trpc".into(), O::PROCEDURE.into()],
            query,
            body,
            max_reply_bytes: self.budgets.bytes,
        };

        rest::request_target(
            "/",
            &request.path_segments,
            &request.query,
            self.budgets.bytes,
        )
        .map_err(Error::Transport)?;
        let reply = self
            .transport
            .exchange(request)
            .await
            .map_err(|error| Error::Transport(rest::redact_error(error)))?;

        response::decode(reply, self.budgets)
    }
}
