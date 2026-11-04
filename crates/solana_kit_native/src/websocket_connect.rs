//! One handshake future owns DNS, TCP and TLS until the caller receives the socket.
use crate::{ConfigError, WsConfig, WsError, endpoint::Endpoint};
use std::sync::Arc;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
};
use tokio_rustls::{TlsConnector, rustls::pki_types::ServerName};
use tokio_tungstenite::{
    WebSocketStream,
    tungstenite::{
        client::IntoClientRequest,
        http::{HeaderMap, HeaderValue, Request},
    },
};

pub(crate) trait SocketIo: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> SocketIo for T {}
pub(crate) type Socket = WebSocketStream<Box<dyn SocketIo>>;

pub(crate) fn endpoint(value: &str) -> Result<Endpoint, ConfigError> {
    let mapped = if let Some(suffix) = value.strip_prefix("wss://") {
        format!("https://{suffix}")
    } else if let Some(suffix) = value.strip_prefix("ws://") {
        format!("http://{suffix}")
    } else {
        return Err(ConfigError("endpoint must use ws or wss"));
    };

    Endpoint::parse(&mapped)
}

pub(crate) async fn open(
    value: &str,
    endpoint: Endpoint,
    config: &WsConfig,
) -> Result<Socket, WsError> {
    open_selected(value, endpoint, config, None)
        .await
        .map(|(socket, _)| socket)
}

/// Performs one explicit upgrade, optionally offering WebSocket subprotocols.
pub(crate) async fn open_selected(
    value: &str,
    endpoint: Endpoint,
    config: &WsConfig,
    protocols: Option<&[&str]>,
) -> Result<(Socket, Vec<String>), WsError> {
    let tls = config.tls.build().map_err(WsError::Config)?;
    let stream = connect_stream(&endpoint, tls).await?;
    let request = client_request(value, config, protocols)?;
    let (socket, response) =
        tokio_tungstenite::client_async_with_config(request, stream, Some(config.protocol()))
            .await
            .map_err(super::websocket::socket_error)?;
    ensure_no_extensions(response.headers())?;
    let protocols = match protocols {
        Some(_) => response_protocols(response.headers())?,
        None => Vec::new(),
    };

    Ok((socket, protocols))
}

async fn connect_stream(
    endpoint: &Endpoint,
    tls: tokio_rustls::rustls::ClientConfig,
) -> Result<Box<dyn SocketIo>, WsError> {
    let tcp = TcpStream::connect((endpoint.host.as_str(), endpoint.port))
        .await
        .map_err(|_| WsError::Io)?;

    if !endpoint.secure {
        return Ok(Box::new(tcp));
    }

    let name = ServerName::try_from(endpoint.host.clone()).map_err(|_| WsError::Tls)?;
    let stream = TlsConnector::from(Arc::new(tls))
        .connect(name, tcp)
        .await
        .map_err(|_| WsError::Tls)?;

    Ok(Box::new(stream))
}

fn client_request(
    value: &str,
    config: &WsConfig,
    protocols: Option<&[&str]>,
) -> Result<Request<()>, WsError> {
    let mut request = value
        .into_client_request()
        .map_err(|_| WsError::Handshake)?;
    request.headers_mut().extend(config.headers.clone());

    if let Some(protocols) = protocols {
        let joined = HeaderValue::from_str(&protocols.join(", "))
            .map_err(|_| WsError::Config(ConfigError("invalid WebSocket subprotocol")))?;
        request
            .headers_mut()
            .insert("sec-websocket-protocol", joined);
    }

    for value in request.headers_mut().values_mut() {
        value.set_sensitive(true);
    }

    Ok(request)
}

fn response_protocols(headers: &HeaderMap) -> Result<Vec<String>, WsError> {
    headers
        .get_all("sec-websocket-protocol")
        .iter()
        .map(|value| value.to_str().map(str::to_owned))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| WsError::Handshake)
}

fn ensure_no_extensions(headers: &HeaderMap) -> Result<(), WsError> {
    // Tungstenite 0.29 does not reject unsolicited extension headers itself.
    if headers.contains_key("sec-websocket-extensions") {
        return Err(WsError::Handshake);
    }

    Ok(())
}
