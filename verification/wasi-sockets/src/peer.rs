//! Independently asserted local TLS and WebSocket peer observations.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, ensure};
use rustls::{ServerConnection, StreamOwned};
use tungstenite::{Message, WebSocket};

const QUIET_LIMIT: Duration = Duration::from_millis(240);
const RETRY_TICK: Duration = Duration::from_millis(5);
const HANDSHAKE_RECORD: u8 = 22;
type Peer = WebSocket<StreamOwned<ServerConnection, TcpStream>>;

/// Requires acceptance and validates the protocol independently of guest values.
pub fn fixture(listener: TcpListener, mode: &str) -> Result<String> {
    let accepted = accept(&listener)?;

    if mode == "denied" || mode == "unavailable" {
        ensure!(accepted.is_none(), "host allowed denied socket");

        return Ok("no peer connection".to_owned());
    }

    let mut tcp = accepted.context("guest never connected")?;
    tcp.set_nonblocking(false)?;
    tcp.set_read_timeout(Some(Duration::from_secs(2)))?;
    tcp.set_write_timeout(Some(Duration::from_secs(2)))?;

    if mode == "plaintext" {
        return plaintext(&mut tcp);
    }

    let mut tls = StreamOwned::new(
        ServerConnection::new(solana_kit_wasi_sockets_proof::server()?)?,
        tcp,
    );

    if mode == "wrong-ca" || mode == "wrong-name" {
        ensure!(
            tls.conn.complete_io(&mut tls.sock).is_err(),
            "invalid certificate accepted by guest"
        );

        return Ok("peer observed TLS validation alert".to_owned());
    }

    if mode == "bad-upgrade" {
        return bad_upgrade(&mut tls);
    }

    let mut socket = tungstenite::accept_hdr(tls, Upgrade)?;

    match mode {
        "exchange" => exchange(&mut socket)?,
        "corrupt" => {
            ensure!(
                socket.read()? == Message::Text("subscribe:7".into()),
                "wrong text request"
            );
            socket.send(Message::Text("notification:42".into()))?;
        }
        "oversized" => socket.send(Message::Binary(vec![5; 65].into()))?,
        "cancel" | "retain" | "unload" => return quiet(&mut socket, mode),
        _ => anyhow::bail!("unknown fixture mode"),
    }

    Ok("validated TLS plus real WebSocket protocol".to_owned())
}

/// Bounds peer acceptance, including the controls requiring no accepted socket.
fn accept(listener: &TcpListener) -> Result<Option<TcpStream>> {
    let deadline = Instant::now() + Duration::from_secs(4);

    loop {
        match listener.accept() {
            Ok(pair) => return Ok(Some(pair.0)),

            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(error) => return Err(error.into()),
        }

        if Instant::now() >= deadline {
            return Ok(None);
        }

        thread::sleep(RETRY_TICK);
    }
}

/// Verifies the guest really started TLS before offering an invalid plaintext peer.
fn plaintext(tcp: &mut TcpStream) -> Result<String> {
    let mut bytes = [0_u8; 5];
    tcp.read_exact(&mut bytes)?;
    ensure!(
        bytes[0] == HANDSHAKE_RECORD && bytes[1] == 3,
        "missing real TLS handshake"
    );
    tcp.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?;

    Ok("real TLS ClientHello rejected plaintext".to_owned())
}

/// Returns a genuine HTTP rejection inside the validated TLS stream.
fn bad_upgrade(tls: &mut StreamOwned<ServerConnection, TcpStream>) -> Result<String> {
    let mut request = [0_u8; 2048];
    let size = tls.read(&mut request)?;
    ensure!(
        std::str::from_utf8(&request[..size])?.contains("GET /rpc HTTP/1.1"),
        "wrong upgrade path"
    );
    tls.write_all(b"HTTP/1.1 403 Forbidden\r\nContent-Length: 0\r\n\r\n")?;

    Ok("valid TLS; explicit upgrade rejection".to_owned())
}

/// Checks independent request path/authority rather than accepting arbitrary input.
struct Upgrade;

impl tungstenite::handshake::server::Callback for Upgrade {
    fn on_request(
        self,
        request: &tungstenite::handshake::server::Request,
        response: tungstenite::handshake::server::Response,
    ) -> std::result::Result<
        tungstenite::handshake::server::Response,
        tungstenite::handshake::server::ErrorResponse,
    > {
        assert_eq!(request.uri().path(), "/rpc");
        assert_eq!(
            request
                .headers()
                .get("host")
                .and_then(|value| value.to_str().ok()),
            Some("localhost")
        );

        Ok(response)
    }
}

/// Uses distinct literals rather than echoing arbitrary input back to the client.
fn exchange(socket: &mut Peer) -> Result<()> {
    ensure!(
        socket.read()? == Message::Text("subscribe:7".into()),
        "wrong text request"
    );
    socket.send(Message::Text("notification:41".into()))?;
    ensure!(
        socket.read()? == Message::Binary(vec![0, 7, 255].into()),
        "wrong binary request"
    );
    socket.send(Message::Binary(vec![9, 0, 8].into()))?;
    ensure!(
        matches!(socket.read()?, Message::Close(_)),
        "missing close request"
    );
    let closed = socket.flush();
    ensure!(
        closed.is_ok() || matches!(closed, Err(tungstenite::Error::ConnectionClosed)),
        "close flush failed: {closed:?}"
    );

    Ok(())
}

/// Measures EOF before guest exit and keeps a retained-stream negative control.
fn quiet(socket: &mut Peer, mode: &str) -> Result<String> {
    socket.get_mut().sock.set_read_timeout(Some(QUIET_LIMIT))?;
    let start = Instant::now();
    let result = socket.read();
    let elapsed = start.elapsed();
    let timeout = matches!(result, Err(tungstenite::Error::Io(ref error))
        if matches!(error.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut));

    if mode == "retain" {
        ensure!(timeout, "retained control unexpectedly closed: {result:?}");

        return Ok("quiet connection retained past 240ms".to_owned());
    }

    ensure!(
        !timeout && result.is_err(),
        "cancel/unload failed to terminate quiet socket: {result:?}"
    );
    ensure!(
        elapsed < QUIET_LIMIT,
        "closure arrived after cancellation budget"
    );
    let late = socket.send(Message::Text("late:must-not-deliver".into()));

    Ok(format!(
        "quiet EOF before 240ms; post-close send={late:?}; receive={result:?}"
    ))
}
