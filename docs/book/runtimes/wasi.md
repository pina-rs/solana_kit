# WASI

`solana_kit_wasi` is the transport for WebAssembly components on `wasm32-wasip2`. `WasiHttp` sends RPC over `wasi:http`. With the `websocket` feature, `WasiWebSocket` opens sockets over `wasi:sockets` and runs TLS and WebSocket framing inside your guest. Both use a `Reactor` that you create and drive. There is no global executor, thread or background task. Each future polls only nonblocking WASI operations, and you choose when to wait.

## Add the crate

```toml
[dependencies]
solana_kit_client = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_wasi = { git = "https://github.com/pina-rs/solana_kit" }
# For WebSockets:
# solana_kit_wasi = { git = "https://github.com/pina-rs/solana_kit", features = ["websocket"] }
```

```sh
rustup target add wasm32-wasip2
cargo build --target wasm32-wasip2
```

The adapter types exist only on `wasm32-wasip2`. On Preview1 and plain Wasm the crate still exposes the portable client and its `Endpoint` parser, but no networking. The pinned contract is `wasi:http`, `wasi:io` and `wasi:clocks` 0.2.12, through `wasip2` 1.0.4.

<Callout kind="note" title="The websocket feature needs a C toolchain">

The `websocket` feature brings in rustls with the `ring` provider. Building `ring` for `wasm32-wasip2` needs a WASI-capable C compiler and archiver.

</Callout>

## Make an RPC call

Create one `Reactor`, hand a clone to the transport, and run your future with `Reactor::block_on`. This example is adapted from the embedded-host guest in `verification/wasi-http-prototype`:

```rust
use solana_kit_client::{Client, Limits, methods::Balance};
use solana_kit_core::Address;
use solana_kit_wasi::{Reactor, WasiHttp};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reactor = Reactor::default();
    let transport = WasiHttp::new(
        reactor.clone(),
        "https://rpc.invalid/",
        Duration::from_secs(30),
        vec![("authorization".into(), b"Bearer your-token".to_vec())],
    )?;
    let client = Client::new(transport, Limits::default());

    let balance = reactor.block_on(client.call(Balance(Address::from_bytes([0; 32]), None)))??;
    println!("{} lamports", balance.value);
    Ok(())
}
```

`block_on` returns `Result<F::Output, DriveError>`, hence the double `?`. It drives WASI readiness and computations that complete immediately. If your future returns Pending without registering any WASI readiness, `block_on` returns `DriveError` instead of hanging. Embedders with their own loop call `Reactor::drive()` between polls instead. `Reactor::registrations()` reports how many readiness registrations are live, which is useful for checking cleanup.

`WasiHttp::new(reactor, endpoint, timeout, headers)` takes:

- the endpoint, validated at construction,
- one nonzero total timeout covering upload, headers, body and trailers,
- headers as `(String, Vec<u8>)` pairs. Framing and connection headers are rejected.

The futures are local, so you can hold an `Rc` across `.await`.

## The host owns the network policy

In WASI the embedding host, not your guest, runs the HTTP client. That changes who controls what:

| Concern                                    | Owner                                                             |
| ------------------------------------------ | ----------------------------------------------------------------- |
| Destination allowlist                      | The host. A denial reaches you as `TransportFailureKind::Denied`. |
| DNS, TLS roots and certificate checks      | The host. The guest cannot add roots or turn off validation.      |
| Response header and trailer parsing limits | The host.                                                         |
| Body bytes copied into Rust                | The guest, through the client's reply budget.                     |

<Callout kind="warning" title="Wasmtime's CLI flag is not an allowlist">

`wasmtime run -S http=y` lets the component reach any HTTP destination. To confine destinations, embed Wasmtime with an explicit policy.

</Callout>

If the host lacks a required import, the component fails to instantiate. It never becomes a guest `TransportError`.

Dropping a polled exchange removes its readiness registration and releases the request and response resources. Named-host tests confirm that the peer sees EOF while the guest stays alive. Remote cancellation acknowledgement and DNS cancellation are not promised.

## WebSockets and subscriptions

With the `websocket` feature, `WasiWebSocket::connect(reactor, endpoint, config)` opens a `ws://` or `wss://` socket. It uses the same `Reactor`, nonblocking WASI TCP and DNS, Tungstenite 0.29 framing and validating rustls 0.23. The host must grant the DNS and TCP destinations.

For `wss://`, you must supply DER CA roots in `SocketConfig::root_certificates`. There are no default roots. The other fields are `connect_timeout`, `operation_timeout`, `max_frame_bytes`, `max_message_bytes` and `max_write_buffer_bytes`.

`WasiWebSocket` implements the client's `TextSocket` trait, so you can hand it straight to a typed `subscriptions::Session`. `WasiClock` supplies the Session's deadlines from the WASI monotonic clock. This example is adapted from `verification/wasi-text-sockets`:

```rust
use solana_kit_client::subscriptions::{Budget, Clock, Drain, Root, Session};
use solana_kit_wasi::{Reactor, SocketConfig, WasiClock, WasiWebSocket};

const GENERATION: u64 = 1;

let reactor = Reactor::default();
let config = SocketConfig {
    root_certificates: vec![include_bytes!("ca.der").to_vec()],
    ..SocketConfig::default()
};
let socket = reactor.block_on(WasiWebSocket::connect(
    reactor.clone(),
    "wss://rpc.invalid/",
    config,
))??;

let clock = WasiClock::new(reactor.clone());
let deadline = clock.now() + 2_000; // milliseconds
let mut session = Session::new(socket, GENERATION, Budget::default());

let handle = session.subscribe(Root, deadline)?;
if let Drain::Notification(root) = reactor.block_on(session.next(&handle, &clock, deadline))?? {
    println!("new root: {root}");
}

reactor.block_on(session.unsubscribe(handle, &clock, deadline))??;
reactor.block_on(session.shutdown(&clock, deadline))??;
```

`receive` and `poll_receive` are text-only and reject binary. `receive_frame` and `poll_receive_frame` return `ReceivedFrame::Text` or `ReceivedFrame::Binary`. A cancelled send has an uncertain outcome. Shut that socket down and never replay the message. [Subscriptions](../core/subscriptions.md) covers `Session`.

## Status

| Capability                                             | Status                                                                                                                                                                                                                                                                                       |
| ------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `WasiHttp`                                             | Runs in CI under Wasmtime 50.0.0, both the CLI and a private embedded host: TLS, denial, early responses, deadlines, cancellation and Store replacement                                                                                                                                      |
| `WasiWebSocket`, `WasiClock` and a typed `Session`     | Executed locally under Wasmtime 50.0.0 on macOS ARM64: real WS/WSS peers, TLS negatives, frame bounds, quiet cancellation, CLI unload and embedded Store drop/replacement. [Current receipt](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/wasi-text-sockets-current.md) |
| `WasiRest` (provider REST routes)                      | Source extension; compilation and real REST replay are pending                                                                                                                                                                                                                               |
| Other WASI hosts, installed mobile apps, OS DNS stalls | Unverified                                                                                                                                                                                                                                                                                   |
| Preview1 networking                                    | Not provided                                                                                                                                                                                                                                                                                 |

The receipts are [WASI host contract](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/wasi-contract.md), the [historical WASI socket experiment](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/wasi-sockets.md) and the [current public text socket and clock execution](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/wasi-text-sockets-current.md). The HTTP prototype's README is in [`verification/wasi-http-prototype`](https://github.com/pina-rs/solana_kit/tree/main/verification/wasi-http-prototype).
