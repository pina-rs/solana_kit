//! Public wallet calls exercise primary identifiers and invalidation cleanup.
use super::*;
use solana_kit_client::{
    confirmation::{self, Confirmation, Event, Observer, Policy},
    methods::Commitment,
    subscriptions::SignatureInfo,
};
use solana_kit_core::{AccountMeta, AccountRole, Address, Instruction, Signature};
use solana_kit_signers::TransactionSendingSigner;
use std::future::{Ready, pending, ready};

#[wasm_bindgen_test]
fn caller_pipeline_errors_keep_their_original_cause_and_request_classification() {
    use solana_kit_signers::Error as PipelineError;

    for error in [
        PipelineError::TooManySigners,
        PipelineError::ConflictingSigner,
        PipelineError::MissingSender,
        PipelineError::MultipleSenders,
    ] {
        let failure = WalletFailure::from(error);
        assert_eq!(failure.code(), WalletErrorCode::InvalidRequest);
        assert!(failure.source().unwrap().is::<PipelineError>());
    }

    for error in [PipelineError::ForeignSigner, PipelineError::ChangedSnapshot] {
        let failure = WalletFailure::from(error);
        assert_eq!(failure.code(), WalletErrorCode::InvalidResponse);
        assert!(failure.source().unwrap().is::<PipelineError>());
    }
}

fn listeners(harness: &Harness) -> f64 {
    Reflect::get(&harness.fixture, &"listeners".into())
        .unwrap()
        .dyn_into::<Function>()
        .unwrap()
        .call0(&harness.fixture)
        .unwrap()
        .as_f64()
        .unwrap()
}

fn off_attempts(harness: &Harness) -> f64 {
    let runtime = Reflect::get(&harness.fixture, &"runtime".into()).unwrap();
    Reflect::get(&runtime, &"offAttempts".into())
        .unwrap()
        .as_f64()
        .unwrap()
}

#[wasm_bindgen_test]
fn invalid_change_releases_listener_once_on_unregister_close_and_drop() {
    for cleanup in 0..3 {
        let harness = Harness::new();
        let mut discovery = WalletDiscovery::start(WalletRegistry::new()).unwrap();
        let wallet = discovery.wallets().remove(0);
        harness.call("malformed");
        assert!(!wallet.is_available());
        assert_eq!(listeners(&harness), 1.0);
        assert_eq!(discovery.take_failures().len(), 1);

        match cleanup {
            0 => harness.call("close"),
            1 => discovery.close(),
            _ => {
                drop(discovery);
                assert_eq!(listeners(&harness), 0.0);
                assert_eq!(off_attempts(&harness), 1.0);
                continue;
            }
        }

        discovery.close();
        drop(discovery);
        assert_eq!(listeners(&harness), 0.0);
        assert_eq!(off_attempts(&harness), 1.0);
    }
}

#[wasm_bindgen_test]
fn invalid_change_preserves_metadata_and_original_unsubscribe_failures() {
    let harness = Harness::new();
    let mut discovery = WalletDiscovery::start(WalletRegistry::new()).unwrap();
    let wallet = discovery.wallets().remove(0);
    let original = Object::new();
    harness.runtime("offFailure", &true.into());
    harness.runtime("error", &original);
    harness.call("malformed");

    discovery.close();
    discovery.close();
    let failures = discovery.take_failures();
    assert!(!wallet.is_available());
    assert_eq!(failures.len(), 2);
    assert_eq!(failures[0].code(), WalletErrorCode::InvalidResponse);

    let cause = failures[1]
        .source()
        .unwrap()
        .downcast_ref::<BrowserCause>()
        .unwrap();
    assert!(Object::is(cause.value(), &original));
    assert_eq!(off_attempts(&harness), 1.0);
    // A throwing external off cannot promise remote listener removal.
    assert_eq!(listeners(&harness), 1.0);
    harness.runtime("offFailure", &false.into());
}

struct Confirmed(Signature);

impl Observer for Confirmed {
    type Next<'a> = Ready<Result<Event, confirmation::Failure>>;
    type Close<'a> = Ready<Result<(), confirmation::Failure>>;

    fn next(&mut self) -> Self::Next<'_> {
        ready(Ok(Event::Signature {
            signature: self.0,
            slot: 1,
            commitment: Commitment::Confirmed,
            value: SignatureInfo::Processed { err: None },
        }))
    }

    fn close(&mut self) -> Self::Close<'_> {
        ready(Ok(()))
    }
}

fn secondary_transaction(payer: &Keypair, wallet: Address) -> Transaction {
    let instruction = Instruction {
        program_address: Address::from_bytes([3; 32]),
        accounts: vec![AccountMeta {
            address: wallet,
            role: AccountRole::ReadonlySigner,
        }],
        data: vec![],
    };
    let message = Message::compile(
        Version::Legacy,
        payer.address(),
        lifetime(),
        &[instruction],
        &[],
    )
    .unwrap();
    Transaction::new(message).sign(payer).unwrap()
}

async fn connected_controller(harness: &Harness) -> WalletController {
    let discovery = harness.discovery();
    let controller = WalletController::new(discovery.clone(), "solana:localnet".into()).unwrap();
    controller
        .connect(discovery.wallets().remove(0), false)
        .unwrap()
        .await
        .unwrap();
    controller
}

#[wasm_bindgen_test]
async fn secondary_wallet_returns_only_primary_identifier_to_confirmation() {
    let harness = Harness::new();
    let controller = connected_controller(&harness).await;
    let wallet = controller.signer().unwrap();
    let payer = Keypair::from_seed(&[31; 32]);
    let transaction = secondary_transaction(&payer, wallet.address());
    let primary = payer.sign(transaction.message().bytes());
    let secondary = harness.keypair.sign(transaction.message().bytes());

    assert_ne!(primary, secondary);

    let identifier =
        TransactionSendingSigner::<WalletFailure>::send_transaction(&wallet, &transaction)
            .await
            .unwrap();
    assert_eq!(identifier, primary);

    let confirmation = Confirmation::new(identifier, lifetime(), Policy::default()).unwrap();
    confirmation::wait(confirmation, Confirmed(primary), pending())
        .await
        .unwrap();

    harness.runtime(
        "submissionSignature",
        &Uint8Array::from(secondary.as_bytes().as_slice()),
    );
    assert_eq!(
        TransactionSendingSigner::<WalletFailure>::send_transaction(&wallet, &transaction)
            .await
            .unwrap_err()
            .code(),
        WalletErrorCode::InvalidResponse
    );
}

#[wasm_bindgen_test]
async fn submission_supplies_missing_payer_and_rejects_a_mismatched_payer_signature() {
    let harness = Harness::new();
    let controller = connected_controller(&harness).await;
    let wallet = controller.signer().unwrap();
    let payer = Keypair::from_seed(&[31; 32]);
    let transaction = secondary_transaction(&payer, wallet.address());
    let primary = payer.sign(transaction.message().bytes());
    let secondary = harness.keypair.sign(transaction.message().bytes());
    let unsigned = Transaction::new(transaction.message().clone());

    assert_eq!(
        TransactionSendingSigner::<WalletFailure>::send_transaction(&wallet, &unsigned)
            .await
            .unwrap(),
        primary
    );
    let mismatched = transaction
        .with_signature(payer.address(), transaction.message().bytes(), secondary)
        .unwrap();
    assert_eq!(
        TransactionSendingSigner::<WalletFailure>::send_transaction(&wallet, &mismatched)
            .await
            .unwrap_err()
            .code(),
        WalletErrorCode::InvalidResponse
    );
}
