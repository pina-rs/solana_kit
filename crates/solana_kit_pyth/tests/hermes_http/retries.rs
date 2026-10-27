//! Retry and cancellation assertions observe physical exchange ownership and clock ticks.
use super::{
    LISTING, client,
    clock::{Manual, run},
    peer::{Outcome, Peer, reply},
};
use solana_kit_client::{TransportFailureKind as Kind, subscriptions::ClockError};
use solana_kit_pyth::hermes::{ConfigError, Error, MAX_HTTP_RETRIES, Policy};
use std::{
    future::Future,
    task::{Context, Poll, Waker},
};

const REQUEST_TIMEOUT_MS: u64 = 5_000;
const REQUEST_TIMEOUT: u16 = 408;
const TOO_MANY_REQUESTS: u16 = 429;
const SERVER_ERROR: u16 = 500;
const SERVICE_UNAVAILABLE: u16 = 503;
const PROVIDER_EXTENSION_STATUS: u16 = 600;
const TOTAL_BACKOFF_MS: u64 = 700;
const EXHAUSTED_TIMEOUT_MS: u64 = 20_700;

#[test]
fn policy_defaults_and_representability_are_explicit() {
    let policy = Policy::default();
    assert_eq!(
        (
            policy.timeout_ms(),
            policy.http_retries(),
            policy.backoff_ms()
        ),
        (REQUEST_TIMEOUT_MS, 3, 100)
    );

    assert_eq!(Policy::new(0, 0, 0), Err(ConfigError::ZeroTimeout));
    assert_eq!(
        Policy::new(1, MAX_HTTP_RETRIES + 1, 0),
        Err(ConfigError::RetryLimit)
    );
    assert_eq!(Policy::new(1, 2, u64::MAX), Err(ConfigError::BackoffRange));
    assert!(Policy::new(1, MAX_HTTP_RETRIES, 0).is_ok());
}

#[test]
fn reference_status_classes_retry_only_transient_reads() {
    for status in [
        REQUEST_TIMEOUT,
        TOO_MANY_REQUESTS,
        SERVER_ERROR,
        SERVICE_UNAVAILABLE,
        PROVIDER_EXTENSION_STATUS,
    ] {
        let peer = Peer::new([reply(status, b"transient"), reply(200, LISTING)]);
        let clock = Manual::default();
        let client = client(peer.clone(), clock.clone(), Policy::default());
        run(client.get_price_feeds(None, None), &clock).unwrap();
        assert_eq!((peer.count(), clock.now()), (2, 100));
        assert_eq!(peer.events(), ["start", "drop", "start", "drop"]);
    }

    for status in [300, 400, 401, 404] {
        let peer = Peer::new([reply(status, b"private provider body")]);
        let clock = Manual::default();
        let client = client(peer.clone(), clock.clone(), Policy::default());

        let error = run(client.get_price_feeds(None, None), &clock).unwrap_err();
        assert!(!format!("{error:?} {error}").contains("private provider body"));

        let Error::Http {
            status: actual,
            body,
        } = error
        else {
            panic!("expected HTTP failure");
        };
        assert_eq!(actual, status);
        assert_eq!(body, b"private provider body");
        assert_eq!((peer.count(), clock.now()), (1, 0));
    }
}

#[test]
fn retry_exhaustion_keeps_last_body_and_exponential_schedule() {
    let peer = Peer::new([
        reply(SERVICE_UNAVAILABLE, b"one"),
        reply(SERVICE_UNAVAILABLE, b"two"),
        reply(SERVICE_UNAVAILABLE, b"three"),
        reply(SERVICE_UNAVAILABLE, b"last"),
    ]);
    let clock = Manual::default();
    let client = client(peer.clone(), clock.clone(), Policy::default());
    let result = run(client.get_price_feeds(None, None), &clock);
    assert!(
        matches!(result, Err(Error::Http { status:SERVICE_UNAVAILABLE, body }) if body==b"last")
    );
    assert_eq!(
        (peer.count(), clock.now(), peer.live(), clock.pending()),
        (4, TOTAL_BACKOFF_MS, 0, 0)
    );
}

#[test]
fn transient_transport_errors_retry_but_policy_and_capacity_errors_stop() {
    for kind in [Kind::Io, Kind::Deadline] {
        let peer = Peer::new([Outcome::Failure(kind), reply(200, LISTING)]);
        let clock = Manual::default();
        let client = client(peer.clone(), clock.clone(), Policy::default());
        run(client.get_price_feeds(None, None), &clock).unwrap();
        assert_eq!((peer.count(), clock.now()), (2, 100));
    }

    for kind in [
        Kind::Unavailable,
        Kind::Denied,
        Kind::Cancelled,
        Kind::BodyLimit,
        Kind::HeaderLimit,
        Kind::InvalidResponse,
    ] {
        let peer = Peer::new([Outcome::Failure(kind)]);
        let clock = Manual::default();
        let client = client(peer.clone(), clock.clone(), Policy::default());
        let result = run(client.get_price_feeds(None, None), &clock);

        assert!(matches!(result, Err(Error::Transport(error)) if error.kind==kind));
        assert_eq!((peer.count(), clock.now()), (1, 0));
    }
}

#[test]
fn each_attempt_has_its_own_deadline_and_drops_before_the_next_attempt() {
    let peer = Peer::new(std::iter::repeat_with(|| Outcome::Pending).take(4));
    let clock = Manual::default();
    let client = client(peer.clone(), clock.clone(), Policy::default());
    let result = run(client.get_price_feeds(None, None), &clock);

    assert!(matches!(result, Err(Error::Transport(error)) if error.kind==Kind::Deadline));
    assert_eq!(
        (peer.count(), clock.now(), peer.live(), clock.pending()),
        (4, EXHAUSTED_TIMEOUT_MS, 0, 0)
    );
    assert_eq!(
        peer.events(),
        [
            "start", "drop", "start", "drop", "start", "drop", "start", "drop"
        ]
    );
}

#[test]
fn timer_failures_and_unrepresentable_deadlines_do_not_start_extra_requests() {
    for failed_sleep in [0, 1] {
        let peer = Peer::new([reply(SERVICE_UNAVAILABLE, b"retry")]);
        let clock = Manual::default();
        clock.fail_at.set(Some(failed_sleep));
        let client = client(peer.clone(), clock.clone(), Policy::default());
        assert!(matches!(
            run(client.get_price_feeds(None, None), &clock),
            Err(Error::Clock(ClockError::Capacity))
        ));
        assert_eq!(peer.count(), failed_sleep);
        assert_eq!((peer.live(), clock.pending()), (0, 0));
    }

    let peer = Peer::new([]);
    let clock = Manual::default();
    clock.set(u64::MAX);
    let client = client(peer.clone(), clock.clone(), Policy::default());
    assert!(matches!(
        run(client.get_price_feeds(None, None), &clock),
        Err(Error::Clock(ClockError::DeadlineRange))
    ));
    assert_eq!(peer.count(), 0);
}

#[test]
fn dropping_pending_operation_releases_exchange_and_sleep_without_retry() {
    let peer = Peer::new([Outcome::Pending]);
    let clock = Manual::default();
    let client = client(peer.clone(), clock.clone(), Policy::default());
    let future = client.get_price_feeds(None, None);
    let mut future = Box::pin(future);

    assert!(matches!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop())),
        Poll::Pending
    ));
    assert_eq!((peer.live(), clock.pending()), (1, 1));
    drop(future);
    assert_eq!((peer.count(), peer.live(), clock.pending()), (1, 0, 0));
}
