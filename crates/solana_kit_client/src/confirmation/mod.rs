//! Portable confirmation policy with caller-owned observation and cleanup.
//!
//! This module uses `core` and `alloc` machinery and adds no runtime dependency.
//! The surrounding RPC client still requires `std`. Observers own transport,
//! timers, subscription registration and authenticated nonce decoding. A nonce
//! observation is an adapter assertion, never proof from arbitrary caller input.
#[cfg(feature = "confirmation-factories")]
mod factory;
#[cfg(feature = "confirmation-factories")]
mod factory_errors;
mod send;
#[cfg(feature = "confirmation-factories")]
pub use factory::{FactoryConfig, FactoryError, FactoryFailure, NonceError, confirm_signature};

#[cfg(feature = "confirmation-factories")]
mod reconcile;
#[cfg(feature = "confirmation-factories")]
pub use reconcile::reconcile;

#[cfg(feature = "confirmation-factories")]
mod watch;
#[cfg(feature = "confirmation-factories")]
pub use watch::ConfirmationWatch;
#[cfg(feature = "confirmation-factories")]
mod shared;
#[cfg(feature = "confirmation-factories")]
pub use shared::{SessionAccess, confirm_signature_shared};
#[cfg(feature = "confirmation-factories")]
mod submit;

#[cfg(feature = "confirmation-factories")]
pub use submit::submit_and_confirm;

mod state;
pub use send::send_and_confirm;
mod wait;

pub use state::{Confirmation, Event, Failure, Policy, commitment_comparator, timeout_millis};
pub use wait::{Observer, WaitError, wait};
