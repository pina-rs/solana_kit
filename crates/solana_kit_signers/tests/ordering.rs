#![cfg(feature = "crypto")]

mod support;

use solana_kit_core::{Address, Signature, crypto::Keypair, transaction::Transaction};
use solana_kit_signers::{
    Error, SignatureOutput, Signer, SignerCapability, TransactionModifyingSigner,
    TransactionPartialSigner, TransactionSendingSigner, TransactionSigners,
};

const PARTIAL_SEED: u8 = 7;
const MODIFIER_SEED: u8 = 8;
const SENDER_SEED: u8 = 9;
use std::{
    cell::RefCell,
    future::{Ready, ready},
    rc::Rc,
};

struct Backend {
    key: Keypair,
    events: Rc<RefCell<Vec<&'static str>>>,
    replacement: Option<Transaction>,
    submission: Option<Result<Signature, Error>>,
}

impl Signer for Backend {
    fn address(&self) -> Address {
        self.key.address()
    }
}

impl TransactionModifyingSigner for Backend {
    type Modify<'a> = Ready<Result<Transaction, Error>>;

    fn modify_transaction<'a>(&'a self, transaction: &'a Transaction) -> Self::Modify<'a> {
        self.events.borrow_mut().push("modify");
        ready(Ok(self
            .replacement
            .clone()
            .unwrap_or_else(|| transaction.clone())))
    }
}

impl TransactionPartialSigner for Backend {
    type Sign<'a> = Ready<Result<SignatureOutput, Error>>;

    fn sign_transaction<'a>(&'a self, transaction: &'a Transaction) -> Self::Sign<'a> {
        self.events.borrow_mut().push("partial");
        ready(Ok(SignatureOutput::new(
            self.address(),
            transaction.message().bytes(),
            self.key.sign(transaction.message().bytes()),
        )))
    }
}

impl TransactionSendingSigner for Backend {
    type Send<'a> = Ready<Result<Signature, Error>>;

    fn send_transaction<'a>(&'a self, transaction: &'a Transaction) -> Self::Send<'a> {
        self.events.borrow_mut().push("send");

        if let Some(result) = self.submission {
            return ready(result);
        }

        ready(
            transaction
                .sign(&self.key)
                .and_then(|signed| {
                    signed.verify()?;
                    Ok(self.key.sign(transaction.message().bytes()))
                })
                .map_err(Error::from),
        )
    }
}

fn backend(seed: u8, events: &Rc<RefCell<Vec<&'static str>>>) -> Backend {
    Backend {
        key: Keypair::from_seed(&[seed; 32]),
        events: events.clone(),
        replacement: None,
        submission: None,
    }
}

#[tokio::test(flavor = "current_thread")]
async fn missing_signatures_prevent_send_and_backend_failures_propagate() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let partial = backend(PARTIAL_SEED, &events);
    let mut sender = backend(SENDER_SEED, &events);
    let transaction = support::transaction(partial.address(), 1);
    let empty = TransactionSigners::<Backend>::new(&[]).unwrap();

    assert_eq!(
        empty
            .sign_and_send(&transaction, &[&SignerCapability::new(&sender)])
            .await,
        Err(Error::Transaction(
            solana_kit_core::transaction::Error::MissingSignature
        ))
    );
    assert!(events.borrow().is_empty());
    let partial_capability = SignerCapability::new(&partial);
    let signers = TransactionSigners::new(&[&partial_capability]).unwrap();
    sender.submission = Some(Err(Error::Rejected));
    assert_eq!(
        signers
            .sign_and_send(&transaction, &[&SignerCapability::new(&sender)])
            .await,
        Err(Error::Rejected)
    );
    sender.submission = Some(Ok(Signature::from_bytes([0; 64])));
    assert_eq!(
        signers
            .sign_and_send(&transaction, &[&SignerCapability::new(&sender)])
            .await,
        Err(Error::Transaction(
            solana_kit_core::transaction::Error::InvalidSignature
        ))
    );
}

#[tokio::test(flavor = "current_thread")]
async fn modifier_cannot_install_another_owners_signature() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let partial = backend(PARTIAL_SEED, &events);
    let mut modifier = backend(MODIFIER_SEED, &events);
    let transaction = support::transaction(partial.address(), 1);
    modifier.replacement = Some(transaction.sign(&partial.key).unwrap());
    let partial_capability = SignerCapability::new(&partial);
    let signers = TransactionSigners::new(&[&partial_capability]).unwrap();

    assert_eq!(
        signers
            .modify_and_partially_sign(&transaction, &[&SignerCapability::new(&modifier)])
            .await,
        Err(Error::ForeignSigner)
    );
    assert_eq!(*events.borrow(), ["modify"]);
}

#[tokio::test(flavor = "current_thread")]
async fn modifier_then_partial_then_explicit_sender() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let partial = backend(PARTIAL_SEED, &events);
    let modifier = backend(MODIFIER_SEED, &events);
    let sender = backend(SENDER_SEED, &events);
    let base = support::transaction(partial.address(), 1);
    let instruction = solana_kit_core::Instruction {
        program_address: Address::from_bytes([250; 32]),
        accounts: vec![solana_kit_core::AccountMeta {
            address: sender.address(),
            role: solana_kit_core::AccountRole::ReadonlySigner,
        }],
        data: vec![],
    };
    let transaction = Transaction::new(
        solana_kit_core::transaction::Message::compile(
            base.message().version(),
            partial.address(),
            base.message().lifetime(),
            &[instruction],
            &[],
        )
        .unwrap(),
    );
    let partial_capability = SignerCapability::new(&partial);

    let signers = TransactionSigners::new(&[&partial_capability]).unwrap();
    let signed = signers
        .modify_and_partially_sign(&transaction, &[&SignerCapability::new(&modifier)])
        .await
        .unwrap();
    assert_eq!(signed.signatures().iter().flatten().count(), 1);
    signers
        .sign_and_send(&signed, &[&SignerCapability::new(&sender)])
        .await
        .unwrap();
    assert_eq!(*events.borrow(), ["modify", "partial", "partial", "send"]);
}

#[tokio::test(flavor = "current_thread")]
async fn sender_ambiguity_and_capability_conflicts_fail_before_invocation() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let partial = backend(PARTIAL_SEED, &events);
    let sender = backend(MODIFIER_SEED, &events);
    let other = backend(SENDER_SEED, &events);
    let transaction = support::transaction(partial.address(), 1);
    let partial_capability = SignerCapability::new(&partial);
    let signers = TransactionSigners::new(&[&partial_capability]).unwrap();
    assert_eq!(
        signers.sign_and_send::<Backend>(&transaction, &[]).await,
        Err(Error::MissingSender)
    );

    assert_eq!(
        signers
            .sign_and_send(
                &transaction,
                &[
                    &SignerCapability::new(&sender),
                    &SignerCapability::new(&other)
                ]
            )
            .await,
        Err(Error::MultipleSenders)
    );
    assert_eq!(
        signers
            .sign_and_send(&transaction, &[&partial_capability])
            .await,
        Err(Error::ConflictingSigner)
    );
    assert_eq!(
        signers
            .modify_and_partially_sign(&transaction, &[&partial_capability])
            .await,
        Err(Error::ConflictingSigner)
    );
    assert!(events.borrow().is_empty());
}

#[tokio::test(flavor = "current_thread")]
async fn changed_bytes_clear_old_signatures_and_partial_signs_final_snapshot() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let partial = backend(PARTIAL_SEED, &events);
    let mut modifier = backend(MODIFIER_SEED, &events);
    let transaction = support::transaction(partial.address(), 1)
        .sign(&partial.key)
        .unwrap();
    modifier.replacement = Some(support::transaction(partial.address(), 2));
    let partial_capability = SignerCapability::new(&partial);
    let signers = TransactionSigners::new(&[&partial_capability]).unwrap();
    let signed = signers
        .modify_and_partially_sign(&transaction, &[&SignerCapability::new(&modifier)])
        .await
        .unwrap();

    assert_ne!(signed.message().bytes(), transaction.message().bytes());
    assert_ne!(signed.signatures(), transaction.signatures());
    signed.verify().unwrap();
}

#[tokio::test(flavor = "current_thread")]
async fn stale_signature_on_changed_bytes_is_rejected() {
    let events = Rc::new(RefCell::new(Vec::new()));
    let partial = backend(PARTIAL_SEED, &events);
    let mut modifier = backend(MODIFIER_SEED, &events);
    let transaction = support::transaction(partial.address(), 1)
        .sign(&partial.key)
        .unwrap();
    let changed = support::transaction(partial.address(), 2);
    modifier.replacement = Some(
        changed
            .with_signature(
                partial.address(),
                changed.message().bytes(),
                transaction.signatures()[0].unwrap(),
            )
            .unwrap(),
    );
    let partial_capability = SignerCapability::new(&partial);
    let signers = TransactionSigners::new(&[&partial_capability]).unwrap();

    assert_eq!(
        signers
            .modify_and_partially_sign(&transaction, &[&SignerCapability::new(&modifier)])
            .await,
        Err(Error::ChangedSnapshot)
    );
    assert_eq!(*events.borrow(), ["modify"]);
}

#[tokio::test(flavor = "current_thread")]
async fn automatic_composition_preserves_snapshot_and_sender_failure_boundaries() {
    use solana_kit_signers::local::{Composition, Registration as R};
    let events = Rc::new(RefCell::new(Vec::new()));
    let partial = backend(PARTIAL_SEED, &events);
    let mut modifier = backend(MODIFIER_SEED, &events);
    let tx = support::transaction(partial.address(), 1)
        .sign(&partial.key)
        .unwrap();
    let changed = support::transaction(partial.address(), 2);
    modifier.replacement = Some(
        changed
            .with_signature(
                partial.address(),
                changed.message().bytes(),
                tx.signatures()[0].unwrap(),
            )
            .unwrap(),
    );
    let p = SignerCapability::new(&partial);
    let m = SignerCapability::new(&modifier);

    let c = Composition::new(vec![R::partial(&p), R::modifying(&m)]).unwrap();
    assert_eq!(c.partially_sign(&tx).await, Err(Error::ChangedSnapshot));
    assert_eq!(*events.borrow(), ["modify"]);
    drop(c);
    events.borrow_mut().clear();
    let mut foreign = backend(MODIFIER_SEED, &events);
    foreign.replacement = Some(tx.clone());
    let m = SignerCapability::new(&foreign);

    let unsigned = support::transaction(partial.address(), 1);
    assert_eq!(
        Composition::new(vec![R::partial(&p), R::modifying(&m)])
            .unwrap()
            .partially_sign(&unsigned)
            .await,
        Err(Error::ForeignSigner)
    );
    assert_eq!(*events.borrow(), ["modify"]);
    events.borrow_mut().clear();
    for response in [
        Err(Error::Rejected),
        Err(Error::Cancelled),
        Ok(Signature::from_bytes([0; 64])),
    ] {
        let mut sender = backend(SENDER_SEED, &events);
        sender.submission = Some(response);
        let s = SignerCapability::new(&sender);
        let expected = response.and({
            Err(Error::Transaction(
                solana_kit_core::transaction::Error::InvalidSignature,
            ))
        });
        assert_eq!(
            Composition::new(vec![R::partial(&p), R::sending(&s)])
                .unwrap()
                .sign_and_send(&unsigned)
                .await,
            expected
        );
    }
    assert_eq!(
        *events.borrow(),
        ["partial", "send", "partial", "send", "partial", "send"]
    );
}
