use solana_kit_core::{Address, Signature};
use solana_kit_signers::{
    Signer, SignerCapability,
    message::{
        MessageError, MessageModifyingSigner, MessagePartialSigner, MessageSignatureOutput,
        SignableMessage, local, native,
    },
};
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
struct WakeCount(AtomicUsize);
impl Wake for WakeCount {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

struct Local {
    seed: u8,
    pending: u8,
    failure: Option<MessageError>,
    drops: Rc<Cell<usize>>,
    events: Rc<RefCell<Vec<u8>>>,
}

impl Signer for Local {
    fn address(&self) -> Address {
        Address::from_bytes([self.seed; 32])
    }
}

struct LocalOperation<'a> {
    backend: &'a Local,
    messages: &'a [SignableMessage],
    remaining: u8,
    retained: Rc<Cell<usize>>,
}

impl Drop for LocalOperation<'_> {
    fn drop(&mut self) {
        self.retained.set(self.retained.get() + 1);
    }
}

impl Future for LocalOperation<'_> {
    type Output = Result<Vec<MessageSignatureOutput>, MessageError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remaining > 0 {
            self.remaining -= 1;
            cx.waker().wake_by_ref();

            return Poll::Pending;
        }

        self.backend.events.borrow_mut().push(self.backend.seed);

        if let Some(error) = self.backend.failure {
            return Poll::Ready(Err(error));
        }

        let outputs = self
            .messages
            .iter()
            .map(|message| local_output(self.backend, message))
            .collect();
        Poll::Ready(outputs)
    }
}

fn local_output(
    backend: &Local,
    message: &SignableMessage,
) -> Result<MessageSignatureOutput, MessageError> {
    if backend.seed == 1 {
        return MessageSignatureOutput::new(
            message.content(),
            [(
                Address::from_bytes([99; 32]),
                Signature::from_bytes([1; 64]),
            )],
        );
    }

    MessageSignatureOutput::new(
        b"wrong",
        [(backend.address(), Signature::from_bytes([2; 64]))],
    )
}

impl MessagePartialSigner for Local {
    type Sign<'a> = LocalOperation<'a>;

    fn sign_messages<'a>(&'a self, messages: &'a [SignableMessage]) -> Self::Sign<'a> {
        LocalOperation {
            backend: self,
            messages,
            remaining: self.pending,
            retained: Rc::clone(&self.drops),
        }
    }
}

#[test]
fn rc_pending_wake_reverse_completion_ordered_merge_and_cancellation() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let first = Local {
        seed: 1,
        pending: 2,
        failure: None,
        drops: Rc::new(Cell::new(0)),
        events: Rc::clone(&events),
    };
    let second = Local {
        seed: 2,
        pending: 1,
        failure: None,
        drops: Rc::new(Cell::new(0)),
        events: Rc::clone(&events),
    };
    let one = SignerCapability::new(&first);
    let two = SignerCapability::new(&second);
    let pipeline = local::Composition::new(vec![
        local::Registration::partial(&one),
        local::Registration::partial(&two),
    ])
    .unwrap();
    let input = [SignableMessage::new(b"immutable")];
    let wake = Arc::new(WakeCount::default());
    let waker = Waker::from(Arc::clone(&wake));
    let mut cx = Context::from_waker(&waker);
    let mut future = Box::pin(pipeline.sign(&input));
    assert!(future.as_mut().poll(&mut cx).is_pending());
    assert!(future.as_mut().poll(&mut cx).is_pending());
    assert_eq!(*events.borrow(), [2]);
    assert_eq!(
        future.as_mut().poll(&mut cx),
        Poll::Ready(Err(MessageError::ForeignSigner))
    );
    drop(future);
    assert_eq!(*events.borrow(), [2, 1]);
    assert_eq!(
        (
            first.drops.get(),
            second.drops.get(),
            wake.0.load(Ordering::SeqCst)
        ),
        (1, 1, 3)
    );

    let reversed = local::Composition::new(vec![
        local::Registration::partial(&two),
        local::Registration::partial(&one),
    ])
    .unwrap();
    let mut future = Box::pin(reversed.sign(&input));
    assert!(future.as_mut().poll(&mut cx).is_pending());
    assert!(future.as_mut().poll(&mut cx).is_pending());
    assert_eq!(
        future.as_mut().poll(&mut cx),
        Poll::Ready(Err(MessageError::ChangedSnapshot))
    );
    drop(future);
    let mut cancelled = Box::pin(pipeline.sign(&input));
    assert!(cancelled.as_mut().poll(&mut cx).is_pending());
    drop(cancelled);
    assert_eq!((first.drops.get(), second.drops.get()), (3, 3));
}

#[test]
fn first_backend_failure_drops_later_unpolled_work() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let first = Local {
        seed: 1,
        pending: 0,
        failure: Some(MessageError::Cancelled),
        drops: Rc::new(Cell::new(0)),
        events: Rc::clone(&events),
    };
    let second = Local {
        seed: 2,
        pending: 0,
        failure: None,
        drops: Rc::new(Cell::new(0)),
        events: Rc::clone(&events),
    };
    let one = SignerCapability::new(&first);
    let two = SignerCapability::new(&second);
    let pipeline = local::Composition::new(vec![
        local::Registration::partial(&one),
        local::Registration::partial(&two),
    ])
    .unwrap();
    let input = [SignableMessage::new([])];
    let mut future = Box::pin(pipeline.sign(&input));
    assert_eq!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop())),
        Poll::Ready(Err(MessageError::Cancelled))
    );
    drop(future);
    assert_eq!(*events.borrow(), [1]);
    assert_eq!((first.drops.get(), second.drops.get()), (1, 1));
}

struct Native<const SEED: u8>(Arc<AtomicUsize>);

struct PendingOnce<'a> {
    polls: &'a AtomicUsize,
    pending: bool,
}

impl Future for PendingOnce<'_> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        self.polls.fetch_add(1, Ordering::SeqCst);

        if self.pending {
            self.pending = false;
            cx.waker().wake_by_ref();

            return Poll::Pending;
        }

        Poll::Ready(())
    }
}

impl<const SEED: u8> Signer for Native<SEED> {
    fn address(&self) -> Address {
        #[cfg(feature = "crypto")]
        {
            solana_kit_core::crypto::Keypair::from_seed(&[SEED; 32]).address()
        }
        #[cfg(not(feature = "crypto"))]
        {
            Address::from_bytes([SEED; 32])
        }
    }
}

impl<const SEED: u8> MessagePartialSigner for Native<SEED> {
    type Sign<'a> = Pin<
        Box<dyn Future<Output = Result<Vec<MessageSignatureOutput>, MessageError>> + Send + 'a>,
    >;

    fn sign_messages<'a>(&'a self, messages: &'a [SignableMessage]) -> Self::Sign<'a> {
        Box::pin(async move {
            PendingOnce {
                polls: &self.0,
                pending: true,
            }
            .await;
            messages
                .iter()
                .map(|message| {
                    #[cfg(feature = "crypto")]
                    let signature = solana_kit_core::crypto::Keypair::from_seed(&[SEED; 32])
                        .sign(message.content());
                    #[cfg(not(feature = "crypto"))]
                    let signature = Signature::from_bytes([SEED; 64]);
                    MessageSignatureOutput::new(message.content(), [(self.address(), signature)])
                })
                .collect()
        })
    }
}

impl<const SEED: u8> MessageModifyingSigner for Native<SEED> {
    type Modify<'a> =
        Pin<Box<dyn Future<Output = Result<Vec<SignableMessage>, MessageError>> + Send + 'a>>;

    fn modify_messages<'a>(&'a self, messages: &'a [SignableMessage]) -> Self::Modify<'a> {
        Box::pin(async move {
            PendingOnce {
                polls: &self.0,
                pending: true,
            }
            .await;
            assert_eq!(messages.len(), 2);
            messages
                .iter()
                .map(|message| {
                    #[cfg(feature = "crypto")]
                    let signature = solana_kit_core::crypto::Keypair::from_seed(&[SEED; 32])
                        .sign(message.content());
                    #[cfg(not(feature = "crypto"))]
                    let signature = Signature::from_bytes([SEED; 64]);
                    SignableMessage::with_signatures(
                        message.content(),
                        [(self.address(), signature)],
                    )
                })
                .collect()
        })
    }
}

async fn spawn_generic<P, Q>(first: P, second: Q)
where
    P: MessagePartialSigner + MessageModifyingSigner + Send + Sync + 'static,
    Q: MessagePartialSigner + Send + Sync + 'static,
    for<'a> P::Sign<'a>: Send,
    for<'a> P::Modify<'a>: Send,
    for<'a> Q::Sign<'a>: Send,
{
    tokio::spawn(async move {
        let first = SignerCapability::new(&first);
        let second = SignerCapability::new(&second);
        let pipeline = native::Composition::new(vec![
            native::Registration::modifying(&first),
            native::Registration::partial(&second),
        ])
        .unwrap();
        let inputs = [
            SignableMessage::new(b"native"),
            SignableMessage::from_text("é"),
        ];
        let results = pipeline.sign(&inputs).await.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].signatures().len(), 2);
        assert_eq!(results[1].content(), [195, 169]);
    })
    .await
    .unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn native_send_survives_both_heterogeneous_generic_orderings() {
    let polls = Arc::new(AtomicUsize::new(0));
    spawn_generic(
        Native::<7>(Arc::clone(&polls)),
        Native::<8>(Arc::clone(&polls)),
    )
    .await;
    spawn_generic(
        Native::<8>(Arc::clone(&polls)),
        Native::<7>(Arc::clone(&polls)),
    )
    .await;
    assert_eq!(polls.load(Ordering::SeqCst), 8);
}
