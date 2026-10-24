//! One owned socket and HTTP driver, polled in the exchange itself.
use crate::NativeHttp;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::{
    body::{Body, Incoming},
    client::conn::http1,
    http::header,
};
use hyper_util::rt::TokioIo;
use solana_kit_client::{Reply, Request, TransportError, TransportFailureKind as Kind};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
    time::timeout,
};
use tokio_rustls::{TlsConnector, rustls::pki_types::ServerName};

pub(crate) async fn execute(
    adapter: &NativeHttp,
    request: Request,
) -> Result<Reply, TransportError> {
    timeout(adapter.config.exchange_timeout, connect(adapter, request))
        .await
        .map_err(|error| TransportError::with_source(Kind::Deadline, error))?
}

async fn connect(adapter: &NativeHttp, request: Request) -> Result<Reply, TransportError> {
    let stream = timeout(adapter.config.connect_timeout, open(adapter))
        .await
        .map_err(|error| TransportError::with_source(Kind::Deadline, error))??;

    match stream {
        Stream::Plain(stream) => exchange(adapter, request, stream).await,
        Stream::Tls(stream) => exchange(adapter, request, *stream).await,
    }
}

enum Stream {
    Plain(TcpStream),
    Tls(Box<tokio_rustls::client::TlsStream<TcpStream>>),
}

async fn open(adapter: &NativeHttp) -> Result<Stream, TransportError> {
    let endpoint = &adapter.endpoint;
    let socket = TcpStream::connect((endpoint.host.as_str(), endpoint.port))
        .await
        .map_err(|error| TransportError::with_source(Kind::Io, error))?;

    if !endpoint.secure {
        return Ok(Stream::Plain(socket));
    }

    let name = ServerName::try_from(endpoint.host.clone())
        .map_err(|error| TransportError::with_source(Kind::Denied, error))?;
    let stream = TlsConnector::from(adapter.tls.clone())
        .connect(name, socket)
        .await
        .map_err(|error| TransportError::with_source(Kind::Io, error))?;

    Ok(Stream::Tls(Box::new(stream)))
}

async fn exchange<S>(
    adapter: &NativeHttp,
    request: Request,
    stream: S,
) -> Result<Reply, TransportError>
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    let mut builder = http1::Builder::new();
    builder
        .max_headers(adapter.config.max_headers)
        .max_header_size(adapter.config.max_header_bytes)
        .max_buf_size(adapter.config.max_read_buffer);
    let (mut sender, connection) = builder
        .handshake(TokioIo::new(stream))
        .await
        .map_err(http_error)?;

    let limit = request.max_reply_bytes;
    let outgoing = outgoing(adapter, request.body)?;
    let receive = async move {
        let response = sender.send_request(outgoing).await.map_err(http_error)?;
        collect(response, limit).await
    };
    tokio::pin!(connection, receive);

    // The driver is borrowed here, never spawned; cancellation closes the socket.
    tokio::select! {
        result = &mut receive => result,
        result = &mut connection => {
            result.map_err(http_error)?;
            receive.await
        }
    }
}

/// Builds the fixed POST envelope after the connection handshake succeeds.
fn outgoing(
    adapter: &NativeHttp,
    body: Vec<u8>,
) -> Result<hyper::Request<Full<Bytes>>, TransportError> {
    let mut outgoing = hyper::Request::builder()
        .method("POST")
        .uri(&adapter.endpoint.path)
        .body(Full::new(Bytes::from(body)))
        .map_err(|error| TransportError::with_source(Kind::Denied, error))?;

    *outgoing.headers_mut() = adapter.config.headers.clone();
    outgoing.headers_mut().insert(
        header::HOST,
        adapter
            .endpoint
            .authority
            .parse()
            .map_err(|error| TransportError::with_source(Kind::Denied, error))?,
    );
    outgoing.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    outgoing.headers_mut().insert(
        header::CONNECTION,
        header::HeaderValue::from_static("close"),
    );

    Ok(outgoing)
}

async fn collect(
    response: hyper::Response<Incoming>,
    limit: usize,
) -> Result<Reply, TransportError> {
    let status = response.status().as_u16();
    let mut incoming = response.into_body();
    let mut body = Vec::new();

    if incoming.size_hint().lower() > u64::try_from(limit).unwrap_or(u64::MAX) {
        return Err(TransportError::new(Kind::BodyLimit));
    }

    while let Some(frame) = incoming.frame().await {
        let frame = frame.map_err(http_error)?;

        if let Ok(bytes) = frame.into_data() {
            append(&mut body, &bytes, limit)?;
        }
    }

    Ok(Reply { status, body })
}

/// Checks untrusted lengths before reserving or copying into the collected body.
fn append(body: &mut Vec<u8>, bytes: &[u8], limit: usize) -> Result<(), TransportError> {
    if bytes.len() > limit.saturating_sub(body.len()) {
        return Err(TransportError::new(Kind::BodyLimit));
    }

    body.try_reserve_exact(bytes.len())
        .map_err(|error| TransportError::with_source(Kind::BodyLimit, error))?;
    body.extend_from_slice(bytes);

    Ok(())
}

fn http_error(error: hyper::Error) -> TransportError {
    let kind = if error.is_parse_too_large() {
        Kind::HeaderLimit
    } else if error.is_parse() || error.is_incomplete_message() || error.is_body_write_aborted() {
        Kind::InvalidResponse
    } else {
        Kind::Io
    };

    TransportError::with_source(kind, error)
}
