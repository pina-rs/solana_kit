mod support;
use core::future::{Ready, ready};
use solana_kit_core::{Address, Signature, transaction::Transaction};
use solana_kit_signers::local::{Composition, Registration as R};
use solana_kit_signers::{
    Error, Role, SignatureOutput, Signer, SignerCapability, TransactionModifyingSigner,
    TransactionPartialSigner, TransactionSendingSigner,
};
use std::sync::{Arc, Mutex};

struct First {
    seed: u8,
    events: Arc<Mutex<Vec<(u8, char)>>>,
}
struct Second(First);
impl Signer for First {
    fn address(&self) -> Address {
        address(self.seed)
    }
}
impl Signer for Second {
    fn address(&self) -> Address {
        self.0.address()
    }
}
fn address(seed: u8) -> Address {
    #[cfg(feature = "crypto")]
    {
        solana_kit_core::crypto::Keypair::from_seed(&[seed; 32]).address()
    }
    #[cfg(not(feature = "crypto"))]
    {
        Address::from_bytes([seed; 32])
    }
}
impl TransactionPartialSigner for First {
    type Sign<'a> = Pause<Ready<Result<SignatureOutput, Error>>>;
    fn sign_transaction<'a>(&'a self, tx: &'a Transaction) -> Self::Sign<'a> {
        self.events.lock().unwrap().push((self.seed, 'p'));
        #[cfg(feature = "crypto")]
        let signature = solana_kit_core::crypto::Keypair::from_seed(&[self.seed; 32])
            .sign(tx.message().bytes());
        #[cfg(not(feature = "crypto"))]
        let signature = Signature::from_bytes([self.seed; 64]);
        Pause {
            inner: ready(Ok(SignatureOutput::new(
                self.address(),
                tx.message().bytes(),
                signature,
            ))),
            pending: true,
        }
    }
}
impl TransactionModifyingSigner for First {
    type Modify<'a> = Ready<Result<Transaction, Error>>;
    fn modify_transaction<'a>(&'a self, tx: &'a Transaction) -> Self::Modify<'a> {
        self.events.lock().unwrap().push((self.seed, 'm'));
        ready(Ok(tx.clone()))
    }
}
impl TransactionSendingSigner for First {
    type Send<'a> = Ready<Result<Signature, Error>>;
    fn send_transaction<'a>(&'a self, _: &'a Transaction) -> Self::Send<'a> {
        self.events.lock().unwrap().push((self.seed, 's'));
        ready(Ok(Signature::from_bytes([self.seed; 64])))
    }
}
impl TransactionPartialSigner for Second {
    type Sign<'a> = Pause<Ready<Result<SignatureOutput, Error>>>;
    fn sign_transaction<'a>(&'a self, tx: &'a Transaction) -> Self::Sign<'a> {
        self.0.sign_transaction(tx)
    }
}
impl TransactionModifyingSigner for Second {
    type Modify<'a> = Ready<Result<Transaction, Error>>;
    fn modify_transaction<'a>(&'a self, tx: &'a Transaction) -> Self::Modify<'a> {
        self.0.modify_transaction(tx)
    }
}
impl TransactionSendingSigner for Second {
    type Send<'a> = Ready<Result<Signature, Error>>;
    fn send_transaction<'a>(&'a self, tx: &'a Transaction) -> Self::Send<'a> {
        self.0.send_transaction(tx)
    }
}
fn backends() -> (First, Second) {
    let events = Arc::new(Mutex::new(Vec::new()));
    (
        First {
            seed: 7,
            events: events.clone(),
        },
        Second(First { seed: 8, events }),
    )
}
fn roles<'a>(composition: impl Into<Vec<R<'a>>>, sending: bool) -> Result<Vec<Role>, Error> {
    Composition::new(composition)?
        .roles(sending)
        .map(|v| v.into_iter().map(|x| x.1).collect())
}

#[test]
fn seven_explicit_interface_combinations_have_literal_roles() {
    use Role::*;
    let (a, _) = backends();
    let h = SignerCapability::new(&a);
    assert_eq!(roles(R::partial(&h), false), Ok(vec![Partial]));
    assert_eq!(roles(R::modifying(&h), false), Ok(vec![Modifying]));
    assert_eq!(roles(R::sending(&h), false), Ok(vec![Ignored]));
    assert_eq!(roles(R::partial_modifying(&h), false), Ok(vec![Modifying]));
    assert_eq!(roles(R::partial_sending(&h), false), Ok(vec![Partial]));
    assert_eq!(roles(R::modifying_sending(&h), false), Ok(vec![Modifying]));

    assert_eq!(roles(R::composite(&h), false), Ok(vec![Modifying]));
    assert_eq!(roles(R::partial(&h), true), Err(Error::MissingSender));
    assert_eq!(roles(R::modifying(&h), true), Err(Error::MissingSender));
    assert_eq!(
        roles(R::partial_modifying(&h), true),
        Err(Error::MissingSender)
    );
    assert_eq!(roles(R::sending(&h), true), Ok(vec![Sending]));
    assert_eq!(roles(R::partial_sending(&h), true), Ok(vec![Sending]));
    assert_eq!(roles(R::modifying_sending(&h), true), Ok(vec![Sending]));
    assert_eq!(roles(R::composite(&h), true), Ok(vec![Sending]));
}

#[test]
fn source_order_and_sender_preference_control_composite_demotion() {
    use Role::*;
    let (a, b) = backends();
    let x = SignerCapability::new(&a);
    let y = SignerCapability::new(&b);
    assert_eq!(
        roles(R::composite(&x).and(R::composite(&y)), false),
        Ok(vec![Modifying, Partial])
    );
    assert_eq!(
        roles(R::composite(&y).and(R::composite(&x)), false),
        Ok(vec![Modifying, Partial])
    );
    assert_eq!(
        roles(R::partial_modifying(&x).and(R::modifying(&y)), false),
        Ok(vec![Partial, Modifying])
    );
    assert_eq!(
        roles(R::modifying(&x).and(R::modifying_sending(&y)), false),
        Ok(vec![Modifying, Modifying])
    );
    assert_eq!(
        roles(R::composite(&x).and(R::composite(&y)), true),
        Ok(vec![Sending, Modifying])
    );

    assert_eq!(
        roles(R::composite(&x).and(R::sending(&y)), true),
        Ok(vec![Modifying, Sending])
    );
    assert_eq!(
        roles(R::sending(&x).and(R::composite(&y)), true),
        Ok(vec![Sending, Modifying])
    );
    assert_eq!(
        roles(R::partial_sending(&x).and(R::partial_sending(&y)), true),
        Ok(vec![Sending, Partial])
    );
    assert_eq!(
        roles(R::sending(&x).and(R::sending(&y)), true),
        Err(Error::MultipleSenders)
    );
    assert_eq!(
        roles(R::sending(&x).and(R::sending(&x)), true),
        Err(Error::MultipleSenders)
    );
    assert_eq!(
        roles(R::partial(&x).and(R::partial(&x)), false),
        Ok(vec![Partial])
    );
    assert_eq!(
        roles(R::partial(&x).and(R::composite(&x)), false),
        Err(Error::ConflictingSigner)
    );
    let other = SignerCapability::new(&a);

    assert_eq!(
        roles(R::partial(&x).and(R::partial(&other)), false),
        Err(Error::ConflictingSigner)
    );
}

#[tokio::test]
async fn heterogeneous_modifiers_partials_and_batches_observe_order() {
    let (a, b) = backends();
    let x = SignerCapability::new(&a);
    let y = SignerCapability::new(&b);
    let tx = support::transaction(a.address(), 1);
    let composition = Composition::new(R::modifying(&x).and(R::modifying(&y))).unwrap();
    composition.partially_sign(&tx).await.unwrap();
    assert_eq!(*a.events.lock().unwrap(), vec![(7, 'm'), (8, 'm')]);
    a.events.lock().unwrap().clear();

    let composition = Composition::new(R::partial(&x).and(R::partial(&x))).unwrap();
    let result = composition
        .sign_batch(&[tx.clone(), tx.clone()])
        .await
        .unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(*a.events.lock().unwrap(), vec![(7, 'p'), (7, 'p')]);
    a.events.lock().unwrap().clear();
    assert_eq!(
        composition.sign_batch(&vec![tx; 257]).await,
        Err(Error::TooManySigners)
    );
    assert!(a.events.lock().unwrap().is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn generic_heterogeneous_pipeline_is_send_and_spawns() {
    let (a, b) = backends();
    let tx = support::transaction(a.address(), 1);
    let task = spawn(a, b, tx);
    assert_eq!(task.await.unwrap().unwrap(), Signature::from_bytes([8; 64]));

    let (a, b) = backends();
    let tx = support::transaction(b.address(), 1);
    assert_eq!(
        spawn(b, a, tx).await.unwrap().unwrap(),
        Signature::from_bytes([7; 64])
    );
}

fn spawn<P, Q>(
    partial: P,
    sender: Q,
    tx: Transaction,
) -> tokio::task::JoinHandle<Result<Signature, Error>>
where
    P: TransactionPartialSigner + Send + Sync + 'static,
    Q: TransactionSendingSigner + Send + Sync + 'static,
    for<'a> P::Sign<'a>: Send,
    for<'a> Q::Send<'a>: Send,
{
    tokio::spawn(async move {
        let a = SignerCapability::new(&partial);
        let b = SignerCapability::new(&sender);
        let composition = solana_kit_signers::native::Composition::new(vec![
            solana_kit_signers::native::Registration::partial(&a),
            solana_kit_signers::native::Registration::sending(&b),
        ])?;
        composition.sign_and_send(&tx).await
    })
}

struct Pause<F> {
    inner: F,
    pending: bool,
}
impl<F: core::future::Future + Unpin> core::future::Future for Pause<F> {
    type Output = F::Output;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        let this = self.get_mut();

        if this.pending {
            this.pending = false;
            cx.waker().wake_by_ref();

            return core::task::Poll::Pending;
        }

        core::pin::Pin::new(&mut this.inner).poll(cx)
    }
}

#[test]
fn composition_zst_handle_identity_and_pre_effect_bounds() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static CALLS: AtomicUsize = AtomicUsize::new(0);
    struct Stateless;
    impl Signer for Stateless {
        fn address(&self) -> Address {
            Address::from_bytes([242; 32])
        }
    }
    impl TransactionPartialSigner for Stateless {
        type Sign<'a> = Ready<Result<SignatureOutput, Error>>;
        fn sign_transaction<'a>(&'a self, _: &'a Transaction) -> Self::Sign<'a> {
            CALLS.fetch_add(1, Ordering::SeqCst);
            ready(Err(Error::Rejected))
        }
    }
    let a = Box::new(Stateless);
    let b = Box::new(Stateless);
    assert!(core::ptr::eq(&*a, &*b));
    let x = SignerCapability::new(&*a);
    let y = SignerCapability::new(&*b);
    assert!(matches!(
        Composition::new(vec![R::partial(&x), R::partial(&y)]),
        Err(Error::ConflictingSigner)
    ));
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
    let registrations = (0..257).map(|_| R::partial(&x)).collect::<Vec<_>>();

    assert!(matches!(
        Composition::new(registrations),
        Err(Error::TooManySigners)
    ));
    assert_eq!(CALLS.load(Ordering::SeqCst), 0);
}

fn declared<'a, P>(flags: u8, handle: &'a SignerCapability<'a, P>) -> R<'a>
where
    P: TransactionPartialSigner + TransactionModifyingSigner + TransactionSendingSigner,
{
    match flags {
        1 => R::partial(handle),
        2 => R::modifying(handle),
        3 => R::partial_modifying(handle),
        4 => R::sending(handle),
        5 => R::partial_sending(handle),
        6 => R::modifying_sending(handle),
        7 => R::composite(handle),
        _ => panic!("invalid oracle flags"),
    }
}

/// Read literal source output so role expectations do not duplicate the Rust algorithm.
#[test]
fn all_112_literal_dart_role_oracles_match() {
    let (a, b) = backends();
    let x = SignerCapability::new(&a);
    let y = SignerCapability::new(&b);
    let fixture = include_str!("../../../verification/signer-capabilities/fixtures/dart-roles.tsv");
    assert_eq!(fixture.lines().count(), 112);

    for line in fixture.lines() {
        let columns = line.split('\t').collect::<Vec<_>>();
        let flags = columns[0]
            .split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        let mut registrations = vec![declared(flags[0], &x)];

        if flags.len() == 2 {
            registrations.push(declared(flags[1], &y));
        }

        let composition = Composition::new(registrations).unwrap();

        let actual = match composition.roles(columns[1] == "1") {
            Ok(roles) => roles
                .into_iter()
                .map(|(_, role)| match role {
                    Role::Partial => "p",
                    Role::Modifying => "m",
                    Role::Sending => "s",
                    Role::Ignored => "i",
                })
                .collect::<Vec<_>>()
                .join(","),
            Err(Error::MissingSender) => "missing".into(),
            Err(Error::MultipleSenders) => "multiple".into(),
            Err(error) => panic!("unexpected oracle error {error}"),
        };

        assert_eq!(actual, columns[2], "{line}");
    }
}

#[tokio::test]
async fn transaction_batch_stops_at_first_failure() {
    let (a, b) = backends();
    let handle = SignerCapability::new(&a);
    let composition = Composition::new(R::partial(&handle)).unwrap();
    let good = support::transaction(a.address(), 1);
    let bad = support::transaction(b.address(), 1);
    assert_eq!(
        composition.sign_batch(&[good.clone(), bad, good]).await,
        Err(Error::Transaction(
            solana_kit_core::transaction::Error::UnknownSigner
        ))
    );
    assert_eq!(*a.events.lock().unwrap(), vec![(7, 'p'), (7, 'p')]);
}

#[tokio::test]
async fn sender_resolution_errors_precede_all_backend_effects() {
    let (a, b) = backends();
    let x = SignerCapability::new(&a);
    let y = SignerCapability::new(&b);
    let tx = support::transaction(a.address(), 1);
    assert_eq!(
        Composition::new(R::partial(&x))
            .unwrap()
            .sign_and_send(&tx)
            .await,
        Err(Error::MissingSender)
    );
    assert_eq!(
        Composition::new(vec![R::sending(&x), R::sending(&y)])
            .unwrap()
            .sign_and_send(&tx)
            .await,
        Err(Error::MultipleSenders)
    );
    assert!(a.events.lock().unwrap().is_empty());
}
