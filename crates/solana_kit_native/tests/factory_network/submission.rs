//! Submission uses real HTTP exactly once, including uncertain remote acceptance.
use super::*;
use base64::{Engine, engine::general_purpose::STANDARD};
use solana_kit_client::{
    Error,
    confirmation::{FactoryFailure, Failure, submit_and_confirm},
    methods::submission::SendConfig,
};
use solana_kit_core::transaction::{Message as TransactionMessage, Transaction, Version};

const SIGNATURE_BYTE: u8 = 7;
const ROOT_REMOTE: u64 = 11;
const ROOT_VALUE: u64 = 99;
fn transaction() -> Transaction {
    let payer = Address::from_bytes([1; 32]);
    let message =
        TransactionMessage::compile(Version::Legacy, payer, lifetime(), &[], &[]).unwrap();
    Transaction::new(message.clone())
        .with_signature(
            payer,
            message.bytes(),
            Signature::from_bytes([SIGNATURE_BYTE; 64]),
        )
        .unwrap()
}
/// Uncertain acceptance closes only after consuming the exact submitted wire.
async fn http_submission(
    listener: TcpListener,
    uncertain: bool,
    wire: Vec<u8>,
    sends: Arc<AtomicUsize>,
) {
    let requests = if uncertain { 1 } else { 2 };

    for index in 0..requests {
        let (mut stream, _) = listener.accept().await.unwrap();
        let request = json_request(&mut stream).await.unwrap();
        let result = submission_result(&request, &wire, &sends, index);

        if uncertain {
            return;
        }

        let body = serde_json::to_vec(
            &serde_json::json!({"jsonrpc":"2.0","id":request["id"],"result":result}),
        )
        .unwrap();
        let headers = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        stream.write_all(headers.as_bytes()).await.unwrap();
        stream.write_all(&body).await.unwrap();
    }
}
fn submission_result(
    request: &serde_json::Value,
    wire: &[u8],
    sends: &AtomicUsize,
    index: usize,
) -> serde_json::Value {
    if index == 0 {
        assert_eq!(request["method"], "sendTransaction");
        assert_eq!(request["params"][0], STANDARD.encode(wire));
        assert_eq!(request["params"][1]["encoding"], "base64");
        assert_eq!(request["params"][1]["preflightCommitment"], "confirmed");

        sends.fetch_add(1, Ordering::SeqCst);

        serde_json::json!(Signature::from_bytes([SIGNATURE_BYTE; 64]).to_string())
    } else {
        assert_eq!(request["method"], "getSignatureStatuses");
        serde_json::json!({"context":{"slot":10},"value":[{
                "slot":10,"confirmations":1,"err":null,"status":{"Ok":null},
                "confirmationStatus":"confirmed"}]})
    }
}
async fn socket_submission(listener: TcpListener) {
    let (stream, _) = listener.accept().await.unwrap();
    let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();

    for (id, method) in [(1, "rootSubscribe"), (2, "signatureSubscribe")] {
        let request = socket_request(&mut socket).await;
        assert_eq!(request["method"], method);
        socket
            .send(Message::Text(
                serde_json::json!({"jsonrpc":"2.0","id":id,"result":id+10})
                    .to_string()
                    .into(),
            ))
            .await
            .unwrap();
    }

    socket
        .send(Message::Text(
            serde_json::json!({"jsonrpc":"2.0","method":"rootNotification",
        "params":{"subscription":ROOT_REMOTE,"result":ROOT_VALUE}})
            .to_string()
            .into(),
        ))
        .await
        .unwrap();

    for method in ["signatureUnsubscribe", "rootUnsubscribe"] {
        let request = socket_request(&mut socket).await;
        assert_eq!(request["method"], method);
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
async fn socket_request(socket: &mut Peer) -> serde_json::Value {
    let Message::Text(text) = socket.next().await.unwrap().unwrap() else {
        panic!("JSON request");
    };
    serde_json::from_str(&text).unwrap()
}
async fn submission_case(uncertain: bool) {
    let socket_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let http_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let socket_endpoint = format!("ws://{}", socket_listener.local_addr().unwrap());
    let http_endpoint = format!("http://{}", http_listener.local_addr().unwrap());
    let transaction = transaction();
    let sends = Arc::new(AtomicUsize::new(0));
    let http_peer = tokio::spawn(http_submission(
        http_listener,
        uncertain,
        transaction.encode(true).unwrap(),
        sends.clone(),
    ));
    let socket_peer = tokio::spawn(socket_submission(socket_listener));

    let socket = NativeWebSocket::connect(&socket_endpoint, WsConfig::default())
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
    let root = owner
        .with_session(|session| session.subscribe(Root, DEADLINE))
        .unwrap()
        .unwrap();

    let result = submit_and_confirm(
        &client,
        &owner,
        &clock,
        &transaction,
        SendConfig::default(),
        config(),
        pending(),
    )
    .await;
    assert_outcome(result, uncertain);
    close_root(owner, clock, root).await;
    http_peer.await.unwrap();
    socket_peer.await.unwrap();

    assert_eq!(sends.load(Ordering::SeqCst), 1);
}
async fn close_root(owner: Owner, clock: Monotonic, root: Subscription<Root>) {
    assert_eq!(value(&owner, &clock, &root).await, ROOT_VALUE);
    let mut session = owner.into_inner().unwrap();
    session.unsubscribe(root, &clock, DEADLINE).await.unwrap();
    session.shutdown(&clock, DEADLINE).await.unwrap();
}
fn assert_outcome(
    result: Result<
        Signature,
        solana_kit_client::confirmation::FactoryError<solana_kit_native::WsError>,
    >,
    uncertain: bool,
) {
    if !uncertain {
        assert_eq!(result.unwrap(), Signature::from_bytes([SIGNATURE_BYTE; 64]));

        return;
    }

    let error = result.unwrap_err();
    assert!(error.cleanup.is_empty());
    let FactoryFailure::Confirmation(Failure::Rpc(Error::Transport(error))) =
        *error.outcome.unwrap_err()
    else {
        panic!("uncertain original I/O failure");
    };
    assert!(std::error::Error::source(&error).is_some());
}
#[tokio::test]
async fn one_shot_real_http_submission_success_and_uncertainty_preserve_shared_root() {
    submission_case(false).await;
    submission_case(true).await;
}
