//! Real TLS frames drive the ordinary typed Session and the same WASI clock.
use anyhow::{Result, ensure};
use solana_kit_client::subscriptions::{Budget, Clock, Drain, Root, Session};
use solana_kit_wasi::{Reactor, WasiClock, WasiWebSocket};

const DEADLINE_MILLIS: u64 = 2_000;
const GENERATION: u64 = 7;

pub fn run(reactor: &Reactor, socket: WasiWebSocket) -> Result<()> {
    let clock = WasiClock::new(reactor.clone());
    let deadline = clock.now() + DEADLINE_MILLIS;
    let mut session = Session::new(socket, GENERATION, Budget::default());
    let handle = session.subscribe(Root, deadline)?;
    let value = reactor.block_on(session.next(&handle, &clock, deadline))??;
    ensure!(
        matches!(value, Drain::Notification(u64::MAX)),
        "typed root notification"
    );

    reactor.block_on(session.unsubscribe(handle, &clock, deadline))??;
    reactor.block_on(session.shutdown(&clock, deadline))??;
    ensure!(
        reactor.registrations() == 0,
        "joined Session left a readiness owner"
    );
    Ok(())
}
