//! Explicit file fixtures and a local transport exercise the public native seam.

use serde_json::Value;
use solana_kit_client::{Client, Limits, Reply, Request, Transport, TransportError};
use solana_kit_core::Address;
use solana_kit_native::surfpool::{DeployOptions, ProgramSource};
use solana_kit_surfpool::Surfnet;
use std::{
    cell::{Cell, RefCell},
    future::{Future, pending},
    num::NonZeroUsize,
    path::PathBuf,
    pin::Pin,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};
use tokio::fs;

pub const PROGRAM: [u8; 5] = [1, 2, 3, 4, 255];
pub const ADDRESS: Address = Address::from_bytes([7; 32]);
pub const PRIVATE: &str = "private-input-sentinel";
static NEXT_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

pub struct Directory(pub PathBuf);

impl Directory {
    pub async fn new() -> Self {
        let sequence = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "solana-kit-native-surfpool-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir(&path)
            .await
            .expect("fresh explicit test directory");

        Self(path)
    }

    pub async fn write(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.0.join(name);
        fs::write(&path, bytes).await.expect("write owned fixture");

        path
    }

    pub async fn finish(self) {
        fs::remove_dir_all(self.0)
            .await
            .expect("remove owned test fixtures");
    }
}

#[derive(Clone, Default)]
pub struct Observed {
    pub requests: Rc<RefCell<Vec<Value>>>,
    pub accepted_writes: Rc<Cell<usize>>,
    pub pending_drops: Rc<Cell<usize>>,
}

#[derive(Default)]
pub struct Mock {
    pub observed: Observed,
    pub fail_at: Option<usize>,
    pub pending_at: Option<usize>,
    pub create_idl_after: Option<(usize, PathBuf, Vec<u8>)>,
}

impl Mock {
    pub fn client(self) -> (Surfnet<Self>, Observed) {
        let observed = self.observed.clone();

        (Surfnet::new(Client::new(self, Limits::default())), observed)
    }
}

struct PendingDrop(Rc<Cell<usize>>);

impl Drop for PendingDrop {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

impl Transport for Mock {
    type Exchange<'a> = Pin<Box<dyn Future<Output = Result<Reply, TransportError>> + 'a>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        Box::pin(async move {
            let frame: Value = serde_json::from_slice(&request.body).expect("valid client request");
            let index = self.observed.requests.borrow().len() + 1;
            self.observed.requests.borrow_mut().push(frame.clone());

            if self.pending_at == Some(index) {
                let _drop = PendingDrop(Rc::clone(&self.observed.pending_drops));
                pending::<()>().await;
            }

            let payload = self.payload(index, &frame).await;

            Ok(Reply {
                status: 200,
                body: format!(r#"{{"jsonrpc":"2.0","id":{},{} }}"#, frame["id"], payload)
                    .into_bytes(),
            })
        })
    }
}

impl Mock {
    async fn payload(&self, index: usize, frame: &Value) -> String {
        if self.fail_at == Some(index) {
            return format!(r#""error":{{"code":-32000,"message":"{PRIVATE}"}}"#);
        }

        self.accept(index, frame).await;

        r#""result":null"#.to_owned()
    }

    async fn accept(&self, index: usize, frame: &Value) {
        if frame["method"] == "surfnet_writeProgram" {
            self.observed
                .accepted_writes
                .set(self.observed.accepted_writes.get() + 1);
        }

        if let Some((after, path, bytes)) = &self.create_idl_after
            && index == *after
        {
            fs::write(path, bytes)
                .await
                .expect("IDL appears after final accepted program write");
        }
    }
}

pub fn options(program: ProgramSource<'_>) -> DeployOptions<'_> {
    let mut options = DeployOptions::new(ADDRESS, program);
    options.chunk_bytes = NonZeroUsize::new(2).expect("nonzero fixture chunk");

    options
}

pub fn assert_program_writes(observed: &Observed) {
    let requests = observed.requests.borrow();
    let writes: Vec<_> = requests
        .iter()
        .filter(|frame| frame["method"] == "surfnet_writeProgram")
        .collect();

    assert_eq!(writes.len(), 3);

    for (frame, (data, offset)) in writes.iter().zip([("0102", 0), ("0304", 2), ("ff", 4)]) {
        assert_eq!(
            frame["params"],
            serde_json::json!([ADDRESS.to_string(), data, offset])
        );
    }

    assert_eq!(observed.accepted_writes.get(), 3);
}
