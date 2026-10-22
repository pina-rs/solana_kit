use solana_kit_wallet::*;
use std::{
    future::{Future, Ready, ready},
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll},
};

struct Native;
impl WalletFeature for Native {
    fn version(&self) -> &str {
        "1.0.0"
    }
}
impl Connect for Native {
    type Error = WalletError;
    type Future<'a> = Ready<Result<ConnectOutput, WalletError>>;
    fn connect(&self, _: ConnectInput) -> Self::Future<'_> {
        ready(Ok(ConnectOutput::default()))
    }
}

impl SignMessage for Native {
    type Error = WalletError;
    type Future<'a> = Ready<Result<Vec<SignMessageOutput>, WalletError>>;
    fn sign_message(&self, inputs: Vec<SignMessageInput>) -> Self::Future<'_> {
        ready(Ok(inputs
            .into_iter()
            .map(|input| SignMessageOutput {
                signed_message: input.message,
                signature: vec![1],
                signature_type: None,
            })
            .collect()))
    }
}

struct Local(Rc<usize>);
impl WalletFeature for Local {
    fn version(&self) -> &str {
        "1.0.0"
    }
}
struct LocalFuture<'a> {
    value: &'a Rc<usize>,
    pending: bool,
}
impl Future for LocalFuture<'_> {
    type Output = Result<ConnectOutput, WalletError>;
    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.pending {
            self.pending = false;
            context.waker().wake_by_ref();

            return Poll::Pending;
        }

        assert_eq!(**self.value, 42);
        Poll::Ready(Ok(ConnectOutput::default()))
    }
}
impl Connect for Local {
    type Error = WalletError;
    type Future<'a> = LocalFuture<'a>;
    fn connect(&self, _: ConnectInput) -> Self::Future<'_> {
        LocalFuture {
            value: &self.0,
            pending: true,
        }
    }
}

async fn authorize<Backend: Connect>(backend: &Backend) -> Result<ConnectOutput, Backend::Error> {
    backend.connect(ConnectInput::default()).await
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let native = Arc::new(Native);
    let spawned = tokio::spawn(async move {
        tokio::task::yield_now().await;
        assert!(native.sign_message(Vec::new()).await.unwrap().is_empty());
        authorize(&*native).await.unwrap()
    });
    assert!(spawned.await.unwrap().accounts.is_empty());
    let local = Local(Rc::new(42));
    assert!(authorize(&local).await.unwrap().accounts.is_empty());
    println!(
        "Rust 1.89 ordinary consumer executed native Tokio spawn and local Rc across Pending/wake"
    );
}
