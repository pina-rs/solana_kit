use super::MwaError;
use crate::{
    WsConfig,
    websocket_connect::{self, Socket, SocketIo},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use futures_util::{SinkExt, StreamExt};
use solana_kit_wallet::mwa::{
    AssociationPort, LOCAL_WEBSOCKET_PATH, RETRY_DELAYS_MS, WEBSOCKET_BASE64, WEBSOCKET_BINARY,
};
use std::{io, net::Ipv4Addr, time::Duration};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::{Message, client::IntoClientRequest, http::HeaderValue};

const CONTROL_FRAME_BUDGET: usize = 32;

/// Wire encoding selected by the reflector's response subprotocol.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Encoding {
    Binary,
    Base64,
}

/// One socket together with the encoding its negotiated subprotocol requires.
pub(super) struct Connection {
    pub(super) socket: Socket,
    pub(super) encoding: Encoding,
}

pub(super) async fn connect(
    port: AssociationPort,
    config: &WsConfig,
) -> Result<Connection, MwaError> {
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

    Ok(Connection {
        socket,
        encoding: Encoding::Binary,
    })
}

/// Connects to one explicit WSS reflector and returns its first decoded payload.
pub(super) async fn connect_remote(
    endpoint: &str,
    config: &WsConfig,
) -> Result<(Connection, Vec<u8>), MwaError> {
    config.validate(endpoint).map_err(MwaError::Config)?;

    if !config.headers.is_empty() {
        return Err(MwaError::Handshake);
    }

    let parsed = websocket_connect::endpoint(endpoint).map_err(MwaError::Config)?;
    let (socket, protocols) = websocket_connect::open_selected(
        endpoint,
        parsed,
        config,
        Some(&[WEBSOCKET_BINARY, WEBSOCKET_BASE64]),
    )
    .await
    .map_err(MwaError::Transport)?;
    let encoding = selected_encoding(&protocols)?;
    let mut connection = Connection { socket, encoding };
    let payload = receive(&mut connection, config.max_frame_bytes).await?;

    Ok((connection, payload))
}

fn selected_encoding(protocols: &[String]) -> Result<Encoding, MwaError> {
    match protocols {
        [protocol] if protocol == WEBSOCKET_BINARY => Ok(Encoding::Binary),
        [protocol] if protocol == WEBSOCKET_BASE64 => Ok(Encoding::Base64),
        _ => Err(MwaError::Handshake),
    }
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

pub(super) async fn send(
    connection: &mut Connection,
    bytes: Vec<u8>,
    max_frame: usize,
) -> Result<(), MwaError> {
    let message = encode(connection.encoding, bytes, max_frame)?;

    connection
        .socket
        .send(message)
        .await
        .map_err(MwaError::Socket)
}

fn encode(encoding: Encoding, bytes: Vec<u8>, max_frame: usize) -> Result<Message, MwaError> {
    match encoding {
        Encoding::Binary => binary_message(bytes, max_frame),
        Encoding::Base64 => base64_message(bytes, max_frame),
    }
}

fn binary_message(bytes: Vec<u8>, max_frame: usize) -> Result<Message, MwaError> {
    if bytes.len() > max_frame {
        return Err(MwaError::Capacity);
    }

    Ok(Message::Binary(bytes.into()))
}

fn base64_message(bytes: Vec<u8>, max_frame: usize) -> Result<Message, MwaError> {
    let encoded = encoded_bound(bytes.len()).ok_or(MwaError::Capacity)?;

    if encoded > max_frame {
        return Err(MwaError::Capacity);
    }

    Ok(Message::Text(STANDARD.encode(bytes).into()))
}

pub(super) async fn receive(
    connection: &mut Connection,
    max_frame: usize,
) -> Result<Vec<u8>, MwaError> {
    for _ in 0..CONTROL_FRAME_BUDGET {
        let message = match connection.socket.next().await {
            Some(Ok(Message::Ping(_) | Message::Pong(_))) => {
                connection.socket.flush().await.map_err(MwaError::Socket)?;

                continue;
            }
            Some(Ok(Message::Close(_))) | None => return Err(MwaError::Closed),
            Some(Err(error)) => return Err(MwaError::Socket(error)),
            Some(Ok(message)) => message,
        };

        return decode(connection.encoding, message, max_frame);
    }

    Err(MwaError::Capacity)
}

fn decode(encoding: Encoding, message: Message, max_frame: usize) -> Result<Vec<u8>, MwaError> {
    match (encoding, message) {
        (Encoding::Binary, Message::Binary(bytes)) => binary_payload(&bytes, max_frame),
        (Encoding::Base64, Message::Text(text)) => base64_payload(&text, max_frame),
        _ => Err(MwaError::Handshake),
    }
}

fn binary_payload(bytes: &[u8], max_frame: usize) -> Result<Vec<u8>, MwaError> {
    if bytes.len() > max_frame {
        return Err(MwaError::Capacity);
    }

    Ok(bytes.to_vec())
}

fn base64_payload(text: &str, max_frame: usize) -> Result<Vec<u8>, MwaError> {
    if text.len() > max_frame {
        return Err(MwaError::Capacity);
    }

    let bound = decoded_bound(text.len()).ok_or(MwaError::Capacity)?;

    if bound > max_frame {
        return Err(MwaError::Capacity);
    }

    let bytes = STANDARD
        .decode(text.as_bytes())
        .map_err(|_| MwaError::Handshake)?;

    if bytes.len() > max_frame {
        return Err(MwaError::Capacity);
    }

    Ok(bytes)
}

pub(super) async fn close(connection: &mut Connection) -> Result<(), MwaError> {
    connection
        .socket
        .close(None)
        .await
        .map_err(MwaError::Socket)?;

    for _ in 0..CONTROL_FRAME_BUDGET {
        match connection.socket.next().await {
            Some(Ok(Message::Close(_))) | None => return Ok(()),
            Some(Ok(Message::Ping(_) | Message::Pong(_))) => {
                connection.socket.flush().await.map_err(MwaError::Socket)?;
            }
            Some(Err(error)) => return Err(MwaError::Socket(error)),
            _ => return Err(MwaError::Handshake),
        }
    }

    Err(MwaError::Capacity)
}

pub(super) async fn receive_encrypted(
    connection: &mut Connection,
    max_frame: usize,
) -> Result<Vec<u8>, MwaError> {
    for _ in 0..CONTROL_FRAME_BUDGET {
        let frame = receive(connection, max_frame).await?;

        if !frame.is_empty() {
            return Ok(frame);
        }
    }

    Err(MwaError::Capacity)
}

fn encoded_bound(decoded: usize) -> Option<usize> {
    decoded.checked_add(2)?.checked_div(3)?.checked_mul(4)
}

fn decoded_bound(encoded: usize) -> Option<usize> {
    encoded.checked_add(3)?.checked_div(4)?.checked_mul(3)
}
