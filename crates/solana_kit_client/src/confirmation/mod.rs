//! Portable confirmation policy with caller-owned observation and cleanup.
//!
//! This module uses `core` and `alloc` machinery and adds no runtime dependency.
//! The surrounding RPC client still requires `std`. Observers own transport,
//! timers, subscription registration and authenticated nonce decoding. A nonce
//! observation is an adapter assertion, never proof from arbitrary caller input.
mod send;
mod state;
pub use send::send_and_confirm;
mod wait;

pub use state::{Confirmation, Event, Failure, Policy, commitment_comparator, timeout_millis};
pub use wait::{Observer, WaitError, wait};
