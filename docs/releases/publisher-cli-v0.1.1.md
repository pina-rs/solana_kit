# Publisher CLI v0.1.1

Release candidate notes for the native `solana-kit-publisher` binary. This is a
reviewable GitHub release note, not a published release. No tag, binary upload,
GitHub release, registry upload or store submission occurred.

## Install

Release assets provide prebuilt archives for macOS (arm64/x86_64), Linux
(arm64/x86_64, glibc and musl) and Windows (arm64/x86_64). Download the archive
for the target and place `solana-kit-publisher` on `PATH`.

From the repository or a checkout:

```sh
cargo install --git https://github.com/pina-rs/solana_kit solana_kit_dapp_publisher_cli
cargo install --path crates/solana_kit_dapp_publisher_cli
```

The CLI crate is not on crates.io. The library and CLI crates retain
`publish = false` until their SDK dependency chain is published. The
`[package.metadata.binstall]` entry is prepared, but `cargo-binstall` resolves a
crate by registry name first; until the crate is published, use the release
archives or `cargo install --git`.

## What changed since publisher-cli-v0.1.0

`solana-kit-publisher` keeps the same commands and flags as v0.1.0:

- new publication: `--apk-file` or `--apk-url`
- resume: `resume --release-id` or `resume --session-id`

The binary now reports its own Cargo package version. `--version` prints
`solana-kit-publisher 0.1.1`, and the help heading uses the same value. v0.1.0
reported the publisher library constant instead. The library constant now
follows its own package metadata, because the two packages can be released
independently.

The CLI executes the v0.1.1 publisher library. That library adds the typed portal
protocol and bounded ingestion retry, resumable ingestion with metadata
hydration, normalized publication models and projections, publication
attestation and sealed prepared-transaction policy. These features are behind
the existing CLI commands; this release does not add a new command or flag. The
portal, signing, upload and submission behavior remains caller-owned, and the
CLI still does not retry mutations or submit transactions automatically.

## Compatibility

- Minimum supported Rust: 1.89.0.
- Qualified locally: Rust 1.89.0 and 1.99.1 on macOS arm64.
- Archives cover eight targets: aarch64/x86_64 macOS, aarch64/x86_64 Linux
  glibc and musl, and aarch64/x86_64 Windows MSVC.
- The publisher is an off-chain client. It is not an on-chain program and does
  not replace Pina or an official Solana program library.
- Publishing needs a Solana CLI keypair and a portal API key. The keypair and API
  key are never printed or checkpointed.

## Qualification scope and limits

This candidate was qualified as an offline package, not as a live publication:

- 34 dependency-ordered SDK archives were built and freshly reviewed from the
  committed 0.1.1 tree (1,327 regular members, 207-package local checksum
  registry).
- The external consumer passed 9 default and 12 selected-feature tests on Rust
  1.99.1 and Rust 1.89.0, plus the portable Wasm check and missing-archive and
  checksum negative controls.
- The publisher library and CLI archives were reviewed at 0.1.1. Their SHA256
  values are
  `22cba520b161bf9347a93f7131ded0529fe7ee304bc2002cd8891ea00571c5f1` and
  `679d5628f2bf24370b9dab27be4ffa61cec46c0d4fb641e8012c0702f9ebb552`.
- The actual release binary prints `solana-kit-publisher 0.1.1` for both
  `--version` and the `--help` heading.
- No real portal, RPC endpoint, APK upload, transaction signing, store review or
  publication request was executed. The portal-facing paths are verified by
  synthetic and portable tests only.
- No Linux or Windows archive was built or executed locally; those matrix
  entries remain unexecuted until the authorized release workflow runs.
- The whole SDK is not claimed complete. Canonical parity remains 450 complete,
  72 partial and 17,426 pending.
- The SDK registry closure still contains the unpublished LeptosKit Git
  prerequisite. That blocks whole-SDK registry publication, not this CLI's
  archive install path.
