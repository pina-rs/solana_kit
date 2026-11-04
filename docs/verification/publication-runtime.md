# Publication projection runtime receipt

The normalized publication projection already had 32 independent native tests in
`crates/solana_kit_dapp_publisher/tests/publication.rs` and its four modules
(`bundle` 12, `diagnostics` 3, `ingestion` 8, `session` 9). The shared suite
executed only the publisher attestation, portal and workflow families, so these
projections had no Chrome, WASI or LeptosKit evidence. This change reuses every
existing body in four appended portable groups; no production SDK source changed.

The final five-host execution ran from signed executable
`16379ca98e40e8d3aed937017e056b4bac4c1b60`, rebased onto `1d70bc37`. It
includes the review fixes that return missing projected values as errors and
exercise the portable assertion adapter with a deliberately wrong expected
value. The documentation receipt follows that execution without changing SDK
or conformance behavior.

## Suite identity and preserved order

The shared suite is now `solana-kit-core-v57` with **459** cases. The first 455
case names, order and outcomes remain byte-identical to the committed v56 report,
which itself preserves the captured 448-row v55 prefix in
[`parity-completion-prefix-v55.txt`](../../verification/conformance/evidence/parity-completion-prefix-v55.txt).
Four groups append at the end:

- `publisher-publication-bundle-source-projection`
- `publisher-publication-diagnostic-redaction`
- `publisher-publication-ingestion-projection`
- `publisher-publication-session-checkpoint-precedence`

Each group reports its own outcome, so one changed projection cannot suppress the
other three. Every one of the 32 bodies keeps its original fixtures and assertion
text; in portable builds the existing `assert_eq!`/`assert!` calls report the first
failed comparison as a static error instead of unwinding the guest, and the same
bodies still run as the 32 native tests:

```sh
cargo test --locked -p solana_kit_dapp_publisher --features portal --test publication --offline
```

## Actual host execution

On 2026-11-04 the required command completed with exit 0 from the clean tree:

```sh
CHROME='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' \
CHROMEDRIVER=/tmp/solana-kit-runtime-tools/chromedriver-mac-arm64/chromedriver \
WASMTIME=/tmp/solana-kit-runtime-tools/wasmtime-v50.0.0-aarch64-macos/wasmtime \
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 CARGO_NET_OFFLINE=true \
cargo xtask all
```

| Actual host               | Canonical                    | Corruption control           |
| ------------------------- | ---------------------------- | ---------------------------- |
| Native command            | 459/459, exit 0              | 458/459, exit 1              |
| Real Chrome               | Full report, 459/459 passed  | Full report, 458/459 passed  |
| Wasmtime 50 Preview 1     | 459/459, exit 0              | 458/459, exit 1              |
| Wasmtime 50 Preview 2     | 459/459, exit 0              | 458/459, exit 1              |
| Pinned LeptosKit WasmHost | Full JSON, 459/459 committed | Full JSON, 458/459 committed |

The corrupt mode still fails only `sha256-fips-abc`; no projection group is ever
filtered or skipped. The actual interpreter also ran a fresh guest with one fuel
and trapped with `OutOfFuel` before any report commit. Guest imports remain
exactly `leptoskit::{commit,node_create,node_set_props}`. The same invocation
passed 224 dependency-graph commands, 51 dependency negative controls, the
forbidden-import control, and a fresh packaged-core consumer.

The checked-in execution reports are
[`native-success.txt`](../../verification/conformance/evidence/native-success.txt)
and
[`native-corruption.txt`](../../verification/conformance/evidence/native-corruption.txt).
Their SHA-256 values, and those of the preserved v55 prefix, are:

```text
44f904726e73aef44e312c5ae1b4a74f94e66a55e0c0e2313d849a16632ced6d  native-success.txt
b1da101f8c0fa7cacbdc4e7a162541b6d79e0a3ddaf3f7e969d8cee78f68e1e5  native-corruption.txt
6602dd63618190a1fdb81dd25ca2640417fe2b681a1d9cacc1ff691f60bc8dcb  parity-completion-prefix-v55.txt
```

Tools were Rust 1.99.1, Chrome and ChromeDriver 156.0.8078.80, Wasmtime 50.0.0
`ebe5e8df1`, `wasm-bindgen-test-runner` 0.2.128, and the private LeptosKit host
pinned at `7437097c52aa68a3401bd378764e051ed4c9165a`. Artifact SHA-256 values from
that execution:

```text
8f7c8dd74bfcc0c23f7a154a758ab0d0e78ad8eec657eb3f010b35273b0ac6cb  wasm32-unknown-unknown release guest
c1312ece1191e8fc9ce9622688f5d37bd0d8eb04f2f164f0c5c694fb17073e59  wasm32-wasip1 debug command
b65efcbb6203447f7ca783bdfa4f580e52c28802f324d39bc738c0cff2fbc6c1  wasm32-wasip2 debug command
```

The aggregate command includes compilation, Chrome startup and 224 Cargo graph
queries. No per-case timing or performance measurement is claimed.

## Appended behavior

Every body uses the existing public normalized-projection API with independent
literal JSON fixtures and no network, clock, OS thread, executor, device or
credential access.

- Bundle projection covers full source identity and release fields, empty-bundle
  defaults, individual/unknown/absent publisher kinds, description and contact
  precedence, blank-versus-missing contacts, a Unicode-safe UTF-16 description
  bound, deterministic media order, independent install/release fallbacks,
  inferred name and MIME type, signer-authority defaults, and exact wide version
  text.
- Diagnostics cover static progress messages, provider detail that never becomes
  progress text, and debug output that omits session identifiers, provider errors
  and unrecognized-tag values.
- Ingestion covers source-kind inference including unknown values, ready-bundle
  identity and attachments, status and source fallbacks, existing-release reuse,
  future source kinds, explicit attachment replacement, and session-before-release
  metadata URI precedence.
- Session covers stage and receipt checkpoint precedence, failed and unknown
  fallbacks, absent/null/blank stage distinctions, status precedence, timestamps
  that never sample a clock, error text retained only as explicit data, checkpoint
  wire ordering, and direct marker normalization.

## Fuel accounting

The private interpreter still grants one pooled allowance per entry with the
existing rates: 15,000,000 per ordinary case and 5,000,000 per crypto unit. The
four appended groups are pure deterministic projections with no crypto, so they
add only the ordinary-case allowance. The exact current total is
**15,250,000,000**:

- 456 ordinary groups × 15,000,000 = 6,840,000,000
- 257 authenticated transactions + 772 message crypto calls = 5,145,000,000
- 573 MWA backend entries = 2,865,000,000
- 80 appended parity-completion crypto units = 400,000,000

No rate increased, and the appended crypto ledger is unchanged. This is finite
source-owned accounting, not a measured minimum.

## Final local gates

After the review fixes, workspace tests passed 2,012 tests across 296 result
groups. Workspace Clippy and Rustdoc passed with warnings denied; formatting,
dprint, Monochange and the repository style gate passed. Style scored 2,260
files, with every repository and per-file category at least 90. The interpreted
host's four fuel tests and Clippy passed. Rust 1.89 passed the conformance tests
and checks for all three Wasm targets. The inventory tool's 18 tests also pass
with the expanded maintained report.

## Limits

This is off-chain, host-neutral projection evidence. It adds no live wallet,
validator, registry publication, store submission, external upload, installed
device or network RPC claim. The suite count and receipt do not mark the whole SDK
complete. Actual command outputs and statuses were captured under
`target/runtime-evidence/` for this execution and retained, with the three Wasm
artifacts, at `/tmp/solana-kit-publication-parent-evidence/` before target
cleanup. Those disposable local artifacts are not committed.
