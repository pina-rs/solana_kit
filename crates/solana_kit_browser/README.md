# Solana Kit browser HTTP

This crate is exclusively for off-chain browser application code. Use Pina or
Solana's program libraries for on-chain programs. Explicitly importing this
adapter opts into JavaScript. Core and client crates do not gain browser imports.

`BrowserSubscriptionSocket::from(BrowserWebSocket)` adapts the maintained socket
to `subscriptions::Session` without a task or executor. Pending receive and close
operations retain their event wakers and bounded timers between transient polls.
Synchronous sends hand bounded text to browser buffering. The shared Session
receives bounded text and binary frames through the same maintained owner.

`BrowserHttp` implements the executor-neutral public `Transport` using Window
Fetch. It supports browser windows, not workers or interpreted no-JavaScript
guests. Its named exchange future is local and boxes once. Dropping a polled
exchange aborts Fetch and body reception. No task is detached. An unpolled
exchange has not started I/O. The total deadline includes streamed body reception.

The caller supplies an absolute HTTP(S) endpoint, headers and deadline. URL
userinfo and fragments reject. Diagnostics omit endpoint and header values.
Fetch omits credentials and referrers and rejects redirects. The adapter invokes
Fetch once per exchange and adds no retry loop.
Caller authentication remains destination-specific. Browser networking and its
connection pooling remain under browser control. The browser can use its own
proxy configuration, DNS cache, CSP and certificate policy. This crate neither
discovers nor configures proxies. Exports exist only on wasm32-unknown-unknown;
that target alone does not prove that a browser Window is available.

The response budget bounds bytes accumulated in Rust before reserve and copy.
Fetch may decompress responses and allocate network chunks before Rust sees them.
This does not bound browser parser buffers, headers, network buffers or a single
JavaScript chunk. No header allocation limit is claimed. CORS determines which
headers are visible. A Content-Length hint is used only for identity-encoded
visible responses. CORS denials, TLS failures and other Fetch rejections are
reported as I/O failures because browsers deliberately hide their distinction.
Opaque responses are rejected. Non-success status is preserved for the client.
HTTP failures retain their exact original JavaScript value in `BrowserCause`.
Normal Debug and Display omit that value. Explicit error-source traversal and
downcasting to `BrowserCause` lets callers borrow it through `value()`, including
its object identity and any private diagnostics. Retaining an error also retains
the JavaScript value and objects reachable from it. These local sources do not
promise Send or Sync. Native and WASI error sources keep those bounds.

## WebSocket

`BrowserWebSocket::connect(endpoint, WsConfig::default()).await` opens one WS or WSS
socket. The browser validates WSS certificates using its actual trust policy.
`send(&mut self, text)` admits a complete bounded UTF-8 frame once or returns
`Backpressure` before sending it. `receive(&mut self).await` borrows the owner.
Cancelling receive releases its timer and waker while retaining the socket and
queued messages. `close(self).await` consumes the owner and discards queued data.
Cancelling connect or close drops the socket. Drop detaches all handlers and asks
the browser to close local I/O. A positive finite operation deadline ends a socket
that remains quiet. No receive task, executor, retry or reconnect is installed.

Use the independent `solana_kit_client::subscriptions::Connection` to register
before send, drain its complete outbound frames, and ingest each received frame
with its generation. The application owns its protocol ticks and terminates that
Connection when the socket ends. Socket and Connection Drop affect their own
state separately. Signature completion leaves other subscriptions active.

`WsConfig` bounds endpoint spelling, subprotocol offers, message payload bytes,
queued message count and bytes, and browser bufferedAmount. `receive()` retains
its text-only contract and rejects binary. The source-only `receive_frame()` and
`BrowserSubscriptionSocket` expose explicit text or ArrayBuffer-backed binary to
the client `Session`. Blob and unknown representations remain errors. The browser
can allocate a complete event and payload before Rust checks its size. Exact UTF-8
length is computed from JavaScript UTF-16 before converting text. Frame and queue
admission precede Rust text or binary copying.
No bound on browser framing, network, TLS, or whole-process memory is promised.

The new Session path retains operation deadlines between short polls and awaits
the actual browser close event. Its Helius consumer keeps Rc across real socket
awaits. Binary forwarding and these new bodies still require browser replay;
historical text-only evidence does not verify the current source extension.

Browsers cannot set arbitrary Authorization headers. Caller URL query parameters
and subprotocols are supported under browser policy. Browser-managed cookies may
be sent. Explicit WS and private destinations are permitted, so applications must
validate untrusted destinations themselves. Config/socket Debug and error output
omit endpoint, subprotocol, message and close-reason values. Browser errors hide
handshake and TLS causes and use `Browser`. Applications retain responsibility for
their own frame logging and browser developer tools. The executed scope is Chrome
Window; Worker, native UI webviews, interpreted Wasm and mobile devices remain
unverified. See the [ownership design](../../docs/architecture/browser-websocket.md)
and [execution receipt](../../docs/verification/browser-websocket.md).

The unverified REST source extension adds `BrowserRest<D>` over the existing
Window Fetch owner. Named routes retain approved roots and configured headers.
Encoded segments and ordered duplicate query pairs append to the approved base
path and fixed query. Decoded dot segments and GET bodies are denied before Fetch.
Each exchange derives content type from absent, JSON or form bytes, owns its
abort controller and deadline, omits ambient credentials and referrers, rejects
redirects and reuses bounded streamed reception. Browser CORS still applies.
The new public provider consumer has actual Fetch calls with Rc across awaits.
It still needs an admitted browser driver and real peer replay. Historical RPC
or socket evidence does not verify this REST source.
