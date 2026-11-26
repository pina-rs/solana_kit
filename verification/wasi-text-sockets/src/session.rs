//! Real TLS frames drive the ordinary typed Session and the same WASI clock.
use anyhow::{Result, ensure};
use core::{future::poll_fn, ops::ControlFlow, pin::Pin};
use futures_core::Stream;
use solana_kit_client::subscriptions::{Budget, Clock, Root, Session};
use solana_kit_wasi::{Reactor, WasiClock, WasiWebSocket};

const DEADLINE_MILLIS: u64 = 2_000;
const CLEANUP_MILLIS: u64 = 500;
const GENERATION: u64 = 7;
const CALLBACK_ROOT: u64 = 7;

pub fn run(reactor: &Reactor, socket: WasiWebSocket) -> Result<()> {
    let clock = WasiClock::new(reactor.clone());
    let deadline = clock.now() + DEADLINE_MILLIS;
    let mut session = Session::new(socket, GENERATION, Budget::default());
    let handle = session.subscribe(Root, deadline)?;

    let mut stream = session.notifications(&handle, &clock, CLEANUP_MILLIS);
    let first = reactor.block_on(poll_fn(|cx| Pin::new(&mut stream).poll_next(cx)))?;
    let Some(first) = first else {
        anyhow::bail!("notification stream ended before the root value");
    };
    ensure!(first? == u64::MAX, "typed root notification");
    drop(stream);

    let mut second = None;
    reactor.block_on(
        session.run_callback(&handle, &clock, CLEANUP_MILLIS, |value| {
            second = Some(value);
            ControlFlow::Break(())
        }),
    )??;
    ensure!(second == Some(CALLBACK_ROOT), "callback root notification");

    reactor.block_on(session.unsubscribe(handle, &clock, deadline))??;
    reactor.block_on(session.shutdown(&clock, deadline))??;
    ensure!(
        reactor.registrations() == 0,
        "joined Session left a readiness owner"
    );
    Ok(())
}
