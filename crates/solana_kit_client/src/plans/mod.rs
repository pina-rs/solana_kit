//! Caller-owned transaction plan execution with structured concurrency.
//!
//! Leaf futures own their message and borrow only the external operation owner.
//! Dropping execution drops every pending leaf. Parallel results retain plan order.
mod scheduler;
pub use scheduler::{ExecutionError, ExecutionMode, Executor, LeafOutcome, execute};
mod boundary;
mod diagnostics;
pub use diagnostics::{FailureAction, FailureDetails, FailureReport};

pub use boundary::{
    Boundary, BoundaryContext, BoundaryExecution, BoundaryFuture, BoundaryOutcome, Sender, Signer,
    StageError,
};

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

/// Explicit cooperative cancellation. Active operations settle; unstarted leaves cancel.
///
/// Callers needing immediate transport teardown drop the execution future.
#[derive(Clone, Debug, Default)]
pub struct Cancellation(Arc<AtomicBool>);
impl Cancellation {
    /// Requests cancellation before the next leaf starts.
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Release);
    }
    /// Whether cancellation has been requested.
    #[must_use]
    pub fn is_canceled(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
}
