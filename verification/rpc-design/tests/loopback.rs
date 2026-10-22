//! Real loopback socket ownership, separate from host-wide adapter promises.

use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;

use solana_kit_rpc_design::methods::Balance;
use solana_kit_rpc_design::{Client, Reply, Request, Transport, TransportError};
use solana_kit_rpc_design::{Error, Limits, TransportFailureKind};

struct Loopback(SocketAddr);

impl Transport for Loopback {
    type Exchange<'a> = Pin<Box<dyn Future<Output = Result<Reply, TransportError>> + Send + 'a>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        Box::pin(exchange(self.0, request))
    }
}

const SUCCESS: u16 = 200;
const HEADER_ALLOWANCE: usize = 1024;
const RETAINED_OBSERVATION: Duration = Duration::from_millis(80);
const BEFORE_ABORT: Duration = Duration::from_millis(160);

async fn exchange(address: SocketAddr, request: Request) -> Result<Reply, TransportError> {
    let mut stream = TcpStream::connect(address)
        .await
        .map_err(|_| TransportError::new(TransportFailureKind::Io))?;
    let headers = format!(
        "POST / HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        request.body.len()
    );
    stream
        .write_all(headers.as_bytes())
        .await
        .map_err(|_| TransportError::new(TransportFailureKind::Io))?;
    stream
        .write_all(&request.body)
        .await
        .map_err(|_| TransportError::new(TransportFailureKind::Io))?;
    let bytes = read_bounded(&mut stream, request.max_reply_bytes + HEADER_ALLOWANCE).await?;
    let split = bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or(TransportError::new(TransportFailureKind::Io))?;

    if !bytes.starts_with(b"HTTP/1.1 200 OK\r\n") {
        return Err(TransportError::new(TransportFailureKind::Io));
    }

    let body = bytes[split + 4..].to_vec();

    if body.len() > request.max_reply_bytes {
        return Err(TransportError::new(TransportFailureKind::BodyLimit));
    }

    Ok(Reply {
        status: SUCCESS,
        body,
    })
}

async fn read_bounded(stream: &mut TcpStream, limit: usize) -> Result<Vec<u8>, TransportError> {
    let mut bytes = Vec::new();
    let mut chunk = [0; 64];

    loop {
        let count = stream
            .read(&mut chunk)
            .await
            .map_err(|_| TransportError::new(TransportFailureKind::Io))?;

        if count == 0 {
            return Ok(bytes);
        }

        if bytes.len() + count > limit {
            return Err(TransportError::new(TransportFailureKind::BodyLimit));
        }

        bytes.extend_from_slice(&chunk[..count]);
    }
}

fn balance() -> Balance {
    Balance("11111111111111111111111111111111".parse().unwrap(), None)
}

/// Fixture parser asserts the complete exact request before reporting acceptance.
async fn receive(stream: &mut TcpStream) {
    let expected = br#"{"jsonrpc":"2.0","id":1,"method":"getBalance","params":["11111111111111111111111111111111"]}"#;
    let mut bytes = Vec::new();
    let mut byte = [0];

    loop {
        stream.read_exact(&mut byte).await.unwrap();
        bytes.push(byte[0]);

        if bytes.ends_with(b"\r\n\r\n") {
            break;
        }

        assert!(bytes.len() < 1024);
    }

    let headers = String::from_utf8(bytes).unwrap();
    assert!(headers.starts_with("POST / HTTP/1.1\r\n"));
    assert!(headers.contains(&format!("Content-Length: {}\r\n", expected.len())));
    assert!(headers.contains("Content-Type: application/json\r\n"));
    let mut body = vec![0; expected.len()];
    stream.read_exact(&mut body).await.unwrap();
    assert_eq!(body, expected);
}

#[tokio::test]
async fn typed_call_executes_real_http_exchange() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        receive(&mut stream).await;
        let body =
            br#"{"jsonrpc":"2.0","id":1,"result":{"context":{"slot":8},"value":9007199254740993}}"#;
        let header = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        stream.write_all(header.as_bytes()).await.unwrap();
        stream.write_all(body).await.unwrap();
    });
    let client = Client::new(Loopback(address), Limits::default());
    let result = tokio::time::timeout(Duration::from_secs(3), client.call(balance()))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(result.value, 9007199254740993);
    server.await.unwrap();
}

#[tokio::test]
async fn dropping_actual_pending_call_closes_owned_socket() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (accepted, ready) = oneshot::channel();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        receive(&mut stream).await;
        accepted.send(()).unwrap();
        let mut byte = [0];
        // Retaining the live call is an explicit negative control before abort.
        assert!(
            tokio::time::timeout(RETAINED_OBSERVATION, stream.read(&mut byte))
                .await
                .is_err()
        );
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(3), stream.read(&mut byte))
                .await
                .unwrap()
                .unwrap(),
            0
        );
    });
    let call = tokio::spawn(async move {
        let client = Client::new(Loopback(address), Limits::default());
        client.call(balance()).await
    });
    tokio::time::timeout(Duration::from_secs(3), ready)
        .await
        .unwrap()
        .unwrap();
    tokio::time::sleep(BEFORE_ABORT).await;
    call.abort();

    assert!(call.await.unwrap_err().is_cancelled());
    server.await.unwrap();
}

#[test]
fn explicit_transport_failure_remains_distinct() {
    let error = Error::from(TransportError::new(TransportFailureKind::Denied));
    assert!(matches!(error, Error::Transport(error) if error.kind == TransportFailureKind::Denied));
}
