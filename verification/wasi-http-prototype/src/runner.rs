//! Independent bounded loopback peer and actual component supervisor.

mod graphs;
mod imports;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const REPLY: &str = "{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"context\":{\"slot\":7},\"value\":18446744073709551615}}";
const REQUEST: &str = "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"getBalance\",\"params\":[\"11111111111111111111111111111111\"]}";
const PROCESS_LIMIT: Duration = Duration::from_secs(5);
const TICK: Duration = Duration::from_millis(5);
const QUIET: Duration = Duration::from_millis(250);
const OK: u16 = 200;
const UNAVAILABLE: u16 = 503;
const REDIRECT: u16 = 302;

const OVERSIZE: usize = 8193;
const CHUNK: usize = 11;

fn main() {
    graphs::check();
    let host = std::env::args().nth(1).expect("Wasmtime50 executable");
    let version = Command::new(&host)
        .arg("--version")
        .output()
        .expect("version");
    assert_eq!(
        String::from_utf8(version.stdout).expect("UTF8"),
        "wasmtime 50.0.0 (ebe5e8df1 2026-10-20)\n"
    );
    let component = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/wasm32-wasip2/debug/solana_kit_wasi_http_prototype.wasm");
    imports::inspect(&component);

    for mode in [
        "success",
        "chunked",
        "status",
        "redirect",
        "oversize",
        "malformed",
        "headers-deadline",
        "body-deadline",
        "cancel",
        "retain",
        "body-cancel",
        "https-untrusted",
        "wrong-body",
    ] {
        run(&host, &component, mode);
    }
}

/// Joins the real peer before evaluating the component's observable outcome.
fn run(host: &str, component: &Path, mode: &'static str) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let address = listener.local_addr().expect("address");
    listener.set_nonblocking(true).expect("nonblocking");
    let peer = thread::spawn(move || serve(listener, mode));
    let scheme = if mode == "https-untrusted" {
        "https"
    } else {
        "http"
    };

    let endpoint = if mode == "https-untrusted" {
        format!("{scheme}://localhost:{}/rpc", address.port())
    } else {
        format!("{scheme}://{address}/rpc")
    };
    let mut child = Command::new(host)
        .args(["run", "-S", "http=y"])
        .arg(component)
        .arg(endpoint)
        .arg(mode)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("host");
    wait_for_component(&mut child);

    let output = child.wait_with_output().expect("output");
    let stdout = String::from_utf8(output.stdout).expect("UTF8");

    let stderr = String::from_utf8(output.stderr).expect("UTF8");
    peer.join().expect("peer assertion");

    if mode == "wrong-body" {
        assert!(
            !output.status.success() && stderr.contains("assertion"),
            "corrupt reply was not rejected"
        );
    } else {
        assert!(output.status.success(), "{mode}: {stdout} {stderr}");
        assert!(
            stdout.contains(&format!("PASS {mode}")),
            "missing assertion marker"
        );
    }

    println!(
        "PASS {mode}; peer asserted request/auth/body/lifetime; {}",
        stdout.trim().replace('\n', "; ")
    );
}

/// Deadline failure kills and reaps the child rather than detaching the runtime.
fn wait_for_component(child: &mut std::process::Child) {
    let deadline = Instant::now() + PROCESS_LIMIT;

    while child.try_wait().expect("wait").is_none() {
        if Instant::now() >= deadline {
            child.kill().expect("kill");
            child.wait().expect("reap");
            panic!("component deadline exceeded");
        }

        thread::sleep(TICK);
    }
}

fn serve(listener: TcpListener, mode: &str) {
    let mut stream = accept(listener);
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("timeout");

    if mode == "https-untrusted" {
        untrusted_tls(&mut stream);

        return;
    }

    let bytes = request(&mut stream);
    let header_end = bytes
        .windows(4)
        .position(|part| part == b"\r\n\r\n")
        .expect("headers")
        + 4;
    let headers = std::str::from_utf8(&bytes[..header_end])
        .expect("headers UTF8")
        .to_ascii_lowercase();
    assert!(headers.starts_with("post /rpc http/1.1\r\n"));
    assert!(headers.contains("authorization: bearer fixture-only\r\n"));

    assert!(headers.contains("content-type: application/json\r\n"));

    assert_eq!(&bytes[header_end..], REQUEST.as_bytes());

    match mode {
        "cancel" | "retain" => return quiet(&mut stream, mode == "retain"),
        "headers-deadline" => {
            thread::sleep(QUIET);
            return;
        }
        "body-deadline" | "body-cancel" => {
            partial_body(&mut stream, mode);
            return;
        }
        "malformed" => {
            stream.write_all(b"invalid HTTP\r\n\r\n").expect("write");
            return;
        }
        _ => {}
    }

    reply(&mut stream, mode);
}

fn untrusted_tls(stream: &mut TcpStream) {
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
    let mut tls = rustls::ServerConnection::new(std::sync::Arc::new(config)).expect("TLS peer");
    assert!(
        tls.complete_io(stream).is_err(),
        "private fixture CA unexpectedly trusted"
    );
}

fn partial_body(stream: &mut TcpStream, mode: &str) {
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{{",
        REPLY.len()
    )
    .expect("partial body");
    stream.flush().expect("flush");

    if mode == "body-cancel" {
        quiet(stream, false);
    } else {
        thread::sleep(QUIET);
    }
}

/// Distinct status, chunking and corrupted payload fixtures exercise independent failures.
fn reply(stream: &mut TcpStream, mode: &str) {
    let status = match mode {
        "status" => UNAVAILABLE,
        "redirect" => REDIRECT,
        _ => OK,
    };

    let body = match mode {
        "oversize" => "x".repeat(OVERSIZE),
        "wrong-body" => REPLY.replace("18446744073709551615", "42"),
        _ => REPLY.to_owned(),
    };

    if mode == "chunked" {
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n")
            .expect("headers");

        for chunk in body.as_bytes().chunks(CHUNK) {
            write!(stream, "{:x}\r\n", chunk.len()).expect("chunk length");
            stream.write_all(chunk).expect("chunk");
            stream.write_all(b"\r\n").expect("chunk end");
        }

        stream.write_all(b"0\r\n\r\n").expect("eof");
    } else {
        write!(stream, "HTTP/1.1 {status} Fixture\r\nContent-Length: {}\r\nLocation: http://127.0.0.1:1/must-not-follow\r\nConnection: close\r\n\r\n{body}", body.len()).expect("response");
    }
}

fn accept(listener: TcpListener) -> TcpStream {
    let deadline = Instant::now() + Duration::from_secs(4);

    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                stream.set_nonblocking(false).expect("blocking peer");
                return stream;
            }

            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(error) => panic!("accept failed {error}"),
        }

        assert!(Instant::now() < deadline, "guest never connected");
        thread::sleep(TICK);
    }
}

fn request(stream: &mut TcpStream) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut part = [0; 512];

    loop {
        let count = stream.read(&mut part).expect("request read");
        assert_ne!(count, 0);
        bytes.extend_from_slice(&part[..count]);
        assert!(bytes.len() <= 8192);

        if let Some(end) = bytes.windows(4).position(|part| part == b"\r\n\r\n")
            && bytes.len() == end + 4 + REQUEST.len()
        {
            return bytes;
        }
    }
}

fn quiet(stream: &mut TcpStream, retained: bool) {
    stream.set_read_timeout(Some(QUIET)).expect("close timeout");
    let mut byte = [0];
    let read = stream.read(&mut byte);

    if retained {
        let timed_out = matches!(read, Err(ref error) if matches!(error.kind(),
            std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock));
        assert!(timed_out, "retained future closed {read:?}");
    } else {
        assert!(matches!(read, Ok(0)), "drop failed to close {read:?}");
    }
}
