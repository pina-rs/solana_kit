//! Observable request correlation ownership under concurrency and exhaustion.

use std::collections::BTreeSet;
use std::future::{Ready, ready};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::*;
use crate::methods::Balance;

struct Echo(Arc<AtomicUsize>);

impl Transport for Echo {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let request: serde_json::Value = serde_json::from_slice(&request.body).unwrap();
        let id = request["id"].as_u64().unwrap();
        let body = format!(
            r#"{{"jsonrpc":"2.0","id":{id},"result":{{"context":{{"slot":1}},"value":{id}}}}}"#
        );
        self.0.fetch_add(1, Ordering::Relaxed);
        ready(Ok(Reply {
            status: 200,
            body: body.into_bytes(),
        }))
    }
}

fn balance() -> Balance {
    Balance("11111111111111111111111111111111".parse().unwrap(), None)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn concurrent_calls_have_one_unique_id_owner() {
    const CALLS: usize = 32;
    let calls = Arc::new(AtomicUsize::new(0));
    let client = Arc::new(Client::new(Echo(Arc::clone(&calls)), Limits::default()));
    let mut tasks = tokio::task::JoinSet::new();

    for _ in 0..CALLS {
        let client = Arc::clone(&client);
        tasks.spawn(async move { client.call(balance()).await.unwrap().value });
    }

    let mut ids = BTreeSet::new();

    while let Some(id) = tasks.join_next().await {
        assert!(ids.insert(id.unwrap()));
    }

    assert_eq!(ids.len(), CALLS);
    assert_eq!(calls.load(Ordering::Relaxed), CALLS);
}

#[test]
fn exhausted_client_fails_without_exchange_or_wrapped_identity() {
    let calls = Arc::new(AtomicUsize::new(0));
    let client = Client {
        transport: Echo(Arc::clone(&calls)),
        limits: Limits::default(),
        next_id: AtomicU32::new(u32::MAX),
    };

    for _ in 0..2 {
        assert!(matches!(
            futures::executor::block_on(client.call(balance())),
            Err(Error::IdExhausted)
        ));
    }

    assert_eq!(calls.load(Ordering::Relaxed), 0);
}
