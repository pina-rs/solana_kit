use solana_kit_core::{Address, transaction::Transaction};
use solana_kit_signers::{
    Error, SignatureOutput, Signer, SignerCapability, TransactionPartialSigner, local, native,
};
use std::{
    fmt,
    future::{Ready, ready},
    rc::Rc,
};

mod support;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Failure {
    Rule(Error),
    Backend(Rc<String>),
}

impl From<Error> for Failure {
    fn from(error: Error) -> Self {
        Self::Rule(error)
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("typed backend failure")
    }
}

impl std::error::Error for Failure {}

struct LocalBackend(Rc<String>);

impl Signer for LocalBackend {
    fn address(&self) -> Address {
        Address::from_bytes([1; 32])
    }
}

impl TransactionPartialSigner<Failure> for LocalBackend {
    type Sign<'a> = Ready<Result<SignatureOutput, Failure>>;
    fn sign_transaction<'a>(&'a self, _: &'a Transaction) -> Self::Sign<'a> {
        ready(Err(Failure::Backend(self.0.clone())))
    }
}

struct DefaultBackend;

impl Signer for DefaultBackend {
    fn address(&self) -> Address {
        Address::from_bytes([2; 32])
    }
}

impl TransactionPartialSigner for DefaultBackend {
    type Sign<'a> = Ready<Result<SignatureOutput, Error>>;
    fn sign_transaction<'a>(&'a self, _: &'a Transaction) -> Self::Sign<'a> {
        ready(Err(Error::Rejected))
    }
}

#[tokio::test]
async fn mixed_default_and_local_backend_failures_retain_their_owner() {
    let cause = Rc::new(String::from("original local cause"));
    let backend = LocalBackend(cause.clone());
    let default = DefaultBackend;
    let wallet = SignerCapability::new(&backend);
    let local = SignerCapability::new(&default);
    let composition = local::Composition::<Failure>::with_failure(vec![
        local::Registration::partial(&wallet),
        local::Registration::partial_with(&local, Failure::from),
    ])
    .unwrap();
    let error = composition
        .partially_sign(&support::transaction(backend.address(), 3))
        .await
        .unwrap_err();
    let Failure::Backend(returned) = error else {
        panic!("backend cause was flattened")
    };
    assert!(Rc::ptr_eq(&cause, &returned));
}

#[test]
fn default_native_errors_and_futures_keep_send() {
    fn require_send<T: Send>(_: T) {}
    let backend = DefaultBackend;
    let capability = SignerCapability::new(&backend);
    let composition = native::Composition::new(native::Registration::partial(&capability)).unwrap();
    let transaction = support::transaction(backend.address(), 4);
    require_send(Error::Rejected);
    require_send(composition.partially_sign(&transaction));
}
