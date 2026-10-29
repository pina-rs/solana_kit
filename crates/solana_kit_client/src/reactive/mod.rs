//! Executor-neutral reactive state with explicitly owned, cancellable work.
//!
//! Dispatch/connect eagerly publish state and return a driver the caller must
//! poll. Dropping it cancels its actual future or streams. Listeners are owned by
//! their leases; retaining a store alone never retains a callback cycle.
mod action;
mod bridge;
mod cancellation;
mod shared;
pub mod slot_tracking;
mod stream;

pub use action::{ActionDispatch, ActionSnapshot, LocalReactiveActionStore, ReactiveActionStore};
pub use bridge::{BridgeFailure, StoreStream};
pub use cancellation::{CancellationReason, CancellationSource, CancellationToken, Cancelled};
pub use shared::{ListenerLease, ReactiveFailure};
pub use stream::{
    LocalReactiveStreamStore, ReactiveStreamConnection, ReactiveStreamStore, StreamDriver,
    StreamSnapshot,
};
