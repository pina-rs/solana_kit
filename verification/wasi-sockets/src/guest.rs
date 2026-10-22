//! Actual Preview 2 guest exercising validating TLS and bounded WebSocket I/O.

use std::io::ErrorKind;
use std::net::TcpStream;
use std::time::Duration;

use anyhow::{Context, Result, bail, ensure};
use rustls::{CertificateError, Error as TlsError, StreamOwned};
use tungstenite::protocol::WebSocketConfig;
use tungstenite::{Error, Message};

const MESSAGE_BOUND: usize = 64;
const POST_CANCEL_ALIVE: Duration = Duration::from_millis(350);
type Socket = tungstenite::WebSocket<StreamOwned<rustls::ClientConnection, TcpStream>>;

/// Executes one independently supervised local fixture journey.
fn main() -> Result<()> {
    let arguments: Vec<_> = std::env::args().collect();
    let mode = arguments.get(1).context("missing mode")?;
    let port: u16 = arguments.get(2).context("missing port")?.parse()?;
    let socket = TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, port));

    if mode == "denied" || mode == "unavailable" {
        return denied(mode, socket);
    }

    let mut socket = socket?;
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;
    socket.set_write_timeout(Some(Duration::from_secs(2)))?;
    let mut connection =
        solana_kit_wasi_sockets_proof::client(mode == "wrong-ca", mode == "wrong-name")?;
    let handshake = connection.complete_io(&mut socket);

    if matches!(mode.as_str(), "wrong-ca" | "wrong-name" | "plaintext") {
        validate_tls_failure(mode, handshake.err().context("invalid TLS peer accepted")?)?;
    } else {
        handshake?;
        websocket(mode, StreamOwned::new(connection, socket))?;
    }

    println!("PASS {mode}");
    Ok(())
}

/// Requires the capability failure before the independent peer sees a connection.
fn denied(mode: &str, result: std::io::Result<TcpStream>) -> Result<()> {
    let error = result
        .err()
        .context("denied socket unexpectedly connected")?;
    ensure!(
        matches!(
            error.kind(),
            ErrorKind::PermissionDenied | ErrorKind::Unsupported
        ),
        "unexpected socket denial: {error:?}"
    );
    println!("PASS {mode} {error:?}");

    Ok(())
}

/// Retains strict upgrade validation and separate bounded message/frame limits.
fn websocket(mode: &str, stream: StreamOwned<rustls::ClientConnection, TcpStream>) -> Result<()> {
    let config = WebSocketConfig::default()
        .max_message_size(Some(MESSAGE_BOUND))
        .max_frame_size(Some(MESSAGE_BOUND));
    let response =
        tungstenite::client::client_with_config("wss://localhost/rpc", stream, Some(config));

    if mode == "bad-upgrade" {
        ensure!(
            matches!(response, Err(tungstenite::HandshakeError::Failure(Error::Http(ref reply)))
            if reply.status() == tungstenite::http::StatusCode::FORBIDDEN),
            "wrong upgrade failure"
        );

        return Ok(());
    }

    let (mut socket, response) = response.map_err(|error| anyhow::anyhow!(error.to_string()))?;
    ensure!(
        response.status() == tungstenite::http::StatusCode::SWITCHING_PROTOCOLS,
        "missing upgrade status"
    );

    match mode {
        "exchange" | "corrupt" => exchange(&mut socket)?,
        "oversized" => ensure!(
            matches!(socket.read(), Err(Error::Capacity(_))),
            "oversized frame accepted"
        ),
        "cancel" | "retain" | "unload" => quiet(socket, mode)?,
        _ => bail!("unknown mode"),
    }

    Ok(())
}

/// Requires certificate/name rejection from the validating rustls path.
fn validate_tls_failure(mode: &str, error: std::io::Error) -> Result<()> {
    let tls = error
        .get_ref()
        .and_then(|source| source.downcast_ref::<TlsError>());

    let correct = match mode {
        "wrong-ca" => matches!(
            tls,
            Some(TlsError::InvalidCertificate(
                CertificateError::UnknownIssuer
            ))
        ),
        "wrong-name" => matches!(
            tls,
            Some(TlsError::InvalidCertificate(
                CertificateError::NotValidForNameContext { .. }
            ))
        ),
        "plaintext" => matches!(tls, Some(TlsError::InvalidMessage(_))),
        _ => false,
    };

    ensure!(correct, "unexpected TLS error {error:?}");

    Ok(())
}

/// Requires independent text and binary values, then completes close acknowledgement.
fn exchange(socket: &mut Socket) -> Result<()> {
    socket.send(Message::Text("subscribe:7".into()))?;
    ensure!(
        socket.read()? == Message::Text("notification:41".into()),
        "wrong text payload"
    );
    socket.send(Message::Binary(vec![0, 7, 255].into()))?;
    ensure!(
        socket.read()? == Message::Binary(vec![9, 0, 8].into()),
        "wrong binary payload"
    );
    socket.close(None)?;
    ensure!(
        matches!(socket.read()?, Message::Close(_)),
        "missing close acknowledgement"
    );

    Ok(())
}

/// Observes a quiet pending receive before dropping its sole owner, while alive.
fn quiet(mut socket: Socket, mode: &str) -> Result<()> {
    socket.get_mut().sock.set_nonblocking(true)?;
    ensure!(
        matches!(socket.read(), Err(Error::Io(ref error)) if error.kind() == ErrorKind::WouldBlock),
        "quiet receive did not remain pending"
    );
    println!("PENDING {mode}");
    std::io::Write::flush(&mut std::io::stdout())?;

    if mode == "cancel" {
        drop(socket);
        std::thread::sleep(POST_CANCEL_ALIVE);

        return Ok(());
    }

    std::thread::sleep(POST_CANCEL_ALIVE);
    std::hint::black_box(socket);
    Ok(())
}
