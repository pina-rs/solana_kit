//! Explicit loopback destination grants, controlled roots and request ownership.

use http_body_util::BodyExt;
use std::future::Future;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::io::{AsyncRead, AsyncWrite};
use wasmtime_wasi_http::{Error, RequestOptions, WasiBody, WasiHttpHooks};

type IoResult = Result<(), Error>;
type IoFuture = Box<dyn Future<Output = IoResult> + Send>;
type SendResult = Result<(http::Response<WasiBody>, IoFuture), Error>;
const DNS_DELAY: std::time::Duration = std::time::Duration::from_millis(500);

#[derive(Clone, Default)]
pub struct Counts {
    pub active: Arc<AtomicUsize>,
    pub opened: Arc<AtomicUsize>,
}

struct Flight(Counts);

impl Flight {
    fn new(counts: Counts) -> Self {
        counts.active.fetch_add(1, Ordering::SeqCst);
        Self(counts)
    }
}

impl Drop for Flight {
    fn drop(&mut self) {
        self.0.active.fetch_sub(1, Ordering::SeqCst);
    }
}

pub struct Policy {
    pub authority: String,
    pub denied: bool,
    pub denial_delay: Option<std::time::Duration>,
    pub upload_block: bool,
    pub delay_dns: bool,
    pub roots: Arc<rustls::ClientConfig>,
    pub counts: Counts,
}

impl WasiHttpHooks for Policy {
    /// Grants and controlled delays precede actual TCP and normal TLS verification.
    fn send_request(
        &mut self,
        request: http::Request<WasiBody>,
        _: Option<RequestOptions>,
        completion: IoFuture,
    ) -> Box<dyn Future<Output = SendResult> + Send> {
        let allowed = !self.denied
            && request
                .uri()
                .authority()
                .is_some_and(|a| a.as_str() == self.authority);
        let roots = self.roots.clone();
        let counts = self.counts.clone();
        let delay_dns = self.delay_dns;
        let denial_delay = self.denial_delay;

        Box::new(async move {
            let _completion = completion;

            if !allowed {
                return Err(Error::HttpRequestDenied);
            }

            if let Some(delay) = denial_delay {
                let _request = request;
                tokio::time::sleep(delay).await;
                return Err(Error::HttpRequestDenied);
            }

            let flight = Flight::new(counts.clone());

            if delay_dns {
                tokio::time::sleep(DNS_DELAY).await;
            }

            send(request, roots, counts, flight).await
        })
    }

    fn p2_outgoing_body_chunk_size(&mut self) -> usize {
        if self.upload_block { 1 } else { 1024 * 1024 }
    }
}

trait Stream: AsyncRead + AsyncWrite + Unpin + Send + Sync {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send + Sync> Stream for T {}

async fn send(
    mut request: http::Request<WasiBody>,
    roots: Arc<rustls::ClientConfig>,
    counts: Counts,
    flight: Flight,
) -> SendResult {
    let stream = connect(request.uri().clone(), roots, &counts).await?;
    let (mut sender, connection) =
        hyper::client::conn::http1::handshake(wasmtime_wasi_http::io::TokioIo::new(stream)).await?;
    *request.uri_mut() = http::Uri::builder()
        .path_and_query(request.uri().path_and_query().map_or("/", |p| p.as_str()))
        .build()
        .expect("valid path");
    let mut connection = Box::pin(connection);
    let mut sending = Box::pin(sender.send_request(request));
    let mut connection_complete = false;
    let response = tokio::select! {
        response = &mut sending => response?,
        outcome = &mut connection => {
            outcome?;
            connection_complete = true;
            sending.await?
        }
    };
    // The connection and its accounting guard share one owned future. Dropping
    // the guest response cancels both rather than detaching a Tokio task.
    let io: IoFuture = Box::new(async move {
        let _flight = flight;
        if connection_complete {
            Ok(())
        } else {
            connection.await.map_err(Error::from)
        }
    });

    Ok((
        response.map(|body| body.map_err(Error::from).boxed_unsync()),
        io,
    ))
}

async fn connect(
    uri: http::Uri,
    roots: Arc<rustls::ClientConfig>,
    counts: &Counts,
) -> Result<Box<dyn Stream>, Error> {
    let authority = uri.authority().ok_or(Error::HttpRequestUriInvalid)?;
    let port = authority.port_u16().ok_or(Error::HttpRequestUriInvalid)?;
    let name = rustls::pki_types::ServerName::try_from(authority.host().to_owned())
        .map_err(|_| Error::HttpRequestUriInvalid)?;
    let socket = tokio::net::TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, port))
        .await
        .map_err(Error::Connect)?;
    counts.opened.fetch_add(1, Ordering::SeqCst);
    let stream: Box<dyn Stream> = if uri.scheme_str() == Some("https") {
        Box::new(
            tokio_rustls::TlsConnector::from(roots)
                .connect(name, socket)
                .await
                .map_err(Error::Tls)?,
        )
    } else {
        Box::new(socket)
    };

    Ok(stream)
}
