//! Bounded subscription protocol for caller-owned off-chain client I/O.
//!
//! Send each frame returned by [`crate::subscriptions::Connection::outbound`].
//! Pass complete text frames to [`crate::subscriptions::Connection::ingest`].
//! The caller owns sockets, timers and polling.
//! Replacement reports a gap and never replays or reconnects automatically.
mod owner;
mod requests;
mod responses;
pub use owner::*;
pub use requests::*;
pub use responses::*;

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

    /// Whether this notification completes the server's one-shot subscription.
    fn completes(_notification: &Self::Response) -> bool {
        false
    }
}
