#![cfg(all(target_os = "wasi", target_env = "p2"))]

#[path = "../src/reactor.rs"]
mod reactor;
mod support;

use reactor::{DriveError, Reactor};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use support::Probe;
use wasip2::clocks::monotonic_clock::{now, subscribe_duration};
use wasip2::io::poll::Pollable;

const PENDING_NS: u64 = 60_000_000_000;
const READY_NS: u64 = 5_000_000;
const SETTLE_NS: u64 = 10_000_000;

fn pending(reactor: &Reactor) -> reactor::Wait {
    reactor
        .wait(subscribe_duration(PENDING_NS), u64::MAX)
        .expect("registration")
}

#[test]
fn clone_and_cancellation_callbacks_can_inspect_registry() {
    let reactor = Reactor::default();
    let probe = Probe::new(&reactor);
    let waker = probe.waker();
    let mut wait = pending(&reactor);

    assert!(
        Pin::new(&mut wait)
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    assert_eq!(probe.clones(), 1);

    drop(waker);
    drop(wait);
    assert_eq!(reactor.registrations(), 0);
    assert_eq!(probe.drops(), 2);
}

#[test]
fn replacement_drops_previous_waker_outside_registry_borrow() {
    let reactor = Reactor::default();
    let first = Probe::new(&reactor);
    let second = Probe::new(&reactor);
    let mut wait = pending(&reactor);

    for probe in [&first, &second] {
        let waker = probe.waker();
        assert!(
            Pin::new(&mut wait)
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
    }

    assert_eq!(first.drops(), 2);
    assert_eq!(second.drops(), 1);
    drop(wait);
    assert_eq!(second.drops(), 2);
    assert_eq!(reactor.registrations(), 0);
}

#[test]
fn ready_removal_and_unused_waker_drop_allow_reentrancy() {
    complete(subscribe_duration(READY_NS), u64::MAX, Ok(()));
}

#[test]
fn expired_timer_removal_allows_reentrancy() {
    complete(subscribe_duration(PENDING_NS), now() + READY_NS, Err(()));
}

fn complete(signal: Pollable, deadline: u64, expected: Result<(), ()>) {
    let reactor = Reactor::default();
    let probe = Probe::new(&reactor);
    let waker = probe.waker();
    let mut wait = reactor.wait(signal, deadline).expect("registration");
    let mut context = Context::from_waker(&waker);

    assert!(Pin::new(&mut wait).poll(&mut context).is_pending());
    subscribe_duration(SETTLE_NS).block();
    assert_eq!(
        Pin::new(&mut wait).poll(&mut context),
        Poll::Ready(expected)
    );
    assert_eq!(reactor.registrations(), 0);
    assert_eq!(probe.drops(), probe.clones());

    drop(wait);
    drop(waker);
    assert_eq!(probe.drops(), probe.clones() + 1);
}

#[test]
fn capacity_error_preserves_existing_registrations_and_cleanup() {
    let reactor = Reactor::default();
    let waits: Vec<_> = (0..1024).map(|_| pending(&reactor)).collect();
    assert!(reactor.wait(subscribe_duration(0), u64::MAX).is_err());
    assert_eq!(reactor.registrations(), 1024);
    drop(waits);
    assert_eq!(reactor.registrations(), 0);
    drop(pending(&reactor));
    assert_eq!(reactor.registrations(), 0);
}

#[test]
fn wake_delivery_and_driver_errors_allow_reentrancy() {
    let reactor = Reactor::default();
    assert_eq!(reactor.drive(), Err(DriveError::NoReadiness));
    assert_eq!(
        reactor.block_on(std::future::pending::<()>()),
        Err(DriveError::NoReadiness)
    );

    let probe = Probe::new(&reactor);
    let waker = probe.waker();
    let mut wait = reactor
        .wait(subscribe_duration(READY_NS), u64::MAX)
        .expect("registration");
    assert!(
        Pin::new(&mut wait)
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    reactor.drive().expect("ready signal");
    assert_eq!(probe.wakes(), 1);

    assert_eq!(
        Pin::new(&mut wait).poll(&mut Context::from_waker(&waker)),
        Poll::Ready(Ok(()))
    );
    assert_eq!(reactor.registrations(), 0);
}
