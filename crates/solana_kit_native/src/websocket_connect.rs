//! One handshake future owns DNS, TCP and TLS until the caller receives the socket.
use crate::{ConfigError, WsConfig, WsError, endpoint::Endpoint};
use std::sync::Arc;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
};
use tokio_rustls::{TlsConnector, rustls::pki_types::ServerName};
use tokio_tungstenite::{WebSocketStream, tungstenite::client::IntoClientRequest};

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
    let tls = config.tls.build().map_err(WsError::Config)?;
    let tcp = TcpStream::connect((endpoint.host.as_str(), endpoint.port))
        .await
        .map_err(|_| WsError::Io)?;
    let stream: Box<dyn SocketIo> = if endpoint.secure {
        let name = ServerName::try_from(endpoint.host).map_err(|_| WsError::Tls)?;
        Box::new(
            TlsConnector::from(Arc::new(tls))
                .connect(name, tcp)
                .await
                .map_err(|_| WsError::Tls)?,
        )
    } else {
        Box::new(tcp)
    };

    let mut request = value
        .into_client_request()
        .map_err(|_| WsError::Handshake)?;
    request.headers_mut().extend(config.headers.clone());

    for value in request.headers_mut().values_mut() {
        value.set_sensitive(true);
    }

    let (socket, response) =
        tokio_tungstenite::client_async_with_config(request, stream, Some(config.protocol()))
            .await
            .map_err(super::websocket::socket_error)?;

    // Tungstenite 0.29 does not reject unsolicited extension headers itself.
    if response.headers().contains_key("sec-websocket-extensions") {
        return Err(WsError::Handshake);
    }

    Ok(socket)
}
