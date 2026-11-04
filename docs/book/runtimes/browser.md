# Browser

`solana_kit_browser` is the transport for web pages compiled to `wasm32-unknown-unknown`. It provides `BrowserHttp` over Window Fetch and `BrowserWebSocket` over the browser's WebSocket. Its futures are local: they are not `Send`, so you can hold an `Rc` across an `.await`, and you run them on the page's single-threaded executor. Importing this crate is the only thing that brings JavaScript bindings into your build. The core and client crates stay free of browser imports.

## Add the crate

```toml
[dependencies]
solana_kit_client = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_browser = { git = "https://github.com/pina-rs/solana_kit" }
wasm-bindgen-futures = "0.4"
```

Build for the browser target:

```sh
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
```

The crate exports its API only on `wasm32-unknown-unknown`. Compiling for that target does not prove that a browser Window exists at run time. `BrowserHttp` needs a Window. It does not run in Web Workers or in interpreted no-JavaScript guests such as LeptosKit.

Optional features:

| Feature  | Adds                                                                                                        |
| -------- | ----------------------------------------------------------------------------------------------------------- |
| `keys`   | `keys::BrowserEntropy`, Web Crypto `getRandomValues` for `ExportableKeypair::generate`                      |
| `wallet` | `wallet::WalletDiscovery`, `wallet::WalletController` and a typed wallet signer (see [Wallets](wallets.md)) |

## Make an RPC call

```rust
use solana_kit_browser::{BrowserHttp, HttpConfig};
use solana_kit_client::{Client, Limits, methods::Balance};
use solana_kit_core::Address;
use std::time::Duration;

async fn balance(address: Address) -> Result<u64, Box<dyn std::error::Error>> {
    let transport = BrowserHttp::new(
        "https://rpc.invalid/",
        HttpConfig {
            exchange_timeout: Duration::from_secs(10),
            ..Default::default()
        },
    )?;
    let client = Client::new(transport, Limits::default());
    let reply = client.call(Balance(address, None)).await?;
    Ok(reply.value)
}

// Run it on the page's local executor.
wasm_bindgen_futures::spawn_local(async {
    let _ = balance(Address::from_bytes([0; 32])).await;
});
```

`HttpConfig` has two fields:

| Field              | Default | Meaning                                                                    |
| ------------------ | ------- | -------------------------------------------------------------------------- |
| `headers`          | empty   | `(name, value)` pairs sent with each request. Unsafe headers are rejected. |
| `exchange_timeout` | 30 s    | Total deadline, including streamed body reception.                         |

## What Fetch does for you, and what it does not

The browser owns the network. The adapter calls Fetch once per exchange and leaves connection pooling, DNS, proxies, certificate checks and CSP to the browser. Things to know:

- **Credentials and referrers are omitted.** Ambient cookies are not sent. Put authentication in `headers`, scoped to that endpoint.
- **Redirects are rejected.** Nothing is retried.
- **CORS applies.** Your RPC endpoint must allow your page's origin. CORS denials, TLS failures and unreachable hosts all come back as I/O failures, because the browser hides the difference. Opaque responses are rejected.
- **Bodies are bounded in Rust only.** The client's reply budget limits the bytes collected in Rust, after any decompression. It does not bound the browser's own network buffers or a single JavaScript chunk.
- **Non-success status survives.** An HTTP 429 or 503 reaches the client as a status error, not a fabricated success.

Errors print without endpoint or header values. To inspect the original JavaScript error, walk `std::error::Error::source` and downcast to `BrowserCause`. Its `value()` method returns the exact JavaScript value.

## Cancellation

An exchange that has never been polled has not started any I/O. Dropping a polled exchange aborts the Fetch and its body stream through an `AbortController`. No task is detached.

## WebSockets

`BrowserWebSocket::connect(endpoint, WsConfig::default()).await` opens one WS or WSS socket. The browser checks WSS certificates with its own trust policy.

```rust
use solana_kit_browser::{BrowserWebSocket, WsConfig};

let mut socket = BrowserWebSocket::connect("wss://rpc.invalid/", WsConfig::default()).await?;
socket.send(r#"{"jsonrpc":"2.0","id":1,"method":"rootSubscribe"}"#)?;
let text = socket.receive().await?;
socket.close().await?;
```

The calls differ from the native socket in two ways:

- `send(&mut self, text)` is synchronous. It hands one complete frame to the browser, or returns `WsError::Backpressure` without sending anything if the browser's buffer is over budget.
- `receive()` returns the next text message. It rejects binary messages with `WsError::Binary`.

For typed subscriptions, pair the socket with an independent `solana_kit_client::subscriptions::Connection`. The pattern is the same as on [Native](native.md): register first, send each outbound frame, then ingest each received frame with the socket's generation. [Subscriptions](../core/subscriptions.md) describes `Connection`.

`WsConfig` fields:

| Field               | Default | Meaning                                                  |
| ------------------- | ------- | -------------------------------------------------------- |
| `protocols`         | empty   | Subprotocol offers (at most 16).                         |
| `queue_messages`    | 64      | Messages held in the receive queue.                      |
| `queue_bytes`       | 1 MiB   | Payload bytes held in the receive queue.                 |
| `message_bytes`     | 256 KiB | Largest incoming or outgoing message.                    |
| `buffered_bytes`    | 1 MiB   | Largest browser `bufferedAmount` after a send.           |
| `operation_timeout` | 30 s    | Per-operation deadline; a quiet socket past it is ended. |

<Callout kind="warning" title="Browsers cannot send Authorization headers on WebSockets">

There is no `headers` field. Use a query parameter or a subprotocol your provider accepts. The browser may also attach its own cookies to the handshake. Private and plain `ws://` destinations are allowed, so validate any endpoint that comes from untrusted input.

</Callout>

Cancelling `receive()` keeps the socket and its queued messages. `close(self)` consumes the socket and discards queued data. Dropping the socket detaches every handler and asks the browser to close. There is no receive task, reconnect or retry.

## Status

| Capability                                                       | Status                                                                                                                                           |
| ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| `BrowserHttp`                                                    | Runs in CI in pinned Chrome 155 against a local Rust fixture: typed `getBalance`, body limits, gzip, deadlines, cancellation, redirects and CORS |
| `BrowserWebSocket` text sockets                                  | Run in CI in a Chrome Window                                                                                                                     |
| `BrowserSubscriptionSocket`, `receive_frame()` and binary frames | Source extension; needs browser replay                                                                                                           |
| `BrowserRest` (provider REST routes)                             | Source extension; needs a browser driver and real peer replay                                                                                    |
| `wallet` and `keys` features                                     | Tests run in CI in Chrome against a fixture wallet                                                                                               |
| Web Workers, native webviews, mobile browsers                    | Unverified                                                                                                                                       |

The receipts are [browser HTTP](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/browser-http.md) and [browser WebSocket](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/browser-websocket.md). The design notes are in [`docs/architecture`](https://github.com/pina-rs/solana_kit/tree/main/docs/architecture).
