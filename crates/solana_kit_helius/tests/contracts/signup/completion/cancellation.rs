//! Dropping completion destroys actual suspended Developer exchanges and timers.

use super::*;
use solana_kit_client::TransportError;
use std::{
    cell::RefCell,
    collections::VecDeque,
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

type Requests = Rc<RefCell<Vec<ServiceRequest>>>;

struct Controlled {
    replies: RefCell<VecDeque<Option<Reply>>>,
    requests: Requests,
    active: Rc<Cell<usize>>,
    dropped: Rc<Cell<usize>>,
}

struct Exchange {
    reply: Option<Reply>,
    active: Rc<Cell<usize>>,
    dropped: Rc<Cell<usize>>,
}

impl ServiceTransport<Service> for Controlled {
    type Exchange<'a> = Exchange;

    fn exchange(&self, request: ServiceRequest) -> Self::Exchange<'_> {
        assert_eq!(request.service, Service::Developer);
        self.requests.borrow_mut().push(request);
        self.active.set(self.active.get() + 1);
        Exchange {
            reply: self
                .replies
                .borrow_mut()
                .pop_front()
                .expect("scripted exchange"),
            active: self.active.clone(),
            dropped: self.dropped.clone(),
        }
    }
}

impl Future for Exchange {
    type Output = Result<Reply, TransportError>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut()
            .reply
            .take()
            .map_or(Poll::Pending, |reply| Poll::Ready(Ok(reply)))
    }
}

impl Drop for Exchange {
    fn drop(&mut self) {
        self.active.set(self.active.get() - 1);
        self.dropped.set(self.dropped.get() + 1);
    }
}

/// Each stage drops its exchange or timer immediately; the caller's receipt survives.
pub fn dropped_completion() {
    let stages = [
        vec![None],
        vec![Some(reply(COMPLETE)), None],
        vec![Some(reply(COMPLETE)), Some(reply(PROJECTS)), None],
        vec![
            Some(reply(COMPLETE)),
            Some(reply(PROJECTS)),
            Some(reply(EMPTY_KEYS)),
            None,
        ],
    ];

    for script in stages {
        dropped_exchange(script);
    }

    dropped_timer(vec![reply(PENDING)]);
    dropped_timer(vec![reply(COMPLETE), reply("[]")]);
}

fn dropped_exchange(script: Vec<Option<Reply>>) {
    let expected = script.len();
    let active = Rc::new(Cell::new(0));
    let dropped = Rc::new(Cell::new(0));
    let requests = Rc::new(RefCell::new(Vec::new()));
    let client = ServiceClient::new(
        Controlled {
            replies: RefCell::new(script.into()),
            requests: requests.clone(),
            active: active.clone(),
            dropped: dropped.clone(),
        },
        Budgets::default(),
    );
    let input = pending();
    let original = input.clone();
    let mut future = Box::pin(resume_signup_and_pay(
        &client,
        &input,
        policy(),
        || 0,
        |_| ready(Ok::<_, ()>(())),
    ));
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    assert_eq!(requests.borrow().len(), expected);
    assert_eq!(active.get(), 1);
    drop(future);
    assert_eq!(active.get(), 0);
    assert_eq!(dropped.get(), expected);
    assert_eq!(input, original);
}

struct Timer(Rc<Cell<usize>>);

impl Future for Timer {
    type Output = Result<(), ()>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

fn dropped_timer(replies: Vec<Reply>) {
    let expected = replies.len();
    let (client, trace) = client(replies);
    let dropped = Rc::new(Cell::new(0));
    let timer = dropped.clone();
    let input = pending();
    let original = input.clone();
    let mut future = Box::pin(resume_signup_and_pay(
        &client,
        &input,
        policy(),
        || 0,
        move |_| Timer(timer.clone()),
    ));
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    assert_eq!(trace.borrow().len(), expected);
    assert_eq!(dropped.get(), 0);
    drop(future);
    assert_eq!(dropped.get(), 1);
    assert_eq!(input, original);
}
