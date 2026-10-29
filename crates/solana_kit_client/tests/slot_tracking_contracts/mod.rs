//! Public stream contracts shared verbatim with portable conformance runners.

pub mod failures;
pub mod lifecycle;
pub mod ordering;

use core::{
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};
use futures_core::Stream;
use solana_kit_client::{
    Presence,
    methods::{Context as RpcContext, Contextual},
};
use std::{collections::VecDeque, marker::PhantomPinned, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub enum Failure {
    Initial,
    Subscription,
    InitialMapper,
    SubscriptionMapper,
}

type InitialState<T> = Rc<RefCell<Option<Result<Contextual<T>, Failure>>>>;
type SubscriptionQueue<T> = Rc<RefCell<VecDeque<Result<Contextual<T>, Failure>>>>;

pub fn response<T>(slot: u64, value: T) -> Contextual<T> {
    Contextual {
        context: RpcContext {
            slot,
            api_version: Presence::Present("source-api".into()),
        },
        value,
    }
}

pub fn next<S: Stream + Unpin>(stream: &mut S) -> Poll<Option<S::Item>> {
    Pin::new(stream).poll_next(&mut Context::from_waker(Waker::noop()))
}

pub fn value<S, T>(stream: &mut S) -> Contextual<T>
where
    S: Stream<Item = Result<Contextual<T>, Failure>> + Unpin,
{
    match next(stream) {
        Poll::Ready(Some(Ok(value))) => value,
        _ => panic!("expected a current successful response"),
    }
}

pub struct Initial<T> {
    state: InitialState<T>,
    polls: Rc<Cell<usize>>,
    drops: Rc<Cell<usize>>,
    _pin: PhantomPinned,
}

pub struct InitialControl<T> {
    pub state: InitialState<T>,
    pub polls: Rc<Cell<usize>>,
    pub drops: Rc<Cell<usize>>,
}

pub fn initial<T>() -> (Initial<T>, InitialControl<T>) {
    let state = Rc::new(RefCell::new(None));
    let polls = Rc::new(Cell::new(0));
    let drops = Rc::new(Cell::new(0));
    let initial = Initial {
        state: state.clone(),
        polls: polls.clone(),
        drops: drops.clone(),
        _pin: PhantomPinned,
    };

    (
        initial,
        InitialControl {
            state,
            polls,
            drops,
        },
    )
}

impl<T> Future for Initial<T> {
    type Output = Result<Contextual<T>, Failure>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        self.polls.set(self.polls.get() + 1);
        self.state
            .borrow_mut()
            .take()
            .map_or(Poll::Pending, Poll::Ready)
    }
}

impl<T> Drop for Initial<T> {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

pub struct Subscription<T> {
    queue: SubscriptionQueue<T>,
    closed: Rc<Cell<bool>>,
    polls: Rc<Cell<usize>>,
    drops: Rc<Cell<usize>>,
    _pin: PhantomPinned,
}

pub struct SubscriptionControl<T> {
    pub queue: SubscriptionQueue<T>,
    pub closed: Rc<Cell<bool>>,
    pub polls: Rc<Cell<usize>>,
    pub drops: Rc<Cell<usize>>,
}

pub fn subscription<T>() -> (Subscription<T>, SubscriptionControl<T>) {
    let queue = Rc::new(RefCell::new(VecDeque::new()));
    let closed = Rc::new(Cell::new(false));
    let polls = Rc::new(Cell::new(0));
    let drops = Rc::new(Cell::new(0));
    let subscription = Subscription {
        queue: queue.clone(),
        closed: closed.clone(),
        polls: polls.clone(),
        drops: drops.clone(),
        _pin: PhantomPinned,
    };

    (
        subscription,
        SubscriptionControl {
            queue,
            closed,
            polls,
            drops,
        },
    )
}

impl<T> Stream for Subscription<T> {
    type Item = Result<Contextual<T>, Failure>;

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.polls.set(self.polls.get() + 1);

        if let Some(value) = self.queue.borrow_mut().pop_front() {
            return Poll::Ready(Some(value));
        }

        if self.closed.get() {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    }
}

impl<T> Drop for Subscription<T> {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}
