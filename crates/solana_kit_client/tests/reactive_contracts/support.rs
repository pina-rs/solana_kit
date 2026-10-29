use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};
use futures_core::Stream;
use solana_kit_client::reactive::{CancellationToken, LocalReactiveActionStore};
use std::{
    boxed::Box,
    cell::{Cell, RefCell},
    collections::VecDeque,
    rc::Rc,
    vec::Vec,
};

pub fn poll<F: Future + Unpin>(future: &mut F) -> Poll<F::Output> {
    Pin::new(future).poll(&mut Context::from_waker(Waker::noop()))
}

pub fn next<S: Stream + Unpin>(stream: &mut S) -> Poll<Option<S::Item>> {
    Pin::new(stream).poll_next(&mut Context::from_waker(Waker::noop()))
}

#[derive(Default)]
struct ProbeState {
    result: Option<Result<i32, &'static str>>,
    drops: usize,
    polls: usize,
    on_poll: Option<Box<dyn FnOnce()>>,
    on_drop: Option<Box<dyn FnOnce()>>,
    waker: Option<Waker>,
}

#[derive(Clone, Default)]
pub struct ProbeControl(Rc<RefCell<ProbeState>>);

pub struct Probe(pub ProbeControl);

impl ProbeControl {
    pub fn complete(&self, result: Result<i32, &'static str>) {
        let waker = {
            let mut state = self.0.borrow_mut();
            state.result = Some(result);
            state.waker.take()
        };

        if let Some(waker) = waker {
            waker.wake();
        }
    }

    pub fn drops(&self) -> usize {
        self.0.borrow().drops
    }

    pub fn polls(&self) -> usize {
        self.0.borrow().polls
    }

    pub fn on_poll(&self, callback: impl FnOnce() + 'static) {
        self.0.borrow_mut().on_poll = Some(Box::new(callback));
    }

    pub fn on_drop(&self, callback: impl FnOnce() + 'static) {
        self.0.borrow_mut().on_drop = Some(Box::new(callback));
    }
}

impl Future for Probe {
    type Output = Result<i32, &'static str>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let next = cx.waker().clone();
        let (callback, previous) = {
            let mut state = self.0.0.borrow_mut();
            state.polls += 1;
            (state.on_poll.take(), state.waker.replace(next))
        };
        drop(previous);

        if let Some(callback) = callback {
            callback();
        }

        self.0
            .0
            .borrow_mut()
            .result
            .take()
            .map_or(Poll::Pending, Poll::Ready)
    }
}

impl Drop for Probe {
    fn drop(&mut self) {
        let (callback, waker) = {
            let mut state = self.0.0.borrow_mut();
            state.drops += 1;
            (state.on_drop.take(), state.waker.take())
        };
        drop(waker);

        if let Some(callback) = callback {
            callback();
        }
    }
}

type Factory = Box<dyn Fn(CancellationToken<&'static str>, ()) -> Probe>;
pub type Action = LocalReactiveActionStore<(), i32, &'static str, Factory, Probe>;

pub fn action() -> (Action, Rc<RefCell<Vec<ProbeControl>>>) {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let output = calls.clone();
    let factory: Factory = Box::new(move |_, ()| {
        let control = ProbeControl::default();
        output.borrow_mut().push(control.clone());
        Probe(control)
    });
    (Action::new_local(factory), calls)
}

struct QueueState<T> {
    values: VecDeque<T>,
    ended: bool,
    waker: Option<Waker>,
}

pub struct Queue<T> {
    state: Rc<RefCell<QueueState<T>>>,
    drops: Rc<Cell<usize>>,
}

pub struct QueueControl<T> {
    state: Rc<RefCell<QueueState<T>>>,
    drops: Rc<Cell<usize>>,
}

impl<T> Queue<T> {
    pub fn new() -> (Self, QueueControl<T>) {
        let state = Rc::new(RefCell::new(QueueState {
            values: VecDeque::new(),
            ended: false,
            waker: None,
        }));
        let drops = Rc::new(Cell::new(0));
        (
            Self {
                state: state.clone(),
                drops: drops.clone(),
            },
            QueueControl { state, drops },
        )
    }
}

impl<T> QueueControl<T> {
    pub fn push(&self, value: T) {
        let waker = {
            let mut state = self.state.borrow_mut();
            state.values.push_back(value);
            state.waker.take()
        };

        if let Some(waker) = waker {
            waker.wake();
        }
    }

    pub fn finish(&self) {
        let waker = {
            let mut state = self.state.borrow_mut();
            state.ended = true;
            state.waker.take()
        };

        if let Some(waker) = waker {
            waker.wake();
        }
    }

    pub fn drops(&self) -> usize {
        self.drops.get()
    }
}

impl<T> Stream for Queue<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>> {
        let next = cx.waker().clone();
        let (result, previous) = {
            let mut state = self.state.borrow_mut();
            let result = state
                .values
                .pop_front()
                .map(Some)
                .or_else(|| state.ended.then_some(None));
            (result, state.waker.replace(next))
        };
        drop(previous);
        result.map_or(Poll::Pending, Poll::Ready)
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
        let previous = self.state.borrow_mut().waker.take();
        drop(previous);
    }
}
