# Error-contract runtime receipt

This receipt records the actual five-host execution of the fifteen appended
error-contract cases. It supersedes the case count in the
[publication runtime receipt](publication-runtime.md) while leaving that v58
artifact and its hashes intact as history.

## Suite identity and preserved order

The shared suite is `solana-kit-core-v59` with **476** cases. The first 461
case names, order and outcomes remain byte-identical to the committed v58
report, which itself preserves the captured 448-row v55 prefix in
[`parity-completion-prefix-v55.txt`](../../verification/conformance/evidence/parity-completion-prefix-v55.txt).
The fifteen new groups follow the two catalog groups:

- `error-domain-rejects-wrong-range`
- `error-code-enum-rejects-wrong-constant`
- `error-execution-rejects-wrong-expected`
- `error-domain-classification-25`
- `error-code-enum-boundaries`
- `error-diagnostic-source-contract`
- `error-message-rejects-wrong-template`
- `error-message-template-interpolation`
- `error-context-key-conventions`
- `error-context-wire-mounted-values`
- `error-context-wire-literals`
- `error-rpc-enum-offset-callback`
- `error-execution-source-oracle`
- `error-json-rpc-projection`
- `error-simulation-unwrap`

The three source-owned negative controls run before their matching evidence
cases. Each control mutates a fixture value in memory and requires the intended
failure field; the following evidence case still runs and must pass. Every case
returns a reportable error instead of panicking the guest.

## Actual host execution

On 2026-11-04 the required command completed with exit 0 from the final working
tree:

```sh
CHROME='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' \
CHROMEDRIVER=/tmp/solana-kit-runtime-tools/chromedriver-mac-arm64/chromedriver \
WASMTIME=/tmp/solana-kit-runtime-tools/wasmtime-v50.0.0-aarch64-macos/wasmtime \
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 CARGO_NET_OFFLINE=true \
cargo xtask all
```

| Actual host               | Canonical                    | Corruption control           |
| ------------------------- | ---------------------------- | ---------------------------- |
| Native command            | 476/476, exit 0              | 475/476, exit 1              |
| Real Chrome               | Full report, 476/476 passed  | Full report, 475/476 passed  |
| Wasmtime 50 Preview 1     | 476/476, exit 0              | 475/476, exit 1              |
| Wasmtime 50 Preview 2     | 476/476, exit 0              | 475/476, exit 1              |
| Pinned LeptosKit WasmHost | Full JSON, 476/476 committed | Full JSON, 475/476 committed |

The corrupt mode fails only `sha256-fips-abc`; no group is filtered or skipped.
The actual interpreter also ran a fresh guest with one fuel and trapped with
`OutOfFuel` before any report commit. Guest imports remain exactly
`leptoskit::{commit,node_create,node_set_props}`. The same invocation passed the
236 dependency-graph invocations (228 distinct labels), 51 dependency negative controls, the
forbidden-import control and a fresh packaged-core consumer.

## Fuel

The prepared workload receives **15,505,000,000 fuel per entry**, computed as
`15_000_000 × (EXPECTED_CASE_COUNT - 3) + 5_000_000 × (MAX_BATCH + 1 + 3 ×
MESSAGE_COUNT + 4 + 573 + 80)`. The fifteen new groups are deterministic table
and string comparisons with no crypto, so they receive only the unchanged
ordinary-case allowance. No rate, cap or separately accounted group changed.

## Artifact hashes

```text
e2e284b1bb80e6e17084605acbb67bb9a926fb69c98c919f9970f044bce93335  native-success.txt
6dcea4cdc3052f61b97aff9a701026e99c017442668e0a8c2dcd957c1805cc4c  native-corruption.txt
6602dd63618190a1fdb81dd25ca2640417fe2b681a1d9cacc1ff691f60bc8dcb  parity-completion-prefix-v55.txt
```

The checked-in reports are
[`native-success.txt`](../../verification/conformance/evidence/native-success.txt)
and
[`native-corruption.txt`](../../verification/conformance/evidence/native-corruption.txt).

## Scope

These cases prove the contracts described in the
[error-contract adjudication](../parity/error-contracts-adjudication.md). They do
not complete the package, Phase 2 or the SDK. The 37 complete rows cover only
the exact contracts listed there; the 36 partial rows keep their concrete
representation and boundary differences.
