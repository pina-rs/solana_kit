# Browser HTTP execution

This receipt covers an explicit Window Fetch adapter for off-chain application
code. Use Pina or Solana's program libraries for on-chain programs. Browser
networking does not establish interpreted-guest or installed mobile support.

## Executed environment

Local macOS execution uses Rust 1.99.1, wasm-bindgen-test 0.3.78,
wasm-bindgen-test-runner 0.2.128, real Chrome 155.0.8059.85 and ChromeDriver
155.0.8059.84. MSRV 1.89 compiles the public browser API and an ordinary external
source consumer for wasm32-unknown-unknown without repository flags or patches.
The external source consumer compilation is distinct from runtime execution.

The Rust verification command binds two ephemeral IPv4 loopback listeners. It
owns the browser test child process, request workers and redirect sink. It stops
the listeners and joins every worker after the child exits, including failure.
All authentication, cookies, request bytes and responses are synthetic fixtures.
No public cluster, paid service, secret or machine certificate is used.

## Behavioral evidence

Six actual Chrome test groups cover these contracts:

- Typed getBalance checks exact POST, content type and JSON before returning exact
  u64::MAX context and value. An Rc survives an observed Pending. A real browser
  cookie is installed first and the fixture confirms that Fetch omits it.
- Fixed-length, chunked and exposed identity-length responses pass at the Rust
  body limit and fail one byte below it or at zero. A truncated declared body
  fails. HTTP 429 status survives transport and becomes the client's status error.
- A deterministic 24-byte gzip fixture expands to 64 bytes. Exact decoded budget
  succeeds, while 63-byte and 24-byte budgets fail. Both exposed and CORS-hidden
  Content-Encoding exercise actual browser decompression.
- Quiet headers and an incomplete streamed body reach the total deadline. The
  peer observes EOF. A retained pending exchange first keeps its socket open.
  Dropping it produces EOF for both header and body stalls. An unpolled dropped
  exchange starts no request.
- Caller authentication arrives at its explicit destination. A redirect is
  rejected while a distinct listener observes zero connections. CORS denial and
  an unavailable local port return I/O failures without fabricated status.
- Malformed endpoint, userinfo, fragments, malformed ports, unsafe headers and
  invalid deadlines reject. Debug diagnostics omit authentication values.

A native fixture regression forces an accepted socket into nonblocking mode,
delays the first request bytes and proves that the fixture waits correctly. It
fails before the explicit blocking-mode transition. This prevents speculative
browser connections from creating false transport failures.

## Reproduction

Install the pinned Rust toolchains, wasm32-unknown-unknown standard libraries and
wasm-bindgen-cli 0.2.128. Supply CHROME and CHROMEDRIVER paths for the named hosts.
Run from the repository root:

```sh
cargo +1.99.1 run -p solana_kit_browser_http_verification --locked
cargo +1.99.1 run -p solana_kit_browser_http_verification --locked -- --graphs
cargo +1.99.1 test -p solana_kit_browser_http_verification --bins --locked
cargo +1.89.0 check -p solana_kit_browser --target wasm32-unknown-unknown --locked
```

The graph command checks core and client normal/build closures for native,
wasm32-unknown-unknown, wasm32-wasip1 and wasm32-wasip2. Each exact package/version
allowlist rejects an injected JavaScript dependency row. Core retains only bs58.
Client retains its existing 13 external package/version pairs. These are graph
checks and synthetic policy controls, not networking in WASI or wasmi. Public
core and client source remain unchanged by this adapter.

Workspace tests, warning-denied native and browser Clippy and documentation,
formatting, actionlint, Monochange validation and release preview pass locally.
The unchanged per-file quality gate includes authored fixture Rust. Publishing
stays disabled. The browser archive still requires the deliberately unpublished
client crate. An ordinary source consumer compiles separately, so registry
readiness remains unproven. Submitted Linux CI is a separate acceptance step.

## Receive and host limits

The body limit controls Rust's collected decoded bytes before reservation and
copying. Fetch may allocate network chunks or decompress before Rust receives
those bytes. Browser parser buffers and headers cannot be bounded by this API.
CORS can hide metadata. A visible identity encoding is required before treating
Content-Length as a decoded-body hint. Browser event-loop scheduling and timer
clamping apply to deadlines. Browser DNS, proxies, TLS policy and connection
pooling remain browser-owned. CORS and network failures deliberately share an
I/O category because Fetch hides their cause.

Workers, browser subscriptions, wallet discovery, WASI transport, no-JavaScript
guests, LeptosKit networking and generated desktop, Android and iOS application
journeys remain outside this bounded HTTP unit. This receipt does not complete
Phase 4 or the ecosystem.
