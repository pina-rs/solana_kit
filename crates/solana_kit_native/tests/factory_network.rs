//! Actual HTTP and one WebSocket compose two watches with every typed family.
#[path = "factory_network/submission.rs"]
mod submission;
use futures_util::{SinkExt, StreamExt};
use solana_kit_client::{
    Client, Limits,
    confirmation::{FactoryConfig, Policy, SessionAccess, confirm_signature_shared},
    subscriptions::*,
};
use solana_kit_core::{Address, Hash, Signature, transaction::Lifetime};
use solana_kit_native::{
    HttpConfig, NativeHttp, NativeSubscriptionSocket, NativeWebSocket, WsConfig,
};
use std::{
    future::{pending, poll_fn},
    num::NonZeroU64,
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::Notify,
};
use tokio_tungstenite::tungstenite::Message;

const DEADLINE: u64 = 3_000;
const GENERATION: u64 = 7;
const CLEANUP_MILLIS: u64 = 200;
const ACCOUNT_LAMPORTS: u64 = 7;
const OTHER_FAMILIES: usize = 8;
const FIRST_REMOTE: usize = 101;
type Peer = tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>;
const ADDRESS: &str = "11111111111111111111111111111111";

const SIGNATURE: &str = concat!(
    "11111111111111111111111111111111",
    "11111111111111111111111111111111"
);
struct Monotonic(tokio::time::Instant);
impl Clock for Monotonic {
    type Sleep<'a> = Pin<Box<dyn std::future::Future<Output = Result<(), ClockError>> + Send + 'a>>;
    fn now(&self) -> u64 {
        self.0.elapsed().as_millis().try_into().unwrap()
    }
    fn sleep_until(&self, deadline: u64) -> Self::Sleep<'_> {
        Box::pin(async move {
            tokio::time::sleep_until(self.0 + Duration::from_millis(deadline)).await;
            Ok(())
        })
    }
}
fn config() -> FactoryConfig {
    FactoryConfig {
        policy: Policy::default(),
        deadline: DEADLINE,
        poll_interval: NonZeroU64::new(10).unwrap(),
        cleanup_millis: NonZeroU64::new(CLEANUP_MILLIS).unwrap(),
    }
}
fn lifetime() -> Lifetime {
    Lifetime::Blockhash {
        blockhash: Hash::from_bytes([0; 32]),
        last_valid_block_height: 100,
    }
}
async fn json_request(socket: &mut tokio::net::TcpStream) -> Option<serde_json::Value> {
    let mut bytes = Vec::new();
    let mut byte = [0];

    while !bytes.ends_with(b"\r\n\r\n") {
        assert!(bytes.len() < 16_384);
        socket.read_exact(&mut byte).await.ok()?;
        bytes.push(byte[0]);
    }

    let length: usize = std::str::from_utf8(&bytes)
        .unwrap()
        .lines()
        .find_map(|line| {
            line.to_ascii_lowercase()
                .strip_prefix("content-length: ")
                .map(str::to_owned)
        })
        .unwrap()
        .parse()
        .unwrap();
    let mut body = vec![0; length];

    socket.read_exact(&mut body).await.ok()?;
    Some(serde_json::from_slice(&body).unwrap())
}
async fn rpc_peer(
    listener: TcpListener,
    registered: Arc<AtomicUsize>,
    reads: Arc<AtomicUsize>,
    observed: Arc<Notify>,
) {
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let Some(request) = json_request(&mut socket).await else {
            continue;
        };
        assert_eq!(
            registered.load(Ordering::SeqCst),
            10,
            "registration before HTTP"
        );
        reads.fetch_add(1, Ordering::SeqCst);
        observed.notify_one();

        let result = match request["method"].as_str().unwrap() {
            "getSignatureStatuses" => serde_json::json!({"context":{"slot":10},"value":[null]}),
            "getEpochInfo" => serde_json::json!({"absoluteSlot":10,"blockHeight":10,"epoch":0,
                "slotIndex":10,"slotsInEpoch":100,"transactionCount":0}),
            _ => panic!("unexpected read"),
        };

        let body = serde_json::to_vec(&serde_json::json!({"jsonrpc":"2.0","id":request["id"],
            "result":result}))
        .unwrap();
        let headers = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        socket.write_all(headers.as_bytes()).await.unwrap();
        socket.write_all(&body).await.unwrap();
    }
}
fn notifications() -> Vec<(&'static str, serde_json::Value)> {
    let account = serde_json::json!({"executable":false,"lamports":ACCOUNT_LAMPORTS,"owner":ADDRESS,
        "rentEpoch":0,"space":1,"data":["AQ==","base64"]});
    vec![
        (
            "account",
            serde_json::json!({"context":{"slot":1},"value":account}),
        ),
        (
            "program",
            serde_json::json!({"context":{"slot":1},"value":{"pubkey":ADDRESS,"account":account}}),
        ),
        (
            "logs",
            serde_json::json!({"context":{"slot":1},"value":{
                "signature":SIGNATURE,"err":null,"logs":["ok"]}}),
        ),
        (
            "block",
            serde_json::json!({"context":{"slot":1},"value":{"slot":1,"err":null,"block":{
                "blockhash":ADDRESS,"previousBlockhash":ADDRESS,"parentSlot":0,
                "blockTime":null,"blockHeight":null}}}),
        ),
        ("root", serde_json::json!(u64::MAX)),
        ("slot", serde_json::json!({"slot":2,"parent":1,"root":0})),
        (
            "vote",
            serde_json::json!({"hash":ADDRESS,"signature":SIGNATURE,"slots":[1,2],
                "timestamp":null,"votePubkey":ADDRESS}),
        ),
        (
            "slotsUpdates",
            serde_json::json!({"type":"dead","slot":2,"timestamp":3,"err":"bank failure"}),
        ),
    ]
}
async fn register_peer(socket: &mut Peer, registered: &AtomicUsize) {
    let notifications = notifications();

    for id in 1..=10 {
        let Message::Text(text) = socket.next().await.unwrap().unwrap() else {
            panic!("request");
        };
        let request: serde_json::Value = serde_json::from_str(&text).unwrap();
        let family = if id <= OTHER_FAMILIES {
            notifications[id - 1].0
        } else {
            "signature"
        };

        assert_eq!(request["method"], format!("{family}Subscribe"));
        assert_eq!(request["id"], id);
        registered.fetch_add(1, Ordering::SeqCst);
        socket
            .send(Message::Text(
                serde_json::json!({"jsonrpc":"2.0","id":id,"result":100+id})
                    .to_string()
                    .into(),
            ))
            .await
            .unwrap();
    }
}
async fn websocket_peer(
    listener: TcpListener,
    registered: Arc<AtomicUsize>,
    observed: Arc<Notify>,
) {
    let (socket, _) = listener.accept().await.unwrap();
    let mut socket = tokio_tungstenite::accept_async(socket).await.unwrap();
    register_peer(&mut socket, &registered).await;

    for (index, (family, result)) in notifications().into_iter().enumerate() {
        socket
            .send(Message::Text(
                serde_json::json!({"jsonrpc":"2.0","method":format!("{family}Notification"),
            "params":{"subscription":FIRST_REMOTE+index,"result":result}})
                .to_string()
                .into(),
            ))
            .await
            .unwrap();
    }

    for id in [109, 110] {
        socket
            .send(Message::Text(signature_notice(
                id,
                serde_json::json!("receivedSignature"),
            )))
            .await
            .unwrap();
    }

    observed.notified().await;

    for id in [110, 109] {
        socket
            .send(Message::Text(signature_notice(
                id,
                serde_json::json!({"err":null}),
            )))
            .await
            .unwrap();
    }

    cleanup_peer(&mut socket).await;
}
async fn cleanup_peer(socket: &mut Peer) {
    for _ in 0..OTHER_FAMILIES {
        let Message::Text(text) = socket.next().await.unwrap().unwrap() else {
            panic!("unsubscribe");
        };
        let request: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert!(request["method"].as_str().unwrap().ends_with("Unsubscribe"));
        socket
            .send(Message::Text(
                serde_json::json!({"jsonrpc":"2.0","id":request["id"],"result":true})
                    .to_string()
                    .into(),
            ))
            .await
            .unwrap();
    }

    assert!(matches!(
        socket.next().await.unwrap().unwrap(),
        Message::Close(_)
    ));
    socket.flush().await.unwrap();
}
fn signature_notice(
    id: usize,
    value: serde_json::Value,
) -> tokio_tungstenite::tungstenite::Utf8Bytes {
    serde_json::json!({"jsonrpc":"2.0","method":"signatureNotification","params":{
        "subscription":id,"result":{"context":{"slot":10},"value":value}}})
    .to_string()
    .into()
}
async fn value<R: SubscriptionMethod>(
    owner: &Mutex<Session<NativeSubscriptionSocket>>,
    clock: &Monotonic,
    handle: &Subscription<R>,
) -> R::Response {
    let result = poll_fn(|cx| {
        owner
            .with_session(|session| session.poll_next(handle, cx, clock.now(), DEADLINE))
            .unwrap()
    })
    .await
    .unwrap();
    let Drain::Notification(value) = result else {
        panic!("typed notification");
    };
    value
}

type Owner = Mutex<Session<NativeSubscriptionSocket>>;
struct Families {
    account: Subscription<Account>,
    program: Subscription<Program>,
    logs: Subscription<Logs>,
    block: Subscription<Block>,
    root: Subscription<Root>,
    slot: Subscription<Slot>,
    vote: Subscription<Vote>,
    updates: Subscription<SlotsUpdates>,
}
impl Families {
    fn register(session: &mut Session<NativeSubscriptionSocket>, address: Address) -> Self {
        use solana_kit_client::methods::{
            accounts::AccountEncoding,
            transaction_results::{BlockConfig as RpcBlockConfig, TransactionDetails},
        };
        let encoding = AccountConfig {
            encoding: Some(AccountEncoding::Base64),
            ..Default::default()
        };
        let program = ProgramConfig {
            account: encoding,
            filters: None,
        };
        let block = BlockConfig(RpcBlockConfig {
            rewards: Some(false),
            transaction_details: Some(TransactionDetails::None),
            ..Default::default()
        });

        Self {
            account: session
                .subscribe(Account(address, Some(encoding)), DEADLINE)
                .unwrap(),
            program: session
                .subscribe(Program(address, Some(program)), DEADLINE)
                .unwrap(),
            logs: session
                .subscribe(Logs(LogsFilter::All, None), DEADLINE)
                .unwrap(),
            block: session
                .subscribe(Block(BlockFilter::All, Some(block)), DEADLINE)
                .unwrap(),
            root: session.subscribe(Root, DEADLINE).unwrap(),
            slot: session.subscribe(Slot, DEADLINE).unwrap(),
            vote: session.subscribe(Vote, DEADLINE).unwrap(),
            updates: session.subscribe(SlotsUpdates, DEADLINE).unwrap(),
        }
    }
    async fn assert_values(&self, owner: &Owner, clock: &Monotonic, address: Address) {
        assert!(matches!(
            value(owner, clock, &self.account).await.value,
            solana_kit_core::accounts::MaybeAccount::Present(_)
        ));
        assert_eq!(
            value(owner, clock, &self.program).await.value.address,
            address
        );
        assert_eq!(value(owner, clock, &self.logs).await.value.logs, ["ok"]);
        assert_eq!(value(owner, clock, &self.block).await.value.slot, 1);
        assert_eq!(value(owner, clock, &self.root).await, u64::MAX);
        assert_eq!(value(owner, clock, &self.slot).await.slot, 2);
        assert_eq!(value(owner, clock, &self.vote).await.slots, [1, 2]);
        assert!(matches!(
            value(owner, clock, &self.updates).await,
            SlotUpdate::Dead { slot: 2, .. }
        ));
    }
    async fn close(self, mut session: Session<NativeSubscriptionSocket>, clock: &Monotonic) {
        session
            .unsubscribe(self.account, clock, DEADLINE)
            .await
            .unwrap();
        session
            .unsubscribe(self.program, clock, DEADLINE)
            .await
            .unwrap();
        session
            .unsubscribe(self.logs, clock, DEADLINE)
            .await
            .unwrap();
        session
            .unsubscribe(self.block, clock, DEADLINE)
            .await
            .unwrap();
        session
            .unsubscribe(self.root, clock, DEADLINE)
            .await
            .unwrap();
        session
            .unsubscribe(self.slot, clock, DEADLINE)
            .await
            .unwrap();
        session
            .unsubscribe(self.vote, clock, DEADLINE)
            .await
            .unwrap();
        session
            .unsubscribe(self.updates, clock, DEADLINE)
            .await
            .unwrap();

        session.shutdown(clock, DEADLINE).await.unwrap();
    }
}
async fn run_client(ws_endpoint: String, http_endpoint: String) {
    let socket = NativeWebSocket::connect(&ws_endpoint, WsConfig::default())
        .await
        .unwrap();
    let owner = Mutex::new(Session::new(
        NativeSubscriptionSocket::from(socket),
        GENERATION,
        Budget::default(),
    ));
    let client = Client::new(
        NativeHttp::new(&http_endpoint, HttpConfig::default()).unwrap(),
        Limits::default(),
    );
    let clock = Monotonic(tokio::time::Instant::now());
    let address: Address = ADDRESS.parse().unwrap();
    let families = owner
        .with_session(|session| Families::register(session, address))
        .unwrap();

    let first = confirm_signature_shared(
        &client,
        &owner,
        &clock,
        Signature::from_bytes([0; 64]),
        lifetime(),
        config(),
        pending(),
    );
    let second = confirm_signature_shared(
        &client,
        &owner,
        &clock,
        Signature::from_bytes([1; 64]),
        lifetime(),
        config(),
        pending(),
    );
    let (first, second) = tokio::join!(first, second);
    first.unwrap();
    second.unwrap();
    families.assert_values(&owner, &clock, address).await;
    families.close(owner.into_inner().unwrap(), &clock).await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn http_and_all_nine_families_share_one_socket_with_send_factory_future() {
    let socket_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let http = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let ws_endpoint = format!("ws://{}/rpc", socket_listener.local_addr().unwrap());
    let http_endpoint = format!("http://{}/rpc", http.local_addr().unwrap());
    let registered = Arc::new(AtomicUsize::new(0));
    let reads = Arc::new(AtomicUsize::new(0));
    let observed = Arc::new(Notify::new());

    let peer = tokio::spawn(websocket_peer(
        socket_listener,
        registered.clone(),
        observed.clone(),
    ));
    let rpc = tokio::spawn(rpc_peer(http, registered, reads.clone(), observed));
    tokio::spawn(run_client(ws_endpoint, http_endpoint))
        .await
        .unwrap();
    peer.await.unwrap();
    assert!(reads.load(Ordering::SeqCst) > 0);
    rpc.abort();
    assert!(rpc.await.unwrap_err().is_cancelled());
}
