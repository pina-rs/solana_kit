//! Send mock exchanges make sequencing and active-exchange cancellation observable.

use serde_json::Value;
use solana_kit_client::{Client, Limits, Reply, Request, Transport, TransportError};
use solana_kit_surfpool::Surfnet;
use std::{
    future::{Future, pending},
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};
use tokio::sync::Notify;

const RPC_ERROR_CODE: i64 = -32000;

#[derive(Clone, Default)]
pub struct Observed {
    pub frames: Arc<Mutex<Vec<Value>>>,
    pub pending: Arc<Notify>,
    pub drops: Arc<AtomicUsize>,
}

#[derive(Default)]
pub struct Wire {
    pub observed: Observed,
    pub pending_at: Option<usize>,
    pub fail_at: Option<usize>,
}

impl Wire {
    pub fn client(self) -> (Surfnet<Self>, Observed) {
        let observed = self.observed.clone();

        (Surfnet::new(Client::new(self, Limits::default())), observed)
    }
}

struct PendingDrop(Arc<AtomicUsize>);
impl Drop for PendingDrop {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

impl Transport for Wire {
    type Exchange<'a> = Pin<Box<dyn Future<Output = Result<Reply, TransportError>> + Send + 'a>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        Box::pin(async move {
            let frame: Value = serde_json::from_slice(&request.body).unwrap();
            let index = {
                let mut frames = self.observed.frames.lock().unwrap();
                frames.push(frame.clone());
                frames.len()
            };

            if self.pending_at == Some(index) {
                let _drop = PendingDrop(Arc::clone(&self.observed.drops));
                self.observed.pending.notify_one();
                pending::<()>().await;
            }

            let mut reply = serde_json::json!({"jsonrpc":"2.0", "id":frame["id"], "result":null});

            if self.fail_at == Some(index) {
                reply.as_object_mut().unwrap().remove("result");
                reply["error"] = serde_json::json!({
                    "code": RPC_ERROR_CODE,
                    "message": "private-provider-sentinel"
                });
            }

            Ok(Reply {
                status: 200,
                body: serde_json::to_vec(&reply).unwrap(),
            })
        })
    }
}
