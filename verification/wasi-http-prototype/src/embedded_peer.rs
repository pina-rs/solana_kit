//! Independent peers observe wire credentials, TLS rejection and socket lifetime.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const REPLY: &str = "{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"context\":{\"slot\":7},\"value\":18446744073709551615}}";
const ROUNDS: usize = 12;
const OBSERVE: Duration = Duration::from_millis(350);
const CANCEL_LIMIT: Duration = Duration::from_millis(250);
const REDIRECT: u16 = 302;
const OK: u16 = 200;

pub struct Peer {
    pub port: u16,
    task: JoinHandle<()>,
}

impl Peer {
    pub fn start(mode: &str) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("loopback listener");
        let port = listener.local_addr().expect("port").port();
        listener.set_nonblocking(true).expect("nonblocking");
        let mode = mode.to_owned();
        let task = thread::spawn(move || serve(listener, &mode));
        Self { port, task }
    }

    pub fn finish(self) {
        self.task.join().expect("independent peer assertions");
    }
}

/// Denied destinations have no socket; repeated calls each own a fresh connection.
fn serve(listener: TcpListener, mode: &str) {
    if matches!(
        mode,
        "baseline"
            | "denied"
            | "denied-upload"
            | "denied-response"
            | "repeated-denied"
            | "dns-deadline"
    ) {
        observe_no_connection(&listener);

        return;
    }

    let rounds = if mode.starts_with("repeated") {
        ROUNDS
    } else {
        1
    };

    for _ in 0..rounds {
        connection(accept(&listener), mode);
    }
}

fn connection(mut socket: TcpStream, mode: &str) {
    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("read timeout");

    match mode {
        "malformed-chunk" | "truncated" => framing(&mut socket, mode),
        "early-status" => early_status(&mut socket),
        "upload-deadline" => thread::sleep(OBSERVE),
        "cancel" | "repeated-cancel" | "store-drop" | "tls-deadline" => {
            cancelled(&mut socket, mode == "tls-deadline")
        }
        "success" | "tls-untrusted" | "tls-name" => tls(socket, mode),
        _ => respond(&mut socket, mode),
    }
}

fn framing(socket: &mut TcpStream, mode: &str) {
    request(socket);
    let bytes = if mode == "truncated" {
        "HTTP/1.1 200 OK\r\nContent-Length: 100\r\nConnection: close\r\n\r\n{"
    } else {
        "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\nnot-hex\r\n{\r\n"
    };

    socket.write_all(bytes.as_bytes()).expect("framing fixture");
    socket.flush().expect("flush fixture");
}

fn early_status(socket: &mut TcpStream) {
    let mut bytes = Vec::new();
    let mut part = [0; 512];

    while !bytes.windows(4).any(|bytes| bytes == b"\r\n\r\n") {
        let count = socket.read(&mut part).expect("upload headers");
        assert_ne!(count, 0);
        bytes.extend_from_slice(&part[..count]);
        assert!(bytes.len() < 8192);
    }

    socket
        .write_all(b"HTTP/1.1 413 Fixture\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
        .expect("early status");
    socket.flush().expect("status flush");
    thread::sleep(Duration::from_millis(100));
}

fn accept(listener: &TcpListener) -> TcpStream {
    let deadline = Instant::now() + Duration::from_secs(4);

    loop {
        match listener.accept() {
            Ok((socket, _)) => {
                socket.set_nonblocking(false).expect("blocking peer");
                return socket;
            }

            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(error) => panic!("accept: {error}"),
        }

        assert!(Instant::now() < deadline, "guest did not connect");
        thread::sleep(Duration::from_millis(2));
    }
}

fn observe_no_connection(listener: &TcpListener) {
    let deadline = Instant::now() + OBSERVE;

    while Instant::now() < deadline {
        let result = listener.accept();
        assert!(
            matches!(result, Err(error) if error.kind() == std::io::ErrorKind::WouldBlock),
            "denied or unresolved destination was contacted"
        );
        thread::sleep(Duration::from_millis(2));
    }
}

fn tls(mut socket: TcpStream, mode: &str) {
    use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer};
    let certificate =
        CertificateDer::from(include_bytes!("../../wasi-sockets/fixtures/server.der").to_vec());
    let key = PrivatePkcs8KeyDer::from(
        include_bytes!("../../wasi-sockets/fixtures/server-key.der").to_vec(),
    );
    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![certificate], key.into())
        .expect("public fixture identity");
    let mut connection = rustls::ServerConnection::new(Arc::new(config)).expect("TLS server");

    if mode != "success" {
        assert!(
            connection.complete_io(&mut socket).is_err(),
            "invalid TLS accepted"
        );

        return;
    }

    connection
        .complete_io(&mut socket)
        .expect("trusted localhost TLS");
    respond(&mut rustls::Stream::new(&mut connection, &mut socket), mode);
}

fn respond(stream: &mut (impl Read + Write), mode: &str) {
    let identifier = request(stream);
    let body = REPLY.replace("\"id\":1", &format!("\"id\":{identifier}"));
    let status = if mode == "redirect" { REDIRECT } else { OK };
    let target = TcpListener::bind("127.0.0.1:0").expect("redirect observer");
    target.set_nonblocking(true).expect("nonblocking target");
    let port = target.local_addr().expect("target port").port();
    write!(stream, "HTTP/1.1 {status} Fixture\r\nContent-Length: {}\r\nLocation: http://127.0.0.1:{port}/credential-probe\r\nConnection: close\r\n\r\n{body}", body.len()).expect("reply");
    stream.flush().expect("flush");

    if mode == "redirect" {
        observe_no_connection(&target);
    }
}

fn request(stream: &mut impl Read) -> u64 {
    let mut bytes = Vec::new();
    let mut part = [0; 1024];

    loop {
        let count = stream.read(&mut part).expect("request");
        assert_ne!(count, 0);
        bytes.extend_from_slice(&part[..count]);
        assert!(bytes.len() < 8192);

        if let Some(end) = bytes.windows(4).position(|b| b == b"\r\n\r\n") {
            let headers = std::str::from_utf8(&bytes[..end])
                .expect("headers")
                .to_ascii_lowercase();
            let length = headers
                .lines()
                .find_map(|line| line.strip_prefix("content-length: "))
                .expect("length")
                .parse::<usize>()
                .expect("length digits");

            if bytes.len() == end + 4 + length {
                assert!(headers.starts_with("post /rpc http/1.1\r\n"));
                assert!(headers.contains("authorization: bearer fixture-only"));
                assert!(!headers.contains("cookie:"));
                assert!(headers.contains("content-type: application/json"));
                assert!(
                    std::str::from_utf8(&bytes[end + 4..])
                        .expect("body")
                        .contains("\"method\":\"getBalance\"")
                );

                let body = std::str::from_utf8(&bytes[end + 4..]).expect("body UTF8");

                return body
                    .split("\"id\":")
                    .nth(1)
                    .expect("request id")
                    .split(',')
                    .next()
                    .expect("id delimiter")
                    .parse()
                    .expect("numeric id");
            }
        }
    }
}

/// TLS reads handshake bytes until EOF, while HTTP has already consumed the request.
fn cancelled(socket: &mut TcpStream, tls: bool) {
    if !tls {
        request(socket);
    }

    let start = Instant::now();
    let mut bytes = [0; 1024];

    loop {
        match socket.read(&mut bytes) {
            Ok(0) => break,

            Ok(_) if tls => {}
            other => panic!("pending socket did not close: {other:?}"),
        }
    }

    assert!(
        start.elapsed() < CANCEL_LIMIT,
        "cancellation exceeded observation bound"
    );
}
