//! Prover protocol tests. The default cases talk to a local HTTP server.
//! Live and local-prover cases stay ignored unless their environment is set.

use std::collections::VecDeque;
use std::future::Ready;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Value, json};
use solana_kit_client::rest::{self, Method};
use solana_kit_client::{Reply, TransportError, TransportFailureKind};
use solana_kit_core::{AccountRole, Address};

use super::{Delivery, PrivacyProverClient, ProverDestination};
use crate::{
    Circuit, Error, Input, OwnerTag, Proof, ProverFailure, STATE_TREE, Settlement, ShieldedOutput,
    TransactBody, TreeContext, Withdraw, parse_prover_proof, proving_key_sha256,
};

const CIRCUIT: &str = "transfer_confidential_1_2";
const HTTP_OK: u16 = 200;
const HTTP_TOO_MANY: u16 = 429;
const HTTP_ERROR: u16 = 500;
const HEADER_BUDGET: usize = 64 * 1024;
const TARGET_BUDGET: usize = 8 * 1024;
const REQUEST_TIMEOUT_SECS: u64 = 5;

const SHORT_POLL_MS: u64 = 20;
const MEDIUM_POLL_MS: u64 = 50;
const SLOW_POLL_MS: u64 = 200;
const POLL_BOUND_SECS: u64 = 20;
const NULLIFIER_BYTE: u8 = 13;
const FIELD_BYTES: usize = 32;

fn pin() -> &'static str {
    proving_key_sha256(CIRCUIT).expect("pinned circuit")
}

fn proof_value(ar: [&str; 2]) -> Value {
    json!({
        "ar": ar,
        "bs": [["0x0", "0x0"], ["0x0", "0x0"]],
        "krs": ["0x1", "0x2"],
        "provingKeySha256": pin(),
    })
}

fn negated_a() -> [u8; 32] {
    let mut bytes = [0; 32];
    bytes[0] = 0x80;
    bytes[31] = 1;
    bytes
}

#[test]
fn gnark_proof_negates_a_and_keeps_the_g2_bytes() {
    let proof = parse_prover_proof(&proof_value(["0x1", "0x2"]), CIRCUIT).expect("proof");
    let mut compressed = [0; 32];
    compressed[31] = 1;
    assert_eq!(proof.a, negated_a());
    assert_eq!(proof.c, compressed);
    assert_eq!(proof.b, [0; 128]);

    // Solana's canonical big-endian G2 compression test point, kept raw.
    let proof_b: [u8; 128] = [
        40, 57, 233, 205, 180, 46, 35, 111, 215, 5, 23, 93, 12, 71, 118, 225, 7, 46, 247, 147, 47,
        130, 106, 189, 184, 80, 146, 103, 141, 52, 242, 25, 0, 203, 124, 176, 110, 34, 151, 212,
        66, 180, 238, 151, 236, 189, 133, 209, 17, 137, 205, 183, 168, 196, 92, 159, 75, 174, 81,
        168, 18, 86, 176, 56, 16, 26, 210, 20, 18, 81, 122, 142, 104, 62, 251, 169, 98, 141, 21,
        253, 50, 130, 182, 15, 33, 109, 228, 31, 79, 183, 88, 147, 174, 108, 4, 22, 14, 129, 168,
        6, 80, 246, 254, 100, 218, 131, 94, 49, 247, 211, 3, 245, 22, 200, 177, 91, 60, 144, 147,
        174, 90, 17, 19, 189, 62, 147, 152, 18,
    ];
    let mut rows = Vec::new();

    for row in 0..2 {
        let mut pair = Vec::new();

        for column in 0..2 {
            let start = (row * 2 + column) * FIELD_BYTES;
            pair.push(format!(
                "0x{}",
                hex_encode(&proof_b[start..start + FIELD_BYTES])
            ));
        }

        rows.push(pair);
    }

    let mut value = proof_value(["0x0", "0x0"]);
    value["bs"] = json!(rows);
    value["krs"] = json!(["0x0", "0x0"]);
    let proof = parse_prover_proof(&value, CIRCUIT).expect("g2");
    assert_eq!(proof.b, proof_b);
    assert_eq!(proof.a, [0; 32]);
}

#[test]
fn proving_key_and_point_failures_match_the_typescript_cases() {
    assert!(matches!(
        parse_prover_proof(&Value::Null, "not-a-circuit"),
        Err(ProverFailure::UnknownCircuit)
    ));
    assert!(matches!(
        parse_prover_proof(&Value::Null, CIRCUIT),
        Err(ProverFailure::Server)
    ));
    let mut missing = proof_value(["0x0", "0x0"]);
    missing
        .as_object_mut()
        .expect("object")
        .remove("provingKeySha256");
    missing["krs"] = json!(["0x0", "0x0"]);
    assert!(matches!(
        parse_prover_proof(&missing, CIRCUIT),
        Err(ProverFailure::ProvingKeyMissing)
    ));
    let mut foreign = proof_value(["0x0", "0x0"]);
    foreign["provingKeySha256"] = json!("ab".repeat(32));

    foreign["krs"] = json!(["0x0", "0x0"]);
    assert!(matches!(
        parse_prover_proof(&foreign, CIRCUIT),
        Err(ProverFailure::ProvingKeyMismatch)
    ));
    let mut off = proof_value(["0x1", "0x1"]);
    off["krs"] = json!(["0x0", "0x0"]);
    assert!(matches!(
        parse_prover_proof(&off, CIRCUIT),
        Err(ProverFailure::ProofPoint)
    ));
}

#[test]
fn compressed_proof_is_the_transact_proof() {
    let proof = parse_prover_proof(&proof_value(["0x1", "0x2"]), CIRCUIT).expect("proof");
    let instruction = Withdraw {
        payer: address(2),
        input_trees: vec![STATE_TREE],
        output_tree: STATE_TREE,
        owner: address(3),
        nullifier_accounts: vec![address(NULLIFIER_BYTE)],
        settlement: Settlement::SolWithdrawal {
            recipient: address(12),
            amount: 7,
        },
        body: sample_body().with_proof(proof),
    }
    .instruction()
    .expect("instruction");
    assert!(instruction.data.windows(32).any(|window| window == proof.a));
    assert_eq!(instruction.accounts[0].role, AccountRole::WritableSigner);
}

#[test]
fn prover_urls_carry_no_api_key() {
    assert_eq!(
        crate::Cluster::Devnet.prover_url(),
        "https://devnet.helius-rpc.com/v1/spr"
    );
    assert_eq!(crate::local_prover_url(), "http://127.0.0.1:3001");
    assert!(!crate::Cluster::Mainnet.prover_url().contains("api-key"));
}

#[test]
fn an_api_key_in_the_url_is_refused_and_not_echoed() {
    let secret = "super-secret-key";
    let error = PrivacyProverClient::new(
        &format!("http://127.0.0.1:3001/?api-key={secret}"),
        CountingTransport::default(),
    )
    .expect_err("key in url");
    let rendered = format!("{error:?}");
    assert!(matches!(error, Error::Prover(ProverFailure::Config)));
    assert!(!rendered.contains(secret));
}

#[test]
fn an_api_key_before_the_path_is_refused_and_not_echoed() {
    let secret = "super-secret-key";

    for url in [
        format!("http://127.0.0.1:3001?api-key={secret}"),
        format!("http://127.0.0.1:3001/v1/spr?api-key={secret}"),
        format!("https://prover.example?%61pi-key={secret}"),
        "http://attacker.example?.localhost".to_owned(),
    ] {
        // Construction fails before the transport can ever be touched.
        let error =
            PrivacyProverClient::new(&url, CountingTransport::default()).expect_err("query in url");
        assert!(
            matches!(error, Error::Prover(ProverFailure::Config)),
            "{url}"
        );
        assert!(!format!("{error:?}").contains(secret));
    }

    let client = PrivacyProverClient::new("http://127.0.0.1:3001", CountingTransport::default())
        .expect("endpoint");
    let rendered = format!("{client:?}");
    assert!(rendered.contains("127.0.0.1:3001"));
    assert!(!rendered.contains('?'));
}

#[test]
fn a_malformed_or_encoded_authority_is_refused_before_the_transport() {
    for url in [
        "http://ex%41mple.com",
        "http://[not-an-ip]:3001",
        "http://2130706433",
        "http://bad_host.example",
        "http://999.999.999.999",
    ] {
        // Construction fails before the transport can ever be touched.
        let transport = CountingTransport::default();
        let calls = Arc::clone(&transport.calls);
        let error = PrivacyProverClient::new(url, transport).expect_err("malformed authority");
        assert!(
            matches!(error, Error::Prover(ProverFailure::Config)),
            "{url}"
        );
        assert_eq!(*calls.lock().expect("calls"), 0, "{url}");
    }
}

#[test]
fn a_remote_cleartext_prover_is_not_called() {
    let transport = CountingTransport::default();
    let calls = Arc::clone(&transport.calls);
    let client = PrivacyProverClient::new("https://devnet.helius-rpc.com/v1/spr", transport)
        .expect("endpoint");
    let error =
        futures::executor::block_on(client.prove(CIRCUIT, &json!({}))).expect_err("cleartext");
    assert!(matches!(error, Error::Prover(ProverFailure::Cleartext)));
    assert_eq!(*calls.lock().expect("calls"), 0);
}

#[test]
fn http_mock_covers_sync_async_timeout_and_errors() {
    let server = Mock::start();
    let witness = json!({"inputs": []});

    server.respond(HTTP_OK, proof_value(["0x1", "0x2"]).to_string());
    let prover = prover_client(&server, None);
    let proof = futures::executor::block_on(prover.prove(CIRCUIT, &witness)).expect("sync");
    assert_eq!(proof.a, negated_a());
    let seen = server.take();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].method, "POST");
    assert_eq!(seen[0].path, format!("/prove/{CIRCUIT}"));
    assert_eq!(seen[0].header("x-sync").as_deref(), Some("true"));
    assert!(seen[0].header("x-async").is_none());
    assert_eq!(seen[0].body, witness.to_string());
    assert!(!seen[0].path.contains("api-key"));

    server.respond(HTTP_TOO_MANY, "busy".to_owned());
    server.respond(HTTP_OK, json!({"jobId": "job_1"}).to_string());
    server.respond(
        HTTP_OK,
        json!({"status": "completed", "result": proof_value(["0x1", "0x2"])}).to_string(),
    );
    let proof = futures::executor::block_on(prover.prove(&format!("{CIRCUIT}.key"), &witness))
        .expect("async");
    assert_eq!(proof.c[31], 1);
    let seen = server.take();
    assert_eq!(seen[0].header("x-sync").as_deref(), Some("true"));
    assert_eq!(seen[1].header("x-async").as_deref(), Some("true"));

    assert_eq!(seen[1].path, format!("/prove/{CIRCUIT}"));
    assert_eq!(seen[2].method, "GET");
    assert_eq!(seen[2].path, format!("/prove/{CIRCUIT}/status?jobId=job_1"));
}

#[test]
fn http_mock_reports_timeout_malformed_bodies_and_bad_jobs() {
    let server = Mock::start();
    let witness = json!({"inputs": []});
    let prover = prover_client(&server, None);

    server.respond(HTTP_OK, json!({"jobId": "job_1"}).to_string());
    let timeout_client = prover_client(&server, None).with_poll(
        Duration::from_millis(1),
        Duration::from_millis(1),
        Duration::ZERO,
    );
    let error =
        futures::executor::block_on(timeout_client.prove(CIRCUIT, &witness)).expect_err("timeout");
    assert!(matches!(error, Error::Prover(ProverFailure::Timeout)));

    server.respond(HTTP_OK, "nope".to_owned());
    let error = futures::executor::block_on(prover.prove(CIRCUIT, &witness)).expect_err("json");
    assert!(matches!(error, Error::Prover(ProverFailure::Response)));

    server.respond(HTTP_ERROR, "down".to_owned());
    let error = futures::executor::block_on(prover.prove(CIRCUIT, &witness)).expect_err("http");
    assert!(matches!(error, Error::Prover(ProverFailure::Http(500))));

    server.respond(HTTP_OK, json!({"jobId": "bad id"}).to_string());
    let error = futures::executor::block_on(prover.prove(CIRCUIT, &witness)).expect_err("job");
    assert!(matches!(error, Error::Prover(ProverFailure::Job)));
}

#[test]
fn the_transport_sends_the_api_key_as_a_header() {
    let server = Mock::start();
    let mut zero = proof_value(["0x0", "0x0"]);
    zero["krs"] = json!(["0x0", "0x0"]);
    server.respond(HTTP_OK, zero.to_string());
    let prover = prover_client(&server, Some("header-key"));
    futures::executor::block_on(prover.prove(CIRCUIT, &json!({}))).expect("header");
    let seen = server.take();
    assert_eq!(seen[0].header("x-api-key").as_deref(), Some("header-key"));

    assert!(!seen[0].target.contains("api-key"));
}

#[test]
#[ignore = "set PRIVACY_RINGS_LOCAL_PROVER_URL to a loopback prover"]
fn local_prover_accepts_a_loopback_witness() {
    let Ok(url) = std::env::var("PRIVACY_RINGS_LOCAL_PROVER_URL") else {
        return;
    };
    let client = PrivacyProverClient::new(&url, HttpAdapter::new(None))
        .expect("endpoint")
        .with_poll(
            Duration::from_millis(SHORT_POLL_MS),
            Duration::from_millis(MEDIUM_POLL_MS),
            Duration::from_secs(3),
        );
    let result = futures::executor::block_on(
        client.prove(CIRCUIT, &json!({"circuitType": "transfer-confidential"})),
    );

    match result {
        Ok(proof) => assert_eq!(proof.a.len(), 32),
        Err(Error::Prover(ProverFailure::Http(status))) => {
            assert_ne!(status, 401, "a loopback prover must not require a key");
        }
        Err(Error::Prover(ProverFailure::Server | ProverFailure::Response)) => {}
        Err(Error::Prover(ProverFailure::Cleartext | ProverFailure::Config)) => {
            panic!("loopback prover was refused before the request");
        }
        Err(error) => panic!("local prover: {error:?}"),
    }
}

#[test]
#[ignore = "set PRIVACY_RINGS_LOCAL_PROVER_URL; does not use a Helius key"]
fn local_prover_proves_the_published_witness() {
    let Ok(url) = std::env::var("PRIVACY_RINGS_LOCAL_PROVER_URL") else {
        return;
    };
    let witness: Value =
        serde_json::from_str(include_str!("../../tests/fixtures/transfer-2x2.json"))
            .expect("witness");
    let client = PrivacyProverClient::new(&url, HttpAdapter::with_limit(None, 180))
        .expect("endpoint")
        .with_poll(
            Duration::from_millis(MEDIUM_POLL_MS),
            Duration::from_secs(1),
            Duration::from_secs(180),
        );
    let proof = futures::executor::block_on(client.prove("transfer_confidential_2_2", &witness))
        .expect("proof");
    assert_eq!(proof.a.len(), 32);
    assert_eq!(proof.b.len(), 128);
    assert_eq!(proof.c.len(), 32);
}

#[test]
#[ignore = "set HELIUS_PRIVACY_RINGS_LIVE=1 and HELIUS_API_KEY"]
fn live_prover_reaches_helius() {
    if std::env::var("HELIUS_PRIVACY_RINGS_LIVE").ok().as_deref() != Some("1") {
        return;
    }

    let key = std::env::var("HELIUS_API_KEY").expect("HELIUS_API_KEY");
    let url = std::env::var("HELIUS_PRIVACY_RINGS_PROVER_URL")
        .unwrap_or_else(|_| crate::Cluster::Devnet.prover_url());
    let client = PrivacyProverClient::new(&url, HttpAdapter::new(Some(key)))
        .expect("endpoint")
        .allow_unauthenticated_remote_prover(true)
        .with_poll(
            Duration::from_millis(SLOW_POLL_MS),
            Duration::from_secs(1),
            Duration::from_secs(POLL_BOUND_SECS),
        );
    let result = futures::executor::block_on(
        client.prove(CIRCUIT, &json!({"circuitType": "transfer-confidential"})),
    );

    match result {
        Ok(proof) => assert_eq!(proof.b.len(), 128),
        Err(Error::Prover(ProverFailure::Http(status))) => {
            assert_ne!(status, 401, "the API key was not accepted");
            assert!(
                (400..500).contains(&status),
                "unexpected prover status {status}"
            );
        }
        other => panic!("live prover did not answer with a proof or a witness error: {other:?}"),
    }
}

fn prover_client(server: &Mock, api_key: Option<&str>) -> PrivacyProverClient<HttpAdapter> {
    PrivacyProverClient::new(
        &format!("http://127.0.0.1:{}", server.port),
        HttpAdapter::new(api_key.map(str::to_owned)),
    )
    .expect("client")
    .with_poll(
        Duration::from_millis(1),
        Duration::from_millis(4),
        Duration::from_secs(2),
    )
}

fn address(byte: u8) -> Address {
    Address::from_bytes([byte; 32])
}

fn sample_body() -> TransactBody {
    TransactBody {
        expiry_unix_ts: u64::MAX,
        tx_viewing_pk: [4; 33],
        salt: [3; 16],
        data_hash: None,
        ring_data_hash: None,
        outputs: vec![ShieldedOutput {
            utxo_hash: [1; 32],
            owner_tag: OwnerTag::Account(2),
            data: None,
        }],
        messages: Vec::new(),
        private_tx_hash: [8; 32],
        circuit: Circuit::ConfidentialSigner(1, 2, 3),
        proof: Proof::zeroed(),
        inputs: vec![Input {
            nullifier_hash: [11; 32],
            tree_index: 0,
        }],
        tree_contexts: vec![TreeContext {
            utxo_tree_root_index: 4,
            nullifier_tree_root_index: 5,
        }],
    }
}

struct Seen {
    method: String,
    path: String,
    target: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Seen {
    fn header(&self, name: &str) -> Option<String> {
        self.headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.clone())
    }
}

struct Mock {
    port: u16,
    pending: Arc<Mutex<VecDeque<(u16, String)>>>,
    seen: Arc<Mutex<Vec<Seen>>>,
}

impl Mock {
    /// Binds an ephemeral loopback listener and answers the scripted statuses.
    ///
    /// The thread owns the socket so each test can drive a real HTTP exchange
    /// without a shared port.
    fn start() -> Self {
        let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))).expect("bind");
        let port = listener.local_addr().expect("addr").port();
        let pending = Arc::new(Mutex::new(VecDeque::new()));
        let seen = Arc::new(Mutex::new(Vec::new()));
        let pending_thread = Arc::clone(&pending);
        let seen_thread = Arc::clone(&seen);
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else { break };

                serve_one(&mut stream, &pending_thread, &seen_thread);
            }
        });
        Self {
            port,
            pending,
            seen,
        }
    }

    fn respond(&self, status: u16, body: String) {
        self.pending
            .lock()
            .expect("pending")
            .push_back((status, body));
    }

    fn take(&self) -> Vec<Seen> {
        std::mem::take(&mut *self.seen.lock().expect("seen"))
    }
}

struct RawRequest {
    seen: Seen,
}

/// Reads one HTTP/1.1 request from the test server.
///
/// The parser is local so the suite does not depend on an HTTP crate. Header
/// and body limits stay separate because a missing content-length must not
/// wait forever.
fn serve_one(
    stream: &mut TcpStream,
    pending: &Arc<Mutex<VecDeque<(u16, String)>>>,
    seen: &Arc<Mutex<Vec<Seen>>>,
) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
    let Some(request) = read_http(stream) else {
        return;
    };
    seen.lock().expect("seen").push(request.seen);
    let response = pending
        .lock()
        .expect("pending")
        .pop_front()
        .unwrap_or((HTTP_ERROR, "missing script".to_owned()));
    let _ = write_http(stream, response.0, &response.1);
}

fn read_http(stream: &mut TcpStream) -> Option<RawRequest> {
    let (buffer, header_end) = read_header_bytes(stream)?;
    let head = parse_request_head(&buffer[..header_end])?;
    let body = read_body_bytes(stream, &buffer, header_end, head.length)?;

    Some(RawRequest {
        seen: Seen {
            method: head.method,
            path: head.path,
            target: head.target,
            headers: head.headers,
            body,
        },
    })
}

/// Buffers until the header terminator. A client that never sends one is cut
/// off at the header budget so the test thread cannot block the suite.
fn read_header_bytes(stream: &mut TcpStream) -> Option<(Vec<u8>, usize)> {
    let mut buffer = Vec::new();
    let mut chunk = [0; 1024];

    loop {
        let read = stream.read(&mut chunk).ok()?;

        if read == 0 {
            return None;
        }

        buffer.extend_from_slice(&chunk[..read]);

        if let Some(end) = buffer.windows(4).position(|window| window == b"\r\n\r\n") {
            return Some((buffer, end));
        }

        if buffer.len() > HEADER_BUDGET {
            return None;
        }
    }
}

struct RequestHead {
    method: String,
    path: String,
    target: String,
    headers: Vec<(String, String)>,
    length: usize,
}

fn parse_request_head(head_bytes: &[u8]) -> Option<RequestHead> {
    let head = String::from_utf8_lossy(head_bytes).into_owned();
    let mut lines = head.split("\r\n");
    let request_line = lines.next()?;
    let mut parts = request_line.split(' ');
    let method = parts.next()?.to_owned();
    let target = parts.next()?.to_owned();
    let path = target.clone();
    let mut headers = Vec::new();
    let mut length = 0usize;

    for line in lines {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        let name = name.trim().to_owned();
        let value = value.trim().to_owned();

        if name.eq_ignore_ascii_case("content-length") {
            length = value.parse().unwrap_or(0);
        }

        headers.push((name, value));
    }

    Some(RequestHead {
        method,
        path,
        target,
        headers,
        length,
    })
}

fn read_body_bytes(
    stream: &mut TcpStream,
    buffer: &[u8],
    header_end: usize,
    length: usize,
) -> Option<String> {
    let mut body = buffer[header_end + 4..].to_vec();
    let mut chunk = [0; 1024];

    while body.len() < length {
        let read = stream.read(&mut chunk).ok()?;

        if read == 0 {
            break;
        }

        body.extend_from_slice(&chunk[..read]);
    }

    body.truncate(length);
    Some(String::from_utf8_lossy(&body).into_owned())
}

fn write_http(stream: &mut TcpStream, status: u16, body: &str) -> std::io::Result<()> {
    let reason = match status {
        HTTP_OK => "OK",
        HTTP_TOO_MANY => "Too Many Requests",
        _ => "Error",
    };

    write!(
        stream,
        "HTTP/1.1 {status} {reason}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
        body.len()
    )?;
    stream.flush()
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);

    for byte in bytes {
        out.push(HEX[usize::from(byte >> 4)] as char);
        out.push(HEX[usize::from(byte & 0x0f)] as char);
    }

    out
}

#[derive(Default)]
struct CountingTransport {
    calls: Arc<Mutex<usize>>,
}

impl rest::Transport<ProverDestination> for CountingTransport {
    type Exchange<'a>
        = Ready<Result<Reply, TransportError>>
    where
        Self: 'a;

    fn exchange(&self, _request: rest::Request<ProverDestination>) -> Self::Exchange<'_> {
        *self.calls.lock().expect("calls") += 1;
        std::future::ready(Err(TransportError::new(TransportFailureKind::Denied)))
    }
}

struct HttpAdapter {
    agent: ureq::Agent,
    api_key: Option<String>,
}

impl HttpAdapter {
    fn new(api_key: Option<String>) -> Self {
        Self::with_limit(api_key, REQUEST_TIMEOUT_SECS)
    }

    fn with_limit(api_key: Option<String>, seconds: u64) -> Self {
        let agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(seconds)))
            .http_status_as_error(false)
            .max_redirects(0)
            .build()
            .new_agent();
        Self { agent, api_key }
    }

    /// Sends one prover request and returns the raw reply.
    ///
    /// GET and POST use different ureq builders, so the header setup stays in
    /// two helpers instead of one generic call.
    fn execute(&self, request: rest::Request<ProverDestination>) -> Result<Reply, TransportError> {
        let content_type = request.body.content_type();
        let target = rest::request_target(
            request.service.endpoint.path(),
            &request.path_segments,
            &request.query,
            TARGET_BUDGET,
        )?;
        let url = format!("{}{target}", request.service.endpoint.origin());
        let method = request.method;
        let delivery = request.service.delivery;
        let bytes = request.body.into_bytes();
        let response = self.dispatch(method, delivery, &url, content_type, bytes)?;
        let status = response.status().as_u16();
        let body = response
            .into_body()
            .read_to_vec()
            .map_err(|error| TransportError::with_source(TransportFailureKind::Io, error))?;

        if body.len() > request.max_reply_bytes {
            return Err(TransportError::new(TransportFailureKind::Denied));
        }

        Ok(Reply { status, body })
    }

    fn dispatch(
        &self,
        method: Method,
        delivery: Delivery,
        url: &str,
        content_type: Option<&str>,
        bytes: Vec<u8>,
    ) -> Result<ureq::http::Response<ureq::Body>, TransportError> {
        match method {
            Method::Get => self
                .get(url)
                .map_err(|error| TransportError::with_source(TransportFailureKind::Io, error)),
            Method::Post => self
                .post(url, content_type, delivery, bytes)
                .map_err(|error| TransportError::with_source(TransportFailureKind::Io, error)),
            Method::Put | Method::Patch | Method::Delete => {
                Err(TransportError::new(TransportFailureKind::Denied))
            }
        }
    }

    fn get(&self, url: &str) -> Result<ureq::http::Response<ureq::Body>, ureq::Error> {
        let mut call = self.agent.get(url).header("accept", "application/json");

        if let Some(key) = &self.api_key {
            call = call.header("x-api-key", key);
        }

        call.call()
    }

    fn post(
        &self,
        url: &str,
        content_type: Option<&str>,
        delivery: Delivery,
        bytes: Vec<u8>,
    ) -> Result<ureq::http::Response<ureq::Body>, ureq::Error> {
        let mut call = self
            .agent
            .post(url)
            .header("accept", "application/json")
            .header("content-type", content_type.unwrap_or("application/json"));

        call = match delivery {
            Delivery::Sync => call.header("X-Sync", "true"),
            Delivery::Async => call.header("X-Async", "true"),
            Delivery::Status => call,
        };

        if let Some(key) = &self.api_key {
            call = call.header("x-api-key", key);
        }

        call.send(bytes)
    }
}

type RecordedQuery = Vec<(String, String)>;

struct Scripted {
    replies: Mutex<VecDeque<(u16, &'static str)>>,
    paths: Arc<Mutex<Vec<Vec<String>>>>,
    queries: Arc<Mutex<Vec<RecordedQuery>>>,
    deliveries: Arc<Mutex<Vec<Delivery>>>,
    bodies: Arc<Mutex<Vec<Option<Vec<u8>>>>>,
}

impl rest::Transport<ProverDestination> for Scripted {
    type Exchange<'a>
        = Ready<Result<Reply, TransportError>>
    where
        Self: 'a;

    fn exchange(&self, request: rest::Request<ProverDestination>) -> Self::Exchange<'_> {
        self.paths
            .lock()
            .expect("paths")
            .push(request.path_segments);
        self.queries.lock().expect("queries").push(request.query);
        self.deliveries
            .lock()
            .expect("deliveries")
            .push(request.service.delivery);
        self.bodies
            .lock()
            .expect("bodies")
            .push(request.body.as_deref().map(<[u8]>::to_vec));
        let (status, body) = self
            .replies
            .lock()
            .expect("replies")
            .pop_front()
            .expect("reply");

        if status == 0 {
            return std::future::ready(Err(TransportError::new(TransportFailureKind::Denied)));
        }

        std::future::ready(Ok(Reply {
            status,
            body: body.as_bytes().to_vec(),
        }))
    }
}

#[test]
fn indexed_async_poll_uses_the_key_status_route() {
    let completed = format!(
        r#"{{"status":"completed","result":{}}}"#,
        serde_json::to_string(&proof_value(["0x1", "0x2"])).expect("json")
    );
    let completed: &'static str = Box::leak(completed.into_boxed_str());
    let paths = Arc::new(Mutex::new(Vec::new()));
    let queries = Arc::new(Mutex::new(Vec::new()));
    let deliveries = Arc::new(Mutex::new(Vec::new()));
    let bodies = Arc::new(Mutex::new(Vec::new()));
    let witness = json!({"circuitType": "transfer-confidential"});
    let body = witness.to_string();
    let scripted = Scripted {
        replies: Mutex::new(VecDeque::from([
            (HTTP_TOO_MANY, "{}"),
            (HTTP_OK, r#"{"jobId":"job_1"}"#),
            (HTTP_OK, r#"{"status":"queued"}"#),
            (HTTP_OK, completed),
        ])),
        paths: Arc::clone(&paths),
        queries: Arc::clone(&queries),
        deliveries: Arc::clone(&deliveries),
        bodies: Arc::clone(&bodies),
    };
    let client = PrivacyProverClient::new("http://127.0.0.1:9", scripted)
        .expect("endpoint")
        .with_poll(
            Duration::from_millis(1),
            Duration::from_millis(1),
            Duration::from_secs(2),
        );

    let proof =
        futures::executor::block_on(client.prove_indexed(CIRCUIT, &witness)).expect("proof");
    assert_eq!(proof.a, negated_a());

    let seen = paths.lock().expect("paths").clone();
    assert_eq!(seen.len(), 4);
    assert_eq!(seen[0], ["prove", CIRCUIT, "indexed"]);
    assert_eq!(seen[1], ["prove", CIRCUIT, "indexed"]);
    assert_eq!(seen[2], ["prove", CIRCUIT, "status"]);
    assert_eq!(seen[3], ["prove", CIRCUIT, "status"]);
    let deliveries = deliveries.lock().expect("deliveries").clone();
    assert_eq!(
        deliveries,
        [
            Delivery::Sync,
            Delivery::Async,
            Delivery::Status,
            Delivery::Status
        ]
    );
    let queries = queries.lock().expect("queries").clone();
    assert_eq!(queries[0], Vec::<(String, String)>::new());
    assert_eq!(queries[1], Vec::<(String, String)>::new());
    assert_eq!(queries[2], [("jobId".to_owned(), "job_1".to_owned())]);
    assert_eq!(queries[3], queries[2]);
    let bodies = bodies.lock().expect("bodies").clone();
    assert_eq!(bodies[0].as_deref(), Some(body.as_bytes()));
    assert_eq!(bodies[1], bodies[0]);
    assert_eq!(bodies[2], None);
    assert_eq!(bodies[3], None);
}

#[test]
fn indexed_route_and_attestation_use_their_paths() {
    let paths = Arc::new(Mutex::new(Vec::new()));
    let queries = Arc::new(Mutex::new(Vec::new()));
    let scripted = Scripted {
        replies: Mutex::new(VecDeque::from([
            (404, "{}"),
            (404, "not-json"),
            (404, "{\"code\":\"missing\"}"),
            (500, "{}"),
            (200, "not-json"),
            (200, "{\"platform\":\"dstack-tdx\"}"),
        ])),
        paths: Arc::clone(&paths),
        queries: Arc::clone(&queries),
        deliveries: Arc::new(Mutex::new(Vec::new())),
        bodies: Arc::new(Mutex::new(Vec::new())),
    };
    let client = PrivacyProverClient::new("http://127.0.0.1:9", scripted).expect("endpoint");
    let witness = json!({});
    let missing = futures::executor::block_on(client.prove_indexed(CIRCUIT, &witness));
    assert!(matches!(
        missing,
        Err(Error::Prover(ProverFailure::Indexer))
    ));
    let garbled = futures::executor::block_on(client.prove_indexed(CIRCUIT, &witness));
    assert!(matches!(
        garbled,
        Err(Error::Prover(ProverFailure::Indexer))
    ));
    let coded = futures::executor::block_on(client.prove_indexed(CIRCUIT, &witness));
    assert!(matches!(
        coded,
        Err(Error::Prover(ProverFailure::Http(404)))
    ));
    let nonce = [0x11; FIELD_BYTES];
    let unavailable = futures::executor::block_on(client.fetch_unverified_attestation(&nonce));
    assert!(matches!(
        unavailable,
        Err(Error::Prover(ProverFailure::Http(500)))
    ));
    let garbled_document = futures::executor::block_on(client.fetch_unverified_attestation(&nonce));
    assert!(matches!(
        garbled_document,
        Err(Error::Prover(ProverFailure::Response))
    ));
    let document =
        futures::executor::block_on(client.fetch_unverified_attestation(&nonce)).expect("document");
    assert_eq!(document["platform"], "dstack-tdx");
    let seen = paths.lock().expect("paths").clone();
    assert_eq!(seen[0], ["prove", CIRCUIT, "indexed"]);
    assert_eq!(seen[5], ["attestation"]);
    let seen_queries = queries.lock().expect("queries").clone();
    assert_eq!(seen_queries[5][0].0, "nonce");
    assert_eq!(seen_queries[5][0].1, "11".repeat(FIELD_BYTES));
}

fn script(replies: Vec<(u16, &'static str)>) -> Scripted {
    Scripted {
        replies: Mutex::new(VecDeque::from(replies)),
        paths: Arc::new(Mutex::new(Vec::new())),
        queries: Arc::new(Mutex::new(Vec::new())),
        deliveries: Arc::new(Mutex::new(Vec::new())),
        bodies: Arc::new(Mutex::new(Vec::new())),
    }
}

#[test]
fn prover_client_covers_rejected_routes_and_queued_retry() {
    let witness = json!({});
    let remote = PrivacyProverClient::new("http://example.com", script(Vec::new()))
        .expect("endpoint")
        .allow_unauthenticated_remote_prover(true);
    assert!(format!("{remote:?}").contains("example.com"));
    let blocked = futures::executor::block_on(remote.prove(CIRCUIT, &witness));
    assert!(matches!(blocked, Err(Error::Prover(ProverFailure::Config))));
    let local = PrivacyProverClient::new("http://127.0.0.1:9", script(Vec::new())).expect("local");
    assert!(matches!(
        futures::executor::block_on(local.prove("a/b", &witness)),
        Err(Error::Prover(ProverFailure::Circuit))
    ));
    assert!(matches!(
        futures::executor::block_on(local.prove("not-a-key", &witness)),
        Err(Error::Prover(ProverFailure::UnknownCircuit))
    ));
    assert!(
        local
            .wait(Duration::from_secs(1), std::time::Instant::now())
            .is_err()
    );
    let busy = PrivacyProverClient::new(
        "http://127.0.0.1:9",
        script(vec![(HTTP_TOO_MANY, "{}"), (HTTP_TOO_MANY, "{}")]),
    )
    .expect("busy");
    assert!(matches!(
        futures::executor::block_on(busy.prove(CIRCUIT, &witness)),
        Err(Error::Prover(ProverFailure::Http(HTTP_TOO_MANY)))
    ));
    let completed = format!(
        r#"{{"status":"completed","result":{}}}"#,
        serde_json::to_string(&proof_value(["0x1", "0x2"])).expect("json")
    );
    let leaked: &'static str = Box::leak(completed.into_boxed_str());
    let queued = PrivacyProverClient::new(
        "http://127.0.0.1:9",
        script(vec![
            (HTTP_OK, r#"{"jobId":"job-1"}"#),
            (HTTP_ERROR, "{}"),
            (HTTP_OK, leaked),
        ]),
    )
    .expect("queued")
    .with_poll(
        Duration::from_millis(1),
        Duration::from_millis(1),
        Duration::from_secs(2),
    );
    let proof = futures::executor::block_on(queued.prove(CIRCUIT, &witness)).expect("queued proof");
    assert_eq!(proof.a.len(), 32);
    let mut wrapped = proof_value(["0x1", "0x2"]);
    let inner = wrapped.clone();
    wrapped = json!({ "proof": inner });
    assert!(parse_prover_proof(&wrapped, CIRCUIT).is_ok());
    let mut upper = proof_value(["0x1", "0x2"]);
    upper["provingKeySha256"] = json!("AB".repeat(32));
    assert!(matches!(
        parse_prover_proof(&upper, CIRCUIT),
        Err(ProverFailure::Response)
    ));
    let mut number = proof_value(["0x1", "0x2"]);
    number["provingKeySha256"] = json!(1);
    assert!(matches!(
        parse_prover_proof(&number, CIRCUIT),
        Err(ProverFailure::Response)
    ));
    let mut empty_commitment = proof_value(["0x1", "0x2"]);
    empty_commitment["proofCommitment"] = json!([]);
    assert!(parse_prover_proof(&empty_commitment, CIRCUIT).is_ok());
    let mut bad_point = proof_value(["0x1", "0x2"]);
    bad_point["ar"] = json!(["zz", "0x1"]);
    assert!(parse_prover_proof(&bad_point, CIRCUIT).is_err());
    let mut commitment = proof_value(["0x1", "0x2"]);
    commitment["proofCommitment"] = json!(["0x0", "0x0"]);
    commitment["proofCommitmentPok"] = json!(["0x0", "0x0"]);
    assert!(parse_prover_proof(&commitment, CIRCUIT).is_ok());
    let mut short_g1 = proof_value(["0x1", "0x2"]);
    short_g1["ar"] = json!(["0x1"]);
    assert!(parse_prover_proof(&short_g1, CIRCUIT).is_err());
    let mut short_g2 = proof_value(["0x1", "0x2"]);
    short_g2["bs"] = json!([["0x0", "0x0"]]);
    assert!(parse_prover_proof(&short_g2, CIRCUIT).is_err());
    let mut row = proof_value(["0x1", "0x2"]);
    row["bs"] = json!([["0x0"], ["0x0", "0x0"]]);
    assert!(parse_prover_proof(&row, CIRCUIT).is_err());
    let mut wide = proof_value(["0x1", "0x2"]);
    wide["ar"] = json!([format!("0x{}", "f".repeat(65)), "0x1"]);
    assert!(parse_prover_proof(&wide, CIRCUIT).is_err());
    let mut upper_digit = proof_value(["0x1", "0x2"]);
    upper_digit["bs"] = json!([["0xA", "0x0"], ["0x0", "0x0"]]);
    assert!(parse_prover_proof(&upper_digit, CIRCUIT).is_ok());
    let mut modulus = proof_value(["0x1", "0x2"]);
    modulus["ar"] = json!([
        "0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47",
        "0x1"
    ]);
    assert!(parse_prover_proof(&modulus, CIRCUIT).is_err());
    let mut text = proof_value(["0x1", "0x2"]);
    text["ar"] = json!("no");
    assert!(parse_prover_proof(&text, CIRCUIT).is_err());
    let gnark = proof_value(["0x1", "0x2"]);
    let both = json!({"jobId": "job-9", "proof": gnark});
    let both_text: &'static str = Box::leak(both.to_string().into_boxed_str());
    let mixed = PrivacyProverClient::new("http://127.0.0.1:9", script(vec![(HTTP_OK, both_text)]))
        .expect("mixed");
    assert!(futures::executor::block_on(mixed.prove(CIRCUIT, &witness)).is_ok());
    let secure =
        PrivacyProverClient::new("https://example.com", script(vec![(HTTP_OK, both_text)]))
            .expect("https")
            .allow_unauthenticated_remote_prover(true);
    assert!(futures::executor::block_on(secure.prove(CIRCUIT, &witness)).is_ok());
    let huge = "x".repeat(super::MAX_REPLY_BYTES + 1);
    let huge_text: &'static str = Box::leak(huge.into_boxed_str());
    let oversized =
        PrivacyProverClient::new("http://127.0.0.1:9", script(vec![(HTTP_OK, huge_text)]))
            .expect("oversized");
    assert!(matches!(
        futures::executor::block_on(oversized.prove(CIRCUIT, &witness)),
        Err(Error::Prover(ProverFailure::Response))
    ));
    let rejected = PrivacyProverClient::new(
        "http://127.0.0.1:9",
        script(vec![(HTTP_OK, r#"{"jobId":"job-4"}"#), (400, "{}")]),
    )
    .expect("status");
    assert!(matches!(
        futures::executor::block_on(rejected.prove(CIRCUIT, &witness)),
        Err(Error::Prover(ProverFailure::Http(400)))
    ));
    let shaped = PrivacyProverClient::new(
        "http://127.0.0.1:9",
        script(vec![(HTTP_OK, r#"{"jobId":"job-5"}"#), (HTTP_OK, "[]")]),
    )
    .expect("array");
    assert!(matches!(
        futures::executor::block_on(shaped.prove(CIRCUIT, &witness)),
        Err(Error::Prover(ProverFailure::Response))
    ));
    let pending = PrivacyProverClient::new(
        "http://127.0.0.1:9",
        script(vec![
            (HTTP_OK, r#"{"jobId":"job-6"}"#),
            (HTTP_OK, r#"{"status":"working"}"#),
            (HTTP_OK, leaked),
        ]),
    )
    .expect("pending")
    .with_poll(
        Duration::from_millis(1),
        Duration::from_millis(1),
        Duration::from_secs(2),
    );
    assert!(futures::executor::block_on(pending.prove(CIRCUIT, &witness)).is_ok());
    let dropped = PrivacyProverClient::new(
        "http://127.0.0.1:9",
        script(vec![(HTTP_OK, r#"{"jobId":"job-7"}"#), (0, "")]),
    )
    .expect("dropped");
    assert!(matches!(
        futures::executor::block_on(dropped.prove(CIRCUIT, &witness)),
        Err(Error::Prover(ProverFailure::Transport))
    ));
    let mut blank = proof_value(["0x1", "0x2"]);
    blank["ar"] = json!(["0x", "0x1"]);
    assert!(parse_prover_proof(&blank, CIRCUIT).is_err());
    let clear = PrivacyProverClient::new("http://example.com", script(Vec::new())).expect("clear");
    assert!(matches!(
        futures::executor::block_on(clear.prove(CIRCUIT, &witness)),
        Err(Error::Prover(ProverFailure::Cleartext))
    ));
    let bad_job = PrivacyProverClient::new(
        "http://127.0.0.1:9",
        script(vec![(HTTP_OK, r#"{"jobId":"bad id"}"#)]),
    )
    .expect("bad job");
    assert!(matches!(
        futures::executor::block_on(bad_job.prove(CIRCUIT, &witness)),
        Err(Error::Prover(ProverFailure::Job))
    ));
    let expired = PrivacyProverClient::new(
        "http://127.0.0.1:9",
        script(vec![(HTTP_OK, r#"{"jobId":"job-8"}"#)]),
    )
    .expect("expired")
    .with_poll(Duration::ZERO, Duration::from_millis(1), Duration::ZERO);
    assert!(matches!(
        futures::executor::block_on(expired.prove(CIRCUIT, &witness)),
        Err(Error::Prover(ProverFailure::Timeout))
    ));
}

impl rest::Transport<ProverDestination> for HttpAdapter {
    type Exchange<'a>
        = Ready<Result<Reply, TransportError>>
    where
        Self: 'a;

    fn exchange(&self, request: rest::Request<ProverDestination>) -> Self::Exchange<'_> {
        std::future::ready(self.execute(request))
    }
}
