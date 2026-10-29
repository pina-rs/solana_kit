use core::{
    future::Future,
    pin::Pin,
    task::{Context as TaskContext, Poll, Waker},
};
use futures_core::Stream;
use solana_kit_client::{
    Presence,
    methods::{Context, Contextual},
    reactive::{CancellationToken, slot_tracking::LocalSlotTrackingStore},
};
use std::{
    boxed::Box,
    cell::{Cell, RefCell},
    collections::VecDeque,
    format,
    rc::Rc,
    vec::Vec,
};

#[derive(Debug)]
pub struct Failure(pub &'static str);

pub struct Mapped {
    pub origin: &'static str,
    pub value: i32,
    pub marker: Rc<Cell<bool>>,
}

pub fn response(slot: u64, value: i32) -> Contextual<i32> {
    Contextual {
        context: Context {
            slot,
            api_version: Presence::Present(format!("context-{slot}")),
        },
        value,
    }
}

pub fn poll<F: Future + Unpin>(future: &mut F) -> Poll<F::Output> {
    Pin::new(future).poll(&mut TaskContext::from_waker(Waker::noop()))
}

struct InputState {
    values: VecDeque<Result<Contextual<i32>, Failure>>,
    ended: bool,
    drops: usize,
    polls: usize,
    waker: Option<Waker>,
    on_poll: Option<Box<dyn FnOnce()>>,
    on_drop: Option<Box<dyn FnOnce()>>,
}

#[derive(Clone)]
pub struct Control {
    state: Rc<RefCell<InputState>>,
    pub signal: CancellationToken<Failure>,
}

impl Control {
    fn new(signal: CancellationToken<Failure>) -> Self {
        Self {
            state: Rc::new(RefCell::new(InputState {
                values: VecDeque::new(),
                ended: false,
                drops: 0,
                polls: 0,
                waker: None,
                on_poll: None,
                on_drop: None,
            })),
            signal,
        }
    }

    pub fn push(&self, result: Result<Contextual<i32>, Failure>) {
        let waker = {
            let mut state = self.state.borrow_mut();
            state.values.push_back(result);
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
        self.state.borrow().drops
    }
    pub fn polls(&self) -> usize {
        self.state.borrow().polls
    }
    pub fn on_poll(&self, callback: impl FnOnce() + 'static) {
        self.state.borrow_mut().on_poll = Some(Box::new(callback));
    }
    pub fn on_drop(&self, callback: impl FnOnce() + 'static) {
        self.state.borrow_mut().on_drop = Some(Box::new(callback));
    }

    fn read(&self, cx: &mut TaskContext<'_>) -> Poll<Option<Result<Contextual<i32>, Failure>>> {
        let next = cx.waker().clone();
        let (hook, previous) = {
            let mut state = self.state.borrow_mut();
            state.polls += 1;
            (state.on_poll.take(), state.waker.replace(next))
        };
        drop(previous);

        if let Some(hook) = hook {
            hook();
        }
        let mut state = self.state.borrow_mut();
        state
            .values
            .pop_front()
            .map(Some)
            .or_else(|| state.ended.then_some(None))
            .map_or(Poll::Pending, Poll::Ready)
    }

    fn dropped(&self) {
        let (hook, waker) = {
            let mut state = self.state.borrow_mut();
            state.drops += 1;
            (state.on_drop.take(), state.waker.take())
        };
        drop(waker);

        if let Some(hook) = hook {
            hook();
        }
    }
}

pub struct Initial(Control);
pub struct Subscription(Control);

impl Future for Initial {
    type Output = Result<Contextual<i32>, Failure>;
    fn poll(self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        match self.0.read(cx) {
            Poll::Ready(Some(result)) => Poll::Ready(result),
            _ => Poll::Pending,
        }
    }
}

impl Stream for Subscription {
    type Item = Result<Contextual<i32>, Failure>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<Self::Item>> {
        self.0.read(cx)
    }
}

impl Drop for Initial {
    fn drop(&mut self) {
        self.0.dropped();
    }
}
impl Drop for Subscription {
    fn drop(&mut self) {
        self.0.dropped();
    }
}

type InitialFactory = Box<dyn Fn(CancellationToken<Failure>) -> Initial>;
type SubscriptionFactory = Box<dyn Fn(CancellationToken<Failure>) -> Subscription>;
type Mapper = Box<dyn Fn(i32) -> Result<Mapped, Failure>>;
pub type Store = LocalSlotTrackingStore<
    Mapped,
    Failure,
    InitialFactory,
    SubscriptionFactory,
    Mapper,
    Mapper,
    Initial,
    Subscription,
>;

pub struct Fixture {
    pub store: Store,
    pub initial: Rc<RefCell<Vec<Control>>>,
    pub subscription: Rc<RefCell<Vec<Control>>>,
    pub initial_maps: Rc<RefCell<Vec<i32>>>,
    pub subscription_maps: Rc<RefCell<Vec<i32>>>,
    pub fail_value: Rc<Cell<Option<i32>>>,
}

impl Fixture {
    pub fn new() -> Self {
        let initial = Rc::new(RefCell::new(Vec::new()));
        let subscription = Rc::new(RefCell::new(Vec::new()));
        let initial_maps = Rc::new(RefCell::new(Vec::new()));
        let subscription_maps = Rc::new(RefCell::new(Vec::new()));
        let fail_value = Rc::new(Cell::new(None));
        let controls = initial.clone();
        let initial_factory: InitialFactory = Box::new(move |signal| {
            let control = Control::new(signal);
            controls.borrow_mut().push(control.clone());
            Initial(control)
        });
        let controls = subscription.clone();
        let subscription_factory: SubscriptionFactory = Box::new(move |signal| {
            let control = Control::new(signal);
            controls.borrow_mut().push(control.clone());
            Subscription(control)
        });
        let store = Store::new_local(
            initial_factory,
            subscription_factory,
            mapper("initial", initial_maps.clone(), fail_value.clone()),
            mapper(
                "subscription",
                subscription_maps.clone(),
                fail_value.clone(),
            ),
        );
        Self {
            store,
            initial,
            subscription,
            initial_maps,
            subscription_maps,
            fail_value,
        }
    }

    pub fn initial(&self, index: usize) -> Control {
        self.initial.borrow()[index].clone()
    }
    pub fn subscription(&self, index: usize) -> Control {
        self.subscription.borrow()[index].clone()
    }
}

fn mapper(
    origin: &'static str,
    calls: Rc<RefCell<Vec<i32>>>,
    fail_value: Rc<Cell<Option<i32>>>,
) -> Mapper {
    Box::new(move |value| {
        calls.borrow_mut().push(value);

        if fail_value.get() == Some(value) {
            return Err(Failure("mapper failed"));
        }

        Ok(Mapped {
            origin,
            value,
            marker: Rc::new(Cell::new(true)),
        })
    })
}
