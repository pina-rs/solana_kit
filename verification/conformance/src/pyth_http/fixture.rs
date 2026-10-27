//! Finite manual polling drives the real client future without host I/O.
extern crate std;
use alloc::{rc::Rc, vec::Vec};
use core::{
    cell::{Cell, RefCell},
    future::Future,
    pin::{Pin, pin},
    task::{Context, Poll, Waker},
};
use solana_kit_client::{
    Reply, TransportError, rest,
    subscriptions::{Clock, ClockError},
};
use solana_kit_pyth::hermes::{Client, Policy, Service};

pub(super) const LISTING: &[u8] = br#"[{"id":"feed-one","attributes":{}}]"#;
pub(super) const UPDATE: &[u8] = br#"{"binary":{"encoding":"hex"}}"#;
const OK: u16 = 200;

#[derive(Default)]
pub(super) struct State {
    pub requests: RefCell<Vec<rest::Request<Service>>>,
    pub active: Cell<usize>,
    pub dropped: Cell<usize>,
}

pub(super) struct Peer {
    pub state: Rc<State>,
    pub failures: Cell<usize>,
    pub status: u16,
    pub body: &'static [u8],
    pub pending: bool,
}

impl Peer {
    pub fn new(state: Rc<State>, body: &'static [u8]) -> Self {
        Self {
            state,
            failures: Cell::new(0),
            status: OK,
            body,
            pending: false,
        }
    }
}

impl rest::Transport<Service> for Peer {
    type Exchange<'a> = Exchange;

    fn exchange(&self, request: rest::Request<Service>) -> Self::Exchange<'_> {
        let remaining = self.failures.get();
        self.failures.set(remaining.saturating_sub(1));
        self.state.requests.borrow_mut().push(request);
        self.state.active.set(self.state.active.get() + 1);
        let status = if remaining > 0 { self.status } else { OK };
        Exchange {
            state: self.state.clone(),
            pending: self.pending,
            reply: Some(Reply {
                status,
                body: self.body.into(),
            }),
        }
    }
}

pub(super) struct Exchange {
    state: Rc<State>,
    reply: Option<Reply>,
    pending: bool,
}

impl Future for Exchange {
    type Output = Result<Reply, TransportError>;
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if this.pending {
            return Poll::Pending;
        }

        Poll::Ready(this.reply.take().ok_or_else(|| {
            TransportError::new(solana_kit_client::TransportFailureKind::InvalidResponse)
        }))
    }
}

impl Drop for Exchange {
    fn drop(&mut self) {
        self.state.active.set(self.state.active.get() - 1);
        self.state.dropped.set(self.state.dropped.get() + 1);
    }
}

#[derive(Clone, Default)]
pub(super) struct Manual(pub Rc<Cell<u64>>);

impl Clock for Manual {
    type Sleep<'a> = Sleep;
    fn now(&self) -> u64 {
        self.0.get()
    }
    fn sleep_until(&self, deadline: u64) -> Sleep {
        Sleep(self.clone(), deadline)
    }
}

pub(super) struct Sleep(Manual, u64);

impl Future for Sleep {
    type Output = Result<(), ClockError>;
    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0.0.get() >= self.1 {
            return Poll::Ready(Ok(()));
        }

        context.waker().wake_by_ref();
        Poll::Pending
    }
}

/// Manual time uses a finite 100ms step; every pending sleep explicitly wakes.
pub(super) fn finish<F: Future>(future: F, clock: &Manual) -> Result<F::Output, &'static str> {
    const MAX_POLLS: usize = 256;
    const STEP_MS: u64 = 100;
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());

    for _ in 0..MAX_POLLS {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return Ok(output);
        }

        clock.0.set(
            clock
                .0
                .get()
                .checked_add(STEP_MS)
                .ok_or("fixture clock overflow")?,
        );
    }

    Err("Hermes exceeded finite fixture polling")
}

pub(super) fn client(peer: Peer, clock: Manual) -> Client<Peer, Manual> {
    Client::new(peer, clock, Policy::default(), rest::Budgets::default())
}
