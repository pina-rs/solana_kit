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

## Key generation

The optional `keys` feature adds caller-owned `keys::BrowserEntropy` for
`solana_kit_core::crypto::ExportableKeypair::generate`. It uses the current global Web
Crypto `getRandomValues`, including workers exposing that API, and splits large
requests into legal bounded chunks. Host failures retain their original JavaScript
cause behind redacted diagnostics. Temporary typed-array storage is cleared after
copying or failure. This feature adds no task, persistence or implicit backup.

## Browser wallets

The optional `wallet` feature adds explicit Window discovery through Wallet
Standard's `wallet-standard:register-wallet` and `wallet-standard:app-ready`
protocol. Create a `wallet::WalletDiscovery` over a caller-owned
`solana_kit_wallet::WalletRegistry`; retain discovery for the lifetime of the page
integration. No global singleton, injected-wallet name guess or background task is
installed.

Create a `wallet::WalletController` with an `Rc<WalletDiscovery>` and a Solana chain
identifier. Select a discovered wallet and await `controller.connect(wallet,
false)?`. Connection futures establish state immediately: dropping an unpolled or
pending future cancels caller interest and leaves a coherent state. Accounts and
capabilities come from the actual wallet. Live controllers require the standard
connect and change-event features and the requested chain before prompting. Account changes, unregistering a wallet,
disposal and replacement invalidate old signers and wake pending operations.
Window wallets can publish account changes before their connect promise resolves;
that normal handshake is reconciled against the returned authorization.

`controller.signer()?` implements the low-level Kit `MessageSigner`,
`TransactionPartialSigner`, `TransactionModifyingSigner` and
`TransactionSendingSigner` traits with `WalletFailure` as their explicit failure
parameter. Exact-message and partial-transaction interfaces reject modified bytes;
the modifying interface validates the signature over its actual returned message.
Transaction version and account/chain capabilities are checked before a request.
A submitted identifier is checked against the supplied transaction and is never
confirmation of execution.

A wallet and ordinary local key can use the regular
`solana_kit_signers::local::Composition::<WalletFailure>::with_failure` path. Register
the wallet with `Registration::partial` and the key with
`Registration::partial_with(&key_capability, WalletFailure::from)`. The mapper
retains the native signer error as a typed cause. No JavaScript state or `Rc` error
is added to default native error types or portable core dependencies.

Wallet promises are external operations and Wallet Standard does not supply a
universal abort method. Cancellation prevents their stale results from restoring
local authorization; it cannot undo an authorization prompt or submitted
transaction already handled by the wallet. Signing requires a live controller and
its exact account authorization. `WalletFailure` diagnostics redact backend values;
explicit `std::error::Error::source` inspection can retrieve `BrowserCause` and its
original JavaScript object identity.

Call `discovery.close()` when early teardown is needed and inspect
`discovery.take_failures()` for registration, event and cleanup errors. Drop also
removes this owner's Window listener and registrations. A broken wallet's
unsubscribe callback can fail; the original failure is recorded and its closed
registration cannot authorize more work.

This feature targets Window-based off-chain application code. It does not add
browser globals to WASI or interpreted LeptosKit guests. Native/mobile wallet
handover and UI composition require their respective adapters. It is not an
on-chain program API; use Pina or the official Solana program libraries there.
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
