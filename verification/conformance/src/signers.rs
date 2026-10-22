//! Public signer orchestration shared by every portable host.

use alloc::{boxed::Box, rc::Rc, sync::Arc, task::Wake, vec, vec::Vec};
use core::{
    cell::Cell,
    future::{Future, Ready, ready},
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
    task::{Context, Poll, Waker},
};
use solana_kit_core::{
    Address, Hash, Signature,
    crypto::Keypair,
    transaction::{Lifetime, Message, Transaction, Version},
};
use solana_kit_signers::{
    Error, Role, SignatureOutput, Signer, SignerCapability, TransactionModifyingSigner,
    TransactionPartialSigner, TransactionSendingSigner,
    local::{Composition, Registration},
};

mod lifecycle;
mod roles;

type Outcome = Result<(), &'static str>;
/// Fixed successful batch size, shared with private host workload accounting.
pub const MAX_BATCH: usize = 256;

pub(crate) fn cases() -> [crate::CaseResult; 7] {
    [
        crate::case(
            "signer-valid-snapshot",
            crate::require(signer_capability_consumer::valid(), "valid signer snapshot"),
        ),
        crate::case(
            "signer-malformed-signature",
            crate::require(
                signer_capability_consumer::malformed(),
                "zero signer signature",
            ),
        ),
        crate::case(
            "signer-tamper-snapshot",
            crate::require(
                signer_capability_consumer::tamper(),
                "changed signer snapshot",
            ),
        ),
        crate::case("signer-logical-identity", identity()),
        crate::case("signer-heterogeneous-roles", roles::run()),
        crate::case("signer-local-lifecycle", lifecycle::run()),
        crate::case("signer-ordered-batches", batches()),
    ]
}

fn transaction(payer: Address) -> Result<Transaction, &'static str> {
    Message::compile(
        Version::V0,
        payer,
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([1; 32]),
            last_valid_block_height: 100,
        },
        &[],
        &[],
    )
    .map(Transaction::new)
    .map_err(|_| "signer fixture transaction")
}

#[derive(Default)]
struct Counter(AtomicUsize);

impl Wake for Counter {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

fn complete<F: Future>(future: F) -> Result<F::Output, &'static str> {
    let waker = Waker::from(Arc::new(Counter::default()));
    let mut cx = Context::from_waker(&waker);
    let mut future = Box::pin(future);

    match future.as_mut().poll(&mut cx) {
        Poll::Ready(result) => Ok(result),
        Poll::Pending => Err("expected immediate deterministic backend"),
    }
}

struct Stateless;

impl Signer for Stateless {
    fn address(&self) -> Address {
        Address::from_bytes([1; 32])
    }
}

impl TransactionPartialSigner for Stateless {
    type Sign<'a> = Ready<Result<SignatureOutput, Error>>;
    fn sign_transaction<'a>(&'a self, _: &'a Transaction) -> Self::Sign<'a> {
        ready(Err(Error::Rejected))
    }
}

fn identity() -> Outcome {
    let first = Box::new(Stateless);
    let second = Box::new(Stateless);
    let shared = SignerCapability::new(&*first);
    let other = SignerCapability::new(&*second);
    let same_backend = SignerCapability::new(&*first);

    crate::require(
        core::ptr::eq(&*first, &*second),
        "fixture ZST pointers must coincide",
    )?;

    for distinct in [&other, &same_backend] {
        crate::require(
            matches!(
                Composition::new(vec![
                    Registration::partial(&shared),
                    Registration::partial(distinct)
                ]),
                Err(Error::ConflictingSigner)
            ),
            "distinct logical ZST identities must conflict",
        )?;
    }

    let composition = Composition::new(
        (0..256)
            .map(|_| Registration::partial(&shared))
            .collect::<Vec<_>>(),
    )
    .map_err(|_| "shared ZST registration")?;
    crate::require(
        composition.roles(false) == Ok(vec![(first.address(), Role::Partial)]),
        "shared logical identity must deduplicate",
    )?;
    let tx = transaction(first.address())?;
    crate::require(
        complete(composition.partially_sign(&tx))? == Err(Error::Rejected),
        "shared ZST must invoke actual backend",
    )
}

struct Cached {
    output: SignatureOutput,
    effects: Rc<Cell<usize>>,
}

impl Signer for Cached {
    fn address(&self) -> Address {
        self.output.address()
    }
}

impl TransactionPartialSigner for Cached {
    type Sign<'a> = Ready<Result<SignatureOutput, Error>>;
    fn sign_transaction<'a>(&'a self, tx: &'a Transaction) -> Self::Sign<'a> {
        self.effects.set(self.effects.get() + 1);

        if !tx.message().signers().contains(&self.address()) {
            return ready(Err(
                solana_kit_core::transaction::Error::UnknownSigner.into()
            ));
        }

        ready(Ok(self.output.clone()))
    }
}

/// Exercises exact transaction batch limits and first-error effects.
///
/// # Errors
/// Returns the concrete fixture invariant that failed.
pub fn batches() -> Result<(), &'static str> {
    let key = Keypair::from_seed(&[7; 32]);
    let tx = transaction(key.address())?;
    let backend = Cached {
        output: SignatureOutput::new(
            key.address(),
            tx.message().bytes(),
            key.sign(tx.message().bytes()),
        ),
        effects: Rc::new(Cell::new(0)),
    };
    let handle = SignerCapability::new(&backend);
    let composition =
        Composition::new(Registration::partial(&handle)).map_err(|_| "batch composition")?;

    let exact = vec![tx.clone(); MAX_BATCH];
    let output =
        complete(composition.sign_batch(&exact))?.map_err(|_| "exact 256 transaction batch")?;
    crate::require(
        output.len() == MAX_BATCH && backend.effects.get() == MAX_BATCH,
        "exact batch boundary effects",
    )?;
    crate::require(
        output.iter().all(|signed| {
            signed.message().bytes() == tx.message().bytes()
                && signed.signatures() == [Some(backend.output.signature())]
        }),
        "batch output order and immutable signed bytes",
    )?;
    crate::require(
        complete(composition.sign_batch(&vec![tx.clone(); MAX_BATCH + 1]))?
            == Err(Error::TooManySigners)
            && backend.effects.get() == MAX_BATCH,
        "257 batch must reject before effects",
    )?;
    let bad = transaction(Address::from_bytes([99; 32]))?;
    let result = complete(composition.sign_batch(&[tx.clone(), bad, tx]))?;
    crate::require(
        result
            == Err(Error::Transaction(
                solana_kit_core::transaction::Error::UnknownSigner,
            ))
            && backend.effects.get() == MAX_BATCH + 2,
        "ordered batch must stop at second error without rollback",
    )
}
