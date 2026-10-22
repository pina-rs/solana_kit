//! Real public Client calls through a local Preview2 transport prototype.

use solana_kit_client::methods::Balance;
use solana_kit_client::{Client, Error, Limits, TransportFailureKind as Kind};
use solana_kit_core::Address;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::task::{Context, Wake, Waker};
use wasip2::clocks::monotonic_clock;

const LOCAL_VALUE: u8 = 41;
const CANCEL_NANOS: u64 = 40_000_000;
const OBSERVE_NANOS: u64 = 300_000_000;

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

/// Uses one caller polling path for success, deliberate corruption and cancellation.
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let endpoint = &args[1];
    let mode = &args[2];
    let reactor = solana_kit_wasi::Reactor::default();
    let transport = transport(&reactor, endpoint, mode);
    let limits = Limits {
        bytes: 256,
        ..Limits::default()
    };

    let client = Client::new(transport, limits);
    let local = Rc::new(LOCAL_VALUE);

    let held = local.clone();
    let mut exchange = Box::pin(async {
        let result = client
            .call(Balance(Address::from_bytes([0; 32]), None))
            .await;
        assert_eq!(*held, LOCAL_VALUE);
        result
    });
    let count = Arc::new(Counter::default());
    let waker = Waker::from(count.clone());
    let mut context = Context::from_waker(&waker);
    first_poll(exchange.as_mut(), &mut context);
    assert!(reactor.registrations() > 0);
    println!("PENDING {mode}");

    std::io::Write::flush(&mut std::io::stdout()).expect("flush marker");

    if matches!(mode.as_str(), "cancel" | "retain" | "body-cancel") {
        cancel(mode, &reactor, exchange, &mut context);

        return;
    }

    let result = complete(&reactor, exchange.as_mut(), &mut context, &count);

    drop(exchange);
    assert_eq!(reactor.registrations(), 0);

    check_result(mode, result);
    println!("PASS {mode} real Pending+wake, Rc, cleanup");
    assert_eq!(reactor.block_on(async { LOCAL_VALUE }), Ok(LOCAL_VALUE));
}

fn first_poll<F: Future>(exchange: Pin<&mut F>, context: &mut Context<'_>) {
    let before = monotonic_clock::now();
    let first = exchange.poll(context);
    assert!(monotonic_clock::now() - before < 50_000_000, "poll blocked");
    assert!(
        first.is_pending(),
        "local fixture should initially be pending"
    );
}

fn complete<F: Future>(
    reactor: &solana_kit_wasi::Reactor,
    mut exchange: Pin<&mut F>,
    context: &mut Context<'_>,
    count: &Counter,
) -> F::Output {
    loop {
        reactor.drive().expect("registered driver");
        assert!(
            count.0.load(Ordering::Relaxed) > 0,
            "host readiness did not wake caller"
        );

        if let std::task::Poll::Ready(result) = exchange.as_mut().poll(context) {
            return result;
        }
    }
}

fn transport(
    reactor: &solana_kit_wasi::Reactor,
    endpoint: &str,
    mode: &str,
) -> solana_kit_wasi::WasiHttp {
    let timeout = if matches!(mode, "cancel" | "retain" | "body-cancel") {
        2_000_000_000
    } else {
        150_000_000
    };

    solana_kit_wasi::WasiHttp::new(
        reactor.clone(),
        endpoint,
        std::time::Duration::from_nanos(timeout),
        vec![("authorization".into(), b"Bearer fixture-only".to_vec())],
    )
    .expect("valid configuration")
}

fn cancel<F: Future>(
    mode: &str,
    reactor: &solana_kit_wasi::Reactor,
    mut exchange: Pin<Box<F>>,
    context: &mut Context<'_>,
) {
    if mode == "body-cancel" {
        reactor.drive().expect("registered driver");
        assert!(exchange.as_mut().poll(context).is_pending());
    }

    monotonic_clock::subscribe_duration(CANCEL_NANOS).block();

    if mode != "retain" {
        drop(exchange);
        assert_eq!(reactor.registrations(), 0);
    } else {
        assert!(reactor.registrations() > 0);
    }

    monotonic_clock::subscribe_duration(OBSERVE_NANOS).block();
    println!("PASS {mode} guest alive after observation window");
}

fn check_result(mode: &str, result: Result<solana_kit_client::methods::Contextual<u64>, Error>) {
    match mode {
        "success" | "chunked" => assert_eq!(result.expect("valid reply").value, u64::MAX),
        "wrong-body" => assert_eq!(result.expect("valid reply").value, u64::MAX),
        "status" | "redirect" => assert!(matches!(result, Err(Error::Status(503 | 302)))),
        "oversize" => assert!(
            matches!(result, Err(Error::Transport(ref error)) if error.kind == Kind::BodyLimit)
        ),
        "headers-deadline" | "body-deadline" => assert!(
            matches!(result, Err(Error::Transport(ref error)) if error.kind == Kind::Deadline)
        ),
        "malformed" | "https-untrusted" => {
            assert!(matches!(result, Err(Error::Transport(_))))
        }
        _ => panic!("unknown mode"),
    }
}
