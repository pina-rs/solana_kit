//! Submission identity, exact wire preservation and error boundaries.
use core::future::{Ready, ready};
use solana_kit_client::{
    Client, Limits, Reply, Request, Transport, TransportError, TransportFailureKind,
    confirmation::{Event, Failure, Observer, Policy, WaitError, send_and_confirm},
    methods::submission::SendConfig,
    subscriptions::SignatureInfo,
};
use solana_kit_core::{
    Address, Hash, Signature,
    transaction::{Lifetime, Message, Transaction, Version},
};
use std::sync::{Arc, Mutex};

type Requests = Arc<Mutex<Vec<Vec<u8>>>>;
struct Http {
    requests: Requests,
    reply: Option<Signature>,
}
impl Transport for Http {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let json: serde_json::Value = serde_json::from_slice(&request.body).unwrap();
        self.requests.lock().unwrap().push(request.body);
        let Some(signature) = self.reply else {
            return ready(Err(TransportError::new(TransportFailureKind::Io)));
        };
        ready(Ok(Reply {
            status: 200,
            body: serde_json::to_vec(&serde_json::json!({
            "jsonrpc":"2.0", "id":json["id"], "result":signature.to_string() }))
            .unwrap(),
        }))
    }
}
struct Complete(Signature);
impl Observer for Complete {
    type Next<'a> = Ready<Result<Event, Failure>>;
    type Close<'a> = Ready<Result<(), Failure>>;
    fn next(&mut self) -> Self::Next<'_> {
        ready(Ok(Event::Signature {
            signature: self.0,
            slot: u64::MAX,
            commitment: solana_kit_client::methods::Commitment::Finalized,
            value: SignatureInfo::Processed { err: None },
        }))
    }
    fn close(&mut self) -> Self::Close<'_> {
        ready(Ok(()))
    }
}
fn transaction() -> Transaction {
    let payer = Address::from_bytes([1; 32]);
    let message = Message::compile(
        Version::Legacy,
        payer,
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([2; 32]),
            last_valid_block_height: u64::MAX,
        },
        &[],
        &[],
    )
    .unwrap();
    Transaction::new(message.clone())
        .with_signature(payer, message.bytes(), Signature::from_bytes([7; 64]))
        .unwrap()
}
fn client(reply: Option<Signature>) -> (Client<Http>, Requests) {
    let requests = Arc::new(Mutex::new(Vec::new()));
    (
        Client::new(
            Http {
                requests: requests.clone(),
                reply,
            },
            Limits::default(),
        ),
        requests,
    )
}

#[test]
fn exact_bytes_signature_and_integer_options_survive_send() {
    let transaction = transaction();
    let wire = transaction.encode(true).unwrap();
    let expected = transaction.signatures()[0].unwrap();
    let (client, requests) = client(Some(expected));
    let config = SendConfig {
        max_retries: Some(u64::MAX),
        min_context_slot: Some(u64::MAX),
        skip_preflight: Some(false),
        ..SendConfig::default()
    };
    let result = futures::executor::block_on(send_and_confirm(
        &client,
        &transaction,
        config,
        Policy::default(),
        Complete,
        core::future::pending(),
    ))
    .unwrap();
    assert_eq!(result, expected);
    assert_eq!(transaction.encode(true).unwrap(), wire);
    let requests = requests.lock().unwrap();
    assert_eq!(requests.len(), 1);
    let json: serde_json::Value = serde_json::from_slice(&requests[0]).unwrap();
    assert_eq!(json["params"][1]["maxRetries"].as_u64(), Some(u64::MAX));
    assert_eq!(json["params"][1]["minContextSlot"].as_u64(), Some(u64::MAX));
    assert_eq!(json["params"][1]["preflightCommitment"], "confirmed");
    assert_eq!(json["params"][1]["skipPreflight"], false);
    assert_eq!(json["method"], "sendTransaction");
    let encoded = json["params"][0].as_str().unwrap();
    assert_eq!(
        encoded,
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &wire,)
    );
}

#[test]
fn mismatched_reported_signature_does_not_start_observation() {
    let transaction = transaction();
    let (client, requests) = client(Some(Signature::from_bytes([8; 64])));
    let result = futures::executor::block_on(send_and_confirm(
        &client,
        &transaction,
        SendConfig::default(),
        Policy::default(),
        |_| -> Complete { panic!("must not observe foreign transaction") },
        core::future::pending(),
    ));
    assert!(matches!(
        result,
        Err(WaitError::Outcome(
            Failure::ReportedSignatureMismatch { .. }
        ))
    ));
    assert_eq!(requests.lock().unwrap().len(), 1);
}

#[test]
fn transport_failure_and_pre_cancel_never_become_confirmation() {
    let transaction = transaction();
    let (client, requests) = client(None);
    let result = futures::executor::block_on(send_and_confirm(
        &client,
        &transaction,
        SendConfig::default(),
        Policy::default(),
        Complete,
        core::future::pending(),
    ));
    assert!(matches!(result, Err(WaitError::Outcome(Failure::Rpc(_)))));
    assert_eq!(requests.lock().unwrap().len(), 1);

    let result = futures::executor::block_on(send_and_confirm(
        &client,
        &transaction,
        SendConfig::default(),
        Policy::default(),
        Complete,
        core::future::ready(()),
    ));
    assert!(matches!(
        result,
        Err(WaitError::Outcome(Failure::Cancelled))
    ));
    assert_eq!(requests.lock().unwrap().len(), 1);
}

#[test]
fn received_unresolved_lifetime_is_rejected_before_submission() {
    let transaction = transaction();
    let received = Transaction::decode(&transaction.encode(true).unwrap()).unwrap();
    let (client, requests) = client(Some(Signature::from_bytes([7; 64])));
    let result = futures::executor::block_on(send_and_confirm(
        &client,
        &received,
        SendConfig::default(),
        Policy::default(),
        Complete,
        core::future::pending(),
    ));
    assert!(matches!(
        result,
        Err(WaitError::Outcome(Failure::UnresolvedLifetime))
    ));
    assert!(requests.lock().unwrap().is_empty());
}
