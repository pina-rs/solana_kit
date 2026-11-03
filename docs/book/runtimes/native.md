# Native

`solana_kit_native` is the transport for programs that run on an operating system with Tokio: CLIs, servers, bots and desktop backends. It provides `NativeHttp`, a bounded HTTP/1 transport over Hyper with rustls TLS, and `NativeWebSocket`, a bounded text WebSocket. Each one owns its connection directly. There is no pool, background driver or ambient configuration. You supply the endpoint, headers, trust roots and deadlines.

## Add the crate

```toml
[dependencies]
solana_kit_client = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_native = { git = "https://github.com/pina-rs/solana_kit" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

See [Installation](../getting-started/installation.md) for how to pin a revision. The crate uses Tokio for sockets and timers, but it never creates a runtime. You start the runtime yourself.

Optional features:

| Feature | Adds |
| --- | --- |
| `keys` | `keys::NativeEntropy` (OS randomness) and `keys::write_keypair_new` for explicit Solana-keygen JSON backups |
| `mwa` | `mwa::NativeAssociation` and `mwa::NativeMobileWallet`, a caller-owned Mobile Wallet Adapter session (see [Wallets](wallets.md)) |
| `surfpool` | `surfpool::deploy` and process ownership for a local Surfpool validator (see [Surfpool](../services/surfpool.md)) |

## Make an RPC call

`NativeHttp::new` validates the endpoint and configuration, then returns a `Transport`. Pass it to `Client::new`:

```rust
use solana_kit_client::{Client, Limits, methods::Balance};
use solana_kit_core::Address;
use solana_kit_native::{HeaderValue, HttpConfig, NativeHttp};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = HttpConfig::default();
    config.headers.insert(
        "authorization",
        HeaderValue::from_static("Bearer your-token"),
    );

    let transport = NativeHttp::new("https://rpc.invalid/", config)?;
    let client = Client::new(transport, Limits::default());

    let balance = client
        .call(Balance(Address::from_bytes([0; 32]), None))
        .await?;
    println!("{} lamports at slot {}", balance.value, balance.context.slot);
    Ok(())
}
```

Construction performs no network request. Each call opens its own connection, sends one POST and reads the complete reply.

The exchange future is `Send`. You can move a client into `tokio::spawn` on a multi-threaded runtime. The test suite runs a typed `getBalance` call this way and checks that `u64::MAX` survives intact.

## Configure the transport

`HttpConfig` holds every policy for one transport. Its fields are public:

| Field | Default | Meaning |
| --- | --- | --- |
| `headers` | empty | Your authentication and application headers. Framing, `Host`, content type and hop-by-hop headers are rejected. |
| `connect_timeout` | 10 s | Bounds DNS, TCP and the TLS handshake. |
| `exchange_timeout` | 30 s | Bounds the whole exchange, from the first poll to the last body byte. |
| `max_header_bytes` | 16,384 | Limit for the status line plus headers, and for chunk trailers. |
| `max_headers` | 64 | Limit on response header fields. |
| `max_read_buffer` | 16,384 | Hyper's read buffer. Must be at least 8,192 and cover the header limit. |
| `tls` | `TlsRoots::WebPki` | Trust roots for HTTPS. |

The reply body limit comes from the client's request budget (`Limits`), not from `HttpConfig`. The transport checks it before each reserve and copy, whatever `Content-Length` says. Compressed bodies are not decompressed.

### Trust roots

`TlsRoots` has two variants:

- `TlsRoots::WebPki` uses Mozilla's root set, pinned through `webpki-roots`. The OS trust store is never read.
- `TlsRoots::Certificates(Vec<Vec<u8>>)` trusts only the DER-encoded CA certificates you supply.

Certificate chain and hostname verification are always on, and you cannot turn them off.

### What the transport will not do

It does not follow redirects, retry, use a proxy, keep cookies or look up credentials. A non-success status comes back unchanged so the client can decide what to do with it. URLs that contain userinfo or a fragment are rejected. Put credentials in `headers`.

## Deadlines and cancellation

The `exchange_timeout` deadline covers DNS, TCP, TLS, response headers and the whole body. Dropping the exchange future closes its socket, TLS session and HTTP driver.

One caveat: Tokio resolves hostnames on a blocking thread. The OS resolver can finish after you cancel. It can no longer send the HTTP request.

If you spawned the call, dropping the `JoinHandle` only detaches it. To cancel, abort the handle and then await it:

```rust
let handle = tokio::spawn(async move {
    client.call(Balance(Address::from_bytes([0; 32]), None)).await
});

handle.abort();
let _ = handle.await; // the peer now sees the connection close
```

## WebSockets

`NativeWebSocket::connect(endpoint, WsConfig::default()).await` opens one `ws://` or `wss://` socket that you own. It carries text frames. Pair it with an independent `solana_kit_client::subscriptions::Connection`, which handles subscription requests, correlation and typed notifications. The socket only moves frames.

This example is adapted from the loopback test `tests/websocket_subscriptions.rs`:

```rust
use solana_kit_client::subscriptions::{Budget, Connection, Drain, Root};
use solana_kit_native::{NativeWebSocket, WsConfig};

const GENERATION: u64 = 1;
const ACK_DEADLINE: u64 = 100; // in your own monotonic ticks

let mut socket = NativeWebSocket::connect("wss://rpc.invalid/", WsConfig::default()).await?;
let mut connection = Connection::new(GENERATION, Budget::default());

let handle = connection.subscribe(Root, ACK_DEADLINE)?;

while let Some(frame) = connection.outbound() {
    socket.send(std::str::from_utf8(&frame)?).await?;
}

// Feed every received text frame back in, tagged with this socket's generation.
while let Some(text) = socket.receive().await? {
    connection.ingest(GENERATION, text.as_bytes())?;

    if let Drain::Notification(root) = connection.next(&handle)? {
        println!("new root: {root}");
        break;
    }
}

connection.unsubscribe(&handle, ACK_DEADLINE)?;

while let Some(frame) = connection.outbound() {
    socket.send(std::str::from_utf8(&frame)?).await?;
}
socket.close().await?;
```

You own the clock. Drive acknowledgement deadlines with `connection.tick(now, cleanup_deadline)`. If `ingest` fails for the current generation, drop the socket. [Subscriptions](../core/subscriptions.md) covers the `Connection` contract in full.

`WsConfig` holds the socket's limits:

| Field | Default | Meaning |
| --- | --- | --- |
| `headers` | empty | Authentication and application headers. Protocol and cookie headers are rejected. |
| `connect_timeout` | 10 s | DNS, TCP, TLS and the upgrade handshake together. |
| `operation_timeout` | 30 s | Each `send`, `receive` and the graceful `close`. |
| `max_frame_bytes` | 1 MiB | Largest incoming frame and outgoing text. |
| `max_message_bytes` | 1 MiB | Largest reassembled incoming message. |
| `read_buffer_bytes` | 8,192 | Read buffer. Must be positive and at most the frame limit. |
| `max_write_buffer_bytes` | 1 MiB plus frame header | Encoded bytes held for writing. |
| `max_request_bytes` | 16,384 | Endpoint plus header bytes. |
| `tls` | `TlsRoots::WebPki` | Trust roots for `wss://`. |

Cancellation rules:

- Cancelling `receive()` keeps the socket and its parser state, so you can call it again.
- Cancelling `send()` can leave a partial write. Drop the socket afterward, and do not resend automatically.
- `close(self)` consumes the socket and waits for the peer's close within one operation deadline.

There is no reconnect, replay, automatic ping timer or implicit unsubscribe. Use `wss://` on untrusted networks. Accept endpoints only from trusted configuration, because there is no private-destination filter.

<Callout kind="warning" title="Do not enable trace logging on credentialed sockets">

Tungstenite's wire-level trace logs can include complete requests and frames, including your headers.

</Callout>

## Status

| Capability | Status |
| --- | --- |
| `NativeHttp` HTTP/1 and TLS | Runs in CI on Ubuntu, macOS and Windows against local fixtures, plus an MSRV (1.89) build |
| `NativeWebSocket` WS and WSS text, all nine subscription families | Runs in CI on Ubuntu, macOS and Windows with Rust 1.99.1 and 1.89.0 |
| `NativeSubscriptionSocket` (Session adapter, binary frames) | Source extension; the crate README marks it as not yet executed |
| `NativeRest` (provider REST routes) | Source extension; real REST peer replay is pending |
| HTTP/2, connection pooling | Not provided |

The receipts are [native HTTP](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/native-http.md) and [native WebSocket](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/native-websocket.md). All tests use loopback fixtures and synthetic credentials. No public cluster is involved.
