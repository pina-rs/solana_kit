#![cfg(not(target_family = "wasm"))]
//! Byte-budget rejection must not duplicate large caller-owned RPC parameters.
//!
//! The observer records the largest requested allocation on the current thread
//! during a call. It does not measure aggregate allocations or process RSS.

use serde::Serialize;
use solana_kit_client::methods::Method;
use solana_kit_client::{Client, Error, Limits, Reply, Request, Transport, TransportError};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::{Cell, RefCell},
    future::{Future, Ready, ready},
    hint::black_box,
    pin::pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

const INPUT_BYTES: usize = 2 * 1024 * 1024;
const REQUEST_BYTES: usize = 256;
// Allow allocator growth and small framing/response bookkeeping, not an input-sized copy.
const MAX_ALLOCATION: usize = REQUEST_BYTES * 16;
const SMALL_REQUEST: &[u8] =
    br#"{"jsonrpc":"2.0","id":1,"method":"budgetProbe","params":{"payload":"a\"b\\c\n"}}"#;

struct Observer;

thread_local! {
    // Constant initialization avoids allocation or destruction inside allocator callbacks.
    static MAX_REQUEST: Cell<Option<usize>> = const { Cell::new(None) };
}

// SAFETY: System retains allocation ownership; this observer only records requested sizes.
unsafe impl GlobalAlloc for Observer {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record(layout.size());
        // SAFETY: The caller's unchanged layout is forwarded to the backing allocator.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: The original pointer and layout go to the allocator that created them.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        record(size);
        // SAFETY: The pointer, layout and requested size are forwarded unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

#[global_allocator]
static ALLOCATOR: Observer = Observer;

fn record(size: usize) {
    // Thread-local teardown must never cause an allocator callback to unwind.
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
    let maximum = MAX_REQUEST.get().expect("active allocation window");
    drop(window);

    (result, maximum)
}

#[derive(Clone, Default)]
struct Recorder {
    calls: Rc<Cell<usize>>,
    request: Rc<RefCell<Option<Request>>>,
}

impl Transport for Recorder {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        self.calls.set(self.calls.get() + 1);
        self.request.replace(Some(request));

        ready(Ok(Reply {
            status: 200,
            body: br#"{"jsonrpc":"2.0","id":1,"result":7}"#.to_vec(),
        }))
    }
}

#[derive(Serialize)]
struct Params<'a> {
    payload: &'a str,
}

impl Method for Params<'_> {
    type Response = u64;
    const NAME: &'static str = "budgetProbe";

    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        serde_json::from_str(result).map_err(Error::Schema)
    }
}

#[derive(Serialize)]
struct ScalarParams(bool);

impl Method for ScalarParams {
    type Response = u64;
    const NAME: &'static str = "budgetProbe";

    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        serde_json::from_str(result).map_err(Error::Schema)
    }
}

fn poll_once<T>(future: impl Future<Output = T>) -> T {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());

    match future.as_mut().poll(&mut context) {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("immediate request preparation unexpectedly yielded"),
    }
}

fn limits() -> Limits {
    Limits {
        bytes: REQUEST_BYTES,
        ..Limits::default()
    }
}

#[test]
fn observer_counts_known_allocations_only_inside_its_window() {
    black_box(Vec::<u8>::with_capacity(INPUT_BYTES));
    assert_eq!(measure(|| ()).1, 0);
    let (_, maximum) = measure(|| black_box(Vec::<u8>::with_capacity(MAX_ALLOCATION)));
    assert!(maximum >= MAX_ALLOCATION, "known allocation was missed");
    black_box(Vec::<u8>::with_capacity(INPUT_BYTES));
    assert_eq!(measure(|| ()).1, 0);
}

#[test]
fn oversized_borrowed_params_fail_before_io_without_a_large_allocation() {
    let payloads = ["x".repeat(INPUT_BYTES), "\n\"\\".repeat(INPUT_BYTES / 3)];

    for payload in &payloads {
        let transport = Recorder::default();
        let client = Client::new(transport.clone(), limits());
        let params = Params { payload };
        let (result, maximum) = measure(|| poll_once(client.call(params)));

        assert!(matches!(result, Err(Error::BodyLimit)), "{result:?}");
        assert_eq!(transport.calls.get(), 0);
        assert!(transport.request.borrow().is_none());
        assert!(
            maximum <= MAX_ALLOCATION,
            "largest allocation: {maximum} bytes for a {REQUEST_BYTES}-byte request budget"
        );
    }
}

#[test]
fn a_small_request_preserves_escaped_wire_bytes_and_reaches_transport() {
    let transport = Recorder::default();
    let client = Client::new(transport.clone(), limits());
    let params = Params {
        payload: "a\"b\\c\n",
    };
    let (result, maximum) = measure(|| poll_once(client.call(params)));
    let request = transport.request.borrow();
    let request = request.as_ref().expect("one admitted request");

    assert_eq!(result.unwrap(), 7);
    assert_eq!(transport.calls.get(), 1);
    assert_eq!(request.body, SMALL_REQUEST);
    assert_eq!(request.max_reply_bytes, REQUEST_BYTES);
    assert!(maximum <= MAX_ALLOCATION, "largest allocation: {maximum}");
}

#[test]
fn scalar_params_remain_an_envelope_error_before_transport() {
    let transport = Recorder::default();
    let client = Client::new(transport.clone(), limits());
    let (result, maximum) = measure(|| poll_once(client.call(ScalarParams(true))));

    assert!(matches!(result, Err(Error::Envelope)), "{result:?}");
    assert_eq!(transport.calls.get(), 0);
    assert!(transport.request.borrow().is_none());
    assert!(maximum <= MAX_ALLOCATION, "largest allocation: {maximum}");
}
