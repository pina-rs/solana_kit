//! Typed execution results retain the planned immutable message and caller context.
use super::Plan;
use crate::{Signature, transaction::Message};
use alloc::vec::Vec;

/// The terminal state of one transaction operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Status<E> {
    /// Successful submission or signing, with the caller's explicit signature.
    Successful(Signature),
    /// Failure retains the original error value.
    Failed(E),
    /// The leaf never started because execution was canceled.
    Canceled,
}

/// One terminal leaf, including partial context on failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeafResult<C, E> {
    /// Exact immutable message chosen during planning.
    pub planned_message: Message,
    /// Caller-owned typed progress, retained on success and failure.
    pub context: C,
    /// Terminal execution status.
    pub status: Status<E>,
}

/// Result tree retaining the transaction plan's original structure.
pub type TransactionPlanResult<C, E> = Plan<LeafResult<C, E>>;

/// A borrowed classification of every leaf without cloning errors or context.
#[derive(Debug)]
pub struct ResultSummary<'a, C, E> {
    /// Successful leaves in plan order.
    pub successful: Vec<&'a LeafResult<C, E>>,
    /// Failed leaves in plan order.
    pub failed: Vec<&'a LeafResult<C, E>>,
    /// Unstarted canceled leaves in plan order.
    pub canceled: Vec<&'a LeafResult<C, E>>,
}
impl<C, E> ResultSummary<'_, C, E> {
    /// Whether every leaf completed successfully, including the empty tree.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.failed.is_empty() && self.canceled.is_empty()
    }
}
impl<C, E> Plan<LeafResult<C, E>> {
    /// Categorizes every result leaf in depth-first plan order.
    #[must_use]
    pub fn summarize(&self) -> ResultSummary<'_, C, E> {
        let mut summary = ResultSummary {
            successful: Vec::new(),
            failed: Vec::new(),
            canceled: Vec::new(),
        };

        for leaf in self.flatten() {
            match leaf.status {
                Status::Successful(_) => summary.successful.push(leaf),
                Status::Failed(_) => summary.failed.push(leaf),
                Status::Canceled => summary.canceled.push(leaf),
            }
        }

        summary
    }

    /// First failed leaf, or absence when there is no failure.
    #[must_use]
    pub fn first_failure(&self) -> Option<&LeafResult<C, E>> {
        self.flatten()
            .into_iter()
            .find(|leaf| matches!(leaf.status, Status::Failed(_)))
    }
}
