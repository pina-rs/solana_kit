# solana_kit_native

Tokio HTTP/1 and verified TLS transport for off-chain Solana application code.
Use Pina or official Solana program libraries for on-chain programs.

`NativeHttp::new(endpoint, HttpConfig::default())` implements the public
`solana_kit_client::Transport`. Each polled exchange owns its connection and
HTTP driver. It creates no runtime, pool, detached task, redirect, retry, proxy,
cookie store or ambient authentication. Caller headers and DER trust roots are
explicit. Diagnostics omit endpoint and header values. HTTPS verifies both the
certificate chain and destination name; verification cannot be disabled.

The total deadline covers DNS, TCP, TLS, response headers and complete body.
The connect deadline also bounds DNS/TCP/TLS. Dropping the exchange closes owned
network work; Tokio's blocking OS name resolution may finish after cancellation,
but cannot subsequently send HTTP. Dropping a spawned Tokio JoinHandle detaches
it: abort and await the handle to cancel that call.

Response body bytes are checked before each reserve and copy against the
request budget, independent of Content-Length. Hyper's parser enforces header
byte and field-count limits, including the status line; its byte limit also
bounds chunk trailers. The separately configured read buffer is at least 8192
bytes. These are bounded buffers in addition to the bounded collected body;
the body budget is not a promise of an exact whole-process allocation ceiling.
Non-success statuses and their bounded bodies return unchanged for the client
policy. Compressed bodies are not automatically decompressed.

The named native future boxes one Send future because stable Rust cannot name
an async-block associated GAT. Generic Tokio multiworker callers retain Send;
portable/local client futures are unchanged. HTTP/2, connection pooling,
WebSockets, browser/WASI/guest adapters and device journeys are separate work.

```rust
use solana_kit_client::{Client, Limits};
use solana_kit_native::{HttpConfig, NativeHttp};

let transport = NativeHttp::new("https://rpc.invalid/", HttpConfig::default())?;
let client = Client::new(transport, Limits::default());
# let _ = client;
# Ok::<(), Box<dyn std::error::Error>>(())
```

Construction performs no network request. See the repository's native HTTP
verification receipt for executed local controls and remaining platform limits.
