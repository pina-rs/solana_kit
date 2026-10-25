//! Provider requests use actual upstream Calls and application-routed Events.

use crate::{guest::GuestExchange, rest_wire, wire};
use leptoskit_bridge::guest::Client;
use solana_kit_client::{TransportError, TransportFailureKind as Kind, rest};

/// Provider destination names bound to explicitly granted fixed host services.
pub struct GuestRest<D> {
    routes: Vec<(D, Client)>,
    max_request_bytes: usize,
}

impl<D> std::fmt::Debug for GuestRest<D> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("GuestRest")
            .field("route_count", &self.routes.len())
            .finish_non_exhaustive()
    }
}

impl<D: Eq> GuestRest<D> {
    /// Resolves existing named services and sets the complete wire request cap.
    pub fn new(routes: Vec<(D, String)>, max_request_bytes: usize) -> Result<Self, TransportError> {
        if routes.is_empty() || max_request_bytes == 0 || max_request_bytes > u32::MAX as usize {
            return Err(TransportError::new(Kind::Denied));
        }

        let mut resolved = Vec::new();

        for (name, service) in routes {
            if resolved.iter().any(|(other, _)| name == *other) {
                return Err(TransportError::new(Kind::Denied));
            }

            let client = Client::new(&service).map_err(wire::bridge_error)?;
            resolved.push((name, client));
        }

        Ok(Self {
            routes: resolved,
            max_request_bytes,
        })
    }
}

impl<D: Eq> rest::Transport<D> for GuestRest<D> {
    type Exchange<'a>
        = GuestExchange<'a>
    where
        Self: 'a;

    fn exchange(&self, request: rest::Request<D>) -> Self::Exchange<'_> {
        GuestExchange {
            future: Box::pin(async move {
                let client = self
                    .routes
                    .iter()
                    .find(|(name, _)| *name == request.service)
                    .map(|(_, client)| client)
                    .ok_or_else(|| TransportError::new(Kind::Denied))?;
                let limit = request.max_reply_bytes;
                let payload = rest_wire::encode(
                    rest::Request {
                        service: (),
                        method: request.method,
                        path_segments: request.path_segments,
                        query: request.query,
                        body: request.body,
                        max_reply_bytes: limit,
                    },
                    self.max_request_bytes,
                )?;
                let response = client
                    .call(rest_wire::REST, &payload)
                    .map_err(wire::bridge_error)?
                    .await
                    .map_err(wire::bridge_error)?;
                wire::parse_reply(&response, limit)
            }),
        }
    }
}
