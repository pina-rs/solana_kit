//! Caller-owned construction futures, with a ready adapter for pure policies.
use super::{Error, MessageDraft};
use core::future::{Future, Ready, ready};

/// Synchronous local construction policy. Neither method may submit transactions.
pub trait MessageBuilder {
    /// Creates a fresh draft, including reserved instructions.
    ///
    /// # Errors
    /// Returns the caller's construction error.
    fn create(&mut self) -> Result<MessageDraft, Error>;
    /// Updates a proposed draft before exact compilation.
    ///
    /// # Errors
    /// Any error after a stateful packer advanced aborts the whole plan.
    fn update(&mut self, draft: MessageDraft) -> Result<MessageDraft, Error> {
        Ok(draft)
    }
}

/// Construction policy awaited at each actual candidate decision.
///
/// Each future may borrow the builder and owns its proposed draft. The planner
/// awaits it before compilation or commitment. Dropping planning drops the active
/// future and all drafts; external effects already performed cannot be rolled back.
/// There is no executor, retry, detached task, or universal `Send` requirement.
/// Pure [`MessageBuilder`] implementations automatically use ready futures.
pub trait AsyncMessageBuilder {
    /// Concrete construction future, borrowing only this policy.
    type Create<'a>: Future<Output = Result<MessageDraft, Error>>
    where
        Self: 'a;
    /// Concrete update future, owning its proposed draft.
    type Update<'a>: Future<Output = Result<MessageDraft, Error>>
    where
        Self: 'a;
    /// Starts construction only when traversal requires a fresh candidate.
    fn create_async(&mut self) -> Self::Create<'_>;
    /// Starts updating the actual proposal before its dependent fit decision.
    fn update_async(&mut self, draft: MessageDraft) -> Self::Update<'_>;
}

impl<B: MessageBuilder> AsyncMessageBuilder for B {
    type Create<'a>
        = Ready<Result<MessageDraft, Error>>
    where
        B: 'a;
    type Update<'a>
        = Ready<Result<MessageDraft, Error>>
    where
        B: 'a;

    fn create_async(&mut self) -> Self::Create<'_> {
        ready(MessageBuilder::create(self))
    }

    fn update_async(&mut self, draft: MessageDraft) -> Self::Update<'_> {
        ready(MessageBuilder::update(self, draft))
    }
}
