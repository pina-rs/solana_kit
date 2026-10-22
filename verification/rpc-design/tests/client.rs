use std::future::{Future, poll_fn};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::Poll;

use solana_kit_rpc_design::alternatives::{ErasedClient, ErasedTransport};
use solana_kit_rpc_design::json::{Error, Limits};
use solana_kit_rpc_design::methods::{Balance, Method};
use solana_kit_rpc_design::{Client, Reply, Request, Transport, TransportError};

const BALANCE: &str =
    r#"{"jsonrpc":"2.0","id":7,"result":{"context":{"slot":3},"value":18446744073709551615}}"#;

struct Native(Arc<AtomicUsize>);
struct Local(Rc<AtomicUsize>);

type SendExchange<'a> = Pin<Box<dyn Future<Output = Result<Reply, TransportError>> + Send + 'a>>;
type LocalExchange<'a> = Pin<Box<dyn Future<Output = Result<Reply, TransportError>> + 'a>>;

async fn pending_once(counter: &AtomicUsize) {
    let mut pending = true;
    poll_fn(|cx| {
        if pending {
            pending = false;
            counter.fetch_add(1, Ordering::SeqCst);
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }

        Poll::Ready(())
    })
    .await;
}

fn reply(request: Request) -> Reply {
    let frame: serde_json::Value = serde_json::from_slice(&request.body).unwrap();
    assert_eq!(frame["method"], "getBalance");
    assert_eq!(frame["id"], 7);
    assert_eq!(frame["params"][0], "11111111111111111111111111111111");
    assert_eq!(request.max_reply_bytes, 8192);
    Reply {
        status: 200,
        body: BALANCE.as_bytes().to_vec(),
    }
}

impl Transport for Native {
    type Exchange<'a> = SendExchange<'a>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        Box::pin(async move {
            pending_once(&self.0).await;
            Ok(reply(request))
        })
    }
}

impl Transport for Local {
    type Exchange<'a> = LocalExchange<'a>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        Box::pin(async move {
            let held = Rc::clone(&self.0);
            pending_once(&held).await;
            assert_eq!(Rc::strong_count(&held), 3);
            Ok(reply(request))
        })
    }
}

impl ErasedTransport for Local {
    fn exchange(&self, request: Request) -> LocalExchange<'_> {
        Transport::exchange(self, request)
    }
}

/// These bounds must propagate through the actual generic client call.
fn spawn_call<T, R>(
    client: Client<T>,
    method: R,
) -> tokio::task::JoinHandle<Result<R::Response, Error>>
where
    T: Transport + Send + Sync + 'static,
    for<'a> T::Exchange<'a>: Send,
    R: Method + Send + 'static,
    R::Response: Send + 'static,
{
    tokio::spawn(async move { client.call(7, method).await })
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn generic_client_retains_native_send_and_lossless_u64() {
    let counter = Arc::new(AtomicUsize::new(0));
    let client = Client::new(Native(Arc::clone(&counter)), Limits::default());
    let value = spawn_call(client, balance()).await.unwrap().unwrap();
    assert_eq!(value.value, u64::MAX);
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[test]
fn same_generic_client_holds_rc_across_observed_pending() {
    let counter = Rc::new(AtomicUsize::new(0));
    let client = Client::new(Local(Rc::clone(&counter)), Limits::default());
    let value = futures::executor::block_on(client.call(7, balance())).unwrap();
    assert_eq!(value.value, u64::MAX);
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[test]
fn erased_candidate_runs_locally() {
    let counter = Rc::new(AtomicUsize::new(0));
    let client = ErasedClient(Local(Rc::clone(&counter)));
    let value = futures::executor::block_on(client.call(7, balance())).unwrap();
    assert_eq!(value.value, u64::MAX);
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

fn balance() -> Balance {
    Balance("11111111111111111111111111111111".parse().unwrap(), None)
}
