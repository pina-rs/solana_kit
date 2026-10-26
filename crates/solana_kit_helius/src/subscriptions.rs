//! Helius JSON subscriptions reuse the standard typed schemas and bounded owner.
//!
//! Identical requests share one physical registration with independent local queues.

use solana_kit_client::{
    Error,
    methods::Method,
    subscriptions::{ProviderMethod, SubscriptionMethod},
};

/// A standard typed JSON subscription routed through the Helius provider bridge.
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(transparent)]
pub struct JsonSubscription<M>(pub M);

impl<M: SubscriptionMethod> Method for JsonSubscription<M> {
    type Response = M::Response;
    const NAME: &'static str = M::NAME;

    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        self.0.decode(result)
    }
}

impl<M: SubscriptionMethod + 'static> ProviderMethod for JsonSubscription<M> {
    const SHARE: bool = true;
    const UNSUBSCRIBE: &'static str = M::UNSUBSCRIBE;
    const NOTIFICATION: &'static str = M::NOTIFICATION;
    const UNSTABLE: bool = M::UNSTABLE;
    const NOTIFICATION_KIND: solana_kit_client::subscriptions::NotificationKind =
        M::NOTIFICATION_KIND;

    fn decode_binary(&self, bytes: &[u8]) -> Result<Self::Response, Error> {
        self.0.decode_binary(bytes)
    }

    fn completes(notification: &Self::Response) -> bool {
        M::completes(notification)
    }
}
