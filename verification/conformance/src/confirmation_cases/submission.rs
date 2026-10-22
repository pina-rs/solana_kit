//! Exact node request bytes and effect exclusion use public submission only.

use crate::require;
use alloc::{format, rc::Rc, vec::Vec};
use core::{
    cell::RefCell,
    future::{Ready, ready},
};
use solana_kit_client::{
    Client, Limits, Reply, Request, Transport, TransportError, TransportFailureKind,
    confirmation::{Event, Failure, Observer, Policy, WaitError, send_and_confirm},
    methods::{Commitment, submission::SendConfig},
};
use solana_kit_core::{
    Address, Hash, Signature,
    transaction::{Lifetime, Message, Transaction, Version},
};

type Requests = Rc<RefCell<Vec<Vec<u8>>>>;
struct Http {
    requests: Requests,
    reply: Option<Signature>,
}
impl Transport for Http {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        self.requests.borrow_mut().push(request.body);
        let Some(signature) = self.reply else {
            return ready(Err(TransportError::new(TransportFailureKind::Io)));
        };
        ready(Ok(Reply {
            status: 200,
            body: format!(r#"{{"jsonrpc":"2.0","id":1,"result":"{signature}"}}"#).into_bytes(),
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
            commitment: Commitment::Finalized,
            value: solana_kit_client::subscriptions::SignatureInfo::Processed { err: None },
        }))
    }
    fn close(&mut self) -> Self::Close<'_> {
        ready(Ok(()))
    }
}
fn transaction() -> Result<Transaction, &'static str> {
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
    .map_err(|_| "submission message rejected")?;
    Transaction::new(message.clone())
        .with_signature(payer, message.bytes(), Signature::from_bytes([7; 64]))
        .map_err(|_| "fixture signature rejected")
}
fn client(reply: Option<Signature>) -> (Client<Http>, Requests) {
    let requests = Rc::new(RefCell::new(Vec::new()));
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

pub(super) fn run() -> Result<(), &'static str> {
    exact()?;
    excluded()
}
fn exact() -> Result<(), &'static str> {
    let transaction = transaction()?;
    let wire = transaction.encode(true).map_err(|_| "wire rejected")?;
    let expected = Signature::from_bytes([7; 64]);
    let (client, requests) = client(Some(expected));
    let config = SendConfig {
        max_retries: Some(u64::MAX),
        min_context_slot: Some(u64::MAX),
        skip_preflight: Some(false),
        ..SendConfig::default()
    };
    let result = super::finish(send_and_confirm(
        &client,
        &transaction,
        config,
        Policy::default(),
        Complete,
        core::future::pending(),
    ))?;
    require(
        matches!(result, Ok(actual) if actual == expected),
        "primary signature changed",
    )?;
    require(
        transaction
            .encode(true)
            .map_err(|_| "wire reencode failed")?
            == wire,
        "submission mutated signed wire",
    )?;
    let expected_request = r#"{"jsonrpc":"2.0","id":1,"method":"sendTransaction","params":["AQcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcBAAABAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQECAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgA=",{"encoding":"base64","skipPreflight":false,"preflightCommitment":"confirmed","maxRetries":18446744073709551615,"minContextSlot":18446744073709551615}]}"#;
    require(
        requests.borrow().as_slice() == [expected_request.as_bytes()],
        "exact signed request or options changed",
    )
}
fn excluded() -> Result<(), &'static str> {
    let transaction = transaction()?;
    let (client, requests) = client(Some(Signature::from_bytes([8; 64])));
    let observed = core::cell::Cell::new(0);
    let observer = |signature| {
        observed.set(observed.get() + 1);
        Complete(signature)
    };
    let mismatch = super::finish(send_and_confirm(
        &client,
        &transaction,
        SendConfig::default(),
        Policy::default(),
        observer,
        core::future::pending(),
    ))?;
    require(
        matches!(mismatch, Err(WaitError::Outcome(Failure::ReportedSignatureMismatch { expected, actual })) if expected == Signature::from_bytes([7;64]) && *actual == Signature::from_bytes([8;64])),
        "foreign reported signature accepted",
    )?;
    require(
        observed.get() == 0 && requests.borrow().len() == 1,
        "foreign signature started observation",
    )?;
    let cancelled = super::finish(send_and_confirm(
        &client,
        &transaction,
        SendConfig::default(),
        Policy::default(),
        observer,
        ready(()),
    ))?;
    require(
        matches!(cancelled, Err(WaitError::Outcome(Failure::Cancelled)))
            && requests.borrow().len() == 1
            && observed.get() == 0,
        "pre-cancel caused effects",
    )?;
    let received =
        Transaction::decode(&transaction.encode(true).map_err(|_| "wire encode failed")?)
            .map_err(|_| "wire decode failed")?;
    let unresolved = super::finish(send_and_confirm(
        &client,
        &received,
        SendConfig::default(),
        Policy::default(),
        observer,
        core::future::pending(),
    ))?;
    require(
        matches!(
            unresolved,
            Err(WaitError::Outcome(Failure::UnresolvedLifetime))
        ) && requests.borrow().len() == 1
            && observed.get() == 0,
        "unresolved lifetime caused effects",
    )?;
    let (failed, failed_requests) = self::client(None);
    let failure = super::finish(send_and_confirm(
        &failed,
        &transaction,
        SendConfig::default(),
        Policy::default(),
        observer,
        core::future::pending(),
    ))?;
    require(
        matches!(failure, Err(WaitError::Outcome(Failure::Rpc(_))))
            && failed_requests.borrow().len() == 1
            && observed.get() == 0,
        "transport failure became success or observation",
    )
}
