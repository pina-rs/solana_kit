# Solana Kit browser HTTP

This crate is exclusively for off-chain browser application code. Use Pina or
Solana's program libraries for on-chain programs. Explicitly importing this
adapter opts into JavaScript. Core and client crates do not gain browser imports.

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
JavaScript error objects are discarded to avoid retaining unbounded or sensitive
browser diagnostics and cannot satisfy portable Send plus Sync error sources.
