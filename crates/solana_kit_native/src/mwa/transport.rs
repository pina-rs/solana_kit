use super::MwaError;
use crate::{
    WsConfig,
    websocket_connect::{Socket, SocketIo},
};
use futures_util::{SinkExt, StreamExt};
use solana_kit_wallet::mwa::{
    AssociationPort, LOCAL_WEBSOCKET_PATH, RETRY_DELAYS_MS, WEBSOCKET_BINARY,
};
use std::{io, net::Ipv4Addr, time::Duration};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{Message, client::IntoClientRequest, http::HeaderValue};

pub(super) async fn connect(port: AssociationPort, config: &WsConfig) -> Result<Socket, MwaError> {
    let endpoint = format!("ws://127.0.0.1:{}{LOCAL_WEBSOCKET_PATH}", port.value());
    config.validate(&endpoint).map_err(MwaError::Config)?;

    if !config.headers.is_empty() {
        return Err(MwaError::Handshake);
    }

    let tcp = connect_tcp(port).await?;
    let stream: Box<dyn SocketIo> = Box::new(tcp);
    let mut request = endpoint.into_client_request().map_err(MwaError::Socket)?;
    request.headers_mut().insert(
        "sec-websocket-protocol",
        HeaderValue::from_static(WEBSOCKET_BINARY),
    );
    let (socket, response) =
        tokio_tungstenite::client_async_with_config(request, stream, Some(config.protocol()))
            .await
            .map_err(MwaError::Socket)?;

    if response.headers().get("sec-websocket-protocol")
        != Some(&HeaderValue::from_static(WEBSOCKET_BINARY))
        || response.headers().contains_key("sec-websocket-extensions")
    {
        return Err(MwaError::Handshake);
    }

    Ok(socket)
}

async fn connect_tcp(port: AssociationPort) -> Result<TcpStream, MwaError> {
    let mut attempt = 0;

    loop {
        match TcpStream::connect((Ipv4Addr::LOCALHOST, port.value())).await {
            Ok(socket) => return Ok(socket),
            Err(error) if error.kind() == io::ErrorKind::ConnectionRefused => {
                let index = attempt.min(RETRY_DELAYS_MS.len() - 1);
                tokio::time::sleep(Duration::from_millis(u64::from(RETRY_DELAYS_MS[index]))).await;
                attempt = index.saturating_add(1);
            }
            Err(error) => return Err(MwaError::Io(error)),
        }
    }
}

pub(super) async fn send(socket: &mut Socket, bytes: Vec<u8>) -> Result<(), MwaError> {
    socket
        .send(Message::Binary(bytes.into()))
        .await
        .map_err(MwaError::Socket)
}

pub(super) async fn receive(socket: &mut Socket) -> Result<Vec<u8>, MwaError> {
    for _ in 0..32 {
        match socket.next().await {
            Some(Ok(Message::Binary(bytes))) => return Ok(bytes.to_vec()),
            Some(Ok(Message::Ping(_) | Message::Pong(_))) => {
                socket.flush().await.map_err(MwaError::Socket)?;
            }
            Some(Ok(Message::Close(_))) | None => return Err(MwaError::Closed),
            Some(Err(error)) => return Err(MwaError::Socket(error)),
            _ => return Err(MwaError::Handshake),
        }
    }

    Err(MwaError::Capacity)
}

pub(super) async fn close(socket: &mut Socket) -> Result<(), MwaError> {
    socket.close(None).await.map_err(MwaError::Socket)?;

    for _ in 0..32 {
        match socket.next().await {
            Some(Ok(Message::Close(_))) | None => return Ok(()),
            Some(Ok(Message::Ping(_) | Message::Pong(_))) => {
                socket.flush().await.map_err(MwaError::Socket)?;
            }
            Some(Err(error)) => return Err(MwaError::Socket(error)),
            _ => return Err(MwaError::Handshake),
        }
    }

    Err(MwaError::Capacity)
}

pub(super) async fn receive_encrypted(socket: &mut Socket) -> Result<Vec<u8>, MwaError> {
    for _ in 0..32 {
        let frame = receive(socket).await?;

        if !frame.is_empty() {
            return Ok(frame);
        }
    }

    Err(MwaError::Capacity)
}
