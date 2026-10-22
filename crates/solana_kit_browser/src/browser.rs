//! Window Fetch ownership and bounded streamed reception.
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
        Self {
            headers: Vec::new(),
            exchange_timeout: Duration::from_secs(30),
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
        let controller = AbortController::new().map_err(|_| failure(Kind::Unavailable))?;
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
            .map_err(|_| failure(Kind::Unavailable))?;

        Ok(Self {
            controller,
            window: adapter.window.clone(),
            timer,
            _callback: callback,
            expired,
            reader: None,
        })
    }
    fn error(&self) -> TransportError {
        failure(if self.expired.get() {
            Kind::Deadline
        } else {
            Kind::Io
        })
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

async fn execute(adapter: &BrowserHttp, request: Request) -> Result<Reply, TransportError> {
    let mut owner = ExchangeOwner::new(adapter)?;
    let init = RequestInit::new();
    init.set_method("POST");
    init.set_headers(&adapter.headers);
    init.set_body(&Uint8Array::from(request.body.as_slice()));

    init.set_credentials(RequestCredentials::Omit);
    init.set_referrer("");
    init.set_referrer_policy(web_sys::ReferrerPolicy::NoReferrer);
    init.set_redirect(RequestRedirect::Error);
    init.set_mode(RequestMode::Cors);
    init.set_signal(Some(&owner.controller.signal()));
    let fetch = adapter
        .window
        .fetch_with_str_and_init(&adapter.endpoint, &init);
    let response: Response = JsFuture::from(fetch)
        .await
        .map_err(|_| owner.error())?
        .dyn_into()
        .map_err(|_| failure(Kind::InvalidResponse))?;

    if matches!(
        response.type_(),
        ResponseType::Opaque | ResponseType::Opaqueredirect | ResponseType::Error
    ) || response.status() == 0
    {
        return Err(failure(Kind::InvalidResponse));
    }

    check_length(&response, request.max_reply_bytes)?;
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
        .map_err(|_| failure(Kind::Unavailable))?;
    owner.reader = Some(reader);
    let body = collect(&owner, request.max_reply_bytes).await?;

    Ok(Reply { status, body })
}

fn check_length(response: &Response, max: usize) -> Result<(), TransportError> {
    let headers = response.headers();
    let encoding = headers
        .get("content-encoding")
        .map_err(|_| failure(Kind::InvalidResponse))?;

    // CORS can hide Content-Encoding, so an absent header is not an identity guarantee.
    if encoding.as_deref() != Some("identity") {
        return Ok(());
    }

    if let Some(length) = headers
        .get("content-length")
        .map_err(|_| failure(Kind::InvalidResponse))?
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
            .map_err(|_| owner.error())?;
        let done = Reflect::get(&value, &JsValue::from_str("done"))
            .map_err(|_| failure(Kind::InvalidResponse))?;

        if done.as_bool() == Some(true) {
            return Ok(body);
        }

        let chunk = Reflect::get(&value, &JsValue::from_str("value"))
            .map_err(|_| failure(Kind::InvalidResponse))?;
        let chunk = chunk
            .dyn_into::<Uint8Array>()
            .map_err(|_| failure(Kind::InvalidResponse))?;
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
