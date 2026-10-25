//! Local Rc ownership survives a real Pending receive and bounded awaited close.

use crate::require;
use alloc::rc::Rc;
use core::{
    cell::Cell,
    future::{Future, Ready, ready},
    pin::{Pin, pin},
    task::{Context, Poll, Waker},
};
use solana_kit_client::{
    confirmation::{Confirmation, Event, Failure, Observer, Policy, WaitError, wait},
    methods::Commitment,
};
use solana_kit_core::{Hash, transaction::Lifetime};

#[derive(Default)]
struct State {
    polls: Cell<usize>,
    receives_dropped: Cell<usize>,
    close_polls: Cell<usize>,
    drops: Cell<usize>,
    violation: Cell<bool>,
}
struct LocalObserver(Rc<State>);
struct Receive(Rc<State>);
struct Close(Rc<State>);

impl Future for Receive {
    type Output = Result<Event, Failure>;
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.polls.set(self.0.polls.get() + 1);
        Poll::Pending
    }
}
impl Drop for Receive {
    fn drop(&mut self) {
        self.0
            .receives_dropped
            .set(self.0.receives_dropped.get() + 1);
    }
}
impl Future for Close {
    type Output = Result<(), Failure>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.violation.set(self.0.receives_dropped.get() != 1);
        let polls = self.0.close_polls.get();
        self.0.close_polls.set(polls + 1);

        if polls == 0 {
            cx.waker().wake_by_ref();

            return Poll::Pending;
        }

        Poll::Ready(Err(Failure::ObserverEnded))
    }
}
impl Observer for LocalObserver {
    type Next<'a> = Receive;
    type Close<'a> = Close;
    fn next(&mut self) -> Receive {
        Receive(self.0.clone())
    }
    fn close(&mut self) -> Close {
        Close(self.0.clone())
    }
}
impl Drop for LocalObserver {
    fn drop(&mut self) {
        self.0.drops.set(self.0.drops.get() + 1);
    }
}

fn confirmation() -> Result<Confirmation, &'static str> {
    Confirmation::new(
        super::state::signature(),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([6; 32]),
            last_valid_block_height: 7,
        },
        Policy::default(),
    )
    .map_err(|_| "confirmation setup failed")
}

/// Cancellation must release the pending receive before awaited close can finish.
pub(super) fn run() -> Result<(), &'static str> {
    let state = Rc::new(State::default());
    let cancelled = Rc::new(Cell::new(false));
    let cancel = core::future::poll_fn(|_| {
        if cancelled.get() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    });
    let mut context = Context::from_waker(Waker::noop());
    let result = {
        let mut future = pin!(wait(confirmation()?, LocalObserver(state.clone()), cancel));
        require(
            future.as_mut().poll(&mut context).is_pending(),
            "receive never became Pending",
        )?;
        require(
            state.polls.get() == 1 && state.close_polls.get() == 0,
            "receive or close order changed",
        )?;
        cancelled.set(true);
        require(
            future.as_mut().poll(&mut context).is_pending(),
            "close never became Pending",
        )?;

        match future.as_mut().poll(&mut context) {
            Poll::Ready(result) => result,
            Poll::Pending => return Err("finite close did not complete"),
        }
    };
    let preserved = match result {
        Err(WaitError::Cleanup {
            outcome: Err(Failure::Cancelled),
            source,
        }) => matches!(*source, Failure::ObserverEnded),
        _ => false,
    };
    require(preserved, "cleanup lost original cancellation")?;
    closed_once(&state)?;
    already_settled()
}

/// Every owned layer must be gone after the second close poll, with no alias leak.
fn closed_once(state: &Rc<State>) -> Result<(), &'static str> {
    require(
        state.close_polls.get() == 2
            && state.receives_dropped.get() == 1
            && state.drops.get() == 1
            && Rc::strong_count(state) == 1
            && !state.violation.get(),
        "local lifecycle ownership changed",
    )
}

struct Settled {
    next: Rc<Cell<usize>>,
    closed: Rc<Cell<usize>>,
}
const FINALIZED_SLOT: u64 = 20;
impl Observer for Settled {
    type Next<'a> = Ready<Result<Event, Failure>>;
    type Close<'a> = Ready<Result<(), Failure>>;
    fn next(&mut self) -> Self::Next<'_> {
        self.next.set(self.next.get() + 1);
        ready(Ok(super::state::notification(
            FINALIZED_SLOT,
            Commitment::Finalized,
        )))
    }
    fn close(&mut self) -> Self::Close<'_> {
        self.closed.set(self.closed.get() + 1);
        ready(Ok(()))
    }
}
fn already_settled() -> Result<(), &'static str> {
    let next = Rc::new(Cell::new(0));
    let closed = Rc::new(Cell::new(0));
    let mut reducer = confirmation()?;
    require(
        matches!(reducer.observe(Event::Timeout), Some(Err(Failure::Timeout))),
        "timeout did not settle",
    )?;
    let result = super::finish(wait(
        reducer,
        Settled {
            next: next.clone(),
            closed: closed.clone(),
        },
        core::future::pending(),
    ))?;
    require(
        matches!(result, Err(WaitError::Outcome(Failure::AlreadySettled)))
            && next.get() == 0
            && closed.get() == 1,
        "settled observer restarted or did not close",
    )
}
