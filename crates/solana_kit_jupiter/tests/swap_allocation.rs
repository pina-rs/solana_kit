#![cfg(not(target_family = "wasm"))]
//! Request preparation must reject oversized caller data without copying it first.

use solana_kit_client::{Reply, TransportError, rest};
use solana_kit_core::Address;
use solana_kit_jupiter::{Client, Integer, OrderRequest, OrderResponse, Service};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    future::{Future, Ready, ready},
    hint::black_box,
    pin::pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

const INPUT_BYTES: usize = 2 * 1024 * 1024;
const MAX_ALLOCATION: usize = 16 * 1024;

struct Observer;

thread_local! {
    // Constant initialization has no allocation or destructor, including during teardown.
    static MAX_REQUEST: Cell<Option<usize>> = const { Cell::new(None) };
}

// SAFETY: The observer only records requested sizes; System owns every allocation.
unsafe impl GlobalAlloc for Observer {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record(layout.size());
        // SAFETY: Forward the caller's unchanged layout to the backing allocator.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: Forward the pointer and layout to the allocator that created it.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        record(size);
        // SAFETY: Forward the caller's pointer, layout and size unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

#[global_allocator]
static ALLOCATOR: Observer = Observer;

fn record(size: usize) {
    // try_with cannot unwind if thread-local storage has already been destroyed.
    let _ = MAX_REQUEST.try_with(|maximum| {
        if let Some(previous) = maximum.get() {
            maximum.set(Some(previous.max(size)));
        }
    });
}

struct Window;

impl Drop for Window {
    fn drop(&mut self) {
        MAX_REQUEST.set(None);
    }
}

fn measure<T>(operation: impl FnOnce() -> T) -> (T, usize) {
    assert!(MAX_REQUEST.get().is_none(), "nested allocation window");
    let window = Window;
    MAX_REQUEST.set(Some(0));
    let result = operation();
    let maximum = MAX_REQUEST.get().expect("allocation window is active");
    drop(window);
    (result, maximum)
}

#[derive(Default)]
struct NoIo(Rc<Cell<usize>>);

impl rest::Transport<Service> for NoIo {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, _: rest::Request<Service>) -> Self::Exchange<'_> {
        self.0.set(self.0.get() + 1);
        ready(Ok(Reply {
            status: 200,
            body: b"{}".to_vec(),
        }))
    }
}

fn poll_once<T>(future: impl Future<Output = T>) -> T {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    match future.as_mut().poll(&mut context) {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("budget rejection unexpectedly yielded"),
    }
}

fn budgets() -> rest::Budgets {
    rest::Budgets {
        bytes: 64,
        ..rest::Budgets::default()
    }
}

fn integer() -> Integer {
    "9".repeat(INPUT_BYTES).parse().expect("valid integer")
}

fn order(amount: Integer) -> OrderRequest {
    OrderRequest::new(
        Address::from_bytes([0; 32]),
        Address::from_bytes([1; 32]),
        amount,
    )
}

#[test]
fn observer_counts_only_allocations_inside_the_window() {
    black_box(Vec::<u8>::with_capacity(INPUT_BYTES));
    assert_eq!(measure(|| ()).1, 0);
    let (_, maximum) = measure(|| black_box(Vec::<u8>::with_capacity(MAX_ALLOCATION)));
    assert!(
        maximum >= MAX_ALLOCATION,
        "observer missed a known allocation"
    );
    black_box(Vec::<u8>::with_capacity(INPUT_BYTES));
    assert_eq!(measure(|| ()).1, 0);
}

#[test]
fn execution_rejects_large_expiry_without_large_allocation() {
    let accepted = OrderResponse {
        request_id: Some("accepted".into()),
        last_valid_block_height: Some(integer()),
        ..OrderResponse::default()
    };
    let calls = Rc::new(Cell::new(0));
    let client = Client::new(NoIo(calls.clone()), budgets());
    let (result, maximum) = measure(|| poll_once(client.execute_order(&accepted, "signed")));
    assert!(result.is_err());
    assert_eq!(calls.get(), 0);
    assert!(
        maximum <= MAX_ALLOCATION,
        "largest allocation: {maximum} bytes"
    );
}

#[test]
fn query_rejects_large_amount_without_large_allocation() {
    let request = order(integer());
    let calls = Rc::new(Cell::new(0));
    let client = Client::new(
        NoIo(calls.clone()),
        rest::Budgets {
            bytes: 256,
            ..budgets()
        },
    );
    let (result, maximum) = measure(|| poll_once(client.get_order(&request)));
    assert!(result.is_err());
    assert_eq!(calls.get(), 0);
    assert!(
        maximum <= MAX_ALLOCATION,
        "largest allocation: {maximum} bytes"
    );
    let (result, maximum) = measure(|| poll_once(client.build_swap(&request)));
    assert!(result.is_err());
    assert_eq!(calls.get(), 0);
    assert!(
        maximum <= MAX_ALLOCATION,
        "largest allocation: {maximum} bytes"
    );
}

#[test]
fn query_accepts_shadowed_large_amount_without_large_allocation() {
    let mut request = order(integer());
    request
        .extra_query_parameters
        .insert("amount".into(), "1".into());
    let (_, positive_control) = measure(|| black_box(request.amount.clone()));
    assert!(
        positive_control >= INPUT_BYTES,
        "observer missed amount clone"
    );
    let calls = Rc::new(Cell::new(0));
    let client = Client::new(
        NoIo(calls.clone()),
        rest::Budgets {
            bytes: 256,
            ..budgets()
        },
    );

    let (result, maximum) = measure(|| poll_once(client.get_order(&request)));
    assert!(result.is_ok(), "shadowed amount rejected by order");
    assert_eq!(calls.get(), 1);
    assert!(
        maximum <= MAX_ALLOCATION,
        "largest allocation: {maximum} bytes"
    );

    let (result, maximum) = measure(|| poll_once(client.build_swap(&request)));
    assert!(result.is_ok(), "shadowed amount rejected by build");
    assert_eq!(calls.get(), 2);
    assert!(
        maximum <= MAX_ALLOCATION,
        "largest allocation: {maximum} bytes"
    );
}

#[test]
fn query_rejects_large_extra_without_large_allocation() {
    let mut request = order(Integer::from(1));
    request
        .extra_query_parameters
        .insert("extra".into(), "x".repeat(INPUT_BYTES));
    let calls = Rc::new(Cell::new(0));
    let client = Client::new(
        NoIo(calls.clone()),
        rest::Budgets {
            bytes: 256,
            ..budgets()
        },
    );
    let (result, maximum) = measure(|| poll_once(client.get_order(&request)));
    assert!(result.is_err());
    assert_eq!(calls.get(), 0);
    assert!(
        maximum <= MAX_ALLOCATION,
        "largest allocation: {maximum} bytes"
    );
    let (result, maximum) = measure(|| poll_once(client.build_swap(&request)));
    assert!(result.is_err());
    assert_eq!(calls.get(), 0);
    assert!(
        maximum <= MAX_ALLOCATION,
        "largest allocation: {maximum} bytes"
    );
}
