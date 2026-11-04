//! Concrete native adapters owned by the publisher CLI.

use solana_kit_client::{
    Reply, TransportError, TransportFailureKind,
    rest::{self, Transport},
    subscriptions::{Clock, ClockError},
};
use solana_kit_dapp_publisher::portal::{PortalService, UploadRequest, Uploader};
use solana_kit_dapp_publisher::workflow::{SourceReader, Storage};
use std::{
    future::{Future, Ready},
    io,
    path::PathBuf,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const MAX_TARGET_BYTES: usize = 16_384;
const PORTAL_TIMEOUT_SECONDS: u64 = 30;
const UPLOAD_TIMEOUT_SECONDS: u64 = 60;

/// Blocking native portal transport with caller-owned origin and API key.
pub struct PortalTransport {
    origin: String,
    base_path: String,
    api_key: String,
    agent: ureq::Agent,
}

impl PortalTransport {
    /// Binds one HTTPS portal origin and its `/api` base path.
    pub fn new(api_base_url: &str, api_key: &str) -> Result<Self, String> {
        let uri = api_base_url
            .parse::<http::Uri>()
            .map_err(|_| "invalid portal API URL")?;
        let scheme = uri.scheme_str().ok_or("portal API URL requires a scheme")?;
        let authority = uri.authority().ok_or("portal API URL requires a host")?;
        let secure = scheme == "https"
            || (scheme == "http"
                && authority
                    .host()
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|ip| ip.is_loopback()));

        if !secure {
            return Err("portal API URL requires HTTPS".into());
        }

        let origin = format!("{scheme}://{authority}");
        let path = uri.path().trim_end_matches('/').to_owned();
        let agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(PORTAL_TIMEOUT_SECONDS)))
            .http_status_as_error(false)
            .max_redirects(0)
            .build()
            .new_agent();

        Ok(Self {
            origin,
            base_path: if path.is_empty() { "/".into() } else { path },
            api_key: api_key.to_owned(),
            agent,
        })
    }

    /// Sends one typed request without redirects, retries or implicit decoding.
    fn execute(&self, request: rest::Request<PortalService>) -> Result<Reply, TransportError> {
        let target = rest::request_target(
            &self.base_path,
            &request.path_segments,
            &request.query,
            MAX_TARGET_BYTES,
        )?;
        let url = format!("{}{target}", self.origin);
        let content_type = request.body.content_type();
        let body = request.body.into_bytes();
        let response = self
            .send(&request.method, &url, content_type, &body)
            .map_err(|error| TransportError::with_source(TransportFailureKind::Io, error))?;
        let status = response.status().as_u16();
        let body = response
            .into_body()
            .read_to_vec()
            .map_err(|error| TransportError::with_source(TransportFailureKind::Io, error))?;

        Ok(Reply { status, body })
    }

    fn send(
        &self,
        method: &rest::Method,
        url: &str,
        content_type: Option<&str>,
        body: &[u8],
    ) -> Result<ureq::http::Response<ureq::Body>, ureq::Error> {
        match method {
            rest::Method::Get => self
                .agent
                .get(url)
                .header("accept", "application/json")
                .header("x-api-key", &self.api_key)
                .call(),
            rest::Method::Delete => self
                .agent
                .delete(url)
                .header("accept", "application/json")
                .header("x-api-key", &self.api_key)
                .call(),
            rest::Method::Post | rest::Method::Put | rest::Method::Patch => {
                self.send_mutation(method, url, content_type, body)
            }
        }
    }

    fn send_mutation(
        &self,
        method: &rest::Method,
        url: &str,
        content_type: Option<&str>,
        body: &[u8],
    ) -> Result<ureq::http::Response<ureq::Body>, ureq::Error> {
        let builder = match method {
            rest::Method::Post => self.agent.post(url),
            rest::Method::Put => self.agent.put(url),
            rest::Method::Patch => self.agent.patch(url),
            rest::Method::Get | rest::Method::Delete => unreachable!(),
        };

        let builder = builder
            .header("accept", "application/json")
            .header("x-api-key", &self.api_key);

        let builder = match content_type {
            Some(content_type) => builder.header("content-type", content_type),
            None => builder,
        };

        builder.send(body)
    }
}

impl Transport<PortalService> for PortalTransport {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: rest::Request<PortalService>) -> Self::Exchange<'_> {
        std::future::ready(self.execute(request))
    }
}

/// One presigned binary PUT through the same native HTTP stack.
pub struct HttpUploader {
    agent: ureq::Agent,
}

impl Default for HttpUploader {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpUploader {
    /// Creates a bounded HTTPS uploader with redirects disabled.
    #[must_use]
    pub fn new() -> Self {
        let agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(UPLOAD_TIMEOUT_SECONDS)))
            .http_status_as_error(false)
            .max_redirects(0)
            .build()
            .new_agent();

        Self { agent }
    }
}

impl Uploader for HttpUploader {
    type Upload<'a> = Ready<Result<(), TransportError>>;

    fn upload<'a>(&'a self, request: UploadRequest<'a>) -> Self::Upload<'a> {
        std::future::ready(self.send(&request))
    }
}

impl HttpUploader {
    fn send(&self, request: &UploadRequest<'_>) -> Result<(), TransportError> {
        approve_upload_target(&request.target.upload_url)?;
        let response = self
            .agent
            .put(request.target.upload_url.as_str())
            .header("content-type", request.content_type)
            .send(request.body)
            .map_err(|error| TransportError::with_source(TransportFailureKind::Io, error))?;
        let status = response.status().as_u16();
        let _ = response
            .into_body()
            .read_to_vec()
            .map_err(|error| TransportError::with_source(TransportFailureKind::Io, error))?;

        if (200..300).contains(&status) {
            return Ok(());
        }

        Err(TransportError::new(TransportFailureKind::InvalidResponse))
    }
}

fn approve_upload_target(url: &str) -> Result<(), TransportError> {
    let uri = url
        .parse::<http::Uri>()
        .map_err(|_| TransportError::new(TransportFailureKind::Denied))?;
    let scheme = uri
        .scheme_str()
        .ok_or(TransportError::new(TransportFailureKind::Denied))?;
    let authority = uri
        .authority()
        .ok_or(TransportError::new(TransportFailureKind::Denied))?;
    let local_http = scheme == "http"
        && authority
            .host()
            .parse::<std::net::IpAddr>()
            .is_ok_and(|ip| ip.is_loopback());

    if scheme == "https" || local_http {
        return Ok(());
    }

    Err(TransportError::new(TransportFailureKind::Denied))
}

/// One bounded file-backed checkpoint in the current working directory.
pub struct FileStorage {
    path: PathBuf,
}

impl FileStorage {
    /// Binds an explicit checkpoint path.
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Storage for FileStorage {
    type Load<'a> = Ready<Result<Option<Vec<u8>>, TransportError>>;
    type Save<'a> = Ready<Result<(), TransportError>>;

    fn load<'a>(&'a self, _key: &'a str, maximum: usize) -> Self::Load<'a> {
        std::future::ready(self.read_checkpoint(maximum))
    }

    fn save<'a>(&'a mut self, _key: &'a str, bytes: &'a [u8], maximum: usize) -> Self::Save<'a> {
        std::future::ready(self.write_checkpoint(bytes, maximum))
    }
}

impl FileStorage {
    /// Reads at most one checkpoint; an absent file is `Ok(None)`, not an error.
    fn read_checkpoint(&self, maximum: usize) -> Result<Option<Vec<u8>>, TransportError> {
        let Some(metadata) = self.checkpoint_metadata()? else {
            return Ok(None);
        };

        if metadata.len() > maximum as u64 {
            return Err(TransportError::new(TransportFailureKind::BodyLimit));
        }

        std::fs::read(&self.path)
            .map(Some)
            .map_err(|error| TransportError::with_source(TransportFailureKind::Io, error))
    }

    /// Separates an absent checkpoint from other metadata failures.
    fn checkpoint_metadata(&self) -> Result<Option<std::fs::Metadata>, TransportError> {
        match std::fs::metadata(&self.path) {
            Ok(metadata) => Ok(Some(metadata)),
            Err(error) => metadata_error(error),
        }
    }

    fn write_checkpoint(&self, bytes: &[u8], maximum: usize) -> Result<(), TransportError> {
        if bytes.len() > maximum {
            return Err(TransportError::new(TransportFailureKind::BodyLimit));
        }

        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|error| TransportError::with_source(TransportFailureKind::Io, error))?;
        }

        let temporary = self.path.with_extension("tmp");
        std::fs::write(&temporary, bytes)
            .map_err(|error| TransportError::with_source(TransportFailureKind::Io, error))?;

        std::fs::rename(&temporary, &self.path)
            .map_err(|error| TransportError::with_source(TransportFailureKind::Io, error))
    }
}

fn metadata_error(error: io::Error) -> Result<Option<std::fs::Metadata>, TransportError> {
    if error.kind() != io::ErrorKind::NotFound {
        return Err(TransportError::with_source(TransportFailureKind::Io, error));
    }

    Ok(None)
}

/// Reads one bounded local APK through the caller-supplied path token.
pub struct FileSource;

impl SourceReader for FileSource {
    type Read<'a> = Ready<Result<Vec<u8>, TransportError>>;

    fn read<'a>(&'a self, token: &'a str, maximum: usize) -> Self::Read<'a> {
        let result = (|| {
            let metadata = std::fs::metadata(token)
                .map_err(|error| TransportError::with_source(TransportFailureKind::Io, error))?;

            if metadata.len() > maximum as u64 {
                return Err(TransportError::new(TransportFailureKind::BodyLimit));
            }

            std::fs::read(token)
                .map_err(|error| TransportError::with_source(TransportFailureKind::Io, error))
        })();

        std::future::ready(result)
    }
}

/// Monotonic milliseconds backed by the process clock.
#[derive(Clone, Copy)]
pub struct SystemClock {
    start: Instant,
}

impl Default for SystemClock {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemClock {
    /// Starts one monotonic clock origin.
    #[must_use]
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl Clock for SystemClock {
    type Sleep<'a> = Sleep;

    fn now(&self) -> u64 {
        u64::try_from(self.start.elapsed().as_millis()).unwrap_or(u64::MAX)
    }

    fn sleep_until(&self, deadline: u64) -> Self::Sleep<'_> {
        Sleep {
            deadline,
            clock: *self,
        }
    }
}

/// Blocking sleep future; the CLI has no executor or detached task.
pub struct Sleep {
    deadline: u64,
    clock: SystemClock,
}

impl Future for Sleep {
    type Output = Result<(), ClockError>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if self.clock.now() >= self.deadline {
            Poll::Ready(Ok(()))
        } else {
            std::thread::sleep(Duration::from_millis(1));
            Poll::Pending
        }
    }
}

/// Current UTC timestamp in the workflow's accepted ISO-8601 form.
pub fn utc_now() -> Result<String, String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "system clock is before the Unix epoch")?;
    let datetime = time::OffsetDateTime::from_unix_timestamp_nanos(
        i128::try_from(now.as_nanos()).map_err(|_| "system time is out of range")?,
    )
    .map_err(|_| "invalid system time")?;
    datetime
        .format(&time::format_description::well_known::Rfc3339)
        .map_err(|_| "failed to format system time".into())
}
