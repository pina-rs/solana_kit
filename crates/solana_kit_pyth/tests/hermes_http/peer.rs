//! Scripted wire outcomes record actual polling and destruction of each exchange.
use solana_kit_client::{Reply, TransportError, TransportFailureKind, rest};
use solana_kit_pyth::hermes::Service;
use std::{
    cell::RefCell,
    collections::VecDeque,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

pub(super) enum Outcome {
    Reply(Reply),
    Failure(TransportFailureKind),
    Pending,
}

/// Keeps response bytes independent from the Rust models under test.
pub(super) fn reply(status: u16, body: &[u8]) -> Outcome {
    Outcome::Reply(Reply {
        status,
        body: body.into(),
    })
}

#[derive(Default)]
struct State {
    outcomes: VecDeque<Outcome>,
    requests: Vec<rest::Request<Service>>,
    live: usize,
    events: Vec<&'static str>,
}

#[derive(Clone)]
pub(super) struct Peer(Rc<RefCell<State>>);

impl Peer {
    pub fn new(outcomes: impl IntoIterator<Item = Outcome>) -> Self {
        Self(Rc::new(RefCell::new(State {
            outcomes: outcomes.into_iter().collect(),
            ..State::default()
        })))
    }
    pub fn live(&self) -> usize {
        self.0.borrow().live
    }
    pub fn count(&self) -> usize {
        self.0.borrow().requests.len()
    }
    pub fn events(&self) -> Vec<&'static str> {
        self.0.borrow().events.clone()
    }
    pub fn requests(&self) -> Vec<rest::Request<Service>> {
        std::mem::take(&mut self.0.borrow_mut().requests)
    }
}

impl rest::Transport<Service> for Peer {
    type Exchange<'a> = Exchange;

    fn exchange(&self, request: rest::Request<Service>) -> Self::Exchange<'_> {
        Exchange {
            peer: self.clone(),
            request: Some(request),
            outcome: None,
            started: false,
        }
    }
}

pub(super) struct Exchange {
    peer: Peer,
    request: Option<rest::Request<Service>>,
    outcome: Option<Outcome>,
    started: bool,
}

impl Future for Exchange {
    type Output = Result<Reply, TransportError>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if let Some(request) = this.request.take() {
            let mut state = this.peer.0.borrow_mut();
            state.requests.push(request);
            state.events.push("start");
            state.live += 1;
            this.outcome = Some(state.outcomes.pop_front().expect("unexpected request"));
            this.started = true;
        }

        match this
            .outcome
            .take()
            .expect("exchange cannot be repolled after completion")
        {
            Outcome::Reply(reply) => Poll::Ready(Ok(reply)),
            Outcome::Failure(kind) => Poll::Ready(Err(TransportError::new(kind))),
            Outcome::Pending => {
                this.outcome = Some(Outcome::Pending);
                Poll::Pending
            }
        }
    }
}

impl Drop for Exchange {
    fn drop(&mut self) {
        if self.started {
            let mut state = self.peer.0.borrow_mut();
            state.live -= 1;
            state.events.push("drop");
        }
    }
}
