//! Per-call configuration reaches every backend role without retained state.

mod support;

use core::{
    cell::Cell,
    future::{Future, Ready, ready},
    pin::Pin,
    task::{Context, Poll},
};
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Hash, Instruction, Signature,
    transaction::{Lifetime, Message, Transaction, Version},
};
use solana_kit_signers::{
    Error, MessageSigner, SignatureOutput, Signer, SignerCapability, SignerConfig,
    TransactionModifyingSigner, TransactionPartialSigner, TransactionSendingSigner,
    TransactionSignerConfig, TransactionSigners,
    message::{
        MessageError, MessageModifyingSigner, MessagePartialSigner, MessageSignatureOutput,
        SignableMessage, SingularMessageSigner,
        local::{Composition as MessageComposition, Registration as MessageRegistration},
    },
};
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

const MODIFIER: u8 = 3;
const PARTIAL: u8 = 7;
const SENDER: u8 = 9;

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

fn signature(seed: u8, bytes: &[u8]) -> Signature {
    #[cfg(feature = "crypto")]
    {
        solana_kit_core::crypto::Keypair::from_seed(&[seed; 32]).sign(bytes)
    }
    #[cfg(not(feature = "crypto"))]
    {
        let mut signature = [seed.max(1); 64];
        signature[0] = bytes.iter().fold(seed, |sum, byte| sum.wrapping_add(*byte));
        Signature::from_bytes(signature)
    }
}

fn transaction(payer: Address, sender: Address) -> Transaction {
    let instruction = Instruction {
        program_address: Address::from_bytes([250; 32]),
        accounts: vec![AccountMeta {
            address: sender,
            role: AccountRole::ReadonlySigner,
        }],
        data: Vec::new(),
    };
    let lifetime = Lifetime::Blockhash {
        blockhash: Hash::from_bytes([1; 32]),
        last_valid_block_height: 100,
    };
    Transaction::new(
        Message::compile(Version::Legacy, payer, lifetime, &[instruction], &[]).unwrap(),
    )
}

fn configurations() -> [TransactionSignerConfig; 3] {
    [
        TransactionSignerConfig::default(),
        TransactionSignerConfig {
            aborted: false,
            min_context_slot: Some(0),
        },
        TransactionSignerConfig {
            aborted: false,
            min_context_slot: Some(u64::MAX),
        },
    ]
}

struct LocalTransactionBackend {
    seed: u8,
    log: Rc<RefCell<Vec<(u8, TransactionSignerConfig)>>>,
}

impl LocalTransactionBackend {
    fn new(seed: u8, log: &Rc<RefCell<Vec<(u8, TransactionSignerConfig)>>>) -> Self {
        Self {
            seed,
            log: Rc::clone(log),
        }
    }
}

impl Signer for LocalTransactionBackend {
    fn address(&self) -> Address {
        address(self.seed)
    }
}

impl TransactionModifyingSigner for LocalTransactionBackend {
    type Modify<'a> = Ready<Result<Transaction, Error>>;

    fn modify_transaction<'a>(
        &'a self,
        transaction: &'a Transaction,
        config: TransactionSignerConfig,
    ) -> Self::Modify<'a> {
        self.log.borrow_mut().push((self.seed, config));
        ready(Ok(transaction.clone()))
    }
}

impl TransactionPartialSigner for LocalTransactionBackend {
    type Sign<'a> = Ready<Result<SignatureOutput, Error>>;

    fn sign_transaction<'a>(
        &'a self,
        transaction: &'a Transaction,
        config: TransactionSignerConfig,
    ) -> Self::Sign<'a> {
        self.log.borrow_mut().push((self.seed, config));
        ready(Ok(SignatureOutput::new(
            self.address(),
            transaction.message().bytes(),
            signature(self.seed, transaction.message().bytes()),
        )))
    }
}

impl TransactionSendingSigner for LocalTransactionBackend {
    type Send<'a> = Ready<Result<Signature, Error>>;

    fn send_transaction<'a>(
        &'a self,
        transaction: &'a Transaction,
        config: TransactionSignerConfig,
    ) -> Self::Send<'a> {
        self.log.borrow_mut().push((self.seed, config));
        ready(Ok(signature(self.seed, transaction.message().bytes())))
    }
}

#[tokio::test(flavor = "current_thread")]
async fn local_heterogeneous_roles_see_each_call_config() {
    use solana_kit_signers::local::{Composition, Registration};

    let log = Rc::new(RefCell::new(Vec::new()));
    let modifier = LocalTransactionBackend::new(MODIFIER, &log);
    let partial = LocalTransactionBackend::new(PARTIAL, &log);
    let sender = LocalTransactionBackend::new(SENDER, &log);
    let transaction = transaction(partial.address(), sender.address());
    let m = SignerCapability::new(&modifier);
    let p = SignerCapability::new(&partial);
    let s = SignerCapability::new(&sender);
    let composition = Composition::new(vec![
        Registration::partial(&p),
        Registration::modifying(&m),
        Registration::sending(&s),
    ])
    .unwrap();

    for config in configurations() {
        composition
            .sign_and_send(&transaction, config)
            .await
            .unwrap();
    }

    let expected: Vec<_> = configurations()
        .into_iter()
        .flat_map(|config| [(MODIFIER, config), (PARTIAL, config), (SENDER, config)])
        .collect();
    assert_eq!(*log.borrow(), expected);
}

#[tokio::test(flavor = "current_thread")]
async fn typed_pipeline_forwards_each_call_config_to_all_roles() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let modifier = LocalTransactionBackend::new(MODIFIER, &log);
    let partial = LocalTransactionBackend::new(PARTIAL, &log);
    let sender = LocalTransactionBackend::new(SENDER, &log);
    let transaction = transaction(partial.address(), sender.address());
    let p = SignerCapability::new(&partial);
    let signers = TransactionSigners::new(&[&p]).unwrap();

    for config in configurations() {
        log.borrow_mut().clear();
        signers
            .modify_and_partially_sign(&transaction, &[&SignerCapability::new(&modifier)], config)
            .await
            .unwrap();
        signers
            .sign_and_send(&transaction, &[&SignerCapability::new(&sender)], config)
            .await
            .unwrap();
        assert_eq!(
            *log.borrow(),
            vec![
                (MODIFIER, config),
                (PARTIAL, config),
                (PARTIAL, config),
                (SENDER, config)
            ]
        );
    }
}

struct NativeTransactionBackend {
    seed: u8,
    log: Arc<Mutex<Vec<(u8, TransactionSignerConfig)>>>,
}

impl NativeTransactionBackend {
    fn new(seed: u8, log: &Arc<Mutex<Vec<(u8, TransactionSignerConfig)>>>) -> Self {
        Self {
            seed,
            log: Arc::clone(log),
        }
    }
}

impl Signer for NativeTransactionBackend {
    fn address(&self) -> Address {
        address(self.seed)
    }
}

impl TransactionModifyingSigner for NativeTransactionBackend {
    type Modify<'a> = Ready<Result<Transaction, Error>>;

    fn modify_transaction<'a>(
        &'a self,
        transaction: &'a Transaction,
        config: TransactionSignerConfig,
    ) -> Self::Modify<'a> {
        self.log.lock().unwrap().push((self.seed, config));
        ready(Ok(transaction.clone()))
    }
}

impl TransactionPartialSigner for NativeTransactionBackend {
    type Sign<'a> = Ready<Result<SignatureOutput, Error>>;

    fn sign_transaction<'a>(
        &'a self,
        transaction: &'a Transaction,
        config: TransactionSignerConfig,
    ) -> Self::Sign<'a> {
        self.log.lock().unwrap().push((self.seed, config));
        ready(Ok(SignatureOutput::new(
            self.address(),
            transaction.message().bytes(),
            signature(self.seed, transaction.message().bytes()),
        )))
    }
}

impl TransactionSendingSigner for NativeTransactionBackend {
    type Send<'a> = Ready<Result<Signature, Error>>;

    fn send_transaction<'a>(
        &'a self,
        transaction: &'a Transaction,
        config: TransactionSignerConfig,
    ) -> Self::Send<'a> {
        self.log.lock().unwrap().push((self.seed, config));
        ready(Ok(signature(self.seed, transaction.message().bytes())))
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn native_heterogeneous_roles_forward_config_through_a_spawned_task() {
    use solana_kit_signers::native::{Composition, Registration};

    for config in configurations() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let modifier = NativeTransactionBackend::new(MODIFIER, &log);
        let partial = NativeTransactionBackend::new(PARTIAL, &log);
        let sender = NativeTransactionBackend::new(SENDER, &log);
        let expected_log = Arc::clone(&log);

        tokio::spawn(async move {
            let transaction = transaction(partial.address(), sender.address());
            let m = SignerCapability::new(&modifier);
            let p = SignerCapability::new(&partial);
            let s = SignerCapability::new(&sender);
            let composition = Composition::new(vec![
                Registration::partial(&p),
                Registration::modifying(&m),
                Registration::sending(&s),
            ])
            .unwrap();
            composition
                .sign_and_send(&transaction, config)
                .await
                .unwrap();
        })
        .await
        .unwrap();

        assert_eq!(
            *expected_log.lock().unwrap(),
            vec![(MODIFIER, config), (PARTIAL, config), (SENDER, config)]
        );
    }
}

struct PendingPartial {
    seed: u8,
    recorded: Rc<Cell<Option<TransactionSignerConfig>>>,
    pending: Cell<bool>,
}

struct PendingOperation<'a> {
    backend: &'a PendingPartial,
    transaction: &'a Transaction,
}

impl Future for PendingOperation<'_> {
    type Output = Result<SignatureOutput, Error>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.backend.pending.replace(false) {
            context.waker().wake_by_ref();

            return Poll::Pending;
        }

        let bytes = self.transaction.message().bytes();
        Poll::Ready(Ok(SignatureOutput::new(
            self.backend.address(),
            bytes,
            signature(self.backend.seed, bytes),
        )))
    }
}

impl Signer for PendingPartial {
    fn address(&self) -> Address {
        address(self.seed)
    }
}

impl TransactionPartialSigner for PendingPartial {
    type Sign<'a> = PendingOperation<'a>;

    fn sign_transaction<'a>(
        &'a self,
        transaction: &'a Transaction,
        config: TransactionSignerConfig,
    ) -> Self::Sign<'a> {
        self.recorded.set(Some(config));
        PendingOperation {
            backend: self,
            transaction,
        }
    }
}

#[test]
fn rc_host_retains_config_across_a_real_pending_wake() {
    use std::task::Waker;

    let backend = PendingPartial {
        seed: PARTIAL,
        recorded: Rc::new(Cell::new(None)),
        pending: Cell::new(true),
    };
    let capability = SignerCapability::new(&backend);
    let signers = TransactionSigners::new(&[&capability]).unwrap();
    let transaction = support::transaction(backend.address(), 1);
    let config = TransactionSignerConfig {
        aborted: false,
        min_context_slot: Some(0),
    };
    let mut future = Box::pin(signers.sign(&transaction, config));
    let mut context = Context::from_waker(Waker::noop());

    assert!(future.as_mut().poll(&mut context).is_pending());
    assert_eq!(backend.recorded.get(), Some(config));
    assert!(future.as_mut().poll(&mut context).is_ready());
    assert_eq!(backend.recorded.get(), Some(config));
}

#[tokio::test(flavor = "current_thread")]
async fn aborted_transaction_config_precedes_validation_and_effects() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let partial = LocalTransactionBackend::new(PARTIAL, &log);
    let p = SignerCapability::new(&partial);
    let signers = TransactionSigners::new(&[&p]).unwrap();
    let transaction = support::transaction(partial.address(), 1);
    let aborted = TransactionSignerConfig {
        aborted: true,
        min_context_slot: Some(0),
    };

    assert_eq!(
        signers
            .sign_and_send::<LocalTransactionBackend>(&transaction, &[], aborted)
            .await,
        Err(Error::Cancelled)
    );
    assert_eq!(
        signers.sign(&transaction, aborted).await,
        Err(Error::Cancelled)
    );
    assert_eq!(
        signers
            .modify_and_partially_sign::<LocalTransactionBackend>(&transaction, &[], aborted)
            .await,
        Err(Error::Cancelled)
    );
    assert!(log.borrow().is_empty());
}

#[tokio::test(flavor = "current_thread")]
async fn batch_applies_one_call_config_to_every_transaction() {
    use solana_kit_signers::local::{Composition, Registration};

    let log = Rc::new(RefCell::new(Vec::new()));
    let partial = LocalTransactionBackend::new(PARTIAL, &log);
    let p = SignerCapability::new(&partial);
    let composition = Composition::new(Registration::partial(&p)).unwrap();
    let transaction = support::transaction(partial.address(), 1);
    let config = TransactionSignerConfig {
        aborted: false,
        min_context_slot: Some(u64::MAX),
    };
    let results = composition
        .sign_batch(&[transaction.clone(), transaction], config)
        .await
        .unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(*log.borrow(), [(PARTIAL, config), (PARTIAL, config)]);
}

struct MessageBackend {
    seed: u8,
    log: Rc<RefCell<Vec<SignerConfig>>>,
}

impl Signer for MessageBackend {
    fn address(&self) -> Address {
        address(self.seed)
    }
}

impl MessageModifyingSigner for MessageBackend {
    type Modify<'a> = Ready<Result<Vec<SignableMessage>, MessageError>>;

    fn modify_messages<'a>(
        &'a self,
        messages: &'a [SignableMessage],
        config: SignerConfig,
    ) -> Self::Modify<'a> {
        self.log.borrow_mut().push(config);
        ready(
            messages
                .iter()
                .map(|message| {
                    SignableMessage::with_signatures(
                        message.content(),
                        [(self.address(), signature(self.seed, message.content()))],
                    )
                })
                .collect::<Result<Vec<_>, _>>(),
        )
    }
}

impl MessagePartialSigner for MessageBackend {
    type Sign<'a> = Ready<Result<Vec<MessageSignatureOutput>, MessageError>>;

    fn sign_messages<'a>(
        &'a self,
        messages: &'a [SignableMessage],
        config: SignerConfig,
    ) -> Self::Sign<'a> {
        self.log.borrow_mut().push(config);
        ready(
            messages
                .iter()
                .map(|message| {
                    MessageSignatureOutput::new(
                        message.content(),
                        [(self.address(), signature(self.seed, message.content()))],
                    )
                })
                .collect::<Result<Vec<_>, _>>(),
        )
    }
}

#[tokio::test(flavor = "current_thread")]
async fn message_roles_share_config_and_aborted_precedes_every_effect() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let modifier = MessageBackend {
        seed: MODIFIER,
        log: Rc::clone(&log),
    };
    let partial = MessageBackend {
        seed: PARTIAL,
        log: Rc::clone(&log),
    };
    let m = SignerCapability::new(&modifier);
    let p = SignerCapability::new(&partial);
    let pipeline = MessageComposition::new(vec![
        MessageRegistration::modifying(&m),
        MessageRegistration::partial(&p),
    ])
    .unwrap();
    let input = [SignableMessage::new(b"one"), SignableMessage::new(b"two")];
    let config = SignerConfig { aborted: false };

    pipeline.sign(&input, config).await.unwrap();
    assert_eq!(*log.borrow(), [config, config]);

    log.borrow_mut().clear();
    let aborted = SignerConfig { aborted: true };
    assert_eq!(
        pipeline.sign(&[], aborted).await,
        Err(MessageError::Cancelled)
    );
    assert_eq!(
        pipeline
            .sign(&vec![SignableMessage::new([]); 257], aborted)
            .await,
        Err(MessageError::Cancelled)
    );
    let invalid =
        SignableMessage::with_signatures(b"x", [(address(99), Signature::from_bytes([0; 64]))])
            .unwrap();
    assert_eq!(
        pipeline.sign(&[invalid], aborted).await,
        Err(MessageError::Cancelled)
    );
    assert!(log.borrow().is_empty());
}

struct SingularBackend {
    seed: u8,
    calls: Rc<RefCell<Vec<SignerConfig>>>,
}

impl Signer for SingularBackend {
    fn address(&self) -> Address {
        address(self.seed)
    }
}

impl MessageSigner for SingularBackend {
    type Sign<'a> = Ready<Result<SignatureOutput, Error>>;

    fn sign_message<'a>(&'a self, bytes: &'a [u8], config: SignerConfig) -> Self::Sign<'a> {
        self.calls.borrow_mut().push(config);
        ready(Ok(SignatureOutput::new(
            self.address(),
            bytes,
            signature(self.seed, bytes),
        )))
    }
}

#[tokio::test(flavor = "current_thread")]
async fn singular_batch_forwards_one_config_and_honors_aborted_directly() {
    let backend = SingularBackend {
        seed: PARTIAL,
        calls: Rc::new(RefCell::new(Vec::new())),
    };
    let adapter = SingularMessageSigner(backend);
    let handle = SignerCapability::new(&adapter);
    let pipeline = MessageComposition::new(vec![MessageRegistration::partial(&handle)]).unwrap();
    let config = SignerConfig { aborted: false };
    let input = [SignableMessage::new(b"one"), SignableMessage::new(b"two")];

    pipeline.sign(&input, config).await.unwrap();
    assert_eq!(*adapter.0.calls.borrow(), [config, config]);

    let before = adapter.0.calls.borrow().len();
    let aborted = SignerConfig { aborted: true };
    assert_eq!(
        MessagePartialSigner::sign_messages(&adapter, &input, aborted).await,
        Err(MessageError::Cancelled)
    );
    assert_eq!(adapter.0.calls.borrow().len(), before);
}

#[cfg(feature = "crypto")]
#[tokio::test]
async fn direct_keypair_calls_honor_aborted_before_signing() {
    use solana_kit_core::crypto::Keypair;

    let keypair = Keypair::from_seed(&[PARTIAL; 32]);
    let aborted = SignerConfig { aborted: true };
    assert_eq!(
        keypair.sign_message(b"exact bytes", aborted).await,
        Err(Error::Cancelled)
    );
    assert_eq!(
        keypair
            .sign_messages(&[SignableMessage::new(b"exact bytes")], aborted)
            .await,
        Err(MessageError::Cancelled)
    );
    let transaction = support::transaction(keypair.address(), 1);
    assert_eq!(
        keypair
            .sign_transaction(
                &transaction,
                TransactionSignerConfig {
                    aborted: true,
                    min_context_slot: Some(0),
                }
            )
            .await,
        Err(Error::Cancelled)
    );
}

#[test]
fn configuration_types_are_small_explicit_values() {
    assert_eq!(SignerConfig::default(), SignerConfig { aborted: false });
    assert_eq!(
        TransactionSignerConfig::default(),
        TransactionSignerConfig {
            aborted: false,
            min_context_slot: None
        }
    );
    assert_eq!(
        TransactionSignerConfig {
            aborted: true,
            min_context_slot: Some(u64::MAX)
        },
        TransactionSignerConfig {
            aborted: true,
            min_context_slot: Some(u64::MAX)
        }
    );
}
