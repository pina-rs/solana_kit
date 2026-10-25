//! Per-signature state that never owns or persistently borrows the shared socket.
use super::{Confirmation, Event, FactoryConfig, FactoryFailure, Failure};
use crate::subscriptions::{
    Drain, Session, SessionError, SignatureConfig, SignatureNotifications, Subscription, TextSocket,
};
use core::task::{Context, Poll};
use solana_kit_core::{Signature, transaction::Lifetime};

enum Registration {
    Active(Subscription<SignatureNotifications>),
    Closing(Subscription<SignatureNotifications>),
    Closed,
}
/// One confirmation operation on an application-owned, shared Session.
///
/// The application flushes registration before starting `reconcile`, schedules
/// reads at `next_read`, and feeds their results into `observe`. Polls take only
/// transient socket borrows. Other watches and typed registrations retain their
/// queues. Cleanup removes only this watch; the application closes the Session.
/// Dropping a watch cannot acknowledge remote cleanup. Call `poll_cleanup` first.
pub struct ConfirmationWatch {
    signature: Signature,
    reducer: Confirmation,
    config: FactoryConfig,
    registration: Registration,
    next_read: u64,
}
impl ConfirmationWatch {
    /// Installs routing synchronously before the subscribe frame can be sent.
    pub fn register<S: TextSocket>(
        session: &mut Session<S>,
        signature: Signature,
        lifetime: Lifetime,
        config: FactoryConfig,
    ) -> Result<Self, FactoryFailure<S::Error>> {
        let reducer = Confirmation::new(signature, lifetime, config.policy)
            .map_err(FactoryFailure::Confirmation)?;
        Self::register_reducer(session, signature, reducer, config)
    }
    /// Installs the reducer already validated by the existing submission guard.
    pub(super) fn register_reducer<S: TextSocket>(
        session: &mut Session<S>,
        signature: Signature,
        reducer: Confirmation,
        config: FactoryConfig,
    ) -> Result<Self, FactoryFailure<S::Error>> {
        let handle = session
            .subscribe(
                SignatureNotifications(
                    signature,
                    Some(SignatureConfig {
                        commitment: Some(config.policy.commitment),
                        enable_received_notification: Some(true),
                    }),
                ),
                config.deadline,
            )
            .map_err(FactoryFailure::Session)?;

        Ok(Self {
            signature,
            reducer,
            config,
            registration: Registration::Active(handle),
            next_read: 0,
        })
    }
    /// Earliest safe read tick. The caller owns the timer and HTTP future.
    #[must_use]
    pub const fn next_read(&self) -> u64 {
        self.next_read
    }
    /// Applies source-authenticated observations from `reconcile`.
    ///
    /// A stale final status leaves this operation pending. Schedule another read
    /// at `next_read` and keep polling cancellation and the overall deadline.
    pub fn observe(&mut self, events: Vec<Event>, now: u64) -> Option<Result<(), Failure>> {
        self.next_read = now.saturating_add(self.config.poll_interval.get());
        events
            .into_iter()
            .find_map(|event| self.reducer.observe(event))
    }
    /// Polls this registration while retaining other registrations' messages.
    /// The application must schedule an injected timer at the operation deadline.
    pub fn poll_notification<S: TextSocket>(
        &mut self,
        session: &mut Session<S>,
        cx: &mut Context<'_>,
        now: u64,
    ) -> Poll<Result<(), FactoryFailure<S::Error>>> {
        if now >= self.config.deadline {
            return Poll::Ready(Err(FactoryFailure::Confirmation(Failure::Timeout)));
        }

        let Registration::Active(handle) = &self.registration else {
            return Poll::Ready(Err(FactoryFailure::Confirmation(Failure::AlreadySettled)));
        };
        let observation =
            match core::task::ready!(session.poll_next(handle, cx, now, self.config.deadline))
                .map_err(FactoryFailure::Session)?
            {
                Drain::Notification(value) => Event::Signature {
                    signature: self.signature,
                    slot: value.context.slot,
                    commitment: self.config.policy.commitment,
                    value: value.value,
                },
                Drain::Closed => {
                    return Poll::Ready(Err(FactoryFailure::Confirmation(Failure::ObserverEnded)));
                }
                Drain::Pending => return Poll::Pending,
            };

        match self.reducer.observe(observation) {
            Some(result) => Poll::Ready(result.map_err(FactoryFailure::Confirmation)),
            None => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
    /// Cancels and awaits only this registration's remote acknowledgement.
    /// The caller schedules an injected timer at the supplied cleanup deadline.
    pub fn poll_cleanup<S: TextSocket>(
        &mut self,
        session: &mut Session<S>,
        cx: &mut Context<'_>,
        now: u64,
        deadline: u64,
    ) -> Poll<Result<(), SessionError<S::Error>>> {
        if matches!(self.registration, Registration::Closed) {
            return Poll::Ready(Ok(()));
        }

        if now >= deadline {
            return Poll::Ready(Err(SessionError::Deadline));
        }

        if let Registration::Active(handle) = &self.registration {
            session.cancel(handle, deadline)?;
            let Registration::Active(handle) =
                core::mem::replace(&mut self.registration, Registration::Closed)
            else {
                unreachable!();
            };
            self.registration = Registration::Closing(handle);
        }

        let Registration::Closing(handle) = &self.registration else {
            unreachable!()
        };
        core::task::ready!(session.poll_next(handle, cx, now, deadline))?;
        let Registration::Closing(handle) =
            core::mem::replace(&mut self.registration, Registration::Closed)
        else {
            unreachable!();
        };

        Poll::Ready(session.forget(handle))
    }
}
