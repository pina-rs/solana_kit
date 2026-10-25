//! Typed RPC reconciliation and caller-driven confirmation with bounded cleanup.
use super::reconcile;
use super::{Confirmation, Event, Failure, Policy};
use crate::{
    Client, Transport,
    subscriptions::{
        Clock, Drain, Session, SessionError, SignatureConfig, SignatureNotifications, TextSocket,
    },
};
use core::{future::Future, num::NonZeroU64, pin::Pin, task::Poll};
use solana_kit_core::{Address, Signature, accounts::AccountError, transaction::Lifetime};

/// Nonce authentication never turns malformed account data into an expiry event.
#[derive(Debug)]
pub enum NonceError {
    /// Existing exact account/owner/allocation/codec validation failure.
    Account(AccountError),
    /// A legacy or uninitialized nonce cannot authenticate a current token.
    Unusable(Address),
    /// An executable account cannot contain durable nonce state.
    Executable(Address),
}
/// First substantive lifecycle failure, preserved separately from cleanup.
#[derive(Debug)]
pub enum FactoryFailure<E> {
    /// Confirmation policy, transaction or typed RPC failure.
    Confirmation(Failure),
    /// Protocol or actual I/O failure.
    Session(SessionError<E>),
    /// Nonce authentication rejected the account.
    Nonce(NonceError),
}
/// Original outcome and every bounded cleanup failure, in execution order.
#[derive(Debug)]
pub struct FactoryError<E> {
    /// A successful race is retained when cleanup subsequently fails.
    pub outcome: Result<(), Box<FactoryFailure<E>>>,
    /// At most unsubscribe and socket teardown failures.
    pub cleanup: Vec<SessionError<E>>,
}
/// Explicit monotonic timing and commitment policy. No ambient clock is used.
#[derive(Clone, Copy, Debug)]
pub struct FactoryConfig {
    /// Target commitment and minimum reconciliation slot.
    pub policy: Policy,
    /// Absolute monotonic deadline for the complete confirmation race.
    pub deadline: u64,
    /// Nonzero delay between safe read reconciliations.
    pub poll_interval: NonZeroU64,
    /// Fresh bounded cleanup allowance after any outcome.
    pub cleanup_millis: NonZeroU64,
}

pub(super) fn widen<E>(error: FactoryFailure<core::convert::Infallible>) -> FactoryFailure<E> {
    match error {
        FactoryFailure::Confirmation(error) => FactoryFailure::Confirmation(error),
        FactoryFailure::Nonce(error) => FactoryFailure::Nonce(error),
        FactoryFailure::Session(error) => FactoryFailure::Session(match error {
            SessionError::Protocol(error) => SessionError::Protocol(error),
            SessionError::Socket(error) => match error {},
            SessionError::Clock(error) => SessionError::Clock(error),
            SessionError::Eof => SessionError::Eof,
            SessionError::Deadline => SessionError::Deadline,
            SessionError::Unavailable => SessionError::Unavailable,
        }),
    }
}

/// Confirms a known signature through one application-owned socket and client.
///
/// Registration is sent before the first HTTP reconciliation. Each poll can
/// progress socket notifications while HTTP reads or timers remain pending.
/// Cancellation wins simultaneous outcomes. All paths await bounded unsubscribe
/// and socket teardown. Dropping this future instead performs local adapter Drop
/// cancellation and cannot promise remote acknowledgement. No task is spawned.
pub async fn confirm_signature<T, S, C, X>(
    client: &Client<T>,
    mut session: Session<S>,
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
{
    let mut handle = None;

    let mut cancellation = core::pin::pin!(cancellation);

    let outcome = async {
        let (policy, subscription) = register(&mut session, signature, lifetime, config)?;
        handle = Some(subscription);
        let registration =
            flush_registration(&mut session, clock, config.deadline, cancellation.as_mut()).await;

        if registration.is_err() {
            session.terminate();
        }

        registration?;
        watch(
            client,
            &mut session,
            clock,
            handle.as_ref().expect("installed before observation"),
            Subject {
                signature,
                lifetime,
                policy,
                config,
            },
            cancellation.as_mut(),
        )
        .await
    }
    .await;
    cleanup_session(session, clock, handle, config, outcome).await
}

/// Keeps every cleanup failure in execution order even when observation already failed.
async fn cleanup_session<S: TextSocket, C: Clock>(
    mut session: Session<S>,
    clock: &C,
    handle: Option<crate::subscriptions::Subscription<SignatureNotifications>>,
    config: FactoryConfig,
    outcome: Result<(), FactoryFailure<S::Error>>,
) -> Result<(), FactoryError<S::Error>> {
    let mut cleanup = Vec::new();
    let cleanup_deadline = clock.now().saturating_add(config.cleanup_millis.get());

    if let Some(handle) = handle
        && let Err(error) = session.unsubscribe(handle, clock, cleanup_deadline).await
    {
        cleanup.push(error);
    }

    if let Err(error) = session.shutdown(clock, cleanup_deadline).await {
        cleanup.push(error);
    }

    if outcome.is_ok() && cleanup.is_empty() {
        return Ok(());
    }

    Err(FactoryError {
        outcome: outcome.map_err(Box::new),
        cleanup,
    })
}

fn register<S: TextSocket>(
    session: &mut Session<S>,
    signature: Signature,
    lifetime: Lifetime,
    config: FactoryConfig,
) -> Result<
    (
        Confirmation,
        crate::subscriptions::Subscription<SignatureNotifications>,
    ),
    FactoryFailure<S::Error>,
> {
    let policy = Confirmation::new(signature, lifetime, config.policy)
        .map_err(FactoryFailure::Confirmation)?;
    let notification = SignatureNotifications(
        signature,
        Some(SignatureConfig {
            commitment: Some(config.policy.commitment),
            enable_received_notification: Some(true),
        }),
    );
    let subscription = session
        .subscribe(notification, config.deadline)
        .map_err(FactoryFailure::Session)?;

    Ok((policy, subscription))
}

async fn flush_registration<S: TextSocket, C: Clock, X: Future<Output = ()>>(
    session: &mut Session<S>,
    clock: &C,
    deadline: u64,
    mut cancellation: Pin<&mut X>,
) -> Result<(), FactoryFailure<S::Error>> {
    let mut flush = core::pin::pin!(session.flush(clock, deadline));
    core::future::poll_fn(|cx| {
        if cancellation.as_mut().poll(cx).is_ready() {
            return Poll::Ready(Err(FactoryFailure::Confirmation(Failure::Cancelled)));
        }

        flush.as_mut().poll(cx).map_err(FactoryFailure::Session)
    })
    .await
}

struct Subject {
    signature: Signature,
    lifetime: Lifetime,
    policy: Confirmation,
    config: FactoryConfig,
}
/// Races socket observations and safe reads while cancellation keeps priority on every poll.
async fn watch<T: Transport, S: TextSocket, C: Clock, X: Future<Output = ()>>(
    client: &Client<T>,
    session: &mut Session<S>,
    clock: &C,
    subscription: &crate::subscriptions::Subscription<SignatureNotifications>,
    subject: Subject,
    cancellation: X,
) -> Result<(), FactoryFailure<S::Error>> {
    let Subject {
        signature,
        lifetime,
        mut policy,
        config,
    } = subject;
    let mut cancellation = core::pin::pin!(cancellation);
    let mut deadline = core::pin::pin!(clock.sleep_until(config.deadline));
    let mut next_read = clock.now();

    loop {
        let read_at = next_read;
        let read = reconcile_after(client, clock, read_at, signature, lifetime, config.policy);
        let mut read = core::pin::pin!(read);
        let event = core::future::poll_fn(|cx| {
            super::shared::check_stop(cancellation.as_mut(), deadline.as_mut(), cx)?;

            if let Poll::Ready(result) =
                poll_notification(session, subscription, signature, config, clock.now(), cx)
            {
                return Poll::Ready(result);
            }

            read.as_mut().poll(cx)
        })
        .await?;

        for event in event {
            if let Some(result) = policy.observe(event) {
                return result.map_err(FactoryFailure::Confirmation);
            }
        }

        next_read = clock.now().saturating_add(config.poll_interval.get());
    }
}

async fn reconcile_after<T: Transport, C: Clock, E>(
    client: &Client<T>,
    clock: &C,
    at: u64,
    signature: Signature,
    lifetime: Lifetime,
    policy: Policy,
) -> Result<Vec<Event>, FactoryFailure<E>> {
    clock
        .sleep_until(at)
        .await
        .map_err(|error| FactoryFailure::Session(SessionError::Clock(error)))?;
    reconcile(client, signature, lifetime, policy)
        .await
        .map_err(widen)
}

fn poll_notification<S: TextSocket>(
    session: &mut Session<S>,
    subscription: &crate::subscriptions::Subscription<SignatureNotifications>,
    signature: Signature,
    config: FactoryConfig,
    now: u64,
    cx: &mut core::task::Context<'_>,
) -> Poll<Result<Vec<Event>, FactoryFailure<S::Error>>> {
    match session.poll_next(subscription, cx, now, config.deadline) {
        Poll::Ready(Ok(Drain::Notification(value))) => Poll::Ready(Ok(vec![Event::Signature {
            signature,
            slot: value.context.slot,
            commitment: config.policy.commitment,
            value: value.value,
        }])),
        Poll::Ready(Err(error)) => Poll::Ready(Err(FactoryFailure::Session(error))),
        Poll::Ready(Ok(Drain::Closed)) => {
            Poll::Ready(Err(FactoryFailure::Confirmation(Failure::ObserverEnded)))
        }
        _ => Poll::Pending,
    }
}
