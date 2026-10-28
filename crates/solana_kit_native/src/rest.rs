//! Named REST routes reuse the native HTTP socket, TLS and response owner.

use crate::{ConfigError, NativeExchange, NativeHttp, exchange};
use solana_kit_client::{TransportError, TransportFailureKind as Kind, rest};
use std::fmt;

/// Explicit destination-to-root policies for native provider requests.
///
/// Each `NativeHttp` retains its caller-approved endpoint, headers, TLS roots and
/// deadlines. A root may contain a fixed authentication query. Request data only
/// appends encoded relative segments and query pairs. No redirects are followed.
/// Remote roots require HTTPS. Plain HTTP is limited to literal loopback IPs for
/// caller-owned local services; names such as `localhost` are not exempted.
pub struct NativeRest<D> {
    routes: Vec<(D, NativeHttp)>,
    max_target_bytes: usize,
}

impl<D: Eq> NativeRest<D> {
    /// Binds unique named routes and an explicit encoded request-target limit.
    ///
    /// Rejects remote plaintext roots before DNS or socket work, regardless of
    /// whether the caller has attached credentials. No plaintext remote opt-in
    /// exists. Loopback HTTP does not protect against other local processes.
    pub fn new(routes: Vec<(D, NativeHttp)>, max_target_bytes: usize) -> Result<Self, ConfigError> {
        if routes.is_empty() || max_target_bytes == 0 {
            return Err(ConfigError(
                "REST routes and target budget must be nonempty",
            ));
        }

        for (index, (name, adapter)) in routes.iter().enumerate() {
            if !adapter.endpoint.secure
                && !adapter
                    .endpoint
                    .host
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|ip| ip.is_loopback())
            {
                return Err(ConfigError("remote REST routes require HTTPS"));
            }

            if routes[..index].iter().any(|(other, _)| name == other) {
                return Err(ConfigError("duplicate REST destination"));
            }
        }

        Ok(Self {
            routes,
            max_target_bytes,
        })
    }
}

impl<D> fmt::Debug for NativeRest<D> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("NativeRest")
            .field("route_count", &self.routes.len())
            .field("max_target_bytes", &self.max_target_bytes)
            .finish_non_exhaustive()
    }
}

impl<D: Eq + Send + Sync> rest::Transport<D> for NativeRest<D> {
    type Exchange<'a>
        = NativeExchange<'a>
    where
        Self: 'a;

    fn exchange(&self, request: rest::Request<D>) -> Self::Exchange<'_> {
        NativeExchange {
            future: Box::pin(async move {
                let adapter = self
                    .routes
                    .iter()
                    .find(|(name, _)| *name == request.service)
                    .map(|(_, adapter)| adapter)
                    .ok_or_else(|| TransportError::new(Kind::Denied))?;
                let target = rest::request_target(
                    &adapter.endpoint.path,
                    &request.path_segments,
                    &request.query,
                    self.max_target_bytes,
                )?;
                target.parse::<hyper::http::Uri>().map_err(|error| {
                    rest::redact_error(TransportError::with_source(Kind::Denied, error))
                })?;
                let content_type = request.body.content_type();
                let outgoing = exchange::HttpRequest {
                    method: request.method.as_str(),
                    target: target.into(),
                    body: request.body.into_bytes(),
                    content_type,
                    max_reply_bytes: request.max_reply_bytes,
                };
                exchange::execute_http(adapter, outgoing)
                    .await
                    .map_err(rest::redact_error)
            }),
        }
    }
}
