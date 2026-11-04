# Address-family runtime receipt

This receipt records the actual five-host execution of the four appended
address-family cases. It supersedes only the suite identity and case count of the
[error-contract runtime receipt](error-contracts-runtime.md); that v59 artifact
and its hashes remain history.

## Suite identity and preserved order

The shared suite is `solana-kit-core-v60` with **480** cases. The first 461 case
names, order and outcomes remain byte-identical to the committed v58 report,
which itself preserves the captured 448-row v55 prefix in
[`parity-completion-prefix-v55.txt`](../../verification/conformance/evidence/parity-completion-prefix-v55.txt).
The four new groups follow every earlier case:

- `address-constant-rejects-wrong-literal`
- `curve-classification-rejects-wrong-expected`
- `curve-compressed-point-classification`
- `native-loader-source-literal-invalid`

The two controls mutate an expected sysvar literal and an expected curve
classification in memory. The following evidence cases still run and must pass.
The native-loader case executes the public parser on the exact 40-character
pinned literal and requires `WrongLength { expected: 32, actual: 30 }`. Every
case returns a reportable error instead of panicking the guest.

## Actual host execution

On 2026-11-04 the required command completed with exit 0 from this working tree:

```sh
CHROME='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' \
CHROMEDRIVER=/tmp/solana-kit-runtime-tools/chromedriver-mac-arm64/chromedriver \
WASMTIME=/tmp/solana-kit-runtime-tools/wasmtime-v50.0.0-aarch64-macos/wasmtime \
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 CARGO_NET_OFFLINE=true \
cargo xtask all
```

The actual tools were Chrome `156.0.8078.80`, ChromeDriver `156.0.8078.80`,
Wasmtime `50.0.0` and LeptosKit revision
`7437097c52aa68a3401bd378764e051ed4c9165a`.

| Actual host               | Canonical                    | Corruption control           |
| ------------------------- | ---------------------------- | ---------------------------- |
| Native command            | 480/480, exit 0              | 479/480, exit 1              |
| Real Chrome               | Full report, 480/480 passed  | Full report, 479/480 passed  |
| Wasmtime 50 Preview 1     | 480/480, exit 0              | 479/480, exit 1              |
| Wasmtime 50 Preview 2     | 480/480, exit 0              | 479/480, exit 1              |
| Pinned LeptosKit WasmHost | Full JSON, 480/480 committed | Full JSON, 479/480 committed |

Chrome and LeptosKit compare complete reports internally; these counts are not
separate stdout summaries. The corrupt mode fails only `sha256-fips-abc`. The
same invocation passed the dependency-graph gates, import controls and the fresh
packaged-core consumer. Guest imports remain exactly
`leptoskit::{commit,node_create,node_set_props}`.

The LeptosKit release build emitted a non-fatal `rust-objcopy` warning because
the installed toolchain could not load `libLLVM.dylib`. The host executable still
built and the suite committed its complete report; no runtime result was skipped.

## Fuel

The prepared workload receives **15,565,000,000 fuel per entry**, computed as
`15_000_000 × (EXPECTED_CASE_COUNT - 3) + 5_000_000 × (MAX_BATCH + 1 + 3 ×
MESSAGE_COUNT + 4 + 573 + 80)`. The four new groups are deterministic literal,
parser and curve comparisons. They receive only the unchanged ordinary-case
allowance; the total moves from 15,505,000,000 by 60,000,000 for four cases. No
rate, cap, `CRYPTO_WORK` category, negative control or one-fuel trap changed.

## Artifact hashes

```text
f03e4e82b4b18505e2a9ad2d9bce4dffeb04c479edc11631ab8222ef9ece6d0d  native-success.txt
9bea3f3e4cc46b0970654bc958a4b0322e5bf8c8d4fecc88648fc18f4c492093  native-corruption.txt
6602dd63618190a1fdb81dd25ca2640417fe2b681a1d9cacc1ff691f60bc8dcb  parity-completion-prefix-v55.txt
```

The checked-in reports are
[`native-success.txt`](../../verification/conformance/evidence/native-success.txt)
and
[`native-corruption.txt`](../../verification/conformance/evidence/native-corruption.txt).

## Toolchain checks

Rust 1.99.1 executed the workspace tests, Clippy, rustdoc, the capability
inventory, the shared suite and `cargo xtask all`. Rust 1.89.0 executed the
scoped core and conformance suites and built the conformance library for
`wasm32-unknown-unknown`, `wasm32-wasip1` and `wasm32-wasip2`, plus the guest for
`wasm32-unknown-unknown`. Each 1.89 invocation set `CARGO`, `RUSTC` and `RUSTDOC`
to the concrete 1.89 paths so no 1.99 binary could be inherited.

The full workspace test suite passed on its second run. The first run stopped on
one unrelated loopback test,
`mwa::tests::remote::dropping_a_pending_join_future_closes_the_actual_reflector_peer`.
That test passed immediately in isolation and in the complete rerun; no address
code, port or runtime behavior was changed.

## Evidence location

The retained command logs, generator, cross-check and exact evidence hashes are
under
`/private/var/folders/lm/90xr38kd5y52p5y6jj2ny_dr0000gn/T/opencode/address-family-rw01/`.
The main run log is `xtask-all.log`; the 1.99 and 1.89 command logs are
alongside it. The temporary 1.89 target directories were removed after the
checks; the retained log directory is about 31 MB.

## Scope

These cases prove the contracts described in the
[address-family adjudication](../parity/address-family-adjudication.md). They do
not complete either package, Phase 2 or the SDK. The 48 complete literal rows
cover only their exact values; `nativeLoaderProgramAddress` stays partial because
its pinned source literal is not an `Address`; the ten curve/PDA helpers keep
their documented residual behavior.
