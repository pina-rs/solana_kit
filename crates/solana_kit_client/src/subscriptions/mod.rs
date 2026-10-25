//! Bounded subscription protocol for caller-owned off-chain client I/O.
//!
//! Send each frame returned by [`crate::subscriptions::Connection::outbound`].
//! Pass complete text frames to [`crate::subscriptions::Connection::ingest`].
//! The caller owns sockets, timers and polling.
//! Replacement reports a gap and never replays or reconnects automatically.
mod clock;
mod owner;
mod provider;
mod requests;
mod responses;
mod session;
pub use clock::{Clock, ClockError};
pub use owner::*;
pub use provider::{NotificationKind, Provider, ProviderMethod};
pub use requests::*;
pub use responses::*;
pub use session::{NotificationPoll, ReceivedFrame, Session, SessionError, TextSocket};

mod sealed {
    pub trait Sealed {}
}

/// Closed catalog of typed subscription requests and notification schemas.
pub trait SubscriptionMethod: crate::methods::Method + sealed::Sealed {
    /// Exact unsubscribe method.
    const UNSUBSCRIBE: &'static str;
    /// Exact incoming notification method.
    const NOTIFICATION: &'static str;
    /// Whether the server must explicitly enable this unstable family.
    const UNSTABLE: bool = false;
    /// Text JSON by default. Binary providers explicitly name their socket namespace.
    const NOTIFICATION_KIND: NotificationKind = NotificationKind::Json;

    /// Concrete provider decoder identity. Built-in families remain sealed and unshared.
    fn schema_identity() -> Option<core::any::TypeId> {
        None
    }

    /// Explicit physical-sharing opt-in. Built-in families always return None.
    fn sharing_identity() -> Option<core::any::TypeId> {
        None
    }

    /// Decodes an explicitly binary notification. Standard JSON families reject it.
    fn decode_binary(&self, _bytes: &[u8]) -> Result<Self::Response, crate::Error> {
        Err(crate::Error::Envelope)
    }

    /// Whether this notification completes the server's one-shot subscription.
    fn completes(_notification: &Self::Response) -> bool {
        false
    }
}
