# dApp Store publisher

Publishing to the Solana dApp Store is split across two crates. `solana_kit_dapp_publisher` is a portable library: publication attestations, a policy check for portal-prepared transactions, the portal protocol, and a resumable publication workflow. `solana_kit_dapp_publisher_cli` builds the native `solana-kit-publisher` command on top of it. The command uploads an APK, signs the release and collection-verification transactions after validating them, creates the attestation, and submits the release for store review.

## Install the CLI

<Tabs>

<Tab label="cargo install">

```sh
cargo install --git https://github.com/pina-rs/solana_kit solana_kit_dapp_publisher_cli
```

From a checkout:

```sh
cargo install --path crates/solana_kit_dapp_publisher_cli
```

</Tab>

<Tab label="Prebuilt archive">

Download an archive from the [`publisher-cli-v0.1.0` release](https://github.com/pina-rs/solana_kit/releases/tag/publisher-cli-v0.1.0). Each archive comes with `.sha256` and `.sha512` checksums.

| Platform | Targets |
| --- | --- |
| macOS | `aarch64-apple-darwin`, `x86_64-apple-darwin` |
| Linux (glibc) | `aarch64-unknown-linux-gnu`, `x86_64-unknown-linux-gnu` |
| Linux (musl) | `aarch64-unknown-linux-musl`, `x86_64-unknown-linux-musl` |
| Windows | `aarch64-pc-windows-msvc`, `x86_64-pc-windows-msvc` (`.zip` and `.tar.gz`) |

</Tab>

<Tab label="cargo-binstall">

The crate's `Cargo.toml` has a `[package.metadata.binstall]` entry, so `cargo-binstall` can resolve the release archives (`publisher-cli-v{version}` tags) instead of compiling from source.

</Tab>

</Tabs>

The crate is not on crates.io, because the SDK crates it depends on are unpublished. Until that changes, `cargo install --git` and the prebuilt archives are the supported ways to install it. Check the install with `solana-kit-publisher --version`, which prints `solana-kit-publisher 0.1.0`.

## Publish a release

<Steps>

### Provide the portal API key

```sh
export DAPP_STORE_API_KEY=...
```

To read the key from a different variable, pass `--api-key-env NAME`. To pipe it in instead, pass `--api-key-stdin`. If you do that, stdin must not be a terminal. The key is never printed or written to the checkpoint.

### Run the publisher

```sh
solana-kit-publisher \
  --apk-file ./app-release.apk \
  --whats-new "First release" \
  --keypair ~/.config/solana/id.json
```

Use `--apk-url https://…` instead of `--apk-file` if the APK is already hosted. The URL must be HTTPS, and you must give exactly one of the two flags. `--keypair` is a Solana CLI keypair JSON file.

### Read the summary

On success the command prints `This app is now in review.`, followed by the release mint, the collection mint and the review ticket, when the portal returns them.

</Steps>

If a run is interrupted, resume it by release or by session:

```sh
solana-kit-publisher resume --release-id <RELEASE_ID> --keypair ~/.config/solana/id.json
solana-kit-publisher resume --session-id <SESSION_ID> --keypair ~/.config/solana/id.json
```

Progress is saved to `.solana-kit-publisher/checkpoint.json` in the working directory. If a checkpoint already exists there, a new publication refuses to start (`existing checkpoint requires resume`).

## Flags

Flags accept either `--name value` or `--name=value`. Boolean flags take no value. Unknown flags and positional arguments are errors.

| Flag | Applies to | Meaning |
| --- | --- | --- |
| `--apk-file <PATH>` | new | Local APK, uploaded through one presigned PUT |
| `--apk-url <HTTPS_URL>` | new | Already hosted APK |
| `--whats-new <TEXT>` | new | Release notes (required) |
| `--keypair <PATH>` | both | Solana CLI keypair JSON (required) |
| `--idempotency-key <KEY>` | new | Explicit ingestion idempotency key. A random one is generated otherwise. |
| `--release-id <ID>` / `--resume-release <ID>` | resume | Resume by release. The two are aliases. |
| `--session-id <ID>` / `--resume-session <ID>` | resume | Resume by publication session. The two are aliases. |
| `--portal-url <URL>` | both | Portal base URL |
| `--api-key-env <NAME>` | both | Environment variable that holds the API key |
| `--api-key-stdin` | both | Read the API key from piped stdin |
| `--rpc-url <URL>` | new | RPC endpoint for the funding preflight |
| `--local-dev` | both | Allow, and require, localhost portal endpoints |
| `--skip-self-update` | both | Accepted only together with `--local-dev`; otherwise the command fails |
| `--verbose` | both | Print the resolved portal and API URLs |
| `--version`, `--help` / `-h` | — | Print the version or the usage text |

When resuming, give exactly one of `--release-id` or `--session-id`. If you pass a flag and its alias with different values, the command fails.

## Endpoints and secrets

| Setting | Source, in priority order |
| --- | --- |
| Portal URL | `--portal-url`, then `DAPP_STORE_PORTAL_URL`, then `http://localhost:3333` with `--local-dev`, otherwise `https://publish.solanamobile.com` |
| Portal API | The portal URL with `/api` appended, for example `https://publish.solanamobile.com/api` |
| API key | `--api-key-stdin`, otherwise the variable named by `--api-key-env`, otherwise `DAPP_STORE_API_KEY` |
| Funding RPC | `--rpc-url`. Without it the default is `https://api.mainnet-beta.solana.com`, except under `--local-dev`, which skips the check. |

The portal must use HTTPS unless `--local-dev` is set. With `--local-dev`, both the portal and the API must be on localhost (`localhost`, `127.0.0.1` or `[::1]`).

Before a new publication starts, the funding preflight reads the signer's balance. It fails if the balance is below 16,000,000 lamports (`MIN_PUBLICATION_SIGNER_BALANCE_LAMPORTS`, 0.016 SOL). If the balance can't be read, the command prints a warning and continues. A failed read is never treated as a zero balance.

## What the CLI does and does not do

It does:

- start or resume the publication workflow, and save each step to the checkpoint before calling the portal;
- validate each prepared mint and collection-verification transaction against the publication policy before signing it with your keypair;
- create the attestation from the portal's block data, then submit the release for store review.

It does not:

- retry mutations;
- print or store signing keys or API keys;
- replay a step whose outcome is ambiguous. Resume stops instead.

## The library

Use `solana_kit_dapp_publisher` directly to build your own publishing tool. Every feature is off by default:

| Feature | Adds |
| --- | --- |
| (default) | `AttestationMessage`, `RequestId` and the funding preflight helpers (`resolve_funding_rpc_url`, `check_signer_balance`, `format_sol_amount`) |
| `crypto` | `validate_prepared_transaction`, `PublicationPolicy`, `ExpectedTransaction`, `PublicationAttestation` and detached-signature verification |
| `cli` | `cli` module: pure flag, endpoint and API-key resolution (`parse_flags`, `resolve_portal_targets`, `resolve_api_key` and related functions) |
| `portal` | `portal` module: thirteen typed tRPC procedures over `rest::Transport<PortalService>`, an `Uploader` trait for the binary PUT, and `create_ingestion_with_retry` |
| `workflow` | `workflow` module: caller-driven ingestion, resume, polling, metadata hydration and the completion stages. Implies `portal` and `crypto`. |

```rust
use solana_kit_core::{Hash, codecs::Limits};
use solana_kit_dapp_publisher::{AttestationMessage, RequestId};

let request_id = RequestId::parse("00000000000000000000000000000042")?;
let message = AttestationMessage::new(100, Hash::from_bytes([0; 32]), request_id,
    Limits::new(1024, 4096))?;
// Present message.bytes() to the application's approved Ed25519 signer.
# Ok::<(), solana_kit_dapp_publisher::Error>(())
```

`validate_prepared_transaction` takes the complete wire bytes and a `ReleaseMint` or `VerifyCollection` policy. It checks that only the Compute Budget and Token Metadata programs are invoked, that the signer set is exactly what you expect, and that every signature already present is valid. It then returns a sealed transaction for you to sign. The seal confirms only that documented policy. It does not prove instruction intent, account authenticity, economic effects or blockhash freshness. The library performs no network requests, file reads or signing.

## Release workflow

The [`publisher CLI`](https://github.com/pina-rs/solana_kit/blob/main/.github/workflows/publisher-cli.yml) workflow builds the binary for all eight targets and attaches the archives to a GitHub release.

- **Trigger:** push a `publisher-cli-vX.Y.Z` tag, or dispatch it manually with a `tag` input. Set `dry_run` to build and archive every target without uploading. `checkout_ref` builds a branch instead of the tag.
- **Jobs:** `prepare` creates a draft release, `upload_assets` builds each target with the pinned Rust 1.99.1 toolchain (`cross` for musl, static CRT on Windows) and uploads archives with checksums, and `publish_release` takes the release out of draft.

## Status

`publisher-cli-v0.1.0` is the first released CLI. The library's capability-ledger IDs remain pending, and the library is unpublished. The parity records cover the attestation and policy slice, the portal and the workflow:

- [`docs/parity/dapp-publisher.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/dapp-publisher.md)
- [`docs/parity/dapp-publisher-portal.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/dapp-publisher-portal.md)
- [`docs/parity/dapp-publisher-workflow.md`](https://github.com/pina-rs/solana_kit/blob/main/docs/parity/dapp-publisher-workflow.md)

These record the synthetic fixtures that were run. They also record that no real portal request, upload or publication was performed during those tests.
