# LeptosKit

`solana_kit_leptoskit` connects the SDK to [LeptosKit](https://github.com/ifiokjr/leptoskit) apps. A LeptosKit app has two halves: an interpreted WebAssembly guest that holds your UI and logic, and a native host that runs the interpreter on desktop or mobile. The guest has no sockets of its own. It asks the host for named services, and the host performs the real network work with [`solana_kit_native`](native.md). The guest never sees an endpoint, header or key.

## How the pieces fit

```text
guest (interpreted Wasm)                  native host
┌────────────────────────────┐           ┌──────────────────────────────────┐
│ Client<GuestHttp>          │  plugin   │ HttpDriver / HttpHost            │
│   GuestHttp::new("rpc")  ──┼── call ──▶│   "rpc" → NativeHttp(endpoint,   │
│                            │           │            headers, TLS, timeout)│
│ your executor + leptoskit_ │◀─ reply ──┼   next_completion → deliver      │
│ poll export                │           │                                  │
└────────────────────────────┘           └──────────────────────────────────┘
```

The host decides which named services exist and where each one connects. A guest that names an uninstalled or ungranted service gets a typed failure, never fabricated data.

## Features

| Feature             | Side           | Adds                                                                                                        |
| ------------------- | -------------- | ----------------------------------------------------------------------------------------------------------- |
| `guest`             | Guest          | `GuestHttp`, `GuestRest`, `GuestWebSocket` and `GuestSubscriptionSocket`. Enables `leptoskit_bridge/guest`. |
| `host`              | Native host    | `HttpDriver`, `HttpHost`, `HostLimits`, `Routed`, `SocketDriver`. Pulls in `solana_kit_native` and Tokio.   |
| `wallet-ui`         | Native host    | `wallet_ui::wallet_panel` and `wallet_ui::panel_channel`, an accessible wallet picker and status view.      |
| `wallet-browser-ui` | Browser bundle | `wallet_ui::BrowserWalletView` over real browser wallet discovery.                                          |
| `wallet-platform`   | Android guest  | `GuestWalletPlatform`: wallet availability, association Intent launch and host entropy.                     |
| `wallet-session`    | Guest          | `GuestMobileWallet` and `GuestLocalWallet`.                                                                 |
| `wallet-host`       | Native host    | `wallet_service`, the native wallet service those guest types talk to.                                      |

Enable `guest` only in the interpreted guest, never in a browser bundle. A browser page uses [Browser](browser.md) directly.

## Add the crate

The crate depends on LeptosKit through Git, at revision `7437097c52aa68a3401bd378764e051ed4c9165a`. Your app must use the same LeptosKit revision, so that both sides agree on one bridge ABI.

```toml
# guest/Cargo.toml
[dependencies]
solana_kit_client = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_leptoskit = { git = "https://github.com/pina-rs/solana_kit", features = ["guest"] }
leptoskit_bridge = { git = "https://github.com/ifiokjr/leptoskit", rev = "7437097c52aa68a3401bd378764e051ed4c9165a", features = ["guest"] }
```

```toml
# host/Cargo.toml
[dependencies]
solana_kit_native = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_leptoskit = { git = "https://github.com/pina-rs/solana_kit", features = ["host"] }
leptoskit_wasm = { git = "https://github.com/ifiokjr/leptoskit", rev = "7437097c52aa68a3401bd378764e051ed4c9165a" }
```

Build the guest for `wasm32-unknown-unknown`. Publishing to crates.io waits on a compatible upstream LeptosKit release.

## Guest: make an RPC call

In the guest, `GuestHttp` is an ordinary `Transport`. This example is adapted from `verification/leptoskit-http/guest`:

```rust
use solana_kit_client::{Client, Error, Limits, methods::Balance};
use solana_kit_core::Address;
use solana_kit_leptoskit::GuestHttp;

async fn balance(service: &str) -> Result<u64, Error> {
    let transport = GuestHttp::new(service)?;
    let client = Client::new(transport, Limits::default());
    let reply = client.call(Balance(Address::from_bytes([0; 32]), None)).await?;
    Ok(reply.value)
}
```

The guest application owns three things the adapter does not touch:

- **A local executor.** The fixture uses `futures::executor::LocalPool`.
- **The `leptoskit_poll` export.** The host calls it after waking plugin calls. It should run your executor until it stalls.
- **The single bridge `Events` receiver.** The adapter never takes it. Your app routes its own events.

Futures are local, so an `Rc` can live across `.await`. Dropping a pending call cancels it on the host.

## Host: install services and run the loop

The host binds a `NativeHttp` to each service name. The name selects a fixed endpoint, TLS roots, authentication and deadlines. `HostLimits` caps concurrent jobs, request bytes and reply bytes, whatever the guest asks for.

```rust
use leptoskit_wasm::WasmHost;
use solana_kit_leptoskit::{HostLimits, HttpHost};
use solana_kit_native::{HttpConfig, NativeHttp};

let host = WasmHost::headless(&guest_wasm)?;
let mut app = HttpHost::new(
    host,
    HostLimits { jobs: 4, request_bytes: 64 * 1024, reply_bytes: 1024 * 1024 },
)?;

// `true` grants the service; `false` installs it as denied.
app.install(
    "rpc",
    NativeHttp::new("https://rpc.invalid/", HttpConfig::default())?,
    true,
)?;
app.start(320.0, 480.0)?;

loop {
    // After every guest entry point, drain new calls and cancellations.
    let routed = app.drain()?;
    // routed.requests and routed.cancelled belong to your other services.
    if routed.guest_woken {
        continue; // drain again before waiting on native work
    }
    // Join one native completion and deliver it into the guest.
    if !app.next_completion().await? {
        break; // no outstanding jobs
    }
}

app.shutdown().await?;
```

`HttpHost` owns the interpreter. If your app already owns its `WasmHost`, use `HttpDriver` instead. It takes short `&mut WasmHost` borrows for `install`, `drain` and `deliver`. Its `next_completion().await` borrows only the driver and returns an owned `HttpCompletion`, so your UI can keep using the interpreter while network work runs. Put that wait in the same `select!` loop as your other event sources.

`shutdown().await` stops admission, aborts native work and joins it. Dropping the driver aborts work, but only an awaited `shutdown` proves that cleanup has finished. `HttpDriver::shutdown` leaves your interpreter alive. `HttpHost::shutdown` also unloads it.

<Callout kind="tip" title="Composite apps">

When HTTP, sockets and wallets share one app, a single owner drains the bridge and passes each batch through `HttpDriver::route`, `SocketDriver::route` and `wallet_service::WalletService::route` in turn. Whatever is left goes back to the framework. The crate README describes the bounded 32-turn loop.

</Callout>

## Sockets and REST

The `guest` feature also provides `GuestWebSocket` and `GuestSubscriptionSocket`, which plug into the client's typed `Session`. The host side is `SocketDriver`. `GuestRest` and `HttpDriver::install_rest` carry provider REST calls to a host-owned `NativeRest` root. Both use the same rules: named services, application-owned events and awaited shutdown.

## Wallets in a LeptosKit app

The wallet features add a native wallet picker (`wallet-ui`), an Android Mobile Wallet Adapter path (`wallet-platform`, `wallet-session`, `wallet-host`) and an in-app local key wallet with explicit approval. The runtime manifest selects `solana-wallet`, which installs `wallet_service::from_app_dirs` and Android's `dev.solanakit.wallet.MwaPlatform`. [Wallets](wallets.md) explains each piece.

## Status

| Capability                                                  | Status                                                                                                                                                                                                                    |
| ----------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `GuestHttp` with `HttpHost` and `HttpDriver`                | A real Rust guest runs in a headless `WasmHost` in CI: HTTPS, denial, status, body limit, deadline and cancellation                                                                                                       |
| `GuestWebSocket`, `SocketDriver`, `GuestSubscriptionSocket` | Source extension; the crate README marks it as not yet executed                                                                                                                                                           |
| `GuestRest`, `install_rest`                                 | Source extension; real provider journeys are pending                                                                                                                                                                      |
| Generated desktop, Android and iOS apps                     | The `generated-wallet` CI workflow builds a LeptosKit-generated wallet app and drives it on macOS, an iOS simulator and an Android emulator. The crate README still lists the full set of installed journeys as required. |

The receipts are [LeptosKit HTTP](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/leptoskit-http.md) and [LeptosKit async](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/leptoskit-async.md). The generated app source is in [`verification/leptoskit-app-journey`](https://github.com/pina-rs/solana_kit/tree/main/verification/leptoskit-app-journey).
