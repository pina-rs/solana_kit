//! Owns loopback fixtures and joins them after the concrete browser producer ends.
use serde_json::{Value, json};
use std::{
    env,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread,
    time::Duration,
};
use tungstenite::{Message, accept};
const HEADER_BYTES: usize = 16_384;
const ROOT_REMOTE: u64 = 11;
const SIGNATURE_REMOTE: u64 = 12;
const ROOT_VALUE: u64 = 99;
const SOCKET_TIMEOUT: u64 = 5;
const DELIVERY_DELAY: u64 = 20;
type Socket = tungstenite::WebSocket<TcpStream>;

fn request(stream: &mut TcpStream) -> Option<(String, Vec<u8>)> {
    let mut bytes = Vec::new();
    let mut byte = [0];

    while !bytes.ends_with(b"\r\n\r\n") {
        if bytes.len() >= HEADER_BYTES {
            return None;
        }

        stream.read_exact(&mut byte).ok()?;
        bytes.push(byte[0]);
    }

    let headers = String::from_utf8(bytes).unwrap();

    let length: usize = headers
        .lines()
        .find_map(|line| {
            line.to_ascii_lowercase()
                .strip_prefix("content-length: ")
                .map(str::to_owned)
        })
        .map_or(0, |text| text.parse().unwrap());
    let mut body = vec![0; length];
    stream.read_exact(&mut body).ok()?;
    Some((headers, body))
}
fn http(
    listener: TcpListener,
    stop: Arc<AtomicBool>,
    registered: Arc<AtomicBool>,
    reads: Arc<AtomicUsize>,
) {
    while !stop.load(Ordering::SeqCst) {
        let Some(mut stream) = accept_local(&listener, &stop) else {
            return;
        };
        http_connection(&mut stream, &registered, &reads);
    }
}
fn http_connection(stream: &mut TcpStream, registered: &AtomicBool, reads: &AtomicUsize) {
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    let Some((headers, body)) = request(stream) else {
        return;
    };
    let body = if headers.starts_with("OPTIONS ") {
        Vec::new()
    } else {
        assert!(
            registered.load(Ordering::SeqCst),
            "signature installed before HTTP"
        );
        reads.fetch_add(1, Ordering::SeqCst);

        rpc_result(&body)
    };

    let headers = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: content-type\r\nAccess-Control-Allow-Methods: POST, OPTIONS\r\nConnection: close\r\n\r\n",
        body.len()
    );
    // Cancellation may close Fetch after the WebSocket wins the race.
    if stream.write_all(headers.as_bytes()).is_ok() {
        let _cancelled = stream.write_all(&body);
    }
}
fn rpc_result(body: &[u8]) -> Vec<u8> {
    let request: Value = serde_json::from_slice(body).unwrap();

    let result = match request["method"].as_str().unwrap() {
        "getSignatureStatuses" => json!({"context":{"slot":10},"value":[null]}),
        "getEpochInfo" => json!({"absoluteSlot":10,"blockHeight":10,"epoch":0,
            "slotIndex":10,"slotsInEpoch":100,"transactionCount":0}),
        _ => panic!("unexpected HTTP"),
    };

    serde_json::to_vec(&json!({"jsonrpc":"2.0","id":request["id"],"result":result})).unwrap()
}
fn accept_local(listener: &TcpListener, stop: &AtomicBool) -> Option<TcpStream> {
    while !stop.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _)) => return Some(stream),

            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(2));
            }

            Err(error) => panic!("loopback accept: {error}"),
        }
    }

    None
}
fn socket(
    listener: TcpListener,
    stop: Arc<AtomicBool>,
    registered: Arc<AtomicBool>,
    reads: Arc<AtomicUsize>,
) {
    let Some(stream) = accept_local(&listener, &stop) else {
        return;
    };
    stream.set_nonblocking(false).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(SOCKET_TIMEOUT)))
        .unwrap();
    let mut socket = accept(stream).unwrap();

    register(&mut socket, &registered);
    early_notifications(&mut socket);
    confirm(&mut socket, &reads);
    close(&mut socket);
}
fn register(socket: &mut Socket, registered: &AtomicBool) {
    for (id, method) in [(1, "rootSubscribe"), (2, "signatureSubscribe")] {
        let Message::Text(text) = socket.read().unwrap() else {
            panic!("subscribe");
        };
        let request: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(request["method"], method);

        if id == 2 {
            registered.store(true, Ordering::SeqCst);
        }

        socket
            .send(Message::Text(
                json!({"jsonrpc":"2.0","id":id,"result":10+id})
                    .to_string()
                    .into(),
            ))
            .unwrap();
    }
}
fn early_notifications(socket: &mut Socket) {
    for (method, id, result) in [
        ("rootNotification", ROOT_REMOTE, json!(ROOT_VALUE)),
        (
            "signatureNotification",
            SIGNATURE_REMOTE,
            json!({"context":{"slot":10},"value":"receivedSignature"}),
        ),
    ] {
        socket
            .send(Message::Text(
                json!({"jsonrpc":"2.0","method":method,
            "params":{"subscription":id,"result":result}})
                .to_string()
                .into(),
            ))
            .unwrap();
    }
}
fn confirm(socket: &mut Socket, reads: &AtomicUsize) {
    for _ in 0..500 {
        if reads.load(Ordering::SeqCst) > 0 {
            break;
        }

        thread::sleep(Duration::from_millis(2));
    }

    assert!(
        reads.load(Ordering::SeqCst) > 0,
        "actual Fetch request required"
    );
    thread::sleep(Duration::from_millis(DELIVERY_DELAY));
    socket
        .send(Message::Text(
            json!({"jsonrpc":"2.0","method":"signatureNotification",
        "params":{"subscription":SIGNATURE_REMOTE,"result":{"context":{"slot":10},"value":{"err":null}}}})
            .to_string()
            .into(),
        ))
        .unwrap();
}
fn close(socket: &mut Socket) {
    let Message::Text(text) = socket.read().unwrap() else {
        panic!("root cleanup");
    };
    let request: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(request["method"], "rootUnsubscribe");
    socket
        .send(Message::Text(
            json!({"jsonrpc":"2.0","id":request["id"],"result":true})
                .to_string()
                .into(),
        ))
        .unwrap();
    assert!(matches!(socket.read().unwrap(), Message::Close(_)));

    match socket.flush() {
        Ok(()) | Err(tungstenite::Error::ConnectionClosed) => (),
        Err(error) => panic!("close completion: {error}"),
    }
}
fn main() {
    let mut arguments = env::args().skip(1);
    let cargo = arguments.next().expect("concrete cargo");
    let target = arguments.next().expect("owned target");
    let socket_listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let http_listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let ws_endpoint = format!("ws://{}", socket_listener.local_addr().unwrap());
    let http_endpoint = format!("http://{}", http_listener.local_addr().unwrap());
    http_listener.set_nonblocking(true).unwrap();

    socket_listener.set_nonblocking(true).unwrap();
    let stop = Arc::new(AtomicBool::new(false));
    let registered = Arc::new(AtomicBool::new(false));
    let reads = Arc::new(AtomicUsize::new(0));
    let http_owner = {
        let (stop, registered, reads) = (stop.clone(), registered.clone(), reads.clone());
        thread::spawn(move || http(http_listener, stop, registered, reads))
    };
    let ws_stop = stop.clone();
    let ws_owner = thread::spawn(move || socket(socket_listener, ws_stop, registered, reads));
    let result = Command::new(cargo)
        .args([
            "test",
            "--offline",
            "--locked",
            "-vv",
            "--message-format=json",
            "--lib",
            "--target",
            "wasm32-unknown-unknown",
            "--target-dir",
            &target,
            "--",
            "--nocapture",
        ])
        .env("SOLANA_FACTORY_WS", ws_endpoint)
        .env("SOLANA_FACTORY_HTTP", http_endpoint)
        .env("WASM_BINDGEN_USE_BROWSER", "1")
        .env("WASM_BINDGEN_USE_BROWSER_ONLY", "1")
        .status()
        .unwrap();

    stop.store(true, Ordering::SeqCst);
    http_owner.join().unwrap();
    ws_owner.join().unwrap();
    assert!(result.success(), "actual browser failed");
    println!(
        "actual browser HTTP+WS factory, Promise/Rc and original portable bodies passed; both fixture owners joined"
    );
}
