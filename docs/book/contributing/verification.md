# Verification

In this project, a test that compiles is not evidence. A capability counts only once it has run on the hosts it claims, against fixtures written independently of the code under test, with a negative control showing the check can fail. This page covers the `verification/` harnesses, the receipts in `docs/verification/`, and what each CI workflow runs.

## The shared conformance suite

`verification/conformance` (`solana_kit_conformance`) is the center of it all. It is a host-neutral registry of fixed-vector contract groups for the core, the client, the signers, the program clients and the services. The same registry runs on five hosts:

| Host | Command |
| --- | --- |
| Native Rust | `cargo xtask native` |
| Chrome (`wasm32-unknown-unknown`) | `cargo xtask browser` |
| Wasmtime Preview 1 and Preview 2 | `cargo xtask wasi` |
| LeptosKit headless `WasmHost` (pinned wasmi) | `cargo xtask leptoskit` |

`cargo xtask all` runs every host, then the dependency and import gates and the packaged-core consumer. Each run saves stdout, stderr and the exit status under the ignored `target/runtime-evidence/` directory.

Every suite includes a corruption control. `cargo run -p solana_kit_conformance --locked -- --corrupt-fixture` must report exactly one failing case (the SHA-256 comparison) and exit 1. CI compares that output against committed evidence, so a compile failure cannot pass as the negative result. The suite identity and case count, for example `v55/448`, increase as groups are appended. Earlier cases must remain an exact, unchanged prefix.

## The `verification/` crates

Most of these are private, unpublished harnesses. Several are excluded from the main workspace and keep their own lockfiles.

| Area | Crates |
| --- | --- |
| Shared suite and hosts | `conformance`, `browser`, `guest`, `leptoskit` |
| Transports | `browser-http`, `browser-websocket`, `native-websocket`, `leptoskit-http`, `leptoskit-async`, `wasi-contract`, `wasi-http-prototype`, `wasi-sockets`, `wasi-text-sockets` |
| Core contracts | `core-contracts-portable`, `codec-framing`, `codec-size-consumer`, `collection-layouts`, `collection-layouts-consumer`, `collection-layouts-independent`, `account-contracts-independent`, `account-snapshot-contracts`, `shared-errors`, `shared-errors-independent`, `v1-wire`, `offchain` |
| Transactions and RPC | `rpc-design`, `subscriptions-design`, `transaction-confirmation`, `confirmation-factories`, `instruction-plans`, `durable-nonce`, `reader-transactions` |
| Signers and wallets | `signer-capabilities`, `wallet-consumer`, `wallet-runtime`, `leptoskit-app-journey` |
| Program clients | `stake-loader-portable`, `metaplex-portable`, `remaining-programs-portable`, `token-2022-contracts`, `token-2022-public`, `token-layouts-independent`, `anchor-client`, `codama-consumer` |
| Services | `helius-portable`, `surfpool-native` |
| Boundaries | `boundaries` (async and System host boundaries) |

The `*-independent` crates hold fixtures written without the SDK's own codecs, so an encoder cannot confirm its own output. The `*-consumer` crates use the SDK from outside the workspace, the way a downstream project would.

`surfpool-native` is special. It needs a verified Surfpool 1.7.0 executable and a sandbox profile, and no CI workflow runs it. See [Surfpool](../services/surfpool.md).

## Receipts

[`docs/verification/`](https://github.com/pina-rs/solana_kit/tree/main/docs/verification) holds one receipt per verification unit. Each one records what ran, on which hosts and toolchains, the exact commands, the case counts, the negative controls, and anything still unexecuted. A few to start with:

- [`runtimes.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/runtimes.md): the five-host conformance runs and how to reproduce them, including pinned Chrome, ChromeDriver and Wasmtime versions.
- [`native-http.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/native-http.md), [`browser-websocket.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/browser-websocket.md), [`leptoskit-http.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/leptoskit-http.md): real transport journeys.
- [`style.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/style.md): the per-file Monostyle gate.
- [`jupiter-swap.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/jupiter-swap.md), [`remaining-program-clients.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/remaining-program-clients.md): service and program contract mappings.

A receipt records results. It does not stand in for re-running a check. If a receipt says a host "remains required", that host has not been qualified. See [Parity and status](../reference/parity.md) for how receipts relate to the ledgers.

## CI workflows

Every workflow except the publisher release runs on pull requests and on pushes to `main`. `wasi-contract`, `wasi-sockets` and `docs` are filtered by path.

| Workflow file | Name | What it covers |
| --- | --- | --- |
| `plan.yml` | SDK verification | Native core on Ubuntu, macOS and Windows; MSRV 1.89 checks; conformance with committed success and corruption evidence; the package ledger; mdt documentation reuse; the generated upstream compatibility tables; the Monostyle gate; monochange validation and release preview |
| `runtimes.yml` | Runtime conformance | `cargo xtask all`: native, Chrome, Wasmtime Preview 1 and 2, LeptosKit, gates and consumer |
| `parity.yml` | Capability inventory | `tools/capability_inventory` check and controls against the pinned Dart source, plus `cargo xtask style` |
| `rpc-design.yml` | Typed RPC design | `solana_kit_client` tests on three OSes, Wasm and WASI target checks, the RPC catalog against the pinned reference |
| `native-http.yml` | Native HTTP transport | Real HTTP and TLS journeys on three OSes, the MSRV, and Pyth Hermes over native REST |
| `native-websocket.yml` | Native WebSocket transport | Real WebSocket and TLS journeys on three OSes |
| `browser-http.yml` | Browser HTTP | Window Fetch journeys in Chrome |
| `browser-websocket.yml` | Browser WebSocket | Window WebSocket journeys in Chrome |
| `leptoskit-http.yml` | LeptosKit HTTP guest | Interpreted guest HTTP through the real host |
| `leptoskit-async.yml` | LeptosKit async guest | Guest wake-up and cancellation in the real host |
| `wasi-contract.yml` | WASI Preview 2 contract | Preview 2 component contract under Wasmtime |
| `wasi-http.yml` | WASI HTTP adapter | `solana_kit_wasi` HTTP under Wasmtime |
| `wasi-sockets.yml` | WASI validating TLS WebSocket sockets | Preview 2 sockets with TLS validation |
| `client-sessions.yml` | Public WASI sessions | WASI text sockets and confirmation factories under Wasmtime |
| `boundaries.yml` | Async and System boundaries | Native boundary contracts on three OSes; System hosts in Chrome, Wasmtime and LeptosKit |
| `offchain.yml` | Off-chain application messages | Isolated portable offchain-message cases |
| `v1-wire.yml` | Private v1 wire proof | The independent v1 transaction codec oracle |
| `codegen-design.yml` | Generator design verification | Codama renderer output for System and Compute Budget, Token-2022 proof sources |
| `codama-npm.yml` | Codama NPM renderer | The NPM renderer package and its generated consumer |
| `program-clients.yml` | Fundamental program clients | Lookup table, Config, Loader and Stake: native and portable hosts |
| `metaplex-clients.yml` | Compression and Metaplex clients | Account Compression, Bubblegum, Core and Token Metadata: native and portable hosts |
| `remaining-programs.yml` | Program clients and Helius integration | Attestation Service, Squads and Subscriptions, plus Helius portability across feature modes |
| `generated-wallet.yml` | Generated wallet consumer | The generated LeptosKit wallet app as a guest, and installed journeys on macOS, iOS and Android |
| `docs.yml` | docs | Checks and builds this site from `docs/book` and `docs/site`, and deploys it to GitHub Pages from `main` (see [These docs](docs.md)) |
| `publisher-cli.yml` | publisher CLI | Tag-triggered release build for eight targets (see [dApp Store publisher](../services/dapp-publisher.md)) |

CI pins Rust 1.99.1 (and 1.89.0 for MSRV jobs), Chrome and ChromeDriver 155.0.8059.84, and Wasmtime 50.0.0. It verifies the SHA-256 of every downloaded host before extracting it. The shared host setup lives in `.github/actions/portable-program-hosts`.

## Adding verification for a change

<Steps>

### Write independent fixtures

Write fixture bytes and JSON by hand, or capture them from the pinned reference. Never produce them with the encoder under test.

### Add negative controls

Show that a corrupted fixture, a wrong signature or an exceeded budget fails in the way you expect.

### Register portable groups

Append contract groups to the shared conformance registry so that every host runs them. Keep earlier cases as an unchanged prefix.

### Record the receipt

Update the relevant `docs/verification/` receipt and parity ledger with what actually ran. List anything not yet executed.

</Steps>
