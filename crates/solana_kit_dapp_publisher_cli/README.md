# Solana Kit dApp Publisher CLI

Native publishing CLI for the Solana dApp Store, built on
[`solana_kit_dapp_publisher`](../solana_kit_dapp_publisher).

## Install

From the repository:

```sh
cargo install --git https://github.com/pina-rs/solana_kit solana_kit_dapp_publisher_cli
```

From a checkout:

```sh
cargo install --path crates/solana_kit_dapp_publisher_cli
```

Prebuilt archives are published on GitHub Releases for macOS (arm64/x86_64),
Linux (arm64/x86_64, glibc and musl) and Windows (arm64/x86_64). `cargo-binstall`
uses the `[package.metadata.binstall]` entry in `Cargo.toml`.

The crate is not yet on crates.io because the SDK crates it depends on remain
unpublished. `cargo install --git` and the prebuilt archives are the supported
install paths until that dependency chain is released.

## Usage

```sh
export DAPP_STORE_API_KEY=...
solana-kit-publisher \
  --apk-file ./app-release.apk \
  --whats-new "First release" \
  --keypair ~/.config/solana/id.json
```

Resume an interrupted publication:

```sh
solana-kit-publisher resume --release-id <RELEASE_ID> --keypair ~/.config/solana/id.json
```

The API key defaults to `DAPP_STORE_API_KEY`; use `--api-key-env` to select a
different variable or `--api-key-stdin` to pipe it. `--local-dev` restricts the
portal to localhost. See `solana-kit-publisher --help` for every flag.

## Behavior

The CLI starts or resumes the caller-owned workflow, uploads a local APK through
one presigned PUT, validates and signs prepared mint and collection-verification
transactions with the supplied keypair, creates the attestation from portal block
data, and submits store review. Checkpoints are stored at
`.solana-kit-publisher/checkpoint.json` in the working directory.

It does not retry mutations, submit transactions automatically, or publish normal
implementation crates. Signing keys and API keys are never printed or written to
the checkpoint.
