//! Competing real host-readiness ownership prototypes. Not a production adapter.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::task::{Context, Poll, Wake, Waker};
use wasip2::clocks::monotonic_clock;
use wasip2::io::poll::{Pollable, poll};

#[derive(Default)]
struct Counter(AtomicUsize);
impl Wake for Counter {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

/// Candidate A owns readiness in the future; its dedicated owner drives it.
struct Direct {
    timer: Pollable,
    waiter: Option<Waker>,
}
impl Future for Direct {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<()> {
        if self.timer.ready() {
            return Poll::Ready(());
        }

        self.waiter = Some(context.waker().clone());
        Poll::Pending
    }
}
impl Direct {
    fn drive(&mut self) {
        self.timer.block();
        self.waiter.take().expect("registered").wake();
    }
}

/// Candidate B owns registrations in a caller reactor, with drop guards.
#[derive(Clone, Default)]
struct Reactor(Rc<RefCell<Registry>>);
#[derive(Default)]
struct Registry {
    next: u64,
    waits: BTreeMap<u64, (Pollable, Waker)>,
}
struct Wait {
    reactor: Reactor,
    id: u64,
}
impl Future for Wait {
    type Output = ();
    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<()> {
        let mut registry = self.reactor.0.borrow_mut();
        let (pollable, waker) = registry.waits.get_mut(&self.id).expect("live registration");

        if pollable.ready() {
            return Poll::Ready(());
        }

        waker.clone_from(context.waker());
        Poll::Pending
    }
}
impl Drop for Wait {
    fn drop(&mut self) {
        self.reactor.0.borrow_mut().waits.remove(&self.id);
    }
}
impl Reactor {
    fn wait(&self, pollable: Pollable, waker: &Waker) -> Wait {
        let mut registry = self.0.borrow_mut();
        let id = registry.next;
        registry.next += 1;
        registry.waits.insert(id, (pollable, waker.clone()));
        Wait {
            reactor: self.clone(),
            id,
        }
    }
    fn drive(&self) {
        let ready = {
            let registry = self.0.borrow();
            let entries: Vec<_> = registry.waits.values().collect();
            let pollables: Vec<_> = entries.iter().map(|entry| &entry.0).collect();
            poll(&pollables)
                .into_iter()
                .map(|index| entries[index as usize].1.clone())
                .collect::<Vec<_>>()
        };

        for waker in ready {
            waker.wake();
        }
    }
}

fn main() {
    let counter = Arc::new(Counter::default());
    let waker = Waker::from(counter.clone());
    let mut context = Context::from_waker(&waker);
    let mut direct = Direct {
        timer: monotonic_clock::subscribe_duration(20_000_000),
        waiter: None,
    };
    assert!(Pin::new(&mut direct).poll(&mut context).is_pending());
    direct.drive();
    assert_eq!(counter.0.load(Ordering::Relaxed), 1);
    assert!(Pin::new(&mut direct).poll(&mut context).is_ready());

    drop(direct);
    println!("PASS direct observed Pending, actual host readiness, one wake, resource drop");

    let reactor = Reactor::default();
    let local = Rc::new(41);
    let mut wait = reactor.wait(monotonic_clock::subscribe_duration(20_000_000), &waker);
    assert!(Pin::new(&mut wait).poll(&mut context).is_pending());
    reactor.drive();
    assert_eq!(counter.0.load(Ordering::Relaxed), 2);
    assert!(Pin::new(&mut wait).poll(&mut context).is_ready());
    assert_eq!(*local, 41);

    drop(wait);
    assert!(reactor.0.borrow().waits.is_empty());
    let pending = reactor.wait(monotonic_clock::subscribe_duration(1_000_000_000), &waker);
    assert_eq!(reactor.0.borrow().waits.len(), 1);
    drop(pending);
    assert!(reactor.0.borrow().waits.is_empty());
    println!(
        "PASS registry observed Pending, actual host readiness, one wake, Rc, success/drop unregister"
    );
}
