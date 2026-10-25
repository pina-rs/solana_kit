//! Window Fetch ownership and bounded streamed reception.
use crate::BrowserCause;
use js_sys::{Reflect, Uint8Array};
use solana_kit_client::{Reply, Request, Transport, TransportError, TransportFailureKind as Kind};
use std::{
    cell::Cell,
    fmt,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
    time::Duration,
};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    AbortController, Headers, ReadableStreamDefaultReader, RequestCredentials, RequestInit,
    RequestMode, RequestRedirect, Response, ResponseType, Url, Window,
};

/// Invalid configuration with diagnostics that never include caller values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigError(&'static str);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for ConfigError {}

/// Caller-owned browser headers and a total exchange deadline.
#[derive(Clone)]
pub struct HttpConfig {
    /// Explicit application headers, including destination-specific authentication.
    pub headers: Vec<(String, String)>,
    /// Positive total deadline, at most i32::MAX milliseconds.
    pub exchange_timeout: Duration,
}
impl Default for HttpConfig {
    fn default() -> Self {
        const DEFAULT_EXCHANGE_TIMEOUT: Duration = Duration::from_secs(30);

        Self {
            headers: Vec::new(),
            exchange_timeout: DEFAULT_EXCHANGE_TIMEOUT,
        }
    }
}
impl fmt::Debug for HttpConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HttpConfig")
            .field("exchange_timeout", &self.exchange_timeout)
            .finish_non_exhaustive()
    }
}

/// Browser-window HTTP transport with explicit JavaScript capability requirements.
///
/// It does not support workers. Creating it requires an available Window.
/// Each exchange owns its abort controller and timer through completion or Drop.
pub struct BrowserHttp {
    endpoint: String,
    headers: Headers,
    timeout: i32,
    window: Window,
}

impl fmt::Debug for BrowserHttp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BrowserHttp").finish_non_exhaustive()
    }
}
impl BrowserHttp {
    /// Validates an absolute HTTP(S) endpoint and explicit caller configuration.
    pub fn new(endpoint: &str, config: HttpConfig) -> Result<Self, ConfigError> {
        let window = web_sys::window().ok_or(ConfigError("browser Window unavailable"))?;
        let millis = config.exchange_timeout.as_millis();
        let endpoint = parse_endpoint(endpoint)?;

        if millis == 0 || millis > i32::MAX as u128 {
            return Err(ConfigError("invalid exchange deadline"));
        }

        let headers = parse_headers(config.headers)?;

        Ok(Self {
            endpoint,
            headers,
            timeout: millis as i32,
            window,
        })
    }
}

/// Reject ambiguous spelling before WHATWG URL normalization can hide it.
fn parse_endpoint(endpoint: &str) -> Result<String, ConfigError> {
    let (scheme, rest) = endpoint
        .split_once("://")
        .ok_or(ConfigError("invalid absolute HTTP endpoint"))?;
    let authority = rest.split(['/', '?', '#']).next().unwrap_or("");

    if !scheme.eq_ignore_ascii_case("http") && !scheme.eq_ignore_ascii_case("https") {
        return Err(ConfigError("invalid absolute HTTP endpoint"));
    }

    if authority.is_empty()
        || authority.contains('@')
        || endpoint.contains(['#', '\\'])
        || endpoint
            .bytes()
            .any(|byte| byte.is_ascii_control() || byte == b' ')
    {
        return Err(ConfigError("invalid absolute HTTP endpoint"));
    }

    let url = Url::new(endpoint).map_err(|_| ConfigError("invalid absolute HTTP endpoint"))?;

    if url.host().is_empty() {
        return Err(ConfigError("invalid absolute HTTP endpoint"));
    }

    Ok(url.href())
}

fn parse_headers(values: Vec<(String, String)>) -> Result<Headers, ConfigError> {
    let headers = Headers::new().map_err(|_| ConfigError("browser headers unavailable"))?;
    headers
        .set("content-type", "application/json")
        .map_err(|_| ConfigError("browser headers unavailable"))?;

    for (name, value) in values {
        if reserved(&name) {
            return Err(ConfigError("reserved HTTP header override"));
        }

        headers
            .append(&name, &value)
            .map_err(|_| ConfigError("invalid HTTP header"))?;
    }

    Ok(headers)
}

fn reserved(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    const RESERVED: &[&str] = &[
        "accept-charset",
        "accept-encoding",
        "access-control-request-headers",
        "access-control-request-method",
        "connection",
        "content-length",
        "content-type",
        "cookie",
        "cookie2",
        "date",
        "dnt",
        "expect",
        "host",
        "keep-alive",
        "origin",
        "permissions-policy",
        "referer",
        "te",
        "trailer",
        "transfer-encoding",
        "upgrade",
        "user-agent",
        "via",
        "x-http-method",
        "x-http-method-override",
        "x-method-override",
    ];
    lower.starts_with("proxy-") || lower.starts_with("sec-") || RESERVED.contains(&lower.as_str())
}

/// Local boxed exchange. It starts only when polled and cancels on Drop.
pub struct BrowserExchange<'a> {
    future: Pin<Box<dyn Future<Output = Result<Reply, TransportError>> + 'a>>,
}
impl Future for BrowserExchange<'_> {
    type Output = Result<Reply, TransportError>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.future.as_mut().poll(cx)
    }
}
impl Transport for BrowserHttp {
    type Exchange<'a> = BrowserExchange<'a>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        BrowserExchange {
            future: Box::pin(execute(self, request)),
        }
    }
}

struct ExchangeOwner {
    controller: AbortController,
    window: Window,
    timer: i32,
    _callback: Closure<dyn FnMut()>,
    expired: Rc<Cell<bool>>,
    reader: Option<ReadableStreamDefaultReader>,
}
impl ExchangeOwner {
    fn new(adapter: &BrowserHttp) -> Result<Self, TransportError> {
        let controller =
            AbortController::new().map_err(|cause| caused_failure(Kind::Unavailable, cause))?;
        let expired = Rc::new(Cell::new(false));
        let flag = expired.clone();
        let abort = controller.clone();
        let callback = Closure::new(move || {
            flag.set(true);
            abort.abort();
        });
        let timer = adapter
            .window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                adapter.timeout,
            )
            .map_err(|cause| caused_failure(Kind::Unavailable, cause))?;

        Ok(Self {
            controller,
            window: adapter.window.clone(),
            timer,
            _callback: callback,
            expired,
            reader: None,
        })
    }
    fn error(&self, cause: JsValue) -> TransportError {
        let kind = if self.expired.get() {
            Kind::Deadline
        } else {
            Kind::Io
        };

        caused_failure(kind, cause)
    }
}
impl Drop for ExchangeOwner {
    fn drop(&mut self) {
        self.window.clear_timeout_with_handle(self.timer);
        self.controller.abort();
    }
}
fn failure(kind: Kind) -> TransportError {
    TransportError::new(kind)
}

fn caused_failure(kind: Kind, cause: JsValue) -> TransportError {
    TransportError::with_source(kind, BrowserCause::new(cause))
}

async fn execute(adapter: &BrowserHttp, request: Request) -> Result<Reply, TransportError> {
    execute_http(
        adapter,
        &adapter.endpoint,
        "POST",
        &adapter.headers,
        solana_kit_client::rest::Body::Json(request.body),
        request.max_reply_bytes,
    )
    .await
}

async fn execute_http(
    adapter: &BrowserHttp,
    endpoint: &str,
    method: &str,
    headers: &Headers,
    body: solana_kit_client::rest::Body,
    max_reply_bytes: usize,
) -> Result<Reply, TransportError> {
    let mut owner = ExchangeOwner::new(adapter)?;
    let init = request_init(method, headers, &body, &owner.controller);
    let fetch = adapter.window.fetch_with_str_and_init(endpoint, &init);
    let response: Response = JsFuture::from(fetch)
        .await
        .map_err(|cause| owner.error(cause))?
        .dyn_into()
        .map_err(|cause| caused_failure(Kind::InvalidResponse, cause))?;

    if matches!(
        response.type_(),
        ResponseType::Opaque | ResponseType::Opaqueredirect | ResponseType::Error
    ) || response.status() == 0
    {
        return Err(failure(Kind::InvalidResponse));
    }

    check_length(&response, max_reply_bytes)?;
    let status = response.status();
    let Some(stream) = response.body() else {
        return Ok(Reply {
            status,
            body: Vec::new(),
        });
    };
    let reader = stream
        .get_reader()
        .dyn_into::<ReadableStreamDefaultReader>()
        .map_err(|cause| caused_failure(Kind::Unavailable, cause.into()))?;
    owner.reader = Some(reader);
    let body = collect(&owner, max_reply_bytes).await?;

    Ok(Reply { status, body })
}

// Every route uses the same credential, redirect, referrer and abort policy.
fn request_init(
    method: &str,
    headers: &Headers,
    body: &solana_kit_client::rest::Body,
    controller: &AbortController,
) -> RequestInit {
    let init = RequestInit::new();
    init.set_method(method);
    init.set_headers(headers);

    if let Some(bytes) = body.as_deref() {
        init.set_body(&Uint8Array::from(bytes));
    }

    init.set_credentials(RequestCredentials::Omit);
    init.set_referrer("");
    init.set_referrer_policy(web_sys::ReferrerPolicy::NoReferrer);
    init.set_redirect(RequestRedirect::Error);
    init.set_mode(RequestMode::Cors);
    init.set_signal(Some(&controller.signal()));

    init
}

/// Explicit provider destinations over the existing Window Fetch owner.
pub struct BrowserRest<D> {
    routes: Vec<(D, BrowserHttp)>,
    max_target_bytes: usize,
}

impl<D: Eq> BrowserRest<D> {
    /// Binds unique approved routes and an encoded relative target limit.
    pub fn new(
        routes: Vec<(D, BrowserHttp)>,
        max_target_bytes: usize,
    ) -> Result<Self, ConfigError> {
        if routes.is_empty() || max_target_bytes == 0 {
            return Err(ConfigError(
                "REST routes and target budget must be nonempty",
            ));
        }

        for (index, (name, _)) in routes.iter().enumerate() {
            if routes[..index].iter().any(|(other, _)| name == other) {
                return Err(ConfigError("duplicate REST destination"));
            }
        }

        Ok(Self {
            routes,
            max_target_bytes,
        })
    }
}

impl<D> fmt::Debug for BrowserRest<D> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BrowserRest")
            .field("route_count", &self.routes.len())
            .finish_non_exhaustive()
    }
}

impl<D: Eq> solana_kit_client::rest::Transport<D> for BrowserRest<D> {
    type Exchange<'a>
        = BrowserExchange<'a>
    where
        Self: 'a;

    fn exchange(&self, request: solana_kit_client::rest::Request<D>) -> Self::Exchange<'_> {
        BrowserExchange {
            future: Box::pin(execute_rest(self, request)),
        }
    }
}

async fn execute_rest<D: Eq>(
    routes: &BrowserRest<D>,
    request: solana_kit_client::rest::Request<D>,
) -> Result<Reply, TransportError> {
    let adapter = routes
        .routes
        .iter()
        .find(|(name, _)| *name == request.service)
        .map(|(_, adapter)| adapter)
        .ok_or_else(|| failure(Kind::Denied))?;

    if request.method == solana_kit_client::rest::Method::Get && request.body.as_deref().is_some() {
        return Err(failure(Kind::Denied));
    }

    let endpoint = rest_endpoint(adapter, &request, routes.max_target_bytes)?;
    let headers = rest_headers(adapter, &request.body)?;

    execute_http(
        adapter,
        &endpoint,
        request.method.as_str(),
        &headers,
        request.body,
        request.max_reply_bytes,
    )
    .await
}

fn rest_endpoint<D>(
    adapter: &BrowserHttp,
    request: &solana_kit_client::rest::Request<D>,
    maximum: usize,
) -> Result<String, TransportError> {
    let root = Url::new(&adapter.endpoint).map_err(|cause| caused_failure(Kind::Denied, cause))?;
    let path = format!("{}{}", root.pathname(), root.search());
    let target = solana_kit_client::rest::request_target(
        &path,
        &request.path_segments,
        &request.query,
        maximum,
    )?;

    Ok(format!("{}{target}", root.origin()))
}

fn rest_headers(
    adapter: &BrowserHttp,
    body: &solana_kit_client::rest::Body,
) -> Result<Headers, TransportError> {
    let headers = Headers::new_with_headers(&adapter.headers)
        .map_err(|cause| caused_failure(Kind::Unavailable, cause))?;
    headers
        .delete("content-type")
        .map_err(|cause| caused_failure(Kind::Denied, cause))?;

    if let Some(content_type) = body.content_type() {
        headers
            .set("content-type", content_type)
            .map_err(|cause| caused_failure(Kind::Denied, cause))?;
    }

    Ok(headers)
}

fn check_length(response: &Response, max: usize) -> Result<(), TransportError> {
    let headers = response.headers();
    let encoding = headers
        .get("content-encoding")
        .map_err(|cause| caused_failure(Kind::InvalidResponse, cause))?;

    // CORS can hide Content-Encoding, so an absent header is not an identity guarantee.
    if encoding.as_deref() != Some("identity") {
        return Ok(());
    }

    if let Some(length) = headers
        .get("content-length")
        .map_err(|cause| caused_failure(Kind::InvalidResponse, cause))?
    {
        let length = length
            .parse::<u64>()
            .map_err(|_| failure(Kind::InvalidResponse))?;

        if u128::from(length) > max as u128 {
            return Err(failure(Kind::BodyLimit));
        }
    }

    Ok(())
}
async fn collect(owner: &ExchangeOwner, max: usize) -> Result<Vec<u8>, TransportError> {
    let mut body = Vec::new();
    let reader = owner
        .reader
        .as_ref()
        .ok_or_else(|| failure(Kind::Unavailable))?;

    loop {
        let value = JsFuture::from(reader.read())
            .await
            .map_err(|cause| owner.error(cause))?;
        let done = Reflect::get(&value, &JsValue::from_str("done"))
            .map_err(|cause| caused_failure(Kind::InvalidResponse, cause))?;

        if done.as_bool() == Some(true) {
            return Ok(body);
        }

        let chunk = Reflect::get(&value, &JsValue::from_str("value"))
            .map_err(|cause| caused_failure(Kind::InvalidResponse, cause))?;
        let chunk = chunk
            .dyn_into::<Uint8Array>()
            .map_err(|cause| caused_failure(Kind::InvalidResponse, cause))?;
        let length = chunk.length() as usize;
        let total = body
            .len()
            .checked_add(length)
            .filter(|total| *total <= max)
            .ok_or_else(|| failure(Kind::BodyLimit))?;
        body.try_reserve(length)
            .map_err(|_| failure(Kind::BodyLimit))?;
        let offset = body.len();
        body.resize(total, 0);
        chunk.copy_to(&mut body[offset..]);
    }
}
