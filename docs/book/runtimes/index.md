# Choosing a runtime

The portable crates (`solana_kit_core`, `solana_kit_client` and the program clients) have no networking code. You choose where your code runs by adding one adapter crate. That crate owns the HTTP and WebSocket transport for its host. Every adapter implements the same `solana_kit_client::Transport` trait, so your RPC code is identical everywhere. Only the line that constructs the transport changes.

<Platforms>

<Platform name="Native" crate="solana_kit_native">

Tokio HTTP/1, verified TLS and WebSockets, with deadlines and cancel-on-drop.

</Platform>

<Platform name="Browser" crate="solana_kit_browser">

Window Fetch and the browser WebSocket, with local (non-`Send`) futures.

</Platform>

<Platform name="WASI" crate="solana_kit_wasi">

Preview2 HTTP and sockets, driven by a caller-owned `Reactor`.

</Platform>

<Platform name="LeptosKit" crate="solana_kit_leptoskit">

An interpreted Wasm guest calls named services that a native host owns.

</Platform>

</Platforms>

## Compare the runtimes

| Runtime | Target | Crate | HTTP | WebSocket | Status |
| --- | --- | --- | --- | --- | --- |
| Native | Host targets with Tokio | `solana_kit_native` | `NativeHttp`: HTTP/1 over Hyper, rustls TLS | `NativeWebSocket`: WS and WSS text sockets | HTTP and WebSocket run in CI on Ubuntu, macOS and Windows |
| Browser | `wasm32-unknown-unknown` in a browser Window | `solana_kit_browser` | `BrowserHttp`: Window Fetch | `BrowserWebSocket`: browser WebSocket | HTTP and text sockets run in CI in Chrome; workers, webviews and mobile browsers are unverified |
| WASI | `wasm32-wasip2` | `solana_kit_wasi` | `WasiHttp`: `wasi:http` | `WasiWebSocket` (feature `websocket`): `wasi:sockets` plus guest TLS | HTTP runs under Wasmtime 50; the socket path is newer and still partly pending |
| LeptosKit | Interpreted `wasm32-unknown-unknown` guest plus a native host | `solana_kit_leptoskit` | `GuestHttp` in the guest, `HttpDriver` or `HttpHost` in the host | `GuestWebSocket` and `SocketDriver` | Guest HTTP runs headless in CI; sockets and installed-app journeys are still pending |

Preview1 (`wasm32-wasip1`) and plain Wasm without a browser still compile the portable client, but no networking adapter exists for them.

<Callout kind="important" title="Status is per transport, per host">

A passing test on one host proves nothing for another. Each crate README and each receipt under [`docs/verification`](https://github.com/pina-rs/solana_kit/tree/main/docs/verification) lists exactly which hosts it ran on. The runtime pages here repeat those limits. Check [Parity and status](../reference/parity.md) before you depend on a capability.

</Callout>

## The common shape

All four adapters follow the same rules:

- **You own the runtime.** No adapter starts an executor, spawns a detached task, pools connections or installs a global.
- **You pass configuration in.** Endpoints, headers, trust roots and deadlines are explicit. Nothing is read from the environment.
- **Bounded by default.** Reply bodies are capped by the client's request budget. Each transport adds its own deadlines.
- **Drop cancels.** Dropping an in-flight exchange releases its owned network work. Nothing is retried or redirected automatically.
- **Diagnostics are redacted.** Errors omit endpoints and header values. The original cause stays reachable through `std::error::Error::source`.

Construct a client the same way in every runtime:

<Tabs>

<Tab label="Native">

```rust
use solana_kit_client::{Client, Limits};
use solana_kit_native::{HttpConfig, NativeHttp};

let transport = NativeHttp::new("https://rpc.invalid/", HttpConfig::default())?;
let client = Client::new(transport, Limits::default());
```

</Tab>

<Tab label="Browser">

```rust
use solana_kit_browser::{BrowserHttp, HttpConfig};
use solana_kit_client::{Client, Limits};

let transport = BrowserHttp::new("https://rpc.invalid/", HttpConfig::default())?;
let client = Client::new(transport, Limits::default());
```

</Tab>

<Tab label="WASI">

```rust
use solana_kit_client::{Client, Limits};
use solana_kit_wasi::{Reactor, WasiHttp};
use std::time::Duration;

let reactor = Reactor::default();
let transport = WasiHttp::new(
    reactor.clone(),
    "https://rpc.invalid/",
    Duration::from_secs(30),
    Vec::new(),
)?;
let client = Client::new(transport, Limits::default());
```

</Tab>

<Tab label="LeptosKit">

```rust
use solana_kit_client::{Client, Limits};
use solana_kit_leptoskit::GuestHttp;

// The guest names a service; the native host decides where it goes.
let transport = GuestHttp::new("balance-service")?;
let client = Client::new(transport, Limits::default());
```

</Tab>

</Tabs>

Constructing a transport sends nothing. The first request happens when you poll a call. See [RPC](../core/rpc.md) for the calls you can make once you have a client.

## How to pick

- **A CLI, server, bot or desktop backend:** use [Native](native.md).
- **A web page:** use [Browser](browser.md). Add its `wallet` feature to talk to browser wallets.
- **A WebAssembly component in a WASI host such as Wasmtime:** use [WASI](wasi.md). The embedding host enforces network policy and TLS for HTTP.
- **A LeptosKit desktop or mobile app:** use [LeptosKit](leptoskit.md). The guest uses `guest`; the native shell uses `host`.
- **Wallet connections on any of these:** read [Wallets](wallets.md).

## One test suite, five hosts

The portable core is checked by one conformance suite, `solana_kit_conformance`. `cargo xtask all` runs it natively, in Chrome, in Wasmtime Preview 1 and Preview 2, and in a headless LeptosKit `WasmHost`. Each host can also run alone:

```sh
cargo xtask native
cargo xtask browser
cargo xtask wasi
cargo xtask leptoskit
```

The browser, WASI and LeptosKit journeys need `CHROME`, `CHROMEDRIVER` and `WASMTIME` paths. See [Verification](../contributing/verification.md) for the full setup. This suite covers the portable core. Each transport has its own separate CI workflow.

<Callout kind="note" title="Off-chain only">

Every runtime crate is for off-chain application code. To write on-chain programs, use Pina or the official Solana program libraries.

</Callout>
