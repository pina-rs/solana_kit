#![cfg(not(target_family = "wasm"))]
//! Preparation must respect wire budgets without duplicating large caller data.
//!
//! This native observer counts the largest requested allocation on the calling
//! thread, not aggregate allocation volume, live memory or process RSS.
use solana_kit_client::{Client, Error, Limits, Reply, Request, Transport, TransportError};
use solana_kit_core::Address;
use solana_kit_surfpool::{
    Surfnet,
    builders::SetAccount,
    cheatcodes::WriteProgram,
    deploy::{DeployError, DeployOptions},
    hex::HexBytes,
};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::{Cell, RefCell},
    future::{Future, Ready, ready},
    hint::black_box,
    io::{self, Write},
    num::NonZeroUsize,
    pin::pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

const CHUNK_BYTES: usize = 1024 * 1024;
const INPUT_BYTES: usize = 2 * CHUNK_BYTES;
const REQUEST_BYTES: usize = 1024;
// Permit small serializer growth and framing bookkeeping, not an input-sized copy.
const MAX_ALLOCATION: usize = REQUEST_BYTES * 16;
const SMALL_WRITE: &[u8] = br#"{"jsonrpc":"2.0","id":1,"method":"surfnet_writeProgram","params":["11111111111111111111111111111111","00abff",0]}"#;

struct Observer;

thread_local! {
    // Constant initialization avoids allocation and destruction in allocator callbacks.
    static MAX_REQUEST: Cell<Option<usize>> = const { Cell::new(None) };
}

// SAFETY: System retains ownership; the observer only records requested sizes.
unsafe impl GlobalAlloc for Observer {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record(layout.size());
        // SAFETY: The caller's unchanged layout goes to the backing allocator.
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
    // Thread-local teardown must not make an allocator callback unwind.
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
    requests: Rc<RefCell<Vec<Request>>>,
}

impl Transport for Recorder {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let mut requests = self.requests.borrow_mut();
        requests.push(request);
        let id = requests.len();

        ready(Ok(Reply {
            status: 200,
            body: format!(r#"{{"jsonrpc":"2.0","id":{id},"result":null}}"#).into_bytes(),
        }))
    }
}

/// Validates every encoded byte without allocating an output-sized buffer.
struct HexSink {
    expected_bytes: usize,
    written: usize,
    valid: bool,
}

impl HexSink {
    fn expected(&self, position: usize) -> u8 {
        if position == 0 || position == self.expected_bytes - 1 {
            return b'"';
        }

        b"ba"[position % 2]
    }
}

impl Write for HexSink {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        for (index, byte) in bytes.iter().enumerate() {
            let position = self.written + index;
            let expected = self.expected(position);
            self.valid &= position < self.expected_bytes && *byte == expected;
        }

        self.written += bytes.len();
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn poll_once<T>(future: impl Future<Output = T>) -> T {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());

    match future.as_mut().poll(&mut context) {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("immediate preparation unexpectedly yielded"),
    }
}

fn surfnet(transport: &Recorder, bytes: usize) -> Surfnet<Recorder> {
    Surfnet::new(Client::new(
        transport.clone(),
        Limits {
            bytes,
            ..Limits::default()
        },
    ))
}

fn program() -> Address {
    Address::from_bytes([0; 32])
}

#[test]
fn default_deployment_fits_the_default_wire_budget_and_preserves_all_bytes() {
    let bytes = vec![0xab; 7 * 1024];
    let transport = Recorder::default();
    let limits = Limits::default();
    let surfnet = surfnet(&transport, limits.bytes);
    let result = poll_once(surfnet.deploy(DeployOptions::new(program(), &bytes)));
    assert_eq!(result.unwrap(), program());

    let requests = transport.requests.borrow();
    assert!(requests.len() > 1);
    let mut written = String::new();
    for request in requests.iter() {
        assert!(request.body.len() <= limits.bytes);
        let frame: serde_json::Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(frame["method"], "surfnet_writeProgram");
        assert_eq!(frame["params"][0], program().to_string());
        assert_eq!(
            frame["params"][2].as_u64(),
            Some((written.len() / 2) as u64)
        );
        written.push_str(frame["params"][1].as_str().unwrap());
    }
    assert_eq!(written, "ab".repeat(bytes.len()));
}

fn write_program(data: Vec<u8>) -> WriteProgram {
    WriteProgram {
        program_id: program(),
        data: HexBytes(data),
        offset: 0,
        authority: None,
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
fn hex_serialization_does_not_allocate_a_complete_encoded_copy() {
    let data = HexBytes(vec![0xab; INPUT_BYTES]);
    let mut sink = HexSink {
        expected_bytes: 2 * INPUT_BYTES + 2,
        written: 0,
        valid: true,
    };
    let (result, maximum) = measure(|| serde_json::to_writer(&mut sink, &data));

    assert!(result.is_ok(), "{result:?}");
    assert!(sink.valid, "hex output changed");
    assert_eq!(sink.written, sink.expected_bytes);
    assert!(
        maximum <= MAX_ALLOCATION,
        "largest allocation: {maximum} bytes while streaming {INPUT_BYTES} raw bytes"
    );
}

#[test]
fn oversized_account_data_is_rejected_before_io_without_a_large_allocation() {
    let request = SetAccount::new(program()).with_data(vec![0xab; INPUT_BYTES]);
    let transport = Recorder::default();
    let surfnet = surfnet(&transport, REQUEST_BYTES);
    let (result, maximum) = measure(|| poll_once(surfnet.execute(request)));

    assert!(
        matches!(&result, Err(error) if matches!(&error.cause, Error::BodyLimit)),
        "{result:?}"
    );
    assert!(transport.requests.borrow().is_empty());
    assert!(
        maximum <= MAX_ALLOCATION,
        "largest allocation: {maximum} bytes for a {REQUEST_BYTES}-byte wire budget"
    );
}

#[test]
fn rejected_deployment_does_not_copy_the_configured_chunk() {
    let bytes = vec![0xab; 2 * CHUNK_BYTES + 1];
    let transport = Recorder::default();
    let surfnet = surfnet(&transport, REQUEST_BYTES);
    let mut options = DeployOptions::new(program(), &bytes);
    options.chunk_bytes = NonZeroUsize::new(CHUNK_BYTES).expect("nonzero test chunk");
    let (result, maximum) = measure(|| poll_once(surfnet.deploy(options)));

    assert!(
        matches!(&result, Err(DeployError::Rpc(error)) if matches!(&error.cause, Error::BodyLimit)),
        "{result:?}"
    );
    assert!(transport.requests.borrow().is_empty());
    assert!(
        maximum <= MAX_ALLOCATION,
        "largest allocation: {maximum} bytes with a {CHUNK_BYTES}-byte configured chunk"
    );
}

#[test]
fn write_program_budget_counts_hex_and_complete_json_framing() {
    let request = write_program(vec![0, 0xab, 0xff]);
    let transport = Recorder::default();
    let accepted = surfnet(&transport, SMALL_WRITE.len());
    let (result, maximum) = measure(|| poll_once(accepted.execute(request)));

    assert!(result.unwrap().value.is_null());
    assert_eq!(transport.requests.borrow().len(), 1);
    let requests = transport.requests.borrow();
    assert_eq!(requests[0].body, SMALL_WRITE);
    assert_eq!(requests[0].max_reply_bytes, SMALL_WRITE.len());
    assert!(maximum <= MAX_ALLOCATION, "largest allocation: {maximum}");

    let request = write_program(vec![0, 0xab, 0xff]);
    let transport = Recorder::default();
    let denied = surfnet(&transport, SMALL_WRITE.len() - 1);
    let (result, maximum) = measure(|| poll_once(denied.execute(request)));

    assert!(
        matches!(&result, Err(error) if matches!(&error.cause, Error::BodyLimit)),
        "{result:?}"
    );
    assert!(transport.requests.borrow().is_empty());
    assert!(maximum <= MAX_ALLOCATION, "largest allocation: {maximum}");
}

#[test]
fn deployment_preserves_configured_chunk_bytes_and_offsets() {
    let bytes = [0, 0xab, 0xff, 0x10, 0x80];
    let transport = Recorder::default();
    let surfnet = surfnet(&transport, REQUEST_BYTES);
    let mut options = DeployOptions::new(program(), &bytes);
    options.chunk_bytes = NonZeroUsize::new(2).expect("nonzero test chunk");
    let (result, maximum) = measure(|| poll_once(surfnet.deploy(options)));

    assert_eq!(result.unwrap(), program());
    assert!(maximum <= MAX_ALLOCATION, "largest allocation: {maximum}");
    let requests = transport.requests.borrow();
    assert_eq!(requests.len(), 3);

    for (index, (hex, offset)) in [("00ab", 0), ("ff10", 2), ("80", 4)]
        .into_iter()
        .enumerate()
    {
        let frame: serde_json::Value = serde_json::from_slice(&requests[index].body).unwrap();
        assert_eq!(frame["method"], "surfnet_writeProgram");
        assert_eq!(frame["params"][1], hex);
        assert_eq!(frame["params"][2], offset);
        assert_eq!(requests[index].max_reply_bytes, REQUEST_BYTES);
    }
}
