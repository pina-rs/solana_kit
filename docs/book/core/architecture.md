# Architecture

Solana Kit is a set of small Rust crates for off-chain Solana applications. A portable core holds the values, codecs and transactions. A client crate adds typed RPC on top. Adapter crates connect the client to a real host: Tokio, a browser window, WASI or a LeptosKit guest. Program clients depend only on the core. This page explains how the layers fit together and what each layer will and will not do for you.

> [!IMPORTANT]
> Solana Kit is for off-chain application code only. Do not use it inside Solana on-chain programs; use Pina or the official Solana program libraries there. `no_std + alloc` support exists so clients can run on more hosts. It does not mean on-chain support.

## Crate layers

Dependencies point one way, from adapters and program clients down to the core.

```text
           program clients (solana_kit_system, solana_kit_token, ...)
                        |
                        v
adapters  --->  solana_kit_client  --->  solana_kit_core  <---  solana_kit_signers
(native, browser,        (typed RPC,          (values, codecs,         (async signer
 wasi, leptoskit)         subscriptions,       accounts, messages,      capabilities)
                          confirmation)        transactions, errors)
```

| Layer                | Crates                                                                                                               | Responsibility                                                                                                                                                                                                                                                              |
| -------------------- | -------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Core                 | `solana_kit_core`                                                                                                    | Addresses, hashes, signatures, instructions, bounded binary codecs, account snapshots, sysvar layouts, legacy/v0/v1 messages and transactions, off-chain messages, program errors, shared error codes, instruction plans. Deterministic crypto behind the `crypto` feature. |
| Signers              | `solana_kit_signers`                                                                                                 | Message, partial, modifying and sending signer traits, and composition of several signers.                                                                                                                                                                                  |
| Client               | `solana_kit_client`                                                                                                  | `Client<T>` with the typed RPC catalog, the subscription protocol owner, confirmation strategies, account fetch helpers and transaction introspection.                                                                                                                      |
| Adapters             | `solana_kit_native`, `solana_kit_browser`, `solana_kit_wasi`, `solana_kit_leptoskit`                                 | Concrete HTTP and WebSocket transports, clocks and entropy for one host each.                                                                                                                                                                                               |
| Program clients      | `solana_kit_system`, `solana_kit_token`, `solana_kit_token_2022`, `solana_kit_compute_budget`, and others            | Instruction builders, account codecs, PDAs and custom errors for one program.                                                                                                                                                                                               |
| Wallets and services | `solana_kit_wallet`, `solana_kit_helius`, `solana_kit_privacy_rings`, `solana_kit_jupiter`, `solana_kit_pyth`, `solana_kit_surfpool`, and others | Wallet protocols and third-party integrations.                                                                                                                                                                                                                              |

The `solana_kit` umbrella crate is published as version 0.0.0. It only reserves the name and exports no APIs, so depend on the component crates directly. See [Installation](../getting-started/installation.md) and the [crate reference](../reference/crates.md).

## Portable core

`solana_kit_core` is `#![no_std]` with `alloc` and `#![forbid(unsafe_code)]`. With default features, its only dependency is `bs58`. It has no executor, network, filesystem, JavaScript or random-number source. Optional features add more:

| Feature         | Adds                                                                                                |
| --------------- | --------------------------------------------------------------------------------------------------- |
| `crypto`        | SHA-256, program-derived addresses, deterministic Ed25519 `Keypair` signing and strict verification |
| `keccak`        | Keccak-256                                                                                          |
| `bigint-codecs` | Arbitrary-width integer codecs through `num-bigint`                                                 |
| `text-codecs`   | Base64 text codec                                                                                   |
| `fixed-points`  | Exact decimal and binary fixed-point arithmetic                                                     |

`solana_kit_signers` and most program client crates are also `no_std + alloc`. `solana_kit_client` uses `std`, but it has no runtime and no network dependency. The client does not promise `no_std` RPC.

## Executor-neutral design

No core or client operation starts a runtime, spawns a task or installs a global. They hand you futures, and your executor polls them.

- `Transport::Exchange<'a>` and the signer traits use associated future types. A native adapter can keep its futures `Send`. A browser or interpreted guest can keep `Rc` state across an `.await`.
- Nothing requires a universal `Send` bound. Native generic code can add bounds such as `for<'a> P::Sign<'a>: Send` before spawning.
- Cargo features add capabilities. They never select a host. You pick a host by depending on its adapter crate.

The browser and LeptosKit guests both compile to `wasm32-unknown-unknown`, so an architecture `cfg` cannot tell them apart. For this reason the host choice is an explicit dependency and never a `cfg`. See [Choosing a runtime](../runtimes/index.md).

## You own transports, deadlines and cancellation

The client owns protocol rules. It handles JSON-RPC framing, request IDs, correlation, response validation and byte and depth budgets. Everything that touches the outside world belongs to you or to the adapter you pick:

- **Endpoint and credentials.** The adapter holds them. The client never reads ambient credentials or environment variables.
- **Deadlines.** Adapters enforce request deadlines. Subscription and confirmation code takes explicit monotonic ticks or an injected `Clock`, never an ambient clock.
- **Cancellation.** Dropping an exchange future cancels its owned work. Dropping a Tokio `JoinHandle` only detaches the task, so abort and await it if you need cancellation.
- **Retries.** The client never retries. A failed or cancelled submission can leave node acceptance uncertain, and retry policy is yours.

## Bounded allocations

Every decoder that reads untrusted input takes an explicit budget:

- Binary codecs take `codecs::Limits { max_bytes, max_items }`. Nested decoders share one element budget. Oversized claims fail with `Error::LimitExceeded` before allocating. See [Codecs](codecs.md).
- Transactions are capped at 1232 bytes for legacy/v0 packets (`PACKET_SIZE`) and 4096 bytes for v1 envelopes (`V1_PACKET_SIZE`).
- The RPC client takes `Limits { bytes, depth }`. The default is 8192 bytes and 32 levels of JSON nesting. Adapters must enforce the body limit while receiving.
- Subscription connections take a `Budget` covering registrations, queued frames, retained bytes and per-frame limits. An overflow is reported as an error and never drops data silently.

## Where to read more

<Cards>
<Card title="Codecs" href="codecs.md">Bounded binary layouts and their error contract.</Card>
<Card title="Messages and transactions" href="transactions.md">Compile, sign and decode legacy, v0 and v1 wire.</Card>
<Card title="RPC" href="rpc.md">The typed client, transports and error categories.</Card>
<Card title="Parity and status" href="../reference/parity.md">What is complete, partial or pending.</Card>
</Cards>

The full design rationale is in the repository's [architecture document](https://github.com/pina-rs/solana_kit/tree/main/docs/architecture/design.md). Some signatures in that document are earlier proposals, not current APIs. Where it differs from the crate sources, the sources win.
