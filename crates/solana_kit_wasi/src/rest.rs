//! Provider REST uses the existing Preview2 HTTP resources and Reactor.

use crate::http::{ConfigurationError, Exchange, WasiHttp};
use solana_kit_client::{TransportError, TransportFailureKind as Kind, rest};
use wasip2::http::types::Method;

/// Explicit approved provider routes over caller-driven Preview2 HTTP.
pub struct WasiRest<D> {
    routes: Vec<(D, WasiHttp)>,
    max_target_bytes: usize,
}

impl<D> std::fmt::Debug for WasiRest<D> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WasiRest")
            .field("route_count", &self.routes.len())
            .finish_non_exhaustive()
    }
}

impl<D: Eq> WasiRest<D> {
    /// Binds unique named roots, headers and deadlines without a new reactor.
    pub fn new(
        routes: Vec<(D, WasiHttp)>,
        max_target_bytes: usize,
    ) -> Result<Self, ConfigurationError> {
        if routes.is_empty() || max_target_bytes == 0 {
            return Err(ConfigurationError::Routes);
        }

        for (index, (name, adapter)) in routes.iter().enumerate() {
            if routes[..index].iter().any(|(other, _)| name == other) {
                return Err(ConfigurationError::Routes);
            }

            if adapter.headers.iter().any(|(name, _)| {
                matches!(
                    name.to_ascii_lowercase().as_str(),
                    "trailer"
                        | "expect"
                        | "proxy-authorization"
                        | "proxy-connection"
                        | "keep-alive"
                        | "te"
                )
            }) {
                return Err(ConfigurationError::Headers);
            }
        }

        Ok(Self {
            routes,
            max_target_bytes,
        })
    }
}

impl<D: Eq> rest::Transport<D> for WasiRest<D> {
    type Exchange<'a>
        = Exchange<'a>
    where
        Self: 'a;

    fn exchange(&self, request: rest::Request<D>) -> Self::Exchange<'_> {
        Exchange(Box::pin(async move {
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
            let method = match request.method {
                rest::Method::Get => Method::Get,
                rest::Method::Post => Method::Post,
                rest::Method::Put => Method::Put,
                rest::Method::Patch => Method::Patch,
                rest::Method::Delete => Method::Delete,
            };
            adapter
                .send(method, &target, request.body, request.max_reply_bytes)
                .await
                .map_err(rest::redact_error)
        }))
    }
}
