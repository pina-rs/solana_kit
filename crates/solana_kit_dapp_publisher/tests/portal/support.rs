//! Caller-owned synthetic transports observe the actual public request/future boundary.

use solana_kit_client::{
    Reply, TransportError, rest,
    subscriptions::{Clock, ClockError},
};
use solana_kit_dapp_publisher::portal::{Client, PortalService};
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    format,
    future::{Future, Ready, ready},
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
    vec::Vec,
};

pub const BUDGETS: rest::Budgets = rest::Budgets {
    bytes: 4096,
    depth: 16,
    items: 512,
    string_bytes: 2048,
};

#[derive(Default)]
pub struct State {
    pub requests: RefCell<Vec<rest::Request<PortalService>>>,
    pub replies: RefCell<VecDeque<Option<Reply>>>,
    pub cancelled: Cell<usize>,
}

pub struct Transport(pub Rc<State>);

impl rest::Transport<PortalService> for Transport {
    type Exchange<'a> = Exchange;

    fn exchange(&self, request: rest::Request<PortalService>) -> Self::Exchange<'_> {
        self.0.requests.borrow_mut().push(request);
        let reply = self.0.replies.borrow_mut().pop_front().flatten();
        Exchange {
            state: self.0.clone(),
            reply,
            completed: false,
        }
    }
}

pub struct Exchange {
    state: Rc<State>,
    reply: Option<Reply>,
    completed: bool,
}

impl Future for Exchange {
    type Output = Result<Reply, TransportError>;

    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        match self.reply.take() {
            Some(reply) => {
                self.completed = true;
                Poll::Ready(Ok(reply))
            }
            None => Poll::Pending,
        }
    }
}

impl Drop for Exchange {
    fn drop(&mut self) {
        if !self.completed {
            self.state.cancelled.set(self.state.cancelled.get() + 1);
        }
    }
}

pub fn client(replies: Vec<Option<Reply>>) -> (Client<Transport>, Rc<State>) {
    let state = Rc::new(State {
        replies: RefCell::new(replies.into()),
        ..State::default()
    });
    (Client::new(Transport(state.clone()), BUDGETS), state)
}

pub fn reply(status: u16, body: &str) -> Option<Reply> {
    Some(Reply {
        status,
        body: body.as_bytes().to_vec(),
    })
}

pub fn success(payload: &str) -> Option<Reply> {
    reply(200, &format!("{{\"result\":{{\"data\":{payload}}}}}"))
}

pub fn complete<F: Future>(operation: F) -> Result<F::Output, &'static str> {
    let mut operation = std::pin::pin!(operation);
    match operation
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    {
        Poll::Ready(result) => Ok(result),
        Poll::Pending => Err("synthetic ready portal operation remained pending"),
    }
}

pub struct Native;

impl rest::Transport<PortalService> for Native {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, _: rest::Request<PortalService>) -> Self::Exchange<'_> {
        ready(Ok(Reply {
            status: 200,
            body: br#"{"result":{"data":{"action":"deleted"}}}"#.to_vec(),
        }))
    }
}

pub struct TestClock {
    pub now: Cell<u64>,
    pub deadline: u64,
    pub sleeps: RefCell<Vec<u64>>,
    pub fail: Cell<bool>,
    pub dropped: Cell<usize>,
}

impl TestClock {
    pub fn new(deadline: u64) -> Self {
        Self {
            now: Cell::new(0),
            deadline,
            sleeps: RefCell::new(Vec::new()),
            fail: Cell::new(false),
            dropped: Cell::new(0),
        }
    }
}

pub struct Sleep<'a> {
    clock: &'a TestClock,
    deadline: u64,
}

impl Future for Sleep<'_> {
    type Output = Result<(), ClockError>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if self.clock.fail.get() {
            return Poll::Ready(Err(ClockError::Capacity));
        }

        if self.deadline == self.clock.deadline && self.clock.now.get() < self.deadline {
            return Poll::Pending;
        }

        self.clock.now.set(self.deadline);
        Poll::Ready(Ok(()))
    }
}

impl Drop for Sleep<'_> {
    fn drop(&mut self) {
        self.clock.dropped.set(self.clock.dropped.get() + 1);
    }
}

impl Clock for TestClock {
    type Sleep<'a> = Sleep<'a>;
    fn now(&self) -> u64 {
        self.now.get()
    }
    fn sleep_until(&self, deadline: u64) -> Self::Sleep<'_> {
        self.sleeps.borrow_mut().push(deadline);
        Sleep {
            clock: self,
            deadline,
        }
    }
}
