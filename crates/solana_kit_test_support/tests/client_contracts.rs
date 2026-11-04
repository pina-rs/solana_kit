//! Downstream-style contracts for the public scripted client fixtures.

use solana_kit_client::{
    Client, Error, Limits, Presence, Reply, Request, Transport, TransportError,
    TransportFailureKind,
    methods::{AccountInfo, Balance, Contextual, LatestBlockhash, ReadConfig},
};
use solana_kit_core::{
    Address, Hash,
    accounts::{Account as CoreAccount, MaybeAccount},
};
use solana_kit_test_support::{
    Exchange, PendingExchange, ScriptStep, ScriptedTransport, UnexpectedRequest, canned,
};
use std::{
    cell::Cell,
    error::Error as _,
    future::Future,
    pin::Pin,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};

fn address() -> Address {
    Address::from_bytes([7; 32])
}

fn client(transport: &ScriptedTransport) -> Client<ScriptedTransport> {
    Client::new(transport.clone(), Limits::default())
}

#[test]
fn balance_preserves_exact_request_id_and_u64_max() {
    let transport = ScriptedTransport::new([ScriptStep::success(canned::balance(42, u64::MAX))]);
    let response =
        futures::executor::block_on(client(&transport).call(Balance(address(), None))).unwrap();

    assert_eq!(response.context.slot, 42);
    assert_eq!(response.value, u64::MAX);
    assert_eq!(transport.remaining(), 0);

    let requests = transport.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].id, 1);
    assert_eq!(requests[0].method, "getBalance");
    assert_eq!(
        requests[0].body,
        format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"getBalance","params":["{}"]}}"#,
            address()
        )
        .into_bytes()
    );
    assert_eq!(requests[0].max_reply_bytes, Limits::default().bytes);
}

#[test]
fn cloned_transport_observes_ordered_unique_request_ids() {
    let transport = ScriptedTransport::new([
        ScriptStep::success(canned::balance(1, 10)),
        ScriptStep::success(canned::balance(2, 20)),
    ]);
    let owner = client(&transport);

    assert_eq!(
        futures::executor::block_on(owner.call(Balance(address(), None)))
            .unwrap()
            .value,
        10
    );
    assert_eq!(
        futures::executor::block_on(owner.call(Balance(address(), None)))
            .unwrap()
            .value,
        20
    );

    let ids: Vec<_> = transport
        .requests()
        .iter()
        .map(|request| request.id)
        .collect();
    assert_eq!(ids, [1, 2]);
}

#[test]
fn remote_error_is_correlated_and_keeps_exact_data() {
    let transport = ScriptedTransport::new([ScriptStep::remote_error_with_data(
        -32004,
        "fixture failure",
        r#"{ "future": [ 1, null ] }"#,
    )]);
    let error =
        futures::executor::block_on(client(&transport).call(Balance(address(), None))).unwrap_err();

    let Error::Remote(remote) = error else {
        panic!("remote error expected")
    };
    assert_eq!(remote.code, -32004);
    assert_eq!(remote.message, "fixture failure");
    let Presence::Present(data) = remote.data else {
        panic!("present remote data expected")
    };
    assert_eq!(data.get(), r#"{ "future": [ 1, null ] }"#);
    assert_eq!(transport.requests()[0].id, 1);
}

#[test]
fn injected_transport_failure_retains_its_typed_kind() {
    let transport = ScriptedTransport::new([ScriptStep::failure(TransportError::new(
        TransportFailureKind::Deadline,
    ))]);
    let error =
        futures::executor::block_on(client(&transport).call(Balance(address(), None))).unwrap_err();

    let Error::Transport(failure) = error else {
        panic!("transport failure expected")
    };
    assert_eq!(failure.kind, TransportFailureKind::Deadline);
}

#[test]
fn malformed_raw_replies_are_never_successful() {
    for (status, body) in [
        (200, ""),
        (200, "not json"),
        (
            200,
            r#"{"jsonrpc":"2.0","id":1,"id":1,"result":{"context":{"slot":1},"value":1}}"#,
        ),
    ] {
        let transport = ScriptedTransport::new([ScriptStep::raw(status, body)]);
        let error = futures::executor::block_on(client(&transport).call(Balance(address(), None)))
            .unwrap_err();

        assert!(matches!(error, Error::Json(_)), "{body}: {error:?}");
    }
}

#[test]
fn null_result_and_wrong_status_or_id_keep_client_validation() {
    let null = ScriptedTransport::new([ScriptStep::raw(
        200,
        r#"{"jsonrpc":"2.0","id":1,"result":null}"#,
    )]);
    assert!(matches!(
        futures::executor::block_on(client(&null).call(Balance(address(), None))),
        Err(Error::Schema(_))
    ));

    let status = ScriptedTransport::new([ScriptStep::raw(
        500,
        r#"{"jsonrpc":"2.0","id":1,"result":{"context":{"slot":1},"value":1}}"#,
    )]);
    assert!(matches!(
        futures::executor::block_on(client(&status).call(Balance(address(), None))),
        Err(Error::Status(500))
    ));

    let correlation = ScriptedTransport::new([ScriptStep::raw(
        200,
        r#"{"jsonrpc":"2.0","id":2,"result":{"context":{"slot":1},"value":1}}"#,
    )]);
    assert!(matches!(
        futures::executor::block_on(client(&correlation).call(Balance(address(), None))),
        Err(Error::Correlation)
    ));
}

#[test]
fn exhausted_script_rejects_the_next_request_clearly() {
    let transport = ScriptedTransport::new([ScriptStep::success(canned::balance(1, 1))]);
    let owner = client(&transport);

    futures::executor::block_on(owner.call(Balance(address(), None))).unwrap();
    let error = futures::executor::block_on(owner.call(Balance(address(), None))).unwrap_err();

    let Error::Transport(failure) = error else {
        panic!("transport failure expected")
    };
    assert_eq!(failure.kind, TransportFailureKind::Unavailable);
    let source = failure
        .source()
        .and_then(|source| source.downcast_ref::<UnexpectedRequest>())
        .expect("unexpected request source");
    assert_eq!(source.request.id, 2);
    assert_eq!(source.request.method, "getBalance");
    assert_eq!(source.scripted, 1);
    assert!(source.to_string().contains("after 1 scripted exchanges"));
    assert_eq!(transport.requests().len(), 2);
}

#[derive(Default)]
struct CountingWake(AtomicUsize);

impl Wake for CountingWake {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn pending_exchange_observes_wake_completion_and_delivery() {
    let (step, pending) = ScriptStep::pending();
    let transport = ScriptedTransport::new([step]);
    let owner = client(&transport);
    let mut call = Box::pin(owner.call(Balance(address(), None)));
    let wakes = Arc::new(CountingWake::default());
    let waker = Waker::from(Arc::clone(&wakes));
    let mut context = Context::from_waker(&waker);

    assert!(call.as_mut().poll(&mut context).is_pending());
    let status = pending.status();
    assert_eq!(status.request_id, Some(1));
    assert_eq!(status.polls, 1);
    assert_eq!(status.wakes, 0);
    assert!(status.has_waker);
    assert!(!status.completed);
    assert!(!status.delivered);
    assert!(!status.cancelled);

    assert!(pending.wake());
    assert_eq!(wakes.0.load(Ordering::SeqCst), 1);
    assert_eq!(pending.status().wakes, 1);
    assert!(pending.complete_success(canned::balance(9, u64::MAX)));
    assert_eq!(wakes.0.load(Ordering::SeqCst), 2);
    assert!(!pending.complete_success(canned::balance(9, 1)));

    let Poll::Ready(Ok(response)) = call.as_mut().poll(&mut context) else {
        panic!("completed exchange expected")
    };
    assert_eq!(response.context.slot, 9);
    assert_eq!(response.value, u64::MAX);
    let status = pending.status();
    assert!(status.completed);
    assert!(status.delivered);
    assert!(!status.cancelled);
    assert!(!status.has_waker);
}

#[test]
fn pending_remote_error_uses_the_captured_request_id() {
    let (step, pending) = ScriptStep::pending();
    let transport = ScriptedTransport::new([step]);
    let owner = client(&transport);
    let mut call = Box::pin(owner.call(Balance(address(), None)));
    let mut context = Context::from_waker(Waker::noop());

    assert!(call.as_mut().poll(&mut context).is_pending());
    assert!(pending.complete_remote_error(-32603, "later"));
    let Poll::Ready(Err(Error::Remote(remote))) = call.as_mut().poll(&mut context) else {
        panic!("remote error expected")
    };
    assert_eq!(remote.code, -32603);
    assert_eq!(remote.message, "later");
    assert!(matches!(remote.data, Presence::Missing));
}

#[test]
fn dropping_pending_work_cancels_and_releases_its_owner() {
    let (step, pending) = ScriptStep::pending();
    let transport = ScriptedTransport::new([step]);
    let owner = client(&transport);
    let mut call = Box::pin(owner.call(Balance(address(), None)));
    let mut context = Context::from_waker(Waker::noop());

    assert!(call.as_mut().poll(&mut context).is_pending());
    assert!(pending.status().has_waker);
    drop(call);

    let status = pending.status();
    assert!(status.cancelled);
    assert!(!status.completed);
    assert!(!status.delivered);
    assert!(!status.has_waker);
    assert!(!pending.wake());
    assert!(!pending.complete_success(canned::balance(1, 1)));
    assert_eq!(transport.requests().len(), 1);
    assert_eq!(transport.remaining(), 0);
}

#[test]
fn canned_account_and_blockhash_fixtures_stay_valid() {
    let account = CoreAccount {
        address: address(),
        owner: Address::from_bytes([9; 32]),
        lamports: 10,
        executable: false,
        space: 3,
        rent_epoch: Some(4),
        data: vec![1, 2, 3],
    };
    let account_transport =
        ScriptedTransport::new([ScriptStep::success(canned::account(5, &account))]);
    let account_response = futures::executor::block_on(
        client(&account_transport).call(AccountInfo(address(), ReadConfig::default())),
    )
    .unwrap();
    let MaybeAccount::Present(present) = account_response.value else {
        panic!("present account expected")
    };
    assert_eq!(
        present,
        CoreAccount {
            address: address(),
            owner: Address::from_bytes([9; 32]),
            lamports: 10,
            executable: false,
            space: 3,
            rent_epoch: Some(4),
            data: vec![1, 2, 3],
        }
    );

    let missing_transport =
        ScriptedTransport::new([ScriptStep::success(canned::missing_account(6))]);
    let missing_response = futures::executor::block_on(
        client(&missing_transport).call(AccountInfo(address(), ReadConfig::default())),
    )
    .unwrap();
    assert_eq!(missing_response.value, MaybeAccount::Missing(address()));

    let blockhash = Hash::from_bytes([3; 32]);
    let blockhash_transport = ScriptedTransport::new([ScriptStep::success(
        canned::latest_blockhash(7, blockhash, 88),
    )]);
    let Contextual { context, value } =
        futures::executor::block_on(client(&blockhash_transport).call(LatestBlockhash(None)))
            .unwrap();
    assert_eq!(context.slot, 7);
    assert_eq!(value.blockhash, blockhash);
    assert_eq!(value.last_valid_block_height, 88);
}

#[test]
fn native_send_and_sync_contracts_hold() {
    fn assert_send_sync<T: Send + Sync>() {}
    fn assert_send<T: Send>(_: T) {}

    assert_send_sync::<ScriptedTransport>();
    assert_send_sync::<PendingExchange>();
    assert_send_sync::<Client<ScriptedTransport>>();

    let transport = ScriptedTransport::new([ScriptStep::success(canned::balance(1, 1))]);
    let owner = client(&transport);
    assert_send(owner.call(Balance(address(), None)));
}

struct LocalTransport {
    inner: ScriptedTransport,
    polls: Rc<Cell<usize>>,
}

struct LocalExchange {
    inner: Exchange,
    polls: Rc<Cell<usize>>,
}

impl Future for LocalExchange {
    type Output = Result<Reply, TransportError>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        this.polls.set(this.polls.get() + 1);
        Pin::new(&mut this.inner).poll(context)
    }
}

impl Transport for LocalTransport {
    type Exchange<'a> = LocalExchange;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        LocalExchange {
            inner: self.inner.exchange(request),
            polls: Rc::clone(&self.polls),
        }
    }
}

#[test]
fn local_rc_composition_needs_no_send_exchange() {
    let polls = Rc::new(Cell::new(0));
    let transport = LocalTransport {
        inner: ScriptedTransport::new([ScriptStep::success(canned::balance(3, 4))]),
        polls: Rc::clone(&polls),
    };
    let owner = Client::new(transport, Limits::default());

    let response = futures::executor::block_on(owner.call(Balance(address(), None))).unwrap();
    assert_eq!(response.value, 4);
    assert_eq!(polls.get(), 1);
}

#[cfg(feature = "crypto")]
#[test]
fn deterministic_keypair_is_reproducible_and_signs() {
    use solana_kit_signers::{MessageSigner, SignerConfig};

    let first = solana_kit_test_support::signer::test_keypair();
    let second = solana_kit_test_support::signer::test_keypair();
    assert_eq!(first.address(), second.address());
    let output =
        futures::executor::block_on(first.sign_message(b"fixture", SignerConfig::default()))
            .unwrap();
    assert_eq!(output.address(), first.address());
    assert_eq!(output.bytes(), b"fixture");
    assert!(
        solana_kit_core::crypto::verify(&first.address(), b"fixture", &output.signature()).is_ok()
    );
}
