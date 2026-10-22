//! Same generic client algorithm executes on thread-safe and local transports.

use futures::channel::oneshot;
use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;
use solana_kit_async_boundaries::{LocalTransport, request};
use solana_kit_boundary_contract::{CallError, Client, Reply, Request, Transport};
use std::future::Future;
use std::rc::Rc;

const LOCAL_STATE: u64 = 7;

struct NativeTransport {
    reply: tokio::sync::Mutex<Option<oneshot::Receiver<Reply>>>,
}

impl Transport for NativeTransport {
    type Error = oneshot::Canceled;
    type Exchange<'a> = NativeExchange<'a>;

    fn exchange(&self, _request: Request) -> Self::Exchange<'_> {
        NativeExchange {
            receiver: &self.reply,
            future: None,
        }
    }
}

struct NativeExchange<'a> {
    receiver: &'a tokio::sync::Mutex<Option<oneshot::Receiver<Reply>>>,
    future: Option<oneshot::Receiver<Reply>>,
}

impl Future for NativeExchange<'_> {
    type Output = Result<Reply, oneshot::Canceled>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.future.is_none() {
            self.future = self.receiver.try_lock().expect("only one exchange").take();
        }

        std::pin::Pin::new(self.future.as_mut().expect("one exchange per fixture")).poll(cx)
    }
}

/// The explicit associated-future bound survives the client's await and decode.
async fn native_call<T>(transport: T) -> Result<u64, CallError<T::Error>>
where
    T: Transport + Send + Sync + 'static,
    T::Error: Send + 'static,
    for<'a> T::Exchange<'a>: Send,
{
    tokio::spawn(async move { Client::new(transport).call(request()).await })
        .await
        .expect("native task must complete")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn native_send_survives_generic_client_validation() {
    let (sender, receiver) = oneshot::channel();
    let transport = NativeTransport {
        reply: tokio::sync::Mutex::new(Some(receiver)),
    };
    let call = native_call(transport);
    let completion = async move {
        tokio::task::yield_now().await;
        sender
            .send(Reply {
                id: 41,
                body: 314_u64.to_le_bytes().to_vec(),
            })
            .expect("live exchange");
    };
    let (result, ()) = tokio::join!(call, completion);

    assert_eq!(result, Ok(314));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn generic_client_rejects_another_requests_reply() {
    let (sender, receiver) = oneshot::channel();
    let transport = NativeTransport {
        reply: tokio::sync::Mutex::new(Some(receiver)),
    };
    sender
        .send(Reply {
            id: 99,
            body: 314_u64.to_le_bytes().to_vec(),
        })
        .expect("live exchange");

    assert_eq!(native_call(transport).await, Err(CallError::Correlation));
}

#[test]
fn rc_is_retained_across_pending_on_local_pool() {
    let mut pool = LocalPool::new();
    let state = Rc::new(LOCAL_STATE);
    let observed = Rc::clone(&state);
    let (sender, receiver) = oneshot::channel();

    pool.spawner()
        .spawn_local(async move {
            let client = Client::new(LocalTransport::new(state));
            let reply = client.call(request()).await;
            sender.send(reply).expect("observer is alive");
        })
        .expect("local executor accepts Rc future");

    assert_eq!(Rc::strong_count(&observed), 2);
    assert_eq!(pool.run_until(receiver), Ok(Ok(314)));
    assert_eq!(Rc::strong_count(&observed), 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn generic_client_rejects_malformed_reply_body() {
    let (sender, receiver) = oneshot::channel();
    let transport = NativeTransport {
        reply: tokio::sync::Mutex::new(Some(receiver)),
    };
    sender
        .send(Reply {
            id: 41,
            body: vec![1],
        })
        .expect("live exchange");

    assert_eq!(native_call(transport).await, Err(CallError::MalformedReply));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn generic_client_preserves_transport_failure() {
    let (sender, receiver) = oneshot::channel();
    let transport = NativeTransport {
        reply: tokio::sync::Mutex::new(Some(receiver)),
    };
    drop(sender);

    assert!(matches!(
        native_call(transport).await,
        Err(CallError::Transport(_))
    ));
}
