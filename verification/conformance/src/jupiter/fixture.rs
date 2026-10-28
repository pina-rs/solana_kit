//! Real Jupiter futures use an Rc-bearing manual peer on every maintained host.

use alloc::{rc::Rc, vec::Vec};
use core::{
    cell::{Cell, RefCell},
    future::Future,
    pin::{Pin, pin},
    task::{Context, Poll, Waker},
};
use solana_kit_client::{Reply, TransportError, rest};
use solana_kit_jupiter::{Client, Service};

#[derive(Default)]
pub(super) struct State {
    pub requests: RefCell<Vec<rest::Request<Service>>>,
    pub active: Cell<usize>,
    pub dropped: Cell<usize>,
}

pub(super) struct Peer {
    pub state: Rc<State>,
    pub body: &'static [u8],
    pub status: u16,
    pub pending: bool,
}

impl Peer {
    pub fn new(state: Rc<State>, body: &'static [u8]) -> Self {
        Self {
            state,
            body,
            status: 200,
            pending: false,
        }
    }

    pub fn client(self, budgets: rest::Budgets) -> Client<Self> {
        Client::new(self, budgets)
    }
}

impl rest::Transport<Service> for Peer {
    type Exchange<'a> = Exchange;

    fn exchange(&self, request: rest::Request<Service>) -> Self::Exchange<'_> {
        self.state.requests.borrow_mut().push(request);
        self.state.active.set(self.state.active.get() + 1);
        Exchange {
            state: self.state.clone(),
            reply: Some(Reply {
                status: self.status,
                body: self.body.into(),
            }),
            pending: self.pending,
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

pub(super) fn finish<T>(future: impl Future<Output = T>) -> Result<T, &'static str> {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());

    match future.as_mut().poll(&mut context) {
        Poll::Ready(result) => Ok(result),
        Poll::Pending => Err("Jupiter ready exchange remained pending"),
    }
}
