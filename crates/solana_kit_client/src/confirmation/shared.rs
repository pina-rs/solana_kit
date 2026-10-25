//! Callable operations whose synchronous polls share the application's sole Session.
use super::{ConfirmationWatch, FactoryConfig, FactoryError, FactoryFailure, Failure, reconcile};
use crate::{
    Client, Transport,
    subscriptions::{Clock, ClockError, Session, SessionError, TextSocket},
};
use core::{
    future::{Future, poll_fn},
    pin::Pin,
    task::{Context, Poll},
};
use solana_kit_core::{Signature, transaction::Lifetime};
use std::{cell::RefCell, sync::Mutex};

/// Grants a short synchronous borrow of the application's sole socket owner.
///
/// The callback cannot return a borrow of Session. Operations never retain this
/// access across an await. Applications may implement their own ownership policy.
pub trait SessionAccess<S: TextSocket> {
    /// Runs one synchronous operation, or reports unavailable shared ownership.
    fn with_session<R>(
        &self,
        operation: impl FnOnce(&mut Session<S>) -> R,
    ) -> Result<R, SessionError<S::Error>>;
}
impl<S: TextSocket> SessionAccess<S> for RefCell<Session<S>> {
    fn with_session<R>(
        &self,
        operation: impl FnOnce(&mut Session<S>) -> R,
    ) -> Result<R, SessionError<S::Error>> {
        let mut session = self
            .try_borrow_mut()
            .map_err(|_| SessionError::Unavailable)?;
        Ok(operation(&mut session))
    }
}
impl<S: TextSocket> SessionAccess<S> for Mutex<Session<S>> {
    fn with_session<R>(
        &self,
        operation: impl FnOnce(&mut Session<S>) -> R,
    ) -> Result<R, SessionError<S::Error>> {
        let mut session = self.try_lock().map_err(|_| SessionError::Unavailable)?;
        Ok(operation(&mut session))
    }
}

/// Confirms with transient access to a shared application-owned Session.
///
/// Registration is flushed before HTTP reconciliation. Cancellation wins ready
/// outcomes. Cleanup awaits only this registration; unrelated watches, queues and
/// the socket remain owned by the application. No executor or task is created.
/// Drive operations together from the application's single notification-poll
/// owner. A failed cleanup requires explicit application shutdown or replacement.
pub async fn confirm_signature_shared<T, S, C, X, A>(
    client: &Client<T>,
    access: &A,
    clock: &C,
    signature: Signature,
    lifetime: Lifetime,
    config: FactoryConfig,
    cancellation: X,
) -> Result<(), FactoryError<S::Error>>
where
    T: Transport,
    S: TextSocket,
    C: Clock,
    X: Future<Output = ()>,
    A: SessionAccess<S>,
{
    let mut cancellation = core::pin::pin!(cancellation);

    let mut watch = match access
        .with_session(|session| ConfirmationWatch::register(session, signature, lifetime, config))
    {
        Ok(Ok(watch)) => watch,

        Ok(Err(error)) => return Err(failed(error)),
        Err(error) => return Err(failed(FactoryFailure::Session(error))),
    };
    let outcome = async {
        flush(access, clock, config.deadline, cancellation.as_mut()).await?;
        observe(
            client,
            access,
            clock,
            &mut watch,
            Subject {
                signature,
                lifetime,
                config,
            },
            cancellation.as_mut(),
        )
        .await
    }
    .await;
    finish(access, clock, &mut watch, config, outcome).await
}

pub(super) fn failed<E>(error: FactoryFailure<E>) -> FactoryError<E> {
    FactoryError {
        outcome: Err(Box::new(error)),
        cleanup: Vec::new(),
    }
}
pub(super) async fn flush<S, A, C, X>(
    access: &A,
    clock: &C,
    deadline: u64,
    mut cancellation: Pin<&mut X>,
) -> Result<(), FactoryFailure<S::Error>>
where
    S: TextSocket,
    A: SessionAccess<S>,
    C: Clock,
    X: Future<Output = ()>,
{
    let mut timeout = core::pin::pin!(clock.sleep_until(deadline));
    poll_fn(|cx| {
        check_stop(cancellation.as_mut(), timeout.as_mut(), cx)?;

        match access.with_session(|session| session.poll_flush(cx)) {
            Ok(poll) => poll.map_err(FactoryFailure::Session),
            Err(error) => Poll::Ready(Err(FactoryFailure::Session(error))),
        }
    })
    .await
}

pub(super) struct Subject {
    pub signature: Signature,
    pub lifetime: Lifetime,
    pub config: FactoryConfig,
}
/// Gives cancellation precedence over a simultaneously ready operation deadline.
pub(super) fn check_stop<E, X: Future<Output = ()>, D: Future<Output = Result<(), ClockError>>>(
    cancellation: Pin<&mut X>,
    deadline: Pin<&mut D>,
    cx: &mut Context<'_>,
) -> Result<(), FactoryFailure<E>> {
    if cancellation.poll(cx).is_ready() {
        return Err(FactoryFailure::Confirmation(Failure::Cancelled));
    }

    if let Poll::Ready(result) = deadline.poll(cx) {
        result.map_err(|error| FactoryFailure::Session(SessionError::Clock(error)))?;

        return Err(FactoryFailure::Confirmation(Failure::Timeout));
    }

    Ok(())
}
/// Access ends within this poll, before either HTTP or timer state is retained.
fn notification<S: TextSocket, A: SessionAccess<S>>(
    access: &A,
    watch: &mut ConfirmationWatch,
    cx: &mut Context<'_>,
    now: u64,
) -> Poll<Result<(), FactoryFailure<S::Error>>> {
    access
        .with_session(|session| watch.poll_notification(session, cx, now))
        .unwrap_or_else(|error| Poll::Ready(Err(FactoryFailure::Session(error))))
}
/// A generic sleep may reject a second poll after completion.
fn acknowledgement_tick<D: Future<Output = Result<(), ClockError>>>(
    tick: Pin<&mut D>,
    fired: &mut bool,
    cx: &mut Context<'_>,
) -> Result<(), ClockError> {
    if !*fired && let Poll::Ready(result) = tick.poll(cx) {
        *fired = true;

        return result;
    }

    Ok(())
}
/// Races bounded read reconciliation against this watch's notification progress.
pub(super) async fn observe<T, S, A, C, X>(
    client: &Client<T>,
    access: &A,
    clock: &C,
    watch: &mut ConfirmationWatch,
    subject: Subject,
    mut cancellation: Pin<&mut X>,
) -> Result<(), FactoryFailure<S::Error>>
where
    T: Transport,
    S: TextSocket,
    A: SessionAccess<S>,
    C: Clock,
    X: Future<Output = ()>,
{
    let Subject {
        signature,
        lifetime,
        config,
    } = subject;

    let mut timeout = core::pin::pin!(clock.sleep_until(config.deadline));

    loop {
        let wake = access
            .with_session(|session| session.next_deadline())
            .map_err(FactoryFailure::Session)?
            .filter(|tick| *tick > clock.now())
            .unwrap_or(config.deadline)
            .min(config.deadline);
        let mut acknowledgement = core::pin::pin!(clock.sleep_until(wake));
        let mut acknowledgement_fired = false;
        let read_at = watch.next_read();
        let read = async {
            clock
                .sleep_until(read_at)
                .await
                .map_err(|error| FactoryFailure::Session(SessionError::Clock(error)))?;
            reconcile(client, signature, lifetime, config.policy)
                .await
                .map_err(super::factory::widen)
        };
        // The read borrows only its immutable schedule, never the mutable watch.
        let mut read = core::pin::pin!(read);
        let events = poll_fn(|cx| {
            check_stop(cancellation.as_mut(), timeout.as_mut(), cx)?;

            acknowledgement_tick(acknowledgement.as_mut(), &mut acknowledgement_fired, cx)
                .map_err(|error| FactoryFailure::Session(SessionError::Clock(error)))?;
            if let Poll::Ready(result) = notification(access, watch, cx, clock.now()) {
                return Poll::Ready(result.map(|()| None));
            }

            read.as_mut().poll(cx).map(|result| result.map(Some))
        })
        .await?;
        let Some(events) = events else {
            return Ok(());
        };

        if let Some(result) = watch.observe(events, clock.now()) {
            return result.map_err(FactoryFailure::Confirmation);
        }
    }
}

/// Keeps the original outcome while awaiting only this registration's cleanup.
pub(super) async fn finish<S, A, C>(
    access: &A,
    clock: &C,
    watch: &mut ConfirmationWatch,
    config: FactoryConfig,
    outcome: Result<(), FactoryFailure<S::Error>>,
) -> Result<(), FactoryError<S::Error>>
where
    S: TextSocket,
    A: SessionAccess<S>,
    C: Clock,
{
    let deadline = clock.now().saturating_add(config.cleanup_millis.get());
    let mut timeout = core::pin::pin!(clock.sleep_until(deadline));
    let cleanup = poll_fn(|cx| {
        if let Poll::Ready(result) = timeout.as_mut().poll(cx) {
            return Poll::Ready(match result {
                Ok(()) => Err(SessionError::Deadline),
                Err(error) => Err(SessionError::Clock(error)),
            });
        }
        match access.with_session(|session| watch.poll_cleanup(session, cx, clock.now(), deadline))
        {
            Ok(poll) => poll,
            Err(error) => Poll::Ready(Err(error)),
        }
    })
    .await;

    if outcome.is_ok() && cleanup.is_ok() {
        return Ok(());
    }

    Err(FactoryError {
        outcome: outcome.map_err(Box::new),
        cleanup: cleanup.err().into_iter().collect(),
    })
}
