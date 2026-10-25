#![cfg(feature = "confirmation-factories")]
//! Nonce observations come from typed RPC account authentication.
use base64::{Engine, engine::general_purpose::STANDARD};
use solana_kit_client::{
    Client, Limits, Reply, Request, Transport, TransportError,
    confirmation::{Event, FactoryFailure, NonceError, Policy, reconcile},
};
use solana_kit_core::{
    Address, Hash, Signature,
    codecs::Encoder,
    transaction::{DurableNonce, Lifetime},
};
use solana_kit_system::{Nonce, NonceCodec, NonceData, NonceState, NonceVersion};
use std::future::{Ready, ready};

const SLOT: u64 = 10;
const NONCE_FEE: u64 = 5_000;
struct Rpc(serde_json::Value);
impl Transport for Rpc {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let request: serde_json::Value = serde_json::from_slice(&request.body).unwrap();

        let result = match request["method"].as_str().unwrap() {
            "getSignatureStatuses" => serde_json::json!({"context":{"slot":SLOT},"value":[null]}),
            "getAccountInfo" => {
                assert_eq!(request["params"][1]["commitment"], "confirmed");
                assert_eq!(request["params"][1]["minContextSlot"], 0);
                serde_json::json!({"context":{"slot":SLOT},"value":self.0})
            }
            _ => panic!("unexpected RPC operation"),
        };

        ready(Ok(Reply {
            status: 200,
            body: serde_json::to_vec(&serde_json::json!({
                "jsonrpc":"2.0","id":request["id"],"result":result,
            }))
            .unwrap(),
        }))
    }
}
fn account(bytes: &[u8], owner: Address) -> serde_json::Value {
    serde_json::json!({"owner":owner.to_string(),"executable":false,"lamports":0,
        "rentEpoch":0,"space":bytes.len(),"data":[STANDARD.encode(bytes),"base64"]})
}
fn encoded(state: NonceState) -> Vec<u8> {
    NonceCodec
        .encode(
            &Nonce {
                version: NonceVersion::Current,
                state,
            },
            solana_kit_system::NONCE_SIZE,
        )
        .unwrap()
}
fn observe(
    value: serde_json::Value,
) -> Result<Vec<Event>, FactoryFailure<std::convert::Infallible>> {
    let address = Address::from_bytes([0; 32]);
    let lifetime = Lifetime::DurableNonce(DurableNonce::new(
        Hash::from_bytes([1; 32]),
        address,
        address,
    ));
    futures::executor::block_on(reconcile(
        &Client::new(Rpc(value), Limits::default()),
        Signature::from_bytes([0; 64]),
        lifetime,
        Policy::default(),
    ))
}
#[test]
fn current_initialized_nonce_and_missing_account_preserve_real_context() {
    let bytes = encoded(NonceState::Initialized(NonceData {
        authority: Address::from_bytes([0; 32]),
        blockhash: Hash::from_bytes([2; 32]),
        lamports_per_signature: NONCE_FEE,
    }));
    let events = observe(account(&bytes, solana_kit_system::SYSTEM_PROGRAM)).unwrap();
    assert!(
        matches!(events.last(), Some(Event::Nonce {slot: SLOT, token: Some(token), ..})
        if *token == Hash::from_bytes([2; 32]))
    );
    assert!(matches!(
        observe(serde_json::Value::Null).unwrap().last(),
        Some(Event::Nonce { token: None, .. })
    ));
}
#[test]
fn wrong_owner_uninitialized_and_truncated_nonce_are_authentication_failures() {
    let uninitialized = encoded(NonceState::Uninitialized([0; 72]));
    assert!(matches!(
        observe(account(&uninitialized, Address::from_bytes([1; 32]))),
        Err(FactoryFailure::Nonce(NonceError::Account(_)))
    ));
    assert!(matches!(
        observe(account(&uninitialized, solana_kit_system::SYSTEM_PROGRAM)),
        Err(FactoryFailure::Nonce(NonceError::Unusable(_)))
    ));
    assert!(matches!(
        observe(account(
            &uninitialized[..solana_kit_system::NONCE_SIZE - 1],
            solana_kit_system::SYSTEM_PROGRAM
        )),
        Err(FactoryFailure::Nonce(NonceError::Account(_)))
    ));
}

struct Timer;
impl solana_kit_client::subscriptions::Clock for Timer {
    type Sleep<'a> = futures::future::Either<
        Ready<Result<(), solana_kit_client::subscriptions::ClockError>>,
        std::future::Pending<Result<(), solana_kit_client::subscriptions::ClockError>>,
    >;
    fn now(&self) -> u64 {
        0
    }
    fn sleep_until(&self, deadline: u64) -> Self::Sleep<'_> {
        if deadline == 0 {
            futures::future::Either::Left(ready(Ok(())))
        } else {
            futures::future::Either::Right(std::future::pending())
        }
    }
}
struct ImmediateSocket {
    frames: std::collections::VecDeque<String>,
    closed: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}
impl solana_kit_client::subscriptions::TextSocket for ImmediateSocket {
    type Error = ();
    fn start_send(&mut self, text: &str) -> Result<(), ()> {
        let request: serde_json::Value = serde_json::from_str(text).unwrap();
        self.frames.push_back(
            serde_json::json!({"jsonrpc":"2.0","id":request["id"],"result":1}).to_string(),
        );
        self.frames.push_back(
            serde_json::json!({"jsonrpc":"2.0","method":"signatureNotification",
            "params":{"subscription":1,"result":{"context":{"slot":SLOT},"value":{"err":null}}}})
            .to_string(),
        );
        Ok(())
    }
    fn poll_flush(&mut self, _: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), ()>> {
        std::task::Poll::Ready(Ok(()))
    }
    fn poll_receive(
        &mut self,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<Option<solana_kit_client::subscriptions::ReceivedFrame>, ()>> {
        self.frames
            .pop_front()
            .map_or(std::task::Poll::Pending, |frame| {
                std::task::Poll::Ready(Ok(Some(
                    solana_kit_client::subscriptions::ReceivedFrame::Text(frame),
                )))
            })
    }
    fn poll_close(&mut self, _: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), ()>> {
        self.closed
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        std::task::Poll::Ready(Ok(()))
    }
}
#[tokio::test]
async fn early_signature_confirms_and_awaits_close_in_native_send_spawn() {
    use solana_kit_client::{
        confirmation::{FactoryConfig, confirm_signature},
        subscriptions::{Budget, Session},
    };
    use std::{
        num::NonZeroU64,
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
    };
    let closed = Arc::new(AtomicUsize::new(0));
    let socket = ImmediateSocket {
        frames: Default::default(),
        closed: closed.clone(),
    };
    tokio::spawn(async move {
        confirm_signature(
            &Client::new(Rpc(serde_json::Value::Null), Limits::default()),
            Session::new(socket, 1, Budget::default()),
            &Timer,
            Signature::from_bytes([0; 64]),
            Lifetime::Blockhash {
                blockhash: Hash::from_bytes([0; 32]),
                last_valid_block_height: SLOT,
            },
            FactoryConfig {
                policy: Policy::default(),
                deadline: SLOT,
                poll_interval: NonZeroU64::new(1).unwrap(),
                cleanup_millis: NonZeroU64::new(1).unwrap(),
            },
            std::future::pending(),
        )
        .await
        .unwrap();
    })
    .await
    .unwrap();
    assert_eq!(closed.load(Ordering::SeqCst), 1);
}

struct UnavailableClock;
impl solana_kit_client::subscriptions::Clock for UnavailableClock {
    type Sleep<'a> = Ready<Result<(), solana_kit_client::subscriptions::ClockError>>;

    fn now(&self) -> u64 {
        0
    }

    fn sleep_until(&self, _: u64) -> Self::Sleep<'_> {
        ready(Err(
            solana_kit_client::subscriptions::ClockError::Unavailable(TransportError::with_source(
                solana_kit_client::TransportFailureKind::Io,
                std::io::Error::other("native confirmation clock source"),
            )),
        ))
    }
}

#[tokio::test]
async fn clock_failure_and_confirmation_future_remain_send_with_original_cause() {
    use solana_kit_client::{
        confirmation::{FactoryConfig, confirm_signature},
        subscriptions::{Budget, ClockError, Session, SessionError},
    };
    use std::{
        num::NonZeroU64,
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
    };

    let closed = Arc::new(AtomicUsize::new(0));
    let socket = ImmediateSocket {
        frames: Default::default(),
        closed: closed.clone(),
    };
    let error = tokio::spawn(async move {
        confirm_signature(
            &Client::new(Rpc(serde_json::Value::Null), Limits::default()),
            Session::new(socket, 1, Budget::default()),
            &UnavailableClock,
            Signature::from_bytes([0; 64]),
            Lifetime::Blockhash {
                blockhash: Hash::from_bytes([0; 32]),
                last_valid_block_height: SLOT,
            },
            FactoryConfig {
                policy: Policy::default(),
                deadline: SLOT,
                poll_interval: NonZeroU64::new(1).unwrap(),
                cleanup_millis: NonZeroU64::new(1).unwrap(),
            },
            std::future::pending(),
        )
        .await
        .unwrap_err()
    })
    .await
    .unwrap();

    let FactoryFailure::Session(SessionError::Clock(ClockError::Unavailable(cause))) =
        *error.outcome.unwrap_err()
    else {
        panic!("native future must retain the clock error");
    };
    assert_eq!(
        std::error::Error::source(&cause).unwrap().to_string(),
        "native confirmation clock source"
    );
    assert!(matches!(
        error.cleanup.last(),
        Some(SessionError::Clock(ClockError::Unavailable(_)))
    ));
    assert_eq!(closed.load(Ordering::SeqCst), 0);
}
