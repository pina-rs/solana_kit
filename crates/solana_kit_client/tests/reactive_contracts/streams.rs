use super::support::{Queue, QueueControl, poll};
use solana_kit_client::reactive::{
    CancellationSource, CancellationToken, LocalReactiveStreamStore, ReactiveStreamConnection,
    StreamSnapshot,
};
use std::{
    boxed::Box,
    cell::{Cell, RefCell},
    future::{Future, Ready, ready},
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
    vec::Vec,
};

type Data = Queue<Result<i32, &'static str>>;

type Errors = Queue<Option<&'static str>>;

type Connection = ReactiveStreamConnection<Data, Errors>;

type Factory =
    Box<dyn Fn(CancellationToken<&'static str>) -> Ready<Result<Connection, &'static str>>>;

pub(super) type Source = LocalReactiveStreamStore<
    i32,
    &'static str,
    Factory,
    Ready<Result<Connection, &'static str>>,
    Data,
    Errors,
>;

type Controls = (
    QueueControl<Result<i32, &'static str>>,
    QueueControl<Option<&'static str>>,
);

pub(super) fn source() -> (Source, Rc<RefCell<Vec<Controls>>>) {
    let controls = Rc::new(RefCell::new(Vec::new()));
    let opened = controls.clone();
    let factory: Factory = Box::new(move |_| {
        let (data, data_control) = Queue::new();
        let (errors, error_control) = Queue::new();
        opened.borrow_mut().push((data_control, error_control));
        ready(Ok(ReactiveStreamConnection { data, errors }))
    });
    (Source::new_local(factory), controls)
}

#[cfg_attr(test, test)]
pub fn reconnect_retains_data_error_and_drops_both_previous_lanes() {
    let (store, controls) = source();
    let mut first = store.connect();

    assert!(matches!(&*store.state(), StreamSnapshot::Loading { .. }));

    assert!(poll(&mut first).is_pending());
    controls.borrow()[0].0.push(Ok(7));

    assert!(poll(&mut first).is_pending());
    controls.borrow()[0].1.push(Some("failure"));

    assert!(poll(&mut first).is_pending());
    let error = store.state().error().unwrap().clone();
    let mut replacement = store.connect();

    assert_eq!(store.state().data().map(|data| **data), Some(7));

    assert!(std::sync::Arc::ptr_eq(
        store.state().error().unwrap(),
        &error
    ));

    assert_eq!(controls.borrow()[0].0.drops(), 1);

    assert_eq!(controls.borrow()[0].1.drops(), 1);

    assert!(matches!(poll(&mut first), Poll::Ready(Err(_))));

    assert!(poll(&mut replacement).is_pending());
    controls.borrow()[1].0.push(Ok(9));

    assert!(poll(&mut replacement).is_pending());

    assert_eq!(store.state().data().map(|data| **data), Some(9));

    assert!(store.state().error().is_none());
}

#[cfg_attr(test, test)]
pub fn recoverable_errors_nulls_and_source_end_preserve_source_semantics() {
    let (store, controls) = source();
    let mut driver = store.connect();

    assert!(poll(&mut driver).is_pending());
    controls.borrow()[0].1.push(None);

    assert!(poll(&mut driver).is_pending());

    assert!(matches!(&*store.state(), StreamSnapshot::Loading { .. }));
    controls.borrow()[0].0.push(Err("first"));

    assert!(poll(&mut driver).is_pending());
    let first = store.state().error().unwrap().clone();
    controls.borrow()[0].1.push(Some("ignored second"));

    assert!(poll(&mut driver).is_pending());

    assert!(std::sync::Arc::ptr_eq(
        store.state().error().unwrap(),
        &first
    ));
    controls.borrow()[0].0.push(Ok(5));

    assert!(poll(&mut driver).is_pending());

    assert!(store.state().error().is_none());
    controls.borrow()[0].0.finish();
    controls.borrow()[0].1.push(Some("after data ended"));

    assert!(poll(&mut driver).is_pending());

    assert!(matches!(&*store.state(), StreamSnapshot::Error { .. }));
    let final_snapshot = store.state();
    controls.borrow()[0].1.finish();

    assert!(matches!(poll(&mut driver), Poll::Ready(Ok(()))));

    assert!(std::sync::Arc::ptr_eq(&store.state(), &final_snapshot));
    drop(driver);

    assert!(std::sync::Arc::ptr_eq(&store.state(), &final_snapshot));
}

#[cfg_attr(test, test)]
pub fn caller_scope_reset_dispose_and_driver_drop_cancel_stream_work() {
    let (store, controls) = source();
    let caller = CancellationSource::new();
    caller.cancel(None);
    let mut rejected = store.connect_with_signal(caller.token());

    assert!(controls.borrow().is_empty());

    assert!(matches!(poll(&mut rejected), Poll::Ready(Err(_))));
    let caller = CancellationSource::new();
    let mut old = store.connect_with_signal(caller.token());

    assert!(poll(&mut old).is_pending());
    let mut new = store.connect();
    caller.cancel(Some("too late"));

    assert!(poll(&mut new).is_pending());
    controls.borrow()[1].0.push(Ok(8));

    assert!(poll(&mut new).is_pending());

    assert_eq!(store.state().data().map(|data| **data), Some(8));
    store.reset();

    assert_eq!(controls.borrow()[1].0.drops(), 1);

    assert!(matches!(&*store.state(), StreamSnapshot::Idle));
    let mut dropped = store.connect();

    assert!(poll(&mut dropped).is_pending());
    drop(dropped);

    assert_eq!(controls.borrow()[2].1.drops(), 1);
    let mut disposed = store.connect();

    assert!(poll(&mut disposed).is_pending());
    store.dispose();

    assert_eq!(controls.borrow()[3].0.drops(), 1);

    assert!(matches!(poll(&mut disposed), Poll::Ready(Err(_))));
}

struct Opening(Rc<Cell<usize>>);

impl Future for Opening {
    type Output = Result<Connection, &'static str>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl Drop for Opening {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[cfg_attr(test, test)]
pub fn pending_factory_is_destroyed_by_reset_dispose_and_unpolled_drop() {
    let drops = Rc::new(Cell::new(0));
    let tracked = drops.clone();
    let store = LocalReactiveStreamStore::new_local(move |_| Opening(tracked.clone()));
    let mut first = store.connect();

    assert!(poll(&mut first).is_pending());
    store.reset();

    assert_eq!(drops.get(), 1);
    let second = store.connect();
    drop(second);

    assert_eq!(drops.get(), 2);
    let third = store.connect();
    store.dispose();

    assert_eq!(drops.get(), 3);
    drop((first, third));

    assert_eq!(drops.get(), 3);
}
