mod support;

use solana_kit_core::{Address, transaction::Transaction};
use solana_kit_signers::{
    Error, SignatureOutput, Signer, SignerCapability, TransactionModifyingSigner,
    TransactionPartialSigner, TransactionSigners,
};
use std::{
    cell::Cell,
    future::{Ready, ready},
    sync::atomic::{AtomicUsize, Ordering},
};

static EFFECTS: [AtomicUsize; 3] = [const { AtomicUsize::new(0) }; 3];

struct Stateless<const TEST: usize>;

impl<const TEST: usize> Signer for Stateless<TEST> {
    fn address(&self) -> Address {
        Address::from_bytes([37; 32])
    }
}

impl<const TEST: usize> TransactionPartialSigner for Stateless<TEST> {
    type Sign<'a> = Ready<Result<SignatureOutput, Error>>;

    fn sign_transaction<'a>(&'a self, _: &'a Transaction) -> Self::Sign<'a> {
        EFFECTS[TEST].fetch_add(1, Ordering::SeqCst);
        ready(Err(Error::Rejected))
    }
}

impl<const TEST: usize> TransactionModifyingSigner for Stateless<TEST> {
    type Modify<'a> = Ready<Result<Transaction, Error>>;

    fn modify_transaction<'a>(&'a self, transaction: &'a Transaction) -> Self::Modify<'a> {
        EFFECTS[TEST].fetch_add(1, Ordering::SeqCst);
        ready(Ok(transaction.clone()))
    }
}

#[test]
fn separately_owned_zero_sized_resources_have_distinct_capabilities() {
    let first = Box::new(Stateless::<0>);
    let second = Box::new(Stateless::<0>);
    let first_capability = SignerCapability::new(first.as_ref());
    let second_capability = SignerCapability::new(second.as_ref());

    assert!(matches!(
        TransactionSigners::new(&[&first_capability, &second_capability]),
        Err(Error::ConflictingSigner)
    ));
    assert_eq!(EFFECTS[0].load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn repeated_zero_sized_capability_shares_one_operation() {
    let backend = Stateless::<1>;
    let capability = SignerCapability::new(&backend);
    let transaction = support::transaction(backend.address(), 1);
    let signers = TransactionSigners::new(&[&capability, &capability]).unwrap();

    assert_eq!(
        signers.partially_sign(&transaction).await,
        Err(Error::Rejected)
    );
    assert_eq!(EFFECTS[1].load(Ordering::SeqCst), 1);
    assert!(matches!(
        TransactionSigners::new(&[&capability, &SignerCapability::new(&backend)]),
        Err(Error::ConflictingSigner)
    ));
}

#[tokio::test]
async fn zero_sized_modifier_identity_and_role_conflicts_precede_effects() {
    let first = Stateless::<2>;
    let second = Stateless::<2>;
    let first_capability = SignerCapability::new(&first);
    let second_capability = SignerCapability::new(&second);
    let transaction = support::transaction(first.address(), 1);
    let empty = TransactionSigners::<Stateless<2>>::new(&[]).unwrap();

    assert_eq!(
        empty
            .modify_and_partially_sign(&transaction, &[&first_capability, &second_capability])
            .await,
        Err(Error::ConflictingSigner)
    );
    let partials = TransactionSigners::new(&[&first_capability]).unwrap();
    assert_eq!(
        partials
            .modify_and_partially_sign(&transaction, &[&second_capability])
            .await,
        Err(Error::ConflictingSigner)
    );
    assert_eq!(EFFECTS[2].load(Ordering::SeqCst), 0);
    assert_eq!(
        empty
            .modify_and_partially_sign(&transaction, &[&first_capability, &first_capability])
            .await,
        Ok(transaction)
    );
    assert_eq!(EFFECTS[2].load(Ordering::SeqCst), 1);
}

struct Stateful {
    address: Address,
    effects: Cell<usize>,
}

impl Signer for Stateful {
    fn address(&self) -> Address {
        self.address
    }
}

impl TransactionPartialSigner for Stateful {
    type Sign<'a> = Ready<Result<SignatureOutput, Error>>;

    fn sign_transaction<'a>(&'a self, _: &'a Transaction) -> Self::Sign<'a> {
        self.effects.set(self.effects.get() + 1);
        ready(Err(Error::Rejected))
    }
}

#[tokio::test]
async fn nonzero_backends_preserve_address_conflicts_and_distinct_addresses() {
    let first = Stateful {
        address: Address::from_bytes([11; 32]),
        effects: Cell::new(0),
    };
    let same_address = Stateful {
        address: first.address,
        effects: Cell::new(0),
    };
    let second = Stateful {
        address: Address::from_bytes([12; 32]),
        effects: Cell::new(0),
    };
    let first_capability = SignerCapability::new(&first);
    let second_capability = SignerCapability::new(&second);
    let transaction = support::transaction(first.address, 1);

    assert!(matches!(
        TransactionSigners::new(&[&first_capability, &SignerCapability::new(&same_address)]),
        Err(Error::ConflictingSigner)
    ));
    assert_eq!(first.effects.get() + same_address.effects.get(), 0);
    let signers =
        TransactionSigners::new(&[&first_capability, &first_capability, &second_capability])
            .unwrap();
    assert_eq!(
        signers.partially_sign(&transaction).await,
        Err(Error::Rejected)
    );
    assert_eq!((first.effects.get(), second.effects.get()), (1, 1));
}
