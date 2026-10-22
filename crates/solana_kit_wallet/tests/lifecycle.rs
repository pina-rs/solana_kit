use solana_kit_wallet::*;
use std::{
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};

#[derive(Default)]
struct Authorization {
    granted: Cell<bool>,
    cancelled: Cell<usize>,
    wake: RefCell<Option<Waker>>,
}

struct LocalBackend(Rc<Authorization>);
impl WalletFeature for LocalBackend {
    fn version(&self) -> &str {
        "1.0.0"
    }
}
struct Authorize<'a> {
    state: &'a Rc<Authorization>,
    completed: bool,
}
impl Future for Authorize<'_> {
    type Output = Result<ConnectOutput, WalletError>;
    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.state.granted.get() {
            self.completed = true;

            return Poll::Ready(Ok(ConnectOutput::default()));
        }

        self.state.wake.replace(Some(context.waker().clone()));
        Poll::Pending
    }
}
impl Drop for Authorize<'_> {
    fn drop(&mut self) {
        self.state.wake.borrow_mut().take();

        if !self.completed {
            self.state.cancelled.set(self.state.cancelled.get() + 1);
        }
    }
}
impl Connect for LocalBackend {
    type Error = WalletError;
    type Future<'a> = Authorize<'a>;
    fn connect(&self, _: ConnectInput) -> Self::Future<'_> {
        Authorize {
            state: &self.0,
            completed: false,
        }
    }
}

#[derive(Default)]
struct Counter(AtomicUsize);
impl Wake for Counter {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn real_pending_authorization_wakes_and_drop_cancels_without_executor() {
    let state = Rc::new(Authorization::default());
    let backend = LocalBackend(state.clone());
    let counter = Arc::new(Counter::default());
    let waker = Waker::from(counter.clone());
    let mut context = Context::from_waker(&waker);
    let mut pending = Box::pin(backend.connect(ConnectInput::default()));
    assert!(pending.as_mut().poll(&mut context).is_pending());
    state.granted.set(true);

    state.wake.borrow_mut().take().unwrap().wake();
    assert_eq!(counter.0.load(Ordering::SeqCst), 1);
    assert!(pending.as_mut().poll(&mut context).is_ready());
    drop(pending);
    assert_eq!(state.cancelled.get(), 0);
    state.granted.set(false);
    let mut cancelled = Box::pin(backend.connect(ConnectInput::default()));
    assert!(cancelled.as_mut().poll(&mut context).is_pending());
    drop(cancelled);
    assert_eq!(state.cancelled.get(), 1);
    assert!(state.wake.borrow().is_none());
}

type ChangeListener = Box<dyn Fn(&WalletChange)>;
#[derive(Default)]
struct EventBackend(Rc<RefCell<Option<ChangeListener>>>);
impl WalletFeature for EventBackend {
    fn version(&self) -> &str {
        "1.0.0"
    }
}
struct ChangeSubscription(Rc<RefCell<Option<ChangeListener>>>);
impl Drop for ChangeSubscription {
    fn drop(&mut self) {
        self.0.borrow_mut().take();
    }
}
impl Events for EventBackend {
    type Subscription = ChangeSubscription;
    fn on_change(&self, listener: ChangeListener) -> Self::Subscription {
        self.0.replace(Some(listener));
        ChangeSubscription(self.0.clone())
    }
}

#[test]
fn concrete_change_capability_distinguishes_absence_and_clear_and_unsubscribes() {
    let backend = EventBackend::default();
    let calls = Rc::new(Cell::new(0));
    let observed = calls.clone();
    let subscription = backend.on_change(Box::new(move |change| {
        assert!(change.chains.is_none());
        assert!(change.features.is_none());
        assert!(change.accounts.as_ref().unwrap().is_empty());
        observed.set(observed.get() + 1);
    }));
    let listener = backend.0.borrow_mut().take().unwrap();
    listener(&WalletChange {
        accounts: Some(vec![]),
        ..WalletChange::default()
    });
    backend.0.replace(Some(listener));
    assert_eq!(calls.get(), 1);

    drop(subscription);
    assert!(backend.0.borrow().is_none());
}

struct SignFuture<'a> {
    authorization: Authorize<'a>,
    message: Option<Vec<SignMessageInput>>,
}
impl Future for SignFuture<'_> {
    type Output = Result<Vec<SignMessageOutput>, WalletError>;
    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.authorization).poll(context) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(error)) => Poll::Ready(Err(error)),
            Poll::Ready(Ok(_)) => Poll::Ready(Ok(self
                .message
                .take()
                .unwrap()
                .into_iter()
                .map(|input| SignMessageOutput {
                    signed_message: input.message,
                    signature: vec![1],
                    signature_type: None,
                })
                .collect())),
        }
    }
}
impl SignMessage for LocalBackend {
    type Error = WalletError;
    type Future<'a> = SignFuture<'a>;
    fn sign_message(&self, message: Vec<SignMessageInput>) -> Self::Future<'_> {
        SignFuture {
            authorization: Authorize {
                state: &self.0,
                completed: false,
            },
            message: Some(message),
        }
    }
}

#[test]
fn local_signing_future_retains_rc_pending_wake_and_cancellation() {
    let state = Rc::new(Authorization::default());
    let backend = LocalBackend(state.clone());
    let mut context = Context::from_waker(Waker::noop());
    let mut future = Box::pin(backend.sign_message(vec![]));
    assert!(future.as_mut().poll(&mut context).is_pending());
    drop(future);
    assert_eq!(state.cancelled.get(), 1);
    let mut future = Box::pin(backend.sign_message(vec![]));

    assert!(future.as_mut().poll(&mut context).is_pending());
    state.granted.set(true);
    state.wake.borrow_mut().take().unwrap().wake();
    assert!(
        matches!(future.as_mut().poll(&mut context), Poll::Ready(Ok(messages)) if messages.is_empty())
    );
    drop(future);
    assert_eq!(state.cancelled.get(), 1);
}
