mod support;

#[cfg(not(feature = "crypto"))]
use solana_kit_core::Signature;
use solana_kit_core::{Address, transaction::Transaction};
use solana_kit_signers::{
    Error, SignatureOutput, Signer, SignerCapability, TransactionPartialSigner, TransactionSigners,
};
use std::{
    cell::Cell,
    future::Future,
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

struct Local {
    address: Address,
    seed: u8,
    events: Rc<Cell<usize>>,
    dropped: Rc<Cell<bool>>,
    failure: Option<Error>,
}

struct Operation<'a> {
    signer: &'a Local,
    transaction: &'a Transaction,
    pending: bool,
}

impl Future for Operation<'_> {
    type Output = Result<SignatureOutput, Error>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        self.signer.events.set(self.signer.events.get() + 1);

        if self.pending {
            self.pending = false;
            context.waker().wake_by_ref();

            return Poll::Pending;
        }

        if let Some(error) = self.signer.failure {
            return Poll::Ready(Err(error));
        }

        #[cfg(feature = "crypto")]
        let signature = solana_kit_core::crypto::Keypair::from_seed(&[self.signer.seed; 32])
            .sign(self.transaction.message().bytes());
        #[cfg(not(feature = "crypto"))]
        let signature = Signature::from_bytes([self.signer.seed; 64]);

        Poll::Ready(Ok(SignatureOutput::new(
            self.signer.address,
            self.transaction.message().bytes(),
            signature,
        )))
    }
}

impl Drop for Operation<'_> {
    fn drop(&mut self) {
        self.signer.dropped.set(true);
    }
}

impl Signer for Local {
    fn address(&self) -> Address {
        self.address
    }
}

impl TransactionPartialSigner for Local {
    type Sign<'a> = Operation<'a>;

    fn sign_transaction<'a>(&'a self, transaction: &'a Transaction) -> Self::Sign<'a> {
        Operation {
            signer: self,
            transaction,
            pending: true,
        }
    }
}

#[derive(Default)]
struct Counter(std::sync::atomic::AtomicUsize);

impl Wake for Counter {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

fn signer(failure: Option<Error>) -> Local {
    signer_for_seed(7, failure)
}

fn signer_for_seed(seed: u8, failure: Option<Error>) -> Local {
    #[cfg(feature = "crypto")]
    let address = solana_kit_core::crypto::Keypair::from_seed(&[seed; 32]).address();
    #[cfg(not(feature = "crypto"))]
    let address = Address::from_bytes([seed; 32]);

    Local {
        address,
        seed,
        events: Rc::new(Cell::new(0)),
        dropped: Rc::new(Cell::new(false)),
        failure,
    }
}

#[test]
fn partials_share_snapshot_and_both_suspend_before_either_completes() {
    const FIRST_SEED: u8 = 7;
    const SECOND_SEED: u8 = 8;
    let first = signer_for_seed(FIRST_SEED, None);
    let second = signer_for_seed(SECOND_SEED, None);
    let base = support::transaction(first.address, 1);
    let instruction = solana_kit_core::Instruction {
        program_address: Address::from_bytes([250; 32]),
        accounts: vec![solana_kit_core::AccountMeta {
            address: second.address,
            role: solana_kit_core::AccountRole::ReadonlySigner,
        }],
        data: vec![],
    };

    let transaction = Transaction::new(
        solana_kit_core::transaction::Message::compile(
            base.message().version(),
            first.address,
            base.message().lifetime(),
            &[instruction],
            &[],
        )
        .unwrap(),
    );
    let first_capability = SignerCapability::new(&first);
    let second_capability = SignerCapability::new(&second);
    let signers = TransactionSigners::new(&[&first_capability, &second_capability]).unwrap();
    let waker = Waker::from(Arc::new(Counter::default()));
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(signers.sign(&transaction));

    assert!(future.as_mut().poll(&mut context).is_pending());
    assert_eq!((first.events.get(), second.events.get()), (1, 1));
    let Poll::Ready(Ok(result)) = future.as_mut().poll(&mut context) else {
        panic!("both signers should complete")
    };
    assert_eq!(result.message().bytes(), transaction.message().bytes());
    assert!(result.signatures().iter().all(Option::is_some));
}

#[test]
fn local_rc_survives_real_pending_wake_and_drop_cancels() {
    let signer = signer(None);
    let transaction = support::transaction(signer.address, 1);
    let capability = SignerCapability::new(&signer);
    let signers = TransactionSigners::new(&[&capability]).unwrap();
    let counter = Arc::new(Counter::default());
    let waker = Waker::from(counter.clone());
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(signers.sign(&transaction));

    assert!(future.as_mut().poll(&mut context).is_pending());
    assert_eq!(signer.events.get(), 1);
    assert_eq!(counter.0.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert!(future.as_mut().poll(&mut context).is_ready());
    assert_eq!(signer.events.get(), 2);

    signer.dropped.set(false);
    let mut cancelled = Box::pin(signers.sign(&transaction));
    assert!(cancelled.as_mut().poll(&mut context).is_pending());
    drop(cancelled);
    assert!(signer.dropped.get());
}

#[tokio::test(flavor = "current_thread")]
async fn backend_rejection_and_cancellation_propagate() {
    for error in [Error::Rejected, Error::Cancelled] {
        let signer = signer(Some(error));
        let transaction = support::transaction(signer.address, 1);
        assert_eq!(
            TransactionSigners::new(&[&SignerCapability::new(&signer)])
                .unwrap()
                .sign(&transaction)
                .await,
            Err(error)
        );
    }
}

#[test]
fn erased_local_future_prototype_also_suspends_but_loses_send() {
    let signer = signer(None);
    let transaction = support::transaction(signer.address, 1);
    let mut erased: Pin<Box<dyn Future<Output = Result<SignatureOutput, Error>> + '_>> =
        Box::pin(signer.sign_transaction(&transaction));
    let waker = Waker::from(Arc::new(Counter::default()));
    let mut context = Context::from_waker(&waker);
    assert!(erased.as_mut().poll(&mut context).is_pending());
    assert!(erased.as_mut().poll(&mut context).is_ready());
}
