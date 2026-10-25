#![cfg(all(feature = "wallet", target_arch = "wasm32", target_os = "unknown"))]

use js_sys::{Function, Object, Reflect, Uint8Array};
use solana_kit_browser::{
    BrowserCause,
    wallet::{BrowserWalletPanel, Connection, WalletController, WalletDiscovery, WalletFailure},
};
use solana_kit_core::{
    Hash,
    crypto::Keypair,
    transaction::{Lifetime, Message, Transaction, Version},
};
use solana_kit_signers::{
    MessageSigner, Signer, SignerCapability, TransactionPartialSigner, local,
};
use solana_kit_wallet::ui::{PanelAction, PanelState};
use solana_kit_wallet::{Wallet, WalletErrorCode, WalletRegistry};
use std::{
    error::Error,
    future::Future,
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};
use wasm_bindgen::{JsCast, JsValue, closure::Closure, prelude::wasm_bindgen};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[path = "wallet/corrections.rs"]
mod corrections;

#[wasm_bindgen(module = "/tests/wallet_fixture.js")]
extern "C" {
    fn fixture(
        address: &str,
        public_key: &Uint8Array,
        sign_message: &Function,
        sign_transaction: &Function,
    ) -> JsValue;
}

struct Harness {
    fixture: JsValue,
    keypair: Rc<Keypair>,
}

impl Harness {
    fn new() -> Self {
        let keypair = Rc::new(Keypair::from_seed(&[17; 32]));
        let message_key = keypair.clone();
        let sign_message =
            Closure::<dyn FnMut(Uint8Array) -> Uint8Array>::new(move |bytes: Uint8Array| {
                Uint8Array::from(message_key.sign(&bytes.to_vec()).as_bytes().as_slice())
            })
            .into_js_value()
            .dyn_into::<Function>()
            .unwrap();
        let transaction_key = keypair.clone();
        let sign_transaction =
            Closure::<dyn FnMut(Uint8Array) -> Uint8Array>::new(move |bytes: Uint8Array| {
                let transaction = Transaction::decode(&bytes.to_vec())
                    .unwrap()
                    .sign(&transaction_key)
                    .unwrap();
                Uint8Array::from(transaction.encode(false).unwrap().as_slice())
            })
            .into_js_value()
            .dyn_into::<Function>()
            .unwrap();
        let fixture = fixture(
            &keypair.address().to_string(),
            &Uint8Array::from(keypair.address().as_bytes().as_slice()),
            &sign_message,
            &sign_transaction,
        );
        Self { fixture, keypair }
    }

    fn call(&self, method: &str) {
        Reflect::get(&self.fixture, &method.into())
            .unwrap()
            .dyn_into::<Function>()
            .unwrap()
            .call0(&self.fixture)
            .unwrap();
    }

    fn runtime(&self, key: &str, value: &JsValue) {
        let runtime = Reflect::get(&self.fixture, &"runtime".into()).unwrap();
        assert!(Reflect::set(&runtime, &key.into(), value).unwrap());
    }

    fn discovery(&self) -> Rc<WalletDiscovery> {
        Rc::new(WalletDiscovery::start(WalletRegistry::new()).unwrap())
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        self.call("close");
    }
}

fn lifetime() -> Lifetime {
    Lifetime::Blockhash {
        blockhash: Hash::from_bytes([9; 32]),
        last_valid_block_height: 100,
    }
}

fn transaction(keypair: &Keypair) -> Transaction {
    Transaction::new(
        Message::compile(Version::Legacy, keypair.address(), lifetime(), &[], &[]).unwrap(),
    )
}

#[wasm_bindgen_test]
async fn public_discovery_controller_and_typed_signatures() {
    let harness = Harness::new();
    let discovery = harness.discovery();
    assert_eq!(discovery.wallets().len(), 1);
    harness.call("announce");
    assert_eq!(discovery.wallets().len(), 1);
    let wallet = discovery.wallets().remove(0);
    let controller = WalletController::new(discovery.clone(), "solana:localnet".into()).unwrap();
    controller
        .connect(wallet.clone(), false)
        .unwrap()
        .await
        .unwrap();
    assert!(matches!(
        controller.state().connection,
        Connection::Connected { .. }
    ));
    let signer = controller.signer().unwrap();
    let signed = MessageSigner::<WalletFailure>::sign_message(&signer, b"exact message")
        .await
        .unwrap();
    signed.validate(signer.address(), b"exact message").unwrap();
    let input = transaction(&harness.keypair);
    let output = TransactionPartialSigner::<WalletFailure>::sign_transaction(&signer, &input)
        .await
        .unwrap();
    output
        .apply(signer.address(), &input)
        .unwrap()
        .verify()
        .unwrap();
    assert_eq!(wallet.accounts().len(), 1);
    assert!(discovery.take_failures().is_empty());
    harness.call("close");
    assert!(discovery.wallets().is_empty());
    assert!(matches!(
        controller.state().connection,
        Connection::Disconnected
    ));
    assert_eq!(
        MessageSigner::<WalletFailure>::sign_message(&signer, b"stale")
            .await
            .unwrap_err()
            .code(),
        WalletErrorCode::Disconnected
    );
}

#[wasm_bindgen_test]
async fn original_js_rejection_and_account_revocation_remain_visible() {
    let harness = Harness::new();
    let discovery = harness.discovery();
    let controller = WalletController::new(discovery.clone(), "solana:localnet".into()).unwrap();
    controller
        .connect(discovery.wallets().remove(0), false)
        .unwrap()
        .await
        .unwrap();
    let signer = controller.signer().unwrap();
    let original = Object::new();
    Reflect::set(&original, &"private".into(), &"never print this".into()).unwrap();
    harness.runtime("mode", &"reject".into());
    harness.runtime("error", &original);
    let error = MessageSigner::<WalletFailure>::sign_message(&signer, b"message")
        .await
        .unwrap_err();
    let cause = error
        .source()
        .unwrap()
        .downcast_ref::<BrowserCause>()
        .unwrap();
    assert!(Object::is(cause.value(), &original));
    assert!(!format!("{error:?}: {error}").contains("never print"));
    harness.call("revoke");
    assert!(matches!(
        controller.state().connection,
        Connection::Disconnected
    ));
    harness.runtime("mode", &"ready".into());
    harness.call("restore");
    assert_eq!(
        MessageSigner::<WalletFailure>::sign_message(&signer, b"stale")
            .await
            .unwrap_err()
            .code(),
        WalletErrorCode::Disconnected
    );
}

#[wasm_bindgen_test]
async fn mixed_local_and_wallet_pipeline_preserves_backend_causes() {
    let harness = Harness::new();
    let discovery = harness.discovery();
    let controller = WalletController::new(discovery.clone(), "solana:localnet".into()).unwrap();
    controller
        .connect(discovery.wallets().remove(0), false)
        .unwrap()
        .await
        .unwrap();
    let signer = controller.signer().unwrap();
    let wallet = SignerCapability::new(&signer);
    let keypair = Keypair::from_seed(&[29; 32]);
    let local_key = SignerCapability::new(&keypair);
    let composition = local::Composition::<WalletFailure>::with_failure(vec![
        local::Registration::partial(&wallet),
        local::Registration::partial_with(&local_key, WalletFailure::from),
    ])
    .unwrap();
    let instruction = solana_kit_core::Instruction {
        program_address: solana_kit_core::Address::from_bytes([3; 32]),
        accounts: vec![solana_kit_core::AccountMeta {
            address: keypair.address(),
            role: solana_kit_core::AccountRole::ReadonlySigner,
        }],
        data: vec![],
    };
    let input = Transaction::new(
        Message::compile(
            Version::Legacy,
            signer.address(),
            lifetime(),
            &[instruction],
            &[],
        )
        .unwrap(),
    );
    composition.sign(&input).await.unwrap().verify().unwrap();
    let original = Object::new();
    harness.runtime("mode", &"reject".into());
    harness.runtime("error", &original);
    let error = composition.sign(&input).await.unwrap_err();
    assert!(Object::is(
        error
            .source()
            .unwrap()
            .downcast_ref::<BrowserCause>()
            .unwrap()
            .value(),
        &original
    ));
}

struct Noop;
impl Wake for Noop {
    fn wake(self: Arc<Self>) {}
}

#[wasm_bindgen_test]
fn dropped_unpolled_and_pending_operations_reset_authority() {
    let harness = Harness::new();
    let discovery = harness.discovery();
    let controller = WalletController::new(discovery.clone(), "solana:localnet".into()).unwrap();
    let pending = controller
        .connect(discovery.wallets().remove(0), false)
        .unwrap();
    drop(pending);
    assert!(matches!(
        controller.state().connection,
        Connection::Disconnected
    ));
    let pending = controller
        .connect(discovery.wallets().remove(0), false)
        .unwrap();
    controller.dispose();
    let waker = Waker::from(Arc::new(Noop));
    let mut context = Context::from_waker(&waker);
    let mut pending = pending;
    assert!(matches!(
        Pin::as_mut(&mut pending).poll(&mut context),
        Poll::Ready(Err(_))
    ));
}

#[derive(Default)]
struct Counter(std::sync::atomic::AtomicUsize);
impl Wake for Counter {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

#[wasm_bindgen_test]
async fn pending_wallet_promise_is_woken_and_rejected_when_accounts_are_revoked() {
    let harness = Harness::new();
    let discovery = harness.discovery();
    let controller = WalletController::new(discovery.clone(), "solana:localnet".into()).unwrap();
    controller
        .connect(discovery.wallets().remove(0), false)
        .unwrap()
        .await
        .unwrap();
    let signer = controller.signer().unwrap();
    harness.runtime("mode", &"pending".into());
    let counter = Arc::new(Counter::default());
    let waker = Waker::from(counter.clone());
    let mut context = Context::from_waker(&waker);
    let mut operation = MessageSigner::<WalletFailure>::sign_message(&signer, b"pending");
    assert!(matches!(
        operation.as_mut().poll(&mut context),
        Poll::Pending
    ));
    harness.call("revoke");
    assert!(counter.0.load(std::sync::atomic::Ordering::SeqCst) > 0);
    let Poll::Ready(Err(error)) = operation.as_mut().poll(&mut context) else {
        panic!("revocation did not cancel caller interest");
    };
    assert_eq!(error.code(), WalletErrorCode::Disconnected);
    drop(operation);
    harness.call("settle");
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&JsValue::UNDEFINED))
        .await
        .unwrap();
    assert!(!controller.state().operating);
}

#[wasm_bindgen_test]
async fn malformed_results_and_replaced_feature_versions_are_rejected() {
    let harness = Harness::new();
    let discovery = harness.discovery();
    let controller = WalletController::new(discovery.clone(), "solana:localnet".into()).unwrap();
    controller
        .connect(discovery.wallets().remove(0), false)
        .unwrap()
        .await
        .unwrap();
    let signer = controller.signer().unwrap();
    harness.runtime("mode", &"empty".into());
    let failure = MessageSigner::<WalletFailure>::sign_message(&signer, b"message")
        .await
        .unwrap_err();
    assert_eq!(failure.code(), WalletErrorCode::InvalidResponse);
    assert!(!controller.state().operating);
    harness.runtime("mode", &"ready".into());
    harness.call("unsupported");
    assert_eq!(
        MessageSigner::<WalletFailure>::sign_message(&signer, b"stale")
            .await
            .unwrap_err()
            .code(),
        WalletErrorCode::Disconnected
    );
    let current = controller.signer().unwrap();
    assert_eq!(
        MessageSigner::<WalletFailure>::sign_message(&current, b"current")
            .await
            .unwrap_err()
            .code(),
        WalletErrorCode::UnsupportedFeature
    );
}

#[wasm_bindgen_test]
fn dropping_discovery_removes_window_and_wallet_event_subscriptions() {
    let harness = Harness::new();
    let registry = WalletRegistry::new();
    let discovery = WalletDiscovery::start(registry.clone()).unwrap();
    assert_eq!(registry.wallets().len(), 1);
    drop(discovery);
    assert!(registry.wallets().is_empty());
    harness.call("announce");
    assert!(registry.wallets().is_empty());
    let count = Reflect::get(&harness.fixture, &"listeners".into())
        .unwrap()
        .dyn_into::<Function>()
        .unwrap()
        .call0(&harness.fixture)
        .unwrap();
    assert_eq!(count.as_f64(), Some(0.0));
}

#[wasm_bindgen_test]
fn unsubscribe_failure_is_original_and_registration_authority_is_revoked() {
    let harness = Harness::new();
    let registry = WalletRegistry::new();
    let mut discovery = WalletDiscovery::start(registry.clone()).unwrap();
    let wallet = discovery.wallets().remove(0);
    let original = Object::new();
    harness.runtime("offFailure", &true.into());
    harness.runtime("error", &original);
    discovery.close();
    assert!(registry.wallets().is_empty());
    assert!(!wallet.is_available());
    let failures = discovery.take_failures();
    assert_eq!(failures.len(), 1);
    let cause = failures[0]
        .source()
        .unwrap()
        .downcast_ref::<BrowserCause>()
        .unwrap();
    assert!(Object::is(cause.value(), &original));
    assert!(discovery.take_failures().is_empty());
    harness.runtime("offFailure", &false.into());
}

#[wasm_bindgen_test]
async fn panel_uses_actual_metadata_and_cancel_invalidates_pending_signing() {
    let harness = Harness::new();
    let mut panel = BrowserWalletPanel::new(harness.discovery(), "solana:localnet".into()).unwrap();
    let snapshot = panel.snapshot().unwrap();
    assert_eq!(snapshot.wallets[0].name, "Kit protocol test wallet");
    assert!(snapshot.wallets[0].enabled);
    panel
        .dispatch(PanelAction::Connect(snapshot.wallets[0].key))
        .unwrap()
        .await
        .unwrap();

    let snapshot = panel.snapshot().unwrap();
    assert!(matches!(snapshot.state, PanelState::Connected { .. }));
    assert_eq!(snapshot.accounts.len(), 1);
    let signer = panel.controller().signer().unwrap();
    harness.runtime("mode", &"pending".into());
    let waker = Waker::from(Arc::new(Noop));
    let mut context = Context::from_waker(&waker);
    let mut operation = MessageSigner::<WalletFailure>::sign_message(&signer, b"awaiting approval");
    assert!(operation.as_mut().poll(&mut context).is_pending());

    panel.dispatch(PanelAction::Cancel).unwrap().await.unwrap();
    assert!(matches!(
        panel.snapshot().unwrap().state,
        PanelState::Disconnected
    ));
    let Poll::Ready(Err(error)) = operation.as_mut().poll(&mut context) else {
        panic!("cancel did not release pending signing");
    };
    assert_eq!(error.code(), WalletErrorCode::Disconnected);
    drop(operation);
    harness.call("settle");
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(&JsValue::UNDEFINED))
        .await
        .unwrap();
    assert!(matches!(
        panel.snapshot().unwrap().state,
        PanelState::Disconnected
    ));

    harness.runtime("mode", &"ready".into());
    panel
        .dispatch(PanelAction::Connect(snapshot.wallets[0].key))
        .unwrap()
        .await
        .unwrap();
    let disconnect = panel.dispatch(PanelAction::Disconnect).unwrap();
    assert!(matches!(
        panel.snapshot().unwrap().state,
        PanelState::Disconnecting
    ));
    drop(disconnect);
    assert!(matches!(
        panel.snapshot().unwrap().state,
        PanelState::Disconnected
    ));
}

#[wasm_bindgen_test]
fn panel_cannot_reuse_a_removed_wallet_choice() {
    let harness = Harness::new();
    let mut panel = BrowserWalletPanel::new(harness.discovery(), "solana:localnet".into()).unwrap();
    let choice = panel.snapshot().unwrap().wallets[0].key;
    harness.call("close");
    assert!(panel.snapshot().unwrap().wallets.is_empty());
    assert!(panel.dispatch(PanelAction::Connect(choice)).is_err());
}
