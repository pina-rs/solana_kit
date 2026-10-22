//! Public client journeys for an embedded host with independently owned policy.

use solana_kit_client::methods::Balance;
use solana_kit_client::{Client, Error, Limits, TransportFailureKind as Kind};
use solana_kit_core::Address;
use solana_kit_wasi::{Reactor, WasiHttp};
use std::future::Future;
use std::rc::Rc;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::task::{Context, Poll, Wake, Waker};

const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(150);
const ROUNDS: usize = 12;
const LOCAL_VALUE: u8 = 41;
const HOLD_NANOS: u64 = 500_000_000;
const CANCEL_NANOS: u64 = 40_000_000;
const OBSERVE_NANOS: u64 = 300_000_000;
type BalanceResult = Result<solana_kit_client::methods::Contextual<u64>, Error>;

#[derive(Default)]
struct Counter(AtomicUsize);

impl Wake for Counter {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

/// Upload and CLI baselines exercise host stages alongside the public Client calls.
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let endpoint = &args[1];
    let mode = &args[2];
    println!("START {mode}");
    eprintln!("START {mode}");

    if mode == "baseline" {
        println!("PASS no-network CLI baseline");
        eprintln!("PASS no-network stderr baseline");

        return;
    }

    let reactor = Reactor::default();
    let transport = WasiHttp::new(
        reactor.clone(),
        endpoint,
        TIMEOUT,
        vec![("authorization".into(), b"Bearer fixture-only".to_vec())],
    )
    .expect("fixture endpoint");

    if matches!(mode.as_str(), "early-status" | "upload-deadline") {
        upload(&reactor, &transport, mode);

        return;
    }

    let client = Client::new(transport, Limits::default());
    let rounds = if mode.starts_with("repeated") {
        ROUNDS
    } else {
        1
    };

    for _ in 0..rounds {
        journey(&reactor, &client, mode);
        assert_eq!(reactor.registrations(), 0);
    }

    println!("PASS embedded {mode} Pending/readiness/Rc/registry");
}

/// Cancellation keeps the guest alive; completion requires actual readiness and an Rc owner.
fn journey(reactor: &Reactor, client: &Client<WasiHttp>, mode: &str) {
    let local = Rc::new(LOCAL_VALUE);
    let held = local.clone();
    let mut request = Box::pin(async {
        let reply = client
            .call(Balance(Address::from_bytes([0; 32]), None))
            .await;
        assert_eq!(*held, LOCAL_VALUE);
        reply
    });
    let counter = Arc::new(Counter::default());
    let waker = Waker::from(counter.clone());
    let mut context = Context::from_waker(&waker);
    let start = wasip2::clocks::monotonic_clock::now();
    let first = request.as_mut().poll(&mut context);

    assert!(wasip2::clocks::monotonic_clock::now() - start < 50_000_000);

    if !matches!(mode, "denied" | "repeated-denied") {
        assert!(first.is_pending());
    }

    if mode == "hold" {
        wasip2::clocks::monotonic_clock::subscribe_duration(HOLD_NANOS).block();
        drop(request);

        return;
    }

    if matches!(mode, "cancel" | "repeated-cancel") {
        wasip2::clocks::monotonic_clock::subscribe_duration(CANCEL_NANOS).block();
        drop(request);
        assert_eq!(reactor.registrations(), 0);
        let observation = if mode == "repeated-cancel" {
            CANCEL_NANOS
        } else {
            OBSERVE_NANOS
        };

        wasip2::clocks::monotonic_clock::subscribe_duration(observation).block();

        return;
    }

    let result = match first {
        Poll::Ready(result) => result,
        Poll::Pending => complete(reactor, request.as_mut(), &mut context, &counter),
    };

    drop(request);
    check(mode, result, start);
}

fn complete<F: Future<Output = BalanceResult>>(
    reactor: &Reactor,
    mut request: std::pin::Pin<&mut F>,
    context: &mut Context<'_>,
    counter: &Counter,
) -> BalanceResult {
    loop {
        reactor.drive().expect("registered readiness");

        if let Poll::Ready(result) = request.as_mut().poll(context) {
            assert!(counter.0.load(Ordering::Relaxed) > 0);

            return result;
        }
    }
}

/// Each mode asserts its distinct typed result, while deadline modes also bound elapsed time.
fn check(mode: &str, result: BalanceResult, start: u64) {
    if mode.ends_with("-deadline") {
        assert!(wasip2::clocks::monotonic_clock::now() - start < 250_000_000);
    }

    let expected = match mode {
        "denied" | "denied-upload" | "denied-response" | "repeated-denied" => Some(Kind::Denied),
        // Wasmtime50 retains body failures as Error::Hyper and exposes no
        // structured http-error-code for them. The observable fallback is Io.
        "tls-untrusted" | "tls-name" | "malformed-chunk" | "truncated" => Some(Kind::Io),
        "tls-deadline" | "dns-deadline" => Some(Kind::Deadline),
        _ => None,
    };

    if let Some(expected) = expected {
        println!("OUTCOME {mode} {result:?}");
        assert!(matches!(result, Err(Error::Transport(error)) if error.kind == expected));

        return;
    }

    match mode {
        "success" | "repeated" | "replacement" => {
            assert_eq!(result.expect("balance").value, u64::MAX)
        }
        "redirect" => assert!(matches!(result, Err(Error::Status(302)))),
        _ => panic!("unknown embedded mode"),
    }
}

fn upload(reactor: &Reactor, transport: &WasiHttp, mode: &str) {
    use solana_kit_client::{Request, Transport};
    const UPLOAD_BYTES: usize = 16 * 1024 * 1024;
    const EARLY_STATUS: u16 = 413;
    let result = reactor
        .block_on(transport.exchange(Request {
            body: vec![b'x'; UPLOAD_BYTES],
            max_reply_bytes: 256,
        }))
        .expect("driver");

    if mode == "early-status" {
        assert_eq!(
            result.expect("authoritative early response").status,
            EARLY_STATUS
        );
    } else {
        assert!(matches!(result, Err(error) if error.kind == Kind::Deadline));
    }

    assert_eq!(reactor.registrations(), 0);
    println!("PASS {mode} actual bounded upload");
}
