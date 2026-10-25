//! Shared public-client protocol fixtures; memory transports perform no network I/O.
extern crate std;

use solana_kit_client::{
    Client, Error, Limits, Presence, Reply, Request, Transport, TransportError,
    TransportFailureKind, methods::Balance,
};
use solana_kit_core::Address;
use std::{
    cell::RefCell,
    future::{Future, Ready, ready},
    pin::{Pin, pin},
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
    vec::Vec,
};

const BODY: &str =
    r#"{"jsonrpc":"2.0","id":1,"result":{"context":{"slot":9},"value":18446744073709551615}}"#;
const REQUEST: &[u8] = br#"{"jsonrpc":"2.0","id":1,"method":"getBalance","params":["11111111111111111111111111111111"]}"#;
const OPAQUE: &str = r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32000,"message":"exact","data": { "number":1e400,"padding":"keep this bounded payload larger than the exact request" }}}"#;

struct Fixture(&'static str);

impl Transport for Fixture {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        if request.body != REQUEST {
            return ready(Err(TransportError::new(
                TransportFailureKind::InvalidResponse,
            )));
        }

        ready(Ok(Reply {
            status: 200,
            body: self.0.as_bytes().to_vec(),
        }))
    }
}

fn method() -> Balance {
    Balance(Address::from_bytes([0; 32]), None)
}

fn execute(
    body: &'static str,
    limits: Limits,
) -> Result<solana_kit_client::methods::Contextual<u64>, Error> {
    let client = Client::new(Fixture(body), limits);
    let mut call = pin!(client.call(method()));

    match call.as_mut().poll(&mut Context::from_waker(Waker::noop())) {
        Poll::Ready(value) => value,
        Poll::Pending => Err(Error::Transport(TransportError::new(
            TransportFailureKind::Unavailable,
        ))),
    }
}

/// Checks exact request bytes and lossless required context/value fields.
pub fn balance() -> Result<(), &'static str> {
    let value = execute(BODY, Limits::default()).map_err(|_| "balance request failed")?;

    if value.value != u64::MAX
        || value.context.slot != 9
        || value.context.api_version != Presence::Missing
    {
        return Err("portable RPC request/context/u64 mismatch");
    }

    Ok(())
}

/// Preserves bounded opaque numbers while rejecting duplicate and invalid typed fields.
pub fn boundaries() -> Result<(), &'static str> {
    let limits = Limits {
        bytes: OPAQUE.len(),
        depth: 3,
    };
    let Err(Error::Remote(error)) = execute(OPAQUE, limits) else {
        return Err("opaque remote error was not retained");
    };
    let Presence::Present(data) = error.data else {
        return Err("opaque error data missing");
    };

    if error.code != -32000
        || error.message != "exact"
        || data.get()
            != r#"{ "number":1e400,"padding":"keep this bounded payload larger than the exact request" }"#
    {
        return Err("opaque error data changed");
    }

    opaque_limits(limits)?;
    typed_failures()
}

/// Opaque values still consume the same bounded body and nesting budgets.
fn opaque_limits(limits: Limits) -> Result<(), &'static str> {
    if !matches!(
        execute(
            OPAQUE,
            Limits {
                bytes: OPAQUE.len() - 1,
                ..limits
            }
        ),
        Err(Error::BodyLimit)
    ) || !matches!(
        execute(OPAQUE, Limits { depth: 2, ..limits }),
        Err(Error::DepthLimit)
    ) {
        return Err("opaque body/depth budget ignored");
    }

    Ok(())
}

/// Exact JSON preservation never bypasses duplicate-key or typed schema checks.
fn typed_failures() -> Result<(), &'static str> {
    let duplicate =
        r#"{"jsonrpc":"2.0","id":1,"error":{"code":-1,"message":"bad","data":{"a":1,"\u0061":2}}}"#;
    let schema = r#"{"jsonrpc":"2.0","id":1,"result":{"context":{"slot":9},"value":1e400}}"#;
    let missing = r#"{"jsonrpc":"2.0","id":1,"result":{"context":{"slot":9}}}"#;

    if !matches!(execute(duplicate, Limits::default()), Err(Error::Json(_)))
        || !matches!(execute(schema, Limits::default()), Err(Error::Schema(_)))
        || !matches!(execute(missing, Limits::default()), Err(Error::Schema(_)))
    {
        return Err("duplicate or typed schema was accepted");
    }

    Ok(())
}

struct WakeCount(AtomicUsize);

impl Wake for WakeCount {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

struct Local(Rc<RefCell<Vec<Vec<u8>>>>);

struct LocalExchange {
    held: Rc<RefCell<Vec<Vec<u8>>>>,
    request: Option<Request>,
}

impl Future for LocalExchange {
    type Output = Result<Reply, TransportError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if let Some(request) = this.request.take() {
            this.held.borrow_mut().push(request.body);
            cx.waker().wake_by_ref();

            return Poll::Pending;
        }

        // The Rc is read after suspension, so this concrete exchange stays local.
        let id = this.held.borrow().len();
        let body = if id == 1 {
            BODY.into()
        } else {
            BODY.replace("\"id\":1", "\"id\":2")
        };

        Poll::Ready(Ok(Reply {
            status: 200,
            body: body.into_bytes(),
        }))
    }
}

impl Transport for Local {
    type Exchange<'a> = LocalExchange;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        LocalExchange {
            held: Rc::clone(&self.0),
            request: Some(request),
        }
    }
}

/// Observes wake/Pending, local Rc use, abandonment and the next correlation ID.
pub fn local_pending() -> Result<(), &'static str> {
    let held = Rc::new(RefCell::new(Vec::new()));
    let client = Client::new(Local(Rc::clone(&held)), Limits::default());
    let wake = Arc::new(WakeCount(AtomicUsize::new(0)));
    let waker = Waker::from(Arc::clone(&wake));
    let mut cx = Context::from_waker(&waker);
    drop(client.call(method()));

    {
        let mut abandoned = pin!(client.call(method()));

        if !abandoned.as_mut().poll(&mut cx).is_pending() || wake.0.load(Ordering::SeqCst) != 1 {
            return Err("abandoned call did not observe Pending and wake");
        }
    }

    let mut call = pin!(client.call(method()));

    if !call.as_mut().poll(&mut cx).is_pending() || wake.0.load(Ordering::SeqCst) != 2 {
        return Err("local call did not observe Pending and wake");
    }

    let Poll::Ready(Ok(value)) = call.as_mut().poll(&mut cx) else {
        return Err("local call failed after resumption");
    };
    let requests = held.borrow();
    let second = std::str::from_utf8(REQUEST)
        .map_err(|_| "request fixture UTF8")?
        .replace("\"id\":1", "\"id\":2");

    if value.value != u64::MAX || requests.as_slice() != [REQUEST, second.as_bytes()] {
        return Err("abandonment/correlation or local value mismatch");
    }

    if !matches!(
        execute(
            r#"{"jsonrpc":"2.0","id":2,"result":{"context":{"slot":9},"value":1}}"#,
            Limits::default()
        ),
        Err(Error::Correlation)
    ) {
        return Err("mismatched correlation accepted");
    }

    Ok(())
}

#[test]
fn portable_protocol_cases_execute() {
    balance().unwrap();
    boundaries().unwrap();
    local_pending().unwrap();
}
