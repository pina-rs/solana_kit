use solana_kit_client::{Reply, Request, Transport, TransportError};
use solana_kit_native::{HttpConfig, NativeHttp};
use std::{io, net::SocketAddr, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    task::JoinHandle,
};

pub async fn listener() -> TcpListener {
    TcpListener::bind("127.0.0.1:0").await.unwrap()
}

pub async fn read_request<S: AsyncRead + Unpin>(stream: &mut S) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut byte = [0];

    while !bytes.ends_with(b"\r\n\r\n") {
        assert!(bytes.len() < 16384);
        stream.read_exact(&mut byte).await.unwrap();
        bytes.push(byte[0]);
    }

    let text = std::str::from_utf8(&bytes).unwrap();
    let length: usize = text
        .lines()
        .find_map(|line| {
            line.to_ascii_lowercase()
                .strip_prefix("content-length: ")
                .map(str::to_owned)
        })
        .map_or(0, |length| length.parse().unwrap());
    let start = bytes.len();
    bytes.resize(start + length, 0);
    stream.read_exact(&mut bytes[start..]).await.unwrap();
    bytes
}

pub fn response(status: u16, body: &[u8]) -> Vec<u8> {
    let mut bytes = format!(
        "HTTP/1.1 {status} Fixture\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .into_bytes();
    bytes.extend_from_slice(body);
    bytes
}

pub async fn server(bytes: Vec<u8>) -> (SocketAddr, JoinHandle<Vec<u8>>) {
    let listener = listener().await;
    let address = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let request = read_request(&mut socket).await;
        socket.write_all(&bytes).await.unwrap();
        request
    });
    (address, task)
}

pub fn request(limit: usize) -> Request {
    Request {
        body: b"{}".to_vec(),
        max_reply_bytes: limit,
    }
}

pub async fn raw(
    bytes: Vec<u8>,
    limit: usize,
    config: HttpConfig,
) -> Result<Reply, TransportError> {
    let (address, server) = server(bytes).await;
    let adapter = NativeHttp::new(&format!("http://{address}/rpc?fixture=1"), config).unwrap();
    let reply = adapter.exchange(request(limit)).await;
    server.await.unwrap();
    reply
}

pub async fn eof(socket: &mut TcpStream) -> io::Result<usize> {
    tokio::time::timeout(Duration::from_secs(2), socket.read(&mut [0]))
        .await
        .unwrap()
}
