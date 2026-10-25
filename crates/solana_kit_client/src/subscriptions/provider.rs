//! Explicit typed provider extensions reuse the sealed protocol owner.

use super::{SubscriptionMethod, sealed};
use crate::{Error, methods::Method};
use serde::{Serialize, Serializer};

/// Notification representation selected by one registered typed method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationKind {
    /// JSON-RPC notifications with an exact method and server subscription ID.
    Json,
    /// Binary frames without a server subscription ID.
    ///
    /// A nonempty constant names the binary protocol occupying this connection.
    /// Frames are broadcast only to live registrations in that namespace with
    /// identical subscribe, unsubscribe and notification method names. A distinct
    /// simultaneous binary family is rejected while any earlier registration is
    /// unsettled or active. The schema author declares an unfiltered broadcast
    /// protocol; this representation cannot correlate per-request binary filters.
    Binary(&'static str),
}

/// A typed vendor subscription schema. It does not own a socket or spawn tasks.
///
/// JSON result decoding uses Method::decode. Binary families must opt into one
/// named connection namespace and implement decode_binary. All registration,
/// correlation, queues, cancellation and true acknowledgements remain in Connection.
pub trait ProviderMethod: Method + 'static {
    /// Coalesces identical requests within this concrete schema and connection.
    /// Only the final local handle sends the physical unsubscribe.
    /// Uncertain subscribe or unsubscribe settlement quarantines the key until
    /// connection replacement; known server IDs remain reserved while uncertain.
    const SHARE: bool = false;
    /// Exact unsubscribe method.
    const UNSUBSCRIBE: &'static str;
    /// Exact JSON notification method, including for providers that also use binary frames.
    const NOTIFICATION: &'static str;
    /// Whether explicit server enablement is required.
    const UNSTABLE: bool = false;
    /// Notification representation and any binary namespace.
    const NOTIFICATION_KIND: NotificationKind = NotificationKind::Json;

    /// Decodes one already bounded binary frame. The default rejects binary data.
    fn decode_binary(&self, _bytes: &[u8]) -> Result<Self::Response, Error> {
        Err(Error::Envelope)
    }

    /// Reports whether this notification completes a one-shot subscription.
    fn completes(_notification: &Self::Response) -> bool {
        false
    }
}

/// Explicitly admits one provider-defined schema to the existing sealed owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Provider<M>(pub M);

impl<M: Serialize> Serialize for Provider<M> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<M: ProviderMethod> Method for Provider<M> {
    type Response = M::Response;
    const NAME: &'static str = M::NAME;

    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        self.0.decode(result)
    }
}

impl<M: ProviderMethod> sealed::Sealed for Provider<M> {}

impl<M: ProviderMethod> SubscriptionMethod for Provider<M> {
    const UNSUBSCRIBE: &'static str = M::UNSUBSCRIBE;
    const NOTIFICATION: &'static str = M::NOTIFICATION;
    const UNSTABLE: bool = M::UNSTABLE;
    const NOTIFICATION_KIND: NotificationKind = M::NOTIFICATION_KIND;

    fn schema_identity() -> Option<core::any::TypeId> {
        Some(core::any::TypeId::of::<M>())
    }

    fn sharing_identity() -> Option<core::any::TypeId> {
        M::SHARE.then(core::any::TypeId::of::<M>)
    }

    fn decode_binary(&self, bytes: &[u8]) -> Result<Self::Response, Error> {
        self.0.decode_binary(bytes)
    }

    fn completes(notification: &Self::Response) -> bool {
        M::completes(notification)
    }
}
