//! Pending exchange destruction is observable before retries and cancellation return.
use super::{
    fixture::{Manual, Peer, State, client, finish},
    require,
};
use alloc::{boxed::Box, rc::Rc};
use core::{
    future::Future,
    task::{Context, Waker},
};
use solana_kit_client::TransportFailureKind;
use solana_kit_pyth::hermes::Error;

pub(super) fn run() -> Result<(), &'static str> {
    const EXHAUSTED_TIMEOUT_MS: u64 = 20_700;
    let state = Rc::new(State::default());
    let clock = Manual::default();
    let mut peer = Peer::new(state.clone(), b"");
    peer.pending = true;
    let client = client(peer, clock.clone());
    let result = finish(client.get_price_feeds(None, None), &clock)?;

    require(
        matches!(result,Err(Error::Transport(error)) if error.kind==TransportFailureKind::Deadline),
        "Hermes pending deadline changed",
    )?;
    require(
        (
            state.requests.borrow().len(),
            clock.0.get(),
            state.active.get(),
            state.dropped.get(),
        ) == (4, EXHAUSTED_TIMEOUT_MS, 0, 4),
        "Hermes pending attempt cleanup differs",
    )?;

    cancel(&client, &state)
}

/// Drop one fresh pending operation after the completed deadline sequence.
fn cancel(
    client: &solana_kit_pyth::hermes::Client<Peer, Manual>,
    state: &State,
) -> Result<(), &'static str> {
    const TOTAL_ATTEMPTS: usize = 5;

    let mut future = Box::pin(client.get_price_feeds(None, None));
    require(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending(),
        "Hermes cancellation fixture never pending",
    )?;
    require(state.active.get() == 1, "Hermes exchange owner missing")?;
    drop(future);
    require(
        (
            state.requests.borrow().len(),
            state.active.get(),
            state.dropped.get(),
        ) == (TOTAL_ATTEMPTS, 0, TOTAL_ATTEMPTS),
        "Hermes cancellation left work alive",
    )
}
