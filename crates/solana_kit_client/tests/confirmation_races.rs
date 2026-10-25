#![cfg(feature = "confirmation-factories")]
//! A lifetime observation cannot outrun the final signature lookup.
use solana_kit_client::{
    Client, Limits, Reply, Request, Transport, TransportError,
    confirmation::{Confirmation, Failure, Policy, reconcile},
};
use solana_kit_core::{
    Address, Hash, Signature,
    transaction::{DurableNonce, Lifetime},
};
use std::{
    collections::VecDeque,
    future::{Ready, ready},
    sync::Mutex,
};

struct Rpc(Mutex<VecDeque<(&'static str, serde_json::Value)>>);
const OBSERVED_SLOT: u64 = 20;
const STALE_SLOT: u64 = 19;
const TRANSACTION_SLOT: u64 = 15;
const MIN_SLOT: u64 = 10;
const EXPIRED_HEIGHT: u64 = 12;
const LAST_VALID_HEIGHT: u64 = 11;
const EPOCH_SLOTS: u64 = 100;
impl Transport for Rpc {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let request: serde_json::Value = serde_json::from_slice(&request.body).unwrap();
        let (method, result) = self
            .0
            .lock()
            .unwrap()
            .pop_front()
            .expect("unexpected request");
        assert_eq!(request["method"], method);

        validate_config(method, &request);

        ready(Ok(Reply {
            status: 200,
            body: serde_json::to_vec(&serde_json::json!({
                "jsonrpc":"2.0", "id":request["id"], "result":result,
            }))
            .unwrap(),
        }))
    }
}
fn validate_config(method: &str, request: &serde_json::Value) {
    if method == "getSignatureStatuses" {
        assert_eq!(
            request["params"][1],
            serde_json::json!({"searchTransactionHistory":true})
        );
        return;
    }

    let index = usize::from(method != "getEpochInfo");
    let config = &request["params"][index];
    assert_eq!(config["commitment"], "confirmed");
    assert_eq!(config["minContextSlot"], MIN_SLOT);
}
fn status(slot: u64, value: serde_json::Value) -> serde_json::Value {
    serde_json::json!({"context":{"slot":slot},"value":[value]})
}
fn confirmed() -> serde_json::Value {
    serde_json::json!({
        "slot":TRANSACTION_SLOT,"confirmations":1,"err":null,
        "status":{"Ok":null},"confirmationStatus":"confirmed"
    })
}
fn lifetime(nonce: bool) -> Lifetime {
    if nonce {
        return Lifetime::DurableNonce(DurableNonce::new(
            Hash::from_bytes([1; 32]),
            Address::from_bytes([0; 32]),
            Address::from_bytes([0; 32]),
        ));
    }

    Lifetime::Blockhash {
        blockhash: Hash::from_bytes([1; 32]),
        last_valid_block_height: LAST_VALID_HEIGHT,
    }
}
fn run(
    nonce: bool,
    final_slot: u64,
    final_status: serde_json::Value,
) -> Option<Result<(), Failure>> {
    let method = if nonce {
        "getAccountInfo"
    } else {
        "getEpochInfo"
    };
    let invalid = if nonce {
        serde_json::json!({"context":{"slot":OBSERVED_SLOT},"value":null})
    } else {
        serde_json::json!({
            "absoluteSlot":OBSERVED_SLOT,"blockHeight":EXPIRED_HEIGHT,"epoch":0,
            "slotIndex":OBSERVED_SLOT,"slotsInEpoch":EPOCH_SLOTS,"transactionCount":null
        })
    };

    let rpc = Rpc(Mutex::new(VecDeque::from([
        (
            "getSignatureStatuses",
            status(MIN_SLOT, serde_json::Value::Null),
        ),
        (method, invalid),
        ("getSignatureStatuses", status(final_slot, final_status)),
    ])));
    let client = Client::new(rpc, Limits::default());
    let signature = Signature::from_bytes([0; 64]);
    let policy = Policy {
        min_context_slot: MIN_SLOT,
        ..Policy::default()
    };
    let events =
        futures::executor::block_on(reconcile(&client, signature, lifetime(nonce), policy))
            .unwrap();
    let mut reducer = Confirmation::new(signature, lifetime(nonce), policy).unwrap();

    events.into_iter().find_map(|event| reducer.observe(event))
}
#[cfg_attr(test, test)]
pub fn height_expiry_reconciles_confirmation_that_arrived_during_the_height_read() {
    assert!(matches!(
        run(false, OBSERVED_SLOT, confirmed()),
        Some(Ok(()))
    ));
}
#[cfg_attr(test, test)]
pub fn nonce_invalidation_reconciles_confirmation_that_arrived_during_the_account_read() {
    assert!(matches!(
        run(true, OBSERVED_SLOT, confirmed()),
        Some(Ok(()))
    ));
}
#[cfg_attr(test, test)]
pub fn stale_final_status_cannot_establish_height_or_nonce_expiry() {
    assert!(run(false, STALE_SLOT, serde_json::Value::Null).is_none());
    assert!(run(true, STALE_SLOT, serde_json::Value::Null).is_none());
}
#[cfg_attr(test, test)]
pub fn current_absent_status_still_reports_actual_height_and_nonce_expiry() {
    assert!(matches!(
        run(false, OBSERVED_SLOT, serde_json::Value::Null),
        Some(Err(Failure::BlockHeightExceeded {
            current: EXPIRED_HEIGHT,
            last_valid: LAST_VALID_HEIGHT
        }))
    ));
    assert!(matches!(
        run(true, OBSERVED_SLOT, serde_json::Value::Null),
        Some(Err(Failure::NonceAccountMissing(_)))
    ));
}
#[cfg_attr(test, test)]
pub fn final_execution_failure_is_preserved_before_lifetime_expiry() {
    let failed = serde_json::json!({
        "slot":TRANSACTION_SLOT,"confirmations":1,"err":"AccountNotFound",
        "status":{"Err":"AccountNotFound"},"confirmationStatus":"processed"
    });
    assert!(matches!(
        run(false, OBSERVED_SLOT, failed.clone()),
        Some(Err(Failure::Transaction(_)))
    ));
    assert!(matches!(
        run(true, OBSERVED_SLOT, failed),
        Some(Err(Failure::Transaction(_)))
    ));
}
