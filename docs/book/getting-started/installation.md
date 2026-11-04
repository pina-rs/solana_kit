# Installation

Solana Kit is a set of `solana_kit_*` crates that you depend on straight from the GitHub repository. This page covers which crates to add, the toolchain they need, the optional features, and the targets they build for.

<Callout kind="important" title="Off-chain code only">

Solana Kit is for off-chain application code: services, CLIs, desktop and mobile apps, browser and WASI clients. Do not use it inside Solana on-chain programs. Use [Pina](https://github.com/pina-rs/pina) or the official Solana program libraries for those. Several crates build as `no_std + alloc`; that makes them portable to WebAssembly hosts, and it does not mean they are meant for, or tested on, the on-chain runtime.

</Callout>

## Release status

The SDK crates are not published to crates.io as usable releases yet. The `solana_kit` 0.0.0 crate on crates.io is a name reservation with no SDK APIs, and `solana_kit_core` 0.1.0 is unreleased. Until releases exist, depend on the crates with Cargo git dependencies, or path dependencies on a local checkout.

The project ships in independently verified phases, and many capabilities are still partial or pending. Check [Parity and status](../reference/parity.md) before you rely on a specific feature.

<Steps>

### Check your toolchain

The workspace declares `rust-version = "1.89"` and edition 2024, so you need Rust 1.89 or newer.

The repository itself pins a newer toolchain for development in `rust-toolchain.toml`:

```toml
[toolchain]
channel = "1.99.1"
profile = "minimal"
components = ["clippy", "rustfmt"]
targets = ["wasm32-unknown-unknown", "wasm32-wasip1", "wasm32-wasip2"]
```

You only need that file if you work inside the repository. In your own project, install the WebAssembly targets you plan to build for:

```sh
rustup target add wasm32-unknown-unknown wasm32-wasip1 wasm32-wasip2
```

### Add the crates

Add the crates you need as git dependencies. Cargo finds each crate by name inside the repository workspace. A native application that reads RPC data and builds System transfers looks like this:

```toml
[dependencies]
solana_kit_core = { git = "https://github.com/pina-rs/solana_kit", features = ["crypto"] }
solana_kit_client = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_native = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_system = { git = "https://github.com/pina-rs/solana_kit" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

All crates from one repository revision must match. Pin a commit with `rev = "…"` (the same value on every line) so a later push to `main` cannot change your build.

If you have a local checkout, use path dependencies instead:

```toml
[dependencies]
solana_kit_core = { path = "../solana_kit/crates/solana_kit_core", features = ["crypto"] }
solana_kit_client = { path = "../solana_kit/crates/solana_kit_client" }
```

### Build

```sh
cargo build
```

When the build succeeds, go to the [Quick start](quick-start.md) and make your first RPC call.

</Steps>

## Which crates to add

Start with these. Program and service clients live in their own crates; see [Crates](../reference/crates.md) for the full list.

| Crate                | What it gives you                                                                                                       | `no_std + alloc` |
| -------------------- | ----------------------------------------------------------------------------------------------------------------------- | ---------------- |
| `solana_kit_core`    | Addresses, hashes, signatures, instructions, codecs, account snapshots, sysvars, legacy/v0/v1 messages and transactions | Yes              |
| `solana_kit_client`  | Typed JSON-RPC `Client` and the `Transport` trait, with no runtime or network of its own                                | No, needs `std`  |
| `solana_kit_native`  | `NativeHttp` and `NativeWebSocket` transports on Tokio, with verified TLS                                               | No               |
| `solana_kit_system`  | System Program instruction builders, parsers, nonce account codec and errors                                            | Yes              |
| `solana_kit_signers` | Asynchronous signer capability traits and composition                                                                   | Yes              |

`solana_kit_client` never starts a runtime and never opens a socket itself. A transport crate does the I/O: `solana_kit_native` on native hosts, `solana_kit_browser` in the browser, `solana_kit_wasi` on WASI Preview 2, and `solana_kit_leptoskit` inside LeptosKit apps.

## Feature flags

Every feature is off by default. There is no `std` feature: crates that are portable are always `no_std + alloc`, and `solana_kit_client` always uses `std`.

| Crate                | Feature                  | Adds                                                                                                             |
| -------------------- | ------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `solana_kit_core`    | `crypto`                 | SHA-256, program-derived addresses, Ed25519 `Keypair` signing and verification, `Transaction::sign` and `verify` |
| `solana_kit_core`    | `keccak`                 | Keccak-256                                                                                                       |
| `solana_kit_core`    | `fixed-points`           | Exact decimal and binary fixed-point numbers                                                                     |
| `solana_kit_core`    | `bigint-codecs`          | Arbitrary-width integer codecs                                                                                   |
| `solana_kit_core`    | `text-codecs`            | Base64 text codecs                                                                                               |
| `solana_kit_client`  | `confirmation-factories` | Ready-made signature confirmation over a WebSocket session plus HTTP                                             |
| `solana_kit_client`  | `reactive`               | Caller-driven reactive stores and streams                                                                        |
| `solana_kit_client`  | `resource-estimation`    | Compute and resource limit estimation by simulation                                                              |
| `solana_kit_native`  | `keys`                   | `keys::NativeEntropy` for key generation and `keys::write_keypair_new` for keypair file backups                  |
| `solana_kit_native`  | `surfpool`               | Surfpool program deployment helpers                                                                              |
| `solana_kit_native`  | `mwa`                    | Mobile Wallet Adapter transport                                                                                  |
| `solana_kit_signers` | `crypto`                 | Signer traits implemented for core's `Keypair`                                                                   |
| `solana_kit_browser` | `keys`, `wallet`         | Browser entropy and browser wallet support                                                                       |
| `solana_kit_wasi`    | `websocket`              | WASI WebSocket transport                                                                                         |

Most applications that sign transactions enable `solana_kit_core/crypto`. Without it you can still build and inspect messages, but you cannot sign locally or derive program addresses.

## Targets

| Target                             | Portable crates | Transport                                         |
| ---------------------------------- | --------------- | ------------------------------------------------- |
| Native                             | Yes             | `solana_kit_native` on Tokio                      |
| `wasm32-unknown-unknown` (browser) | Yes             | `solana_kit_browser` (Window Fetch and WebSocket) |
| `wasm32-wasip1`                    | Yes             | None; WASI HTTP needs Preview 2                   |
| `wasm32-wasip2`                    | Yes             | `solana_kit_wasi` (`WasiHttp`)                    |
| LeptosKit interpreted guest        | Yes             | `solana_kit_leptoskit`                            |

"Portable crates" means `solana_kit_core`, `solana_kit_client`, `solana_kit_system` and `solana_kit_signers`. Each one compiles for every target above with no JavaScript or host imports. [Choosing a runtime](../runtimes/index.md) explains each transport and what has been verified on each host.

## Next steps

<Cards>
<Card title="Quick start" href="quick-start.md">Make a typed RPC call with the native transport.</Card>
<Card title="Choosing a runtime" href="../runtimes/index.md">Pick the transport for your host.</Card>
<Card title="Crates" href="../reference/crates.md">See every crate in the workspace.</Card>
</Cards>
