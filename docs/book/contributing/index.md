# Contributing

This page covers how the repository is laid out, which toolchain it pins, and the commands to build, test and lint a change before you open a pull request. The project has a high verification bar: malformed encodings, signing mistakes or integer loss can produce transactions that look valid and are not. Expect to add fixtures and negative controls, not just happy-path tests.

## Repository layout

| Path                                                   | Contents                                                                                                                 |
| ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------ |
| `crates/`                                              | The SDK crates. See [Crates](../reference/crates.md).                                                                    |
| `verification/`                                        | Private harnesses: conformance suites, runtime hosts, boundary and consumer checks. See [Verification](verification.md). |
| `xtask/`                                               | `cargo xtask`, the reproducible verification commands                                                                    |
| `tools/`                                               | `capability_inventory`, `codama-renderer`, `check_plan.rs` and receipt helpers                                           |
| `packages/`                                            | `codama-renderers-rust-solana-kit`, the NPM Codama renderer                                                              |
| `docs/parity/`, `docs/planning/`, `docs/verification/` | Ledgers, the plan and decisions, and execution receipts. See [Parity and status](../reference/parity.md).                |
| `docs/architecture/`                                   | The design and API proposal                                                                                              |
| `docs/book/`, `docs/site/`                             | This book and the site that builds it. See [These docs](docs.md).                                                        |
| `.changeset/`                                          | Pending release notes, consumed by monochange                                                                            |
| `.github/workflows/`                                   | CI. See [Verification](verification.md).                                                                                 |

Some verification crates are excluded from the main workspace (listed under `exclude` in the root `Cargo.toml`) and have their own lockfiles. Run those with `--manifest-path`.

## Toolchain

`rust-toolchain.toml` pins Rust **1.99.1** with `clippy`, `rustfmt` and three Wasm targets: `wasm32-unknown-unknown`, `wasm32-wasip1` and `wasm32-wasip2`. The workspace uses edition 2024 and declares an MSRV of **1.89** (`rust-version`). CI checks the MSRV for the core and selected crates with `cargo +1.89.0`.

There is no devenv or Nix setup. Runtime journeys need a few external executables, which you point at with environment variables:

| Tool                    | Version                      | Used by                                                                                                     |
| ----------------------- | ---------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `wasm-bindgen-cli`      | 0.2.128                      | Browser tests (`.cargo/config.toml` sets `wasm-bindgen-test-runner` as the `wasm32-unknown-unknown` runner) |
| Chrome and ChromeDriver | 155.0.8059.84 in CI          | `CHROME`, `CHROMEDRIVER`                                                                                    |
| Wasmtime                | 50.0.0                       | `WASMTIME`                                                                                                  |
| Monostyle               | 0.3.27 (pinned git revision) | `cargo xtask style`                                                                                         |

## Build and test

From the repository root:

```sh
cargo fmt --all --check
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
```

Test a single crate with `-p`, for example `cargo test -p solana_kit_jupiter --locked`. Crates with optional features should also be checked with `--no-default-features` and each feature combination they document. CI does this for `solana_kit_core` (`crypto`, `fixed-points`) and `solana_kit_pyth`.

The shared conformance suite runs on its own. The corrupt-fixture run is a negative control: it must fail.

```sh
cargo run -p solana_kit_conformance --locked
cargo run -p solana_kit_conformance --locked -- --corrupt-fixture
```

## Verification commands

`.cargo/config.toml` defines the `xtask` alias. Each command validates its arguments before it does anything, and it does not forward Cargo flags.

| Command                         | Runs                                                                            |
| ------------------------------- | ------------------------------------------------------------------------------- |
| `cargo xtask native`            | The conformance suite natively                                                  |
| `cargo xtask browser`           | The same suite in Chrome                                                        |
| `cargo xtask wasi`              | The same suite under Wasmtime Preview 1 and Preview 2                           |
| `cargo xtask leptoskit`         | The same suite in the LeptosKit headless host                                   |
| `cargo xtask gates`             | Guest imports, dependency closures and forbidden fixtures                       |
| `cargo xtask dependency-graphs` | Dependency graph checks                                                         |
| `cargo xtask consumer`          | An external packaged-core consumer                                              |
| `cargo xtask all`               | `native`, `browser`, `wasi`, `leptoskit`, `gates` and `consumer`, in that order |
| `cargo xtask style [root]`      | The per-file Monostyle gate                                                     |

Other commands take a path: `style-report`, `imports`, `dependency-fixture`, `package-review`, `package-report`, `package-rehearsal`, `registry-fixture`, `artifact-capture`, `prepare-wallet` and `prepare-wallet-rpc`. The `package-rehearsal` path is its absolute-path JSON configuration; see the [SDK package rehearsal receipt](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/sdk-package-rehearsal-current.md) for the archive, registry and clean-consumer evidence. To keep nested Cargo calls offline, set the environment rather than a flag: `CARGO_NET_OFFLINE=true cargo xtask consumer`.

## Style and formatting

- **Rust:** `rustfmt.toml` sets edition 2024. Clippy runs with `-D warnings`.
- **Monostyle:** `monostyle.toml` requires readability and complexity scores of at least 90, a maximum line width of 100, and does not exempt generated code. Install the pinned revision and run the gate:

  ```sh
  cargo install monostyle --locked \
    --git https://github.com/ifiokjr/monostyle \
    --rev ae86916d5055fb2c15f665bbe4f0f90bd8abc26e
  cargo xtask style
  ```

  The xtask also checks every individual file, not just the aggregate score. See [`docs/verification/style.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/verification/style.md).
- **Markdown and TOML:** `dprint.json` configures the dprint Markdown and TOML plugins, with `textWrap: maintain`.

## Changesets and releases

Releases are managed by [monochange](https://github.com/monochange/monochange), configured in `monochange.toml`. Changesets are required: when a change touches crates, docs, tools or verification, add a Markdown file to `.changeset/` that names the affected package and bump.

```markdown
---
solana_kit_client: minor
---

# Add caller-driven reactive action and stream stores

Opt into cached action and stream snapshots, scoped cancellation, ...
```

Describe completed behavior only. Prospective work belongs in `docs/planning/tasks.md`, not in a changeset. CI runs `monochange check`, `monochange step validate`, `monochange discover` and `monochange preview`.

`monochange.toml` disables Cargo and npm publishing at the ecosystem level, and every configured package has publishing disabled. The `solana_kit` umbrella is configured as a placeholder publication with its own README. The publisher CLI is released separately through its own tag workflow (see [dApp Store publisher](../services/dapp-publisher.md)).

## Pull requests

The repository has no PR template. The history and the plan show these conventions:

- Keep each PR small, ending in an independently observable result, and merge only after its checks and review pass.
- Use Conventional Commit titles with a scope, for example `feat(publisher): …`, `fix(codecs): …` or `test(wallet): …`.
- Record deviations and evidence in the relevant parity ledger and in `docs/planning/decisions.tsv`. Do not claim status the evidence does not support. See [Parity and status](../reference/parity.md).
- New project code is released under the Unlicense. Reused third-party material keeps its license and notices (`THIRD_PARTY_NOTICES.md` and the `MIT-*.txt` files in each crate).
- Solana Kit is off-chain only. Do not add on-chain program support.
