//! Synthetic authenticated loopback services own and observe real peer sockets.
use anyhow::{Result, ensure};
use solana_kit_native::{HeaderValue, HttpConfig, NativeHttp, TlsRoots};
use std::{sync::Arc, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::TcpListener,
    sync::mpsc,
    task::JoinHandle,
};
use tokio_rustls::{
    TlsAcceptor,
    rustls::{
        self, ServerConfig,
        pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
    },
};

const QUIET_DEADLINE: Duration = Duration::from_millis(80);
const HEADER_LIMIT: usize = 16384;
const UNAVAILABLE: u16 = 503;
const SUCCESS: u16 = 200;

const ROOT: &[u8] = include_bytes!("../../../../crates/solana_kit_native/tests/fixtures/ca.der");
const CERT: &[u8] =
    include_bytes!("../../../../crates/solana_kit_native/tests/fixtures/server.der");
const KEY: &[u8] =
    include_bytes!("../../../../crates/solana_kit_native/tests/fixtures/server-key.der");
const BALANCE: &[u8] = br#"{"jsonrpc":"2.0","id":1,"result":{"context":{"slot":18446744073709551615},"value":18446744073709551615}}"#;
const REQUEST: &[u8] = br#"{"jsonrpc":"2.0","id":1,"method":"getBalance","params":["11111111111111111111111111111111"]}"#;

pub(super) struct Fixture {
    pub service: Option<NativeHttp>,
    pub accepted: mpsc::Receiver<()>,
    pub closed: mpsc::Receiver<()>,
    pub task: JoinHandle<Result<()>>,
}

/// Owns both the configured service and its peer so cancellation remains observable.
pub(super) async fn create(mode: &'static str) -> Result<Fixture> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let port = listener.local_addr()?.port();
    let secure = mode == "secure";
    let endpoint = if secure {
        format!("https://localhost:{port}/rpc")
    } else {
        format!("http://127.0.0.1:{port}/rpc")
    };

    let mut config = HttpConfig {
        exchange_timeout: Duration::from_secs(3),
        ..HttpConfig::default()
    };

    config.headers.insert(
        "authorization",
        HeaderValue::from_static("Bearer synthetic-local-fixture"),
    );

    if secure {
        config.tls = TlsRoots::Certificates(vec![ROOT.to_vec()]);
    }

    if mode == "deadline" {
        config.exchange_timeout = QUIET_DEADLINE;
    }

    let service = NativeHttp::new(&endpoint, config)?;
    let (accepted_tx, accepted) = mpsc::channel(4);
    let (closed_tx, closed) = mpsc::channel(4);
    let task = tokio::spawn(async move {
        let count = if matches!(mode, "quiet" | "rest") {
            2
        } else {
            1
        };
        for _ in 0..count {
            let (socket, _) = listener.accept().await?;
            connection(socket, mode, secure, &accepted_tx, &closed_tx).await?;
        }

        Ok(())
    });
    Ok(Fixture {
        service: Some(service),
        accepted,
        closed,
        task,
    })
}

fn acceptor() -> Result<TlsAcceptor> {
    let config =
        ServerConfig::builder_with_provider(rustls::crypto::ring::default_provider().into())
            .with_safe_default_protocol_versions()?
            .with_no_client_auth()
            .with_single_cert(
                vec![CertificateDer::from(CERT.to_vec())],
                PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(KEY.to_vec())),
            )?;
    Ok(TlsAcceptor::from(Arc::new(config)))
}

/// Every fixture consumes its exact request before choosing a lifecycle or reply failure.
async fn serve<S: AsyncRead + AsyncWrite + Unpin>(
    stream: &mut S,
    mode: &str,
    accepted: &mpsc::Sender<()>,
    closed: &mpsc::Sender<()>,
) -> Result<()> {
    read_request(stream, mode).await?;
    let mut byte = [0];
    accepted.send(()).await?;

    if mode == "quiet" || mode == "deadline" {
        ensure!(
            stream.read(&mut byte).await? == 0,
            "actual owned client EOF"
        );
        closed.send(()).await?;

        return Ok(());
    }
    let status = if mode == "status" {
        UNAVAILABLE
    } else {
        SUCCESS
    };
    let body = if mode == "large" {
        vec![b'x'; 2048]
    } else if mode == "rest" {
        b"done".to_vec()
    } else {
        BALANCE.to_vec()
    };

    let head = format!(
        "HTTP/1.1 {status} Fixture\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(head.as_bytes()).await?;
    stream.write_all(&body).await?;
    closed.send(()).await?;

    Ok(())
}

/// Verifies the complete authenticated request before exposing fixture acceptance.
async fn read_request<S: AsyncRead + Unpin>(stream: &mut S, mode: &str) -> Result<()> {
    let mut bytes = Vec::new();
    let mut byte = [0];

    while !bytes.ends_with(b"\r\n\r\n") {
        ensure!(bytes.len() < HEADER_LIMIT, "request header bound");
        stream.read_exact(&mut byte).await?;
        bytes.push(byte[0]);
    }

    let header = std::str::from_utf8(&bytes)?;
    let target = if mode == "rest" {
        "POST /rpc/a%2Fb?key=a%20b&key=%26 HTTP/1.1\r\n"
    } else {
        "POST /rpc HTTP/1.1\r\n"
    };

    ensure!(header.starts_with(target), "exact HTTP destination");
    ensure!(
        header.contains("authorization: Bearer synthetic-local-fixture\r\n"),
        "host-only auth"
    );

    let length: usize = header
        .lines()
        .find_map(|line| line.strip_prefix("content-length: "))
        .ok_or_else(|| anyhow::anyhow!("missing length"))?
        .parse()?;
    let mut body = vec![0; length];
    stream.read_exact(&mut body).await?;
    let expected = if mode == "rest" {
        b"null".as_slice()
    } else {
        REQUEST
    };

    ensure!(body == expected, "exact public request body");

    Ok(())
}

async fn connection(
    socket: tokio::net::TcpStream,
    mode: &str,
    secure: bool,
    accepted: &mpsc::Sender<()>,
    closed: &mpsc::Sender<()>,
) -> Result<()> {
    if secure {
        let mut stream = acceptor()?.accept(socket).await?;

        return serve(&mut stream, mode, accepted, closed).await;
    }

    let mut stream = socket;
    serve(&mut stream, mode, accepted, closed).await
}
