use solana_kit_core::{
    Hash,
    crypto::Keypair,
    transaction::{Lifetime, Message, Transaction, Version},
};
use solana_kit_signers::{
    Error, SignatureOutput, Signer, SignerCapability, TransactionPartialSigner, TransactionSigners,
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

struct Native(Keypair);

struct Suspended<'a> {
    signer: &'a Native,
    transaction: &'a Transaction,
    pending: bool,
}

impl Future for Suspended<'_> {
    type Output = Result<SignatureOutput, Error>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.pending {
            self.pending = false;
            context.waker().wake_by_ref();

            return Poll::Pending;
        }

        let bytes = self.transaction.message().bytes();
        Poll::Ready(Ok(SignatureOutput::new(
            self.signer.address(),
            bytes,
            self.signer.0.sign(bytes),
        )))
    }
}

impl Signer for Native {
    fn address(&self) -> solana_kit_core::Address {
        self.0.address()
    }
}

impl TransactionPartialSigner for Native {
    type Sign<'a> = Suspended<'a>;

    fn sign_transaction<'a>(&'a self, transaction: &'a Transaction) -> Self::Sign<'a> {
        Suspended {
            signer: self,
            transaction,
            pending: true,
        }
    }
}

fn spawn_signing<P>(signer: P, transaction: Transaction) -> tokio::task::JoinHandle<Transaction>
where
    P: TransactionPartialSigner + Send + Sync + 'static,
    for<'a> P::Sign<'a>: Send,
{
    tokio::spawn(async move {
        tokio::task::yield_now().await;
        TransactionSigners::new(&[&SignerCapability::new(&signer)])
            .unwrap()
            .sign(&transaction)
            .await
            .unwrap()
    })
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn ordinary_consumer_spawns_with_precise_associated_future_bound() {
    let signer = Native(Keypair::from_seed(&[7; 32]));
    let lifetime = Lifetime::Blockhash {
        blockhash: Hash::from_bytes([1; 32]),
        last_valid_block_height: 100,
    };
    let transaction = Transaction::new(
        Message::compile(Version::V0, signer.address(), lifetime, &[], &[]).unwrap(),
    );
    spawn_signing(signer, transaction)
        .await
        .unwrap()
        .verify()
        .unwrap();
}

#[test]
fn isolated_portable_controls_execute_natively() {
    assert!(signer_capability_consumer::valid());
    assert!(signer_capability_consumer::malformed());
    assert!(signer_capability_consumer::tamper());
}
