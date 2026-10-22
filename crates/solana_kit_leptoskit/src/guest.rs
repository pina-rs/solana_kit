//! Uses upstream Call ownership and registered wakers without new FFI.
use crate::wire;
use leptoskit_bridge::guest::Client;
use solana_kit_client::{Reply, Request, Transport, TransportError};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// A fixed host-granted named service; the guest cannot choose a destination.
#[derive(Debug, Clone, Copy)]
pub struct GuestHttp(Client);

impl GuestHttp {
    /// Resolves an installed service; absent services fail without fabricated data.
    pub fn new(service: &str) -> Result<Self, TransportError> {
        Client::new(service).map(Self).map_err(wire::bridge_error)
    }
}

/// Local cancel-on-drop future using the actual upstream plugin call registry.
pub struct GuestExchange<'a> {
    future: Pin<Box<dyn Future<Output = Result<Reply, TransportError>> + 'a>>,
}

impl Future for GuestExchange<'_> {
    type Output = Result<Reply, TransportError>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        self.future.as_mut().poll(context)
    }
}

impl Transport for GuestHttp {
    type Exchange<'a> = GuestExchange<'a>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        GuestExchange {
            future: Box::pin(async move {
                let limit = request.max_reply_bytes;
                let payload = wire::request(request)?;
                let response = self
                    .0
                    .call(wire::POST, &payload)
                    .map_err(wire::bridge_error)?
                    .await
                    .map_err(wire::bridge_error)?;
                wire::parse_reply(&response, limit)
            }),
        }
    }
}
