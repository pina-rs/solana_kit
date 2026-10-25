//! Nonblocking Preview2 HTTP over explicitly driven local readiness.

use crate::endpoint::Endpoint;
use crate::reactor::Reactor;
use solana_kit_client::{Reply, Request, Transport, TransportError, TransportFailureKind as Kind};
use std::future::Future;
use std::pin::Pin;
use wasip2::clocks::monotonic_clock;
use wasip2::http::{outgoing_handler, types::*};
use wasip2::io::streams::StreamError;

const CHUNK_BYTES: usize = 65_536;
const HEADER_FIELDS: usize = 64;
const HEADER_FIELD_BYTES: usize = 8192;

/// Explicit Preview2 HTTP adapter with caller-owned local readiness.
pub struct WasiHttp {
    reactor: Reactor,
    pub(crate) endpoint: Endpoint,
    pub(crate) headers: Vec<(String, Vec<u8>)>,
    timeout_nanos: u64,
}

/// One local owned exchange. Dropping it releases all retained host resources.
pub struct Exchange<'a>(
    pub(crate) Pin<Box<dyn Future<Output = Result<Reply, TransportError>> + 'a>>,
);

impl Future for Exchange<'_> {
    type Output = Result<Reply, TransportError>;

    fn poll(
        mut self: Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.0.as_mut().poll(context)
    }
}

impl Transport for WasiHttp {
    type Exchange<'a> = Exchange<'a>;

    fn exchange(&self, request: Request) -> Exchange<'_> {
        Exchange(Box::pin(self.post(request)))
    }
}

impl WasiHttp {
    /// Creates a transport with explicit endpoint, timeout and application headers.
    ///
    /// # Errors
    /// Rejects invalid endpoints, zero/overflowing timeouts, invalid headers and
    /// caller overrides of transport-owned framing or connection headers.
    pub fn new(
        reactor: Reactor,
        endpoint: &str,
        timeout: std::time::Duration,
        headers: Vec<(String, Vec<u8>)>,
    ) -> Result<Self, ConfigurationError> {
        let endpoint = Endpoint::parse(endpoint).map_err(ConfigurationError::Endpoint)?;
        let timeout_nanos = u64::try_from(timeout.as_nanos())
            .ok()
            .filter(|nanos| *nanos != 0)
            .ok_or(ConfigurationError::Timeout)?;
        validate_headers(&headers)?;
        Fields::from_list(&headers).map_err(|_| ConfigurationError::Headers)?;

        Ok(Self {
            reactor,
            endpoint,
            headers,
            timeout_nanos,
        })
    }

    async fn wait(
        &self,
        signal: wasip2::io::poll::Pollable,
        deadline: u64,
    ) -> Result<(), TransportError> {
        self.reactor
            .wait(signal, deadline)
            .map_err(|()| TransportError::new(Kind::Unavailable))?
            .await
            .map_err(|()| TransportError::new(Kind::Deadline))
    }

    async fn post(&self, request: Request) -> Result<Reply, TransportError> {
        self.send(
            Method::Post,
            &self.endpoint.path,
            solana_kit_client::rest::Body::Json(request.body),
            request.max_reply_bytes,
        )
        .await
    }

    pub(crate) async fn send(
        &self,
        method: Method,
        target: &str,
        request_body: solana_kit_client::rest::Body,
        max_reply_bytes: usize,
    ) -> Result<Reply, TransportError> {
        let deadline = monotonic_clock::now()
            .checked_add(self.timeout_nanos)
            .ok_or_else(|| TransportError::new(Kind::Deadline))?;
        let (outgoing, options) = self.prepare(
            method,
            target,
            request_body.content_type(),
            request_body.as_deref().map_or(0, <[u8]>::len),
        )?;
        let bytes = request_body.into_bytes();
        let body = outgoing
            .body()
            .map_err(|()| TransportError::new(Kind::Io))?;
        let response = outgoing_handler::handle(outgoing, Some(options)).map_err(host_error)?;
        let response = self
            .upload_response(body, &bytes, response, deadline)
            .await?;
        let status = response.status();
        let body = response
            .consume()
            .map_err(|()| TransportError::new(Kind::InvalidResponse))?;

        let body = self.read_body(body, max_reply_bytes, deadline).await?;

        Ok(Reply { status, body })
    }

    async fn upload_response(
        &self,
        body: OutgoingBody,
        bytes: &[u8],
        response: FutureIncomingResponse,
        deadline: u64,
    ) -> Result<IncomingResponse, TransportError> {
        let mut upload = Some(Box::pin(self.write_body(body, bytes, deadline)));
        let mut response = Box::pin(self.response(response, deadline));
        let mut upload_error = None;

        // Servers may complete a response before consuming the upload. Poll
        // both directions so an early response or typed denial owns the result,
        // then drop the upload and its registrations under the original deadline.
        std::future::poll_fn(|context| {
            if let std::task::Poll::Ready(result) = response.as_mut().poll(context) {
                return std::task::Poll::Ready(upload_result(result, upload_error.take()));
            }

            if let Some(writer) = upload.as_mut()
                && let std::task::Poll::Ready(result) = writer.as_mut().poll(context)
            {
                upload_error = result.err();
                upload = None;
            }

            std::task::Poll::Pending
        })
        .await
    }

    fn prepare(
        &self,
        method: Method,
        target: &str,
        content_type: Option<&str>,
        length: usize,
    ) -> Result<(OutgoingRequest, RequestOptions), TransportError> {
        let mut headers = self.headers.clone();

        if let Some(content_type) = content_type {
            headers.push(("content-type".into(), content_type.as_bytes().to_vec()));
        }
        headers.push(("content-length".into(), length.to_string().into_bytes()));
        let fields = Fields::from_list(&headers).map_err(|_| TransportError::new(Kind::Denied))?;
        let outgoing = OutgoingRequest::new(fields);
        outgoing
            .set_method(&method)
            .map_err(|()| TransportError::new(Kind::Unavailable))?;
        outgoing
            .set_authority(Some(&self.endpoint.authority))
            .map_err(|()| TransportError::new(Kind::Denied))?;
        outgoing
            .set_path_with_query(Some(target))
            .map_err(|()| TransportError::new(Kind::Denied))?;

        let scheme = if self.endpoint.secure {
            Scheme::Https
        } else {
            Scheme::Http
        };

        outgoing
            .set_scheme(Some(&scheme))
            .map_err(|()| TransportError::new(Kind::Denied))?;
        let options = RequestOptions::new();
        options
            .set_connect_timeout(Some(self.timeout_nanos))
            .map_err(|()| TransportError::new(Kind::Unavailable))?;
        options
            .set_first_byte_timeout(Some(self.timeout_nanos))
            .map_err(|()| TransportError::new(Kind::Unavailable))?;

        options
            .set_between_bytes_timeout(Some(self.timeout_nanos))
            .map_err(|()| TransportError::new(Kind::Unavailable))?;

        Ok((outgoing, options))
    }

    async fn write_body(
        &self,
        body: OutgoingBody,
        bytes: &[u8],
        deadline: u64,
    ) -> Result<(), TransportError> {
        let stream = body.write().map_err(|()| TransportError::new(Kind::Io))?;
        let mut offset = 0;

        while offset < bytes.len() {
            check_deadline(deadline)?;
            let allowed = stream.check_write().map_err(stream_error)?;

            if allowed == 0 {
                self.wait(stream.subscribe(), deadline).await?;
                continue;
            }

            let count = usize::try_from(allowed)
                .unwrap_or(usize::MAX)
                .min(bytes.len() - offset)
                .min(CHUNK_BYTES);
            stream
                .write(&bytes[offset..offset + count])
                .map_err(stream_error)?;
            offset += count;
        }

        stream.flush().map_err(stream_error)?;

        while stream.check_write().map_err(stream_error)? == 0 {
            self.wait(stream.subscribe(), deadline).await?;
        }

        drop(stream);
        OutgoingBody::finish(body, None).map_err(host_error)
    }

    async fn response(
        &self,
        response: FutureIncomingResponse,
        deadline: u64,
    ) -> Result<IncomingResponse, TransportError> {
        loop {
            check_deadline(deadline)?;

            if let Some(response) = response.get() {
                return response
                    .map_err(|()| TransportError::new(Kind::InvalidResponse))?
                    .map_err(host_error);
            }

            self.wait(response.subscribe(), deadline).await?;
        }
    }

    async fn read_body(
        &self,
        body: IncomingBody,
        limit: usize,
        deadline: u64,
    ) -> Result<Vec<u8>, TransportError> {
        let stream = body
            .stream()
            .map_err(|()| TransportError::new(Kind::InvalidResponse))?;
        let mut bytes = Vec::new();

        loop {
            check_deadline(deadline)?;
            let remaining = limit.saturating_sub(bytes.len());

            match read_chunk(&stream, remaining)? {
                Chunk::Pending => self.wait(stream.subscribe(), deadline).await?,
                Chunk::Bytes(chunk) => append_chunk(&mut bytes, &chunk)?,
                Chunk::Closed => break,
            }
        }

        drop(stream);
        self.finish(IncomingBody::finish(body), deadline).await?;

        Ok(bytes)
    }

    async fn finish(&self, trailers: FutureTrailers, deadline: u64) -> Result<(), TransportError> {
        loop {
            check_deadline(deadline)?;

            if let Some(trailers) = trailers.get() {
                trailers
                    .map_err(|()| TransportError::new(Kind::InvalidResponse))?
                    .map_err(host_error)?;

                return Ok(());
            }

            self.wait(trailers.subscribe(), deadline).await?;
        }
    }
}

enum Chunk {
    Pending,
    Bytes(Vec<u8>),
    Closed,
}

fn append_chunk(bytes: &mut Vec<u8>, chunk: &[u8]) -> Result<(), TransportError> {
    bytes
        .try_reserve_exact(chunk.len())
        .map_err(|error| TransportError::with_source(Kind::BodyLimit, error))?;
    bytes.extend_from_slice(chunk);

    Ok(())
}

fn read_chunk(
    stream: &wasip2::io::streams::InputStream,
    remaining: usize,
) -> Result<Chunk, TransportError> {
    let read = (remaining.min(CHUNK_BYTES) as u64) + 1;

    let chunk = match stream.read(read) {
        Ok(chunk) => chunk,
        Err(StreamError::Closed) => return Ok(Chunk::Closed),
        Err(error) => return Err(stream_error(error)),
    };

    if chunk.len() > remaining {
        return Err(TransportError::new(Kind::BodyLimit));
    }

    if chunk.is_empty() {
        return Ok(Chunk::Pending);
    }

    Ok(Chunk::Bytes(chunk))
}

fn validate_headers(headers: &[(String, Vec<u8>)]) -> Result<(), ConfigurationError> {
    let invalid = headers.iter().any(|(name, value)| {
        name.len().saturating_add(value.len()) > HEADER_FIELD_BYTES
            || matches!(
                name.to_ascii_lowercase().as_str(),
                "content-length"
                    | "content-type"
                    | "host"
                    | "connection"
                    | "transfer-encoding"
                    | "upgrade"
            )
    });

    if headers.len() > HEADER_FIELDS || invalid {
        return Err(ConfigurationError::Headers);
    }

    Ok(())
}

fn check_deadline(deadline: u64) -> Result<(), TransportError> {
    if monotonic_clock::now() >= deadline {
        return Err(TransportError::new(Kind::Deadline));
    }

    Ok(())
}

/// Invalid explicit adapter configuration, before starting network work.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigurationError {
    /// REST routes are empty, duplicated or lack a target limit.
    Routes,
    /// The endpoint cannot be represented by this conservative HTTP contract.
    Endpoint(&'static str),
    /// The timeout is zero or overflows the host's nanosecond clock.
    Timeout,
    /// Headers are invalid, too large or override transport-owned fields.
    Headers,
}

impl std::fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid WASI HTTP configuration: {self:?}")
    }
}

impl std::error::Error for ConfigurationError {}

fn host_error(error: ErrorCode) -> TransportError {
    let kind = match error {
        ErrorCode::HttpRequestDenied => Kind::Denied,
        ErrorCode::ConnectionTimeout
        | ErrorCode::ConnectionReadTimeout
        | ErrorCode::ConnectionWriteTimeout => Kind::Deadline,
        ErrorCode::HttpProtocolError
        | ErrorCode::HttpResponseIncomplete
        | ErrorCode::HttpResponseTransferCoding(_) => Kind::InvalidResponse,
        _ => Kind::Io,
    };

    TransportError::with_source(kind, std::io::Error::other(format!("{error:?}")))
}

fn stream_error(error: StreamError) -> TransportError {
    match error {
        StreamError::Closed => TransportError::new(Kind::Io),
        StreamError::LastOperationFailed(error) => {
            if let Some(code) = http_error_code(&error) {
                return host_error(code);
            }

            TransportError::with_source(Kind::Io, std::io::Error::other(error.to_debug_string()))
        }
    }
}

fn upload_result(
    response: Result<IncomingResponse, TransportError>,
    upload_error: Option<TransportError>,
) -> Result<IncomingResponse, TransportError> {
    match (response, upload_error) {
        (Ok(response), _) => Ok(response),

        (Err(error), _) if error.kind == Kind::Denied => Err(error),
        (Err(_), Some(error)) => Err(error),
        (Err(error), None) => Err(error),
    }
}
