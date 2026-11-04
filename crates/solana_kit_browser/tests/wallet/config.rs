//! A real Wallet Standard fixture observes outgoing transaction options.
use super::*;
use js_sys::Array;
use solana_kit_signers::{
    MessageSigner, SignerConfig, TransactionModifyingSigner, TransactionSendingSigner,
    TransactionSignerConfig,
};

const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;
const ZERO_SLOT: u64 = 0;
const MODIFY_SLOT: u64 = 7;

fn requests(harness: &Harness) -> Vec<JsValue> {
    Reflect::get(&harness.fixture, &"requests".into())
        .unwrap()
        .dyn_into::<Function>()
        .unwrap()
        .call0(&harness.fixture)
        .unwrap()
        .dyn_into::<Array>()
        .unwrap()
        .iter()
        .collect()
}

fn runtime_calls(harness: &Harness) -> f64 {
    let runtime = Reflect::get(&harness.fixture, &"runtime".into()).unwrap();
    Reflect::get(&runtime, &"calls".into())
        .unwrap()
        .as_f64()
        .unwrap()
}

/// Reads exactly the first input's `options.minContextSlot` from each request.
fn slots(harness: &Harness) -> Vec<Option<f64>> {
    requests(harness)
        .iter()
        .map(|request| {
            let options = Reflect::get(request, &"options".into()).unwrap();
            let options = options.dyn_into::<Array>().unwrap();
            let options = options.get(0);

            if options.is_undefined() || options.is_null() {
                return None;
            }

            let slot = Reflect::get(&options, &"minContextSlot".into()).unwrap();

            if slot.is_undefined() {
                None
            } else {
                Some(slot.as_f64().unwrap())
            }
        })
        .collect()
}

fn features(harness: &Harness) -> Vec<String> {
    requests(harness)
        .iter()
        .map(|request| {
            Reflect::get(request, &"feature".into())
                .unwrap()
                .as_string()
                .unwrap()
        })
        .collect()
}

fn assert_aborted(failure: &WalletFailure) {
    assert_eq!(failure.code(), WalletErrorCode::Disconnected);
    assert_eq!(
        failure
            .source()
            .unwrap()
            .downcast_ref::<solana_kit_signers::Error>(),
        Some(&solana_kit_signers::Error::Cancelled)
    );
}

async fn harness() -> (Harness, WalletController) {
    let harness = Harness::new();
    let discovery = harness.discovery();
    let controller = WalletController::new(discovery.clone(), "solana:localnet".into()).unwrap();
    controller
        .connect(discovery.wallets().remove(0), false)
        .unwrap()
        .await
        .unwrap();
    (harness, controller)
}

#[wasm_bindgen_test]
async fn none_slot_is_omitted_from_a_partial_transaction_request() {
    let (harness, controller) = harness().await;
    let signer = controller.signer().unwrap();
    let transaction = transaction(&harness.keypair);
    let signed = TransactionPartialSigner::<WalletFailure>::sign_transaction(
        &signer,
        &transaction,
        TransactionSignerConfig::default(),
    )
    .await
    .unwrap();
    signed
        .apply(signer.address(), &transaction)
        .unwrap()
        .verify()
        .unwrap();

    assert_eq!(slots(&harness), [None]);
    assert_eq!(features(&harness), ["solana:signTransaction"]);
    assert_eq!(runtime_calls(&harness), 1.0);
}

#[wasm_bindgen_test]
async fn exact_safe_slots_reach_each_transaction_feature() {
    let (harness, controller) = harness().await;
    let signer = controller.signer().unwrap();
    let transaction = transaction(&harness.keypair);

    TransactionPartialSigner::<WalletFailure>::sign_transaction(
        &signer,
        &transaction,
        TransactionSignerConfig {
            aborted: false,
            min_context_slot: Some(ZERO_SLOT),
        },
    )
    .await
    .unwrap();
    TransactionPartialSigner::<WalletFailure>::sign_transaction(
        &signer,
        &transaction,
        TransactionSignerConfig {
            aborted: false,
            min_context_slot: Some(MAX_SAFE_INTEGER),
        },
    )
    .await
    .unwrap();
    TransactionModifyingSigner::<WalletFailure>::modify_transaction(
        &signer,
        &transaction,
        TransactionSignerConfig {
            aborted: false,
            min_context_slot: Some(MODIFY_SLOT),
        },
    )
    .await
    .unwrap();
    TransactionSendingSigner::<WalletFailure>::send_transaction(
        &signer,
        &transaction,
        TransactionSignerConfig {
            aborted: false,
            min_context_slot: Some(ZERO_SLOT),
        },
    )
    .await
    .unwrap();

    assert_eq!(
        slots(&harness),
        [
            Some(ZERO_SLOT as f64),
            Some(MAX_SAFE_INTEGER as f64),
            Some(MODIFY_SLOT as f64),
            Some(ZERO_SLOT as f64)
        ]
    );
    assert_eq!(
        features(&harness),
        [
            "solana:signTransaction",
            "solana:signTransaction",
            "solana:signTransaction",
            "solana:signAndSendTransaction"
        ]
    );
    assert_eq!(runtime_calls(&harness), 4.0);
}

#[wasm_bindgen_test]
async fn unsafe_slots_reject_before_a_wallet_request_or_guard() {
    let (harness, controller) = harness().await;
    let signer = controller.signer().unwrap();
    let transaction = transaction(&harness.keypair);
    let before = runtime_calls(&harness);

    for slot in [MAX_SAFE_INTEGER + 1, u64::MAX] {
        let failure = TransactionPartialSigner::<WalletFailure>::sign_transaction(
            &signer,
            &transaction,
            TransactionSignerConfig {
                aborted: false,
                min_context_slot: Some(slot),
            },
        )
        .await
        .unwrap_err();
        assert_eq!(failure.code(), WalletErrorCode::InvalidRequest);
    }

    let failure = TransactionSendingSigner::<WalletFailure>::send_transaction(
        &signer,
        &transaction,
        TransactionSignerConfig {
            aborted: false,
            min_context_slot: Some(MAX_SAFE_INTEGER + 1),
        },
    )
    .await
    .unwrap_err();
    assert_eq!(failure.code(), WalletErrorCode::InvalidRequest);
    assert!(requests(&harness).is_empty());
    assert_eq!(runtime_calls(&harness), before);
    assert!(!controller.state().operating);
}

#[wasm_bindgen_test]
async fn aborted_message_config_precedes_a_wallet_request() {
    let (harness, controller) = harness().await;
    let signer = controller.signer().unwrap();
    let before = runtime_calls(&harness);
    let failure = MessageSigner::<WalletFailure>::sign_message(
        &signer,
        b"aborted",
        SignerConfig { aborted: true },
    )
    .await
    .unwrap_err();

    assert_aborted(&failure);
    assert!(requests(&harness).is_empty());
    assert_eq!(runtime_calls(&harness), before);
    assert!(!controller.state().operating);
}

#[wasm_bindgen_test]
async fn aborted_transaction_configs_precede_wallet_requests() {
    let (harness, controller) = harness().await;
    let signer = controller.signer().unwrap();
    let transaction = transaction(&harness.keypair);
    let before = runtime_calls(&harness);

    let config = TransactionSignerConfig {
        aborted: true,
        min_context_slot: Some(ZERO_SLOT),
    };

    let partial =
        TransactionPartialSigner::<WalletFailure>::sign_transaction(&signer, &transaction, config)
            .await
            .unwrap_err();
    let modifying = TransactionModifyingSigner::<WalletFailure>::modify_transaction(
        &signer,
        &transaction,
        config,
    )
    .await
    .unwrap_err();
    let sending =
        TransactionSendingSigner::<WalletFailure>::send_transaction(&signer, &transaction, config)
            .await
            .unwrap_err();

    for failure in [partial, modifying, sending] {
        assert_aborted(&failure);
    }

    assert!(requests(&harness).is_empty());
    assert_eq!(runtime_calls(&harness), before);
    assert!(!controller.state().operating);
}
