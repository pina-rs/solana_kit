//! Manual time advances only when an actual pending sleep has registered readiness.
use solana_kit_client::subscriptions::{Clock, ClockError};
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    future::Future,
    pin::{Pin, pin},
    rc::Rc,
    task::{Context, Poll, Waker},
};

#[derive(Clone, Default)]
pub(super) struct Manual {
    now: Rc<Cell<u64>>,
    sequence: Rc<Cell<usize>>,
    pending: Rc<RefCell<BTreeMap<usize, (u64, Waker)>>>,
    pub fail_at: Rc<Cell<Option<usize>>>,
}

impl Manual {
    pub fn set(&self, tick: u64) {
        assert!(tick >= self.now.get());
        self.now.set(tick);
    }
    pub fn pending(&self) -> usize {
        self.pending.borrow().len()
    }
    pub fn now(&self) -> u64 {
        self.now.get()
    }

    /// No hidden busy polling: every driver advance requires registered readiness.
    fn advance(&self) {
        let deadline = self
            .pending
            .borrow()
            .values()
            .map(|(deadline, _)| *deadline)
            .min()
            .expect("pending operation must register a timer");
        self.set(deadline);

        for (_, waker) in self.pending.borrow().values() {
            waker.wake_by_ref();
        }
    }
}

impl Clock for Manual {
    type Sleep<'a> = Sleep;
    fn now(&self) -> u64 {
        self.now.get()
    }

    fn sleep_until(&self, deadline: u64) -> Self::Sleep<'_> {
        let id = self.sequence.get();
        self.sequence.set(id + 1);
        Sleep {
            clock: self.clone(),
            id,
            deadline,
            fail: self.fail_at.get() == Some(id),
        }
    }
}

pub(super) struct Sleep {
    clock: Manual,
    id: usize,
    deadline: u64,
    fail: bool,
}

impl Future for Sleep {
    type Output = Result<(), ClockError>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.fail {
            return Poll::Ready(Err(ClockError::Capacity));
        }
        if self.clock.now.get() >= self.deadline {
            return Poll::Ready(Ok(()));
        }

        self.clock
            .pending
            .borrow_mut()
            .insert(self.id, (self.deadline, context.waker().clone()));
        Poll::Pending
    }
}

impl Drop for Sleep {
    fn drop(&mut self) {
        self.clock.pending.borrow_mut().remove(&self.id);
    }
}

/// Runs public futures, advancing only the next real registered clock deadline.
pub(super) fn run<F: Future>(future: F, clock: &Manual) -> F::Output {
    const MAX_POLLS: usize = 256;
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());

    for _ in 0..MAX_POLLS {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }

        clock.advance();
    }

    panic!("finite Hermes operation exceeded driver bound")
}
