# Parity-completion portable runtime receipt

The signed executable source is `444dd895`, the rebased conformance commit whose
parent is `a0cc1cf0`. The actual runtime proof below executed from that clean
tree before this receipt was added. No production SDK source changed. Parent review also requires the
live canonical registration order to match the captured report, rather than comparing only two
checked-in files.

The shared suite is now `solana-kit-core-v56` with **455** cases. The first 448
case names, order, outcomes and fixture bytes remain exactly the captured v55
report in
[`parity-completion-prefix-v55.txt`](../../verification/conformance/evidence/parity-completion-prefix-v55.txt).
Seven groups append at the end:

- `grind-entropy-cancellation`
- `message-decompile-roundtrip`
- `signer-explicit-config`
- `token-mint-ata-plans`
- `client-parsed-account-families`
- `fixed-point-codec-pairing`
- `test-support-scripted-client`

The checked-in execution reports are
[`native-success.txt`](../../verification/conformance/evidence/native-success.txt)
and
[`native-corruption.txt`](../../verification/conformance/evidence/native-corruption.txt).
The corrupt mode still fails only `sha256-fips-abc`, at 454/455, with exit 1.

## Actual host execution

On 2026-11-04 the required command completed with exit 0 from the clean rebased
tree:

```sh
CHROME='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' \
CHROMEDRIVER=/tmp/solana-kit-runtime-tools/chromedriver-mac-arm64/chromedriver \
WASMTIME=/tmp/solana-kit-runtime-tools/wasmtime-v50.0.0-aarch64-macos/wasmtime \
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 CARGO_NET_OFFLINE=true \
cargo +1.99.1 xtask all
```

| Actual host               | Canonical                    | Corruption control           |
| ------------------------- | ---------------------------- | ---------------------------- |
| Native command            | 455/455, exit 0              | 454/455, exit 1              |
| Real Chrome               | Full report, 455/455 passed  | Full report, 454/455 passed  |
| Wasmtime 50 Preview 1     | 455/455, exit 0              | 454/455, exit 1              |
| Wasmtime 50 Preview 2     | 455/455, exit 0              | 454/455, exit 1              |
| Pinned LeptosKit WasmHost | Full JSON, 455/455 committed | Full JSON, 454/455 committed |

The actual interpreter also ran a fresh guest with one fuel and trapped with
`OutOfFuel` before any report commit. Guest imports remain exactly
`leptoskit::{commit,node_create,node_set_props}`. Dependency-graph negative
controls, the forbidden-import control and the fresh packaged-core consumer all
passed in the same invocation.

Tools were Rust 1.99.1, Chrome and ChromeDriver 156.0.8078.80, Wasmtime 50.0.0
`ebe5e8df1`, and the private LeptosKit host pinned at
`fecf8628d326ff7b03f295523e6bb584c3fdf88d`. Artifact SHA-256 values from that
execution:

```text
4542fb9f878b37b7f1419a7971ab8f0f781d44995aeddf3db395a4f6991cc599  wasm32-unknown-unknown release guest
3b734e3e61ec535b9578083247aee3c6e7d72b8fea9519f0351d1d713526fd30  wasm32-wasip1 debug command
9ab19147e5f0f42ee7696de3a906856a3b212c90b82966cb5a10932e3b2f5a90  wasm32-wasip2 debug command
```

## Appended behavior

Every body uses existing public APIs with independent literal fixtures and no
network, clock, OS thread, executor or device access.

- Grinding covers constant-time equality, zero amounts with no entropy,
  invalid and overlong literals rejected before entropy, a full-address match,
  finite exhaustion, a cooperative 32-candidate suspension, and drop
  cancellation with deterministic manual polls.
- Message decompilation roundtrips legacy, v0 and v1 canonical bytes, keeps
  lookup table/index/role provenance and duplicate positions, preserves inline
  v1 configuration, leaves received lifetimes unresolved until the caller
  supplies metadata, and rejects a missing lookup snapshot.
- Explicit signer configuration forwards `None`, `Some(0)` and `Some(u64::MAX)`
  unchanged through a local `Rc`-backed composition, and `aborted: true`
  returns cancellation before any backend effect.
- Token plans cover indivisible create-mint, sequential idempotent ATA plus
  checked mint/transfer, a `ReadonlySigner` authority distinct from the payer,
  custom program identities, derived canonical/custom ATAs, and compiler signer
  requirements.
- Parsed accounts project every declared family through the real
  `jsonParsed` boundary, keep exact `u64` amounts, retain unknown program and
  unknown discriminator raw JSON without `info`, reject duplicate keys and byte
  or depth budget excess, and never reinterpret a base64 fallback as typed data.
- Fixed-point width metadata allows direct `Paired` composition of the built-in
  codec and rejects mismatched widths.
- The scripted client observes the exact request body and captured metadata,
  completes and drops pending exchanges, and enters pending status reentrantly
  from wake and drop callbacks without OS threads.

## Fuel accounting

The private interpreter still grants one pooled allowance per entry. The
existing rates are unchanged: 15,000,000 per ordinary case and 5,000,000 per
crypto unit, with the transaction batch, two message groups and 573 MWA entries
still separately accounted. The exact current total is **15,190,000,000**:

- 452 ordinary groups × 15,000,000 = 6,780,000,000
- 257 authenticated transactions + 772 message crypto calls = 5,145,000,000
- 573 MWA backend entries = 2,865,000,000
- 80 appended crypto units = 400,000,000

The appended ledger is explicit and source-owned: 69 grinding key derivations
(one expected-address fixture, one real match, two exhaustion candidates, 33
cooperative candidates and 32 cancelled candidates), five signer key operations
(one fixture signing and three successful verifications), and six derived
associated-address searches. It is not a measured minimum and does not raise the
ordinary rate.

## Workspace gates

Run with concrete Rust 1.99.1 binaries first on `PATH` and `CARGO`, `RUSTC` and
`RUSTDOC` set to their `rustup which --toolchain 1.99.1` paths:

| Gate                                                                                           | Result                                                               |
| ---------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `cargo test --locked --workspace --all-features --offline`                                     | Passed                                                               |
| `cargo clippy --locked --workspace --all-targets --all-features --offline -- -D warnings`      | Passed                                                               |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --all-features --no-deps --offline` | Passed                                                               |
| `cargo fmt --all -- --check`                                                                   | Passed                                                               |
| `cargo xtask style`                                                                            | Passed, 2,250 files; repository 97.23 readability / 98.53 complexity |
| `cargo xtask dependency-graphs`                                                                | Passed, 224 graph commands plus negative controls                    |
| `monochange check`                                                                             | Passed, 0 issues                                                     |
| Rust 1.89 conformance tests and three Wasm target checks                                       | Passed                                                               |
| `dprint check`                                                                                 | Passed after the handbook formatting repair described below          |

The initial full `dprint` run found 29 Markdown files introduced by the documentation site merge.
The subsequent formatting repair normalized those files with the repository formatter and passed
the full check and the documentation site's real-book build tests. No formatter policy or exclusion
was changed.

## Limits

This is off-chain, host-neutral behavior evidence. It adds no live wallet,
validator, registry publication, store submission, external upload, installed
device or network RPC claim. The suite count and receipt do not mark the whole SDK
complete. Validator and installed application results are recorded separately in the
[program integration](program-integration.md) and
[installed wallet](installed-wallet-current.md) receipts.
