# Portable conformance runtime receipt

The shared suite executes the four normalized publication-projection groups and,
from v58, the 365-constant error-code catalog. The publication groups reuse the
32 existing native bodies; the catalog groups reuse the same shared body as the
native `shared_errors` integration test. The error catalog now references the
existing public constants instead of repeating their numeric literals; their
values and behavior are unchanged.

The final five-host execution ran from the working tree based on `0e5fbce3`,
including the public-constant binding fix, that produced the reports below. It is the same code and report tree
recorded by this change; no compilation-only result is claimed.

The earlier publication-only v57/459 run used signed executable
`16379ca98e40e8d3aed937017e056b4bac4c1b60`, rebased onto `1d70bc37`.
Its historical artifacts remain separate from this v58 execution.

## Suite identity and preserved order

The shared suite is now `solana-kit-core-v58` with **461** cases. The first 459
case names, order and outcomes remain byte-identical to the committed v57 report,
which itself preserves the captured 448-row v55 prefix in
[`parity-completion-prefix-v55.txt`](../../verification/conformance/evidence/parity-completion-prefix-v55.txt).
Six groups append after the original 455 v56 cases:

- `publisher-publication-bundle-source-projection`
- `publisher-publication-diagnostic-redaction`
- `publisher-publication-ingestion-projection`
- `publisher-publication-session-checkpoint-precedence`
- `error-code-catalog-rejects-wrong-literal`
- `error-code-catalog-exact-365`

Each group reports its own outcome, so one changed projection or catalog
comparison cannot suppress the other results. Every one of the 32 publication
bodies keeps its original fixtures and assertion text; in portable builds the
existing `assert_eq!`/`assert!` calls report the first failed comparison as a
static error instead of unwinding the guest, and the same bodies still run as
the 32 native tests:

```sh
cargo test --locked -p solana_kit_dapp_publisher --features portal --test publication --offline
```

## Actual host execution

On 2026-11-04 the required command completed with exit 0 from the working tree:

```sh
CHROME='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' \
CHROMEDRIVER=/tmp/solana-kit-runtime-tools/chromedriver-mac-arm64/chromedriver \
WASMTIME=/tmp/solana-kit-runtime-tools/wasmtime-v50.0.0-aarch64-macos/wasmtime \
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 CARGO_NET_OFFLINE=true \
cargo xtask all
```

| Actual host               | Canonical                    | Corruption control           |
| ------------------------- | ---------------------------- | ---------------------------- |
| Native command            | 461/461, exit 0              | 460/461, exit 1              |
| Real Chrome               | Full report, 461/461 passed  | Full report, 460/461 passed  |
| Wasmtime 50 Preview 1     | 461/461, exit 0              | 460/461, exit 1              |
| Wasmtime 50 Preview 2     | 461/461, exit 0              | 460/461, exit 1              |
| Pinned LeptosKit WasmHost | Full JSON, 461/461 committed | Full JSON, 460/461 committed |

The corrupt mode still fails only `sha256-fips-abc`; no group is filtered or
skipped. The actual interpreter also ran a fresh guest with one fuel and trapped
with `OutOfFuel` before any report commit. Guest imports remain exactly
`leptoskit::{commit,node_create,node_set_props}`. The same invocation passed
224 dependency-graph commands, 51 dependency negative controls, the
forbidden-import control, and a fresh packaged-core consumer.

The checked-in execution reports are
[`native-success.txt`](../../verification/conformance/evidence/native-success.txt)
and
[`native-corruption.txt`](../../verification/conformance/evidence/native-corruption.txt).
Their SHA-256 values, and those of the preserved v55 prefix, are:

```text
d17a4a66dbb79a5a913fa15f50619dd86851064361aa0c55b57955a0eef1f9e3  native-success.txt
4852ad7359b8b13a74cbff4e2b2205449293eff167cb7083430045813fad0d62  native-corruption.txt
6602dd63618190a1fdb81dd25ca2640417fe2b681a1d9cacc1ff691f60bc8dcb  parity-completion-prefix-v55.txt
```

Tools were Rust 1.99.1, Chrome and ChromeDriver 156.0.8078.80, Wasmtime 50.0.0
`ebe5e8df1`, `wasm-bindgen-test-runner` 0.2.128, and the private LeptosKit host
pinned at `7437097c52aa68a3401bd378764e051ed4c9165a`. Artifact SHA-256 values from
the final execution:

```text
b56f17cd0a4005e99cb386efe113933f6848059925eb068b6051174eefa431cc  wasm32-unknown-unknown release guest
dc61c1c0c9c962403935487f07fd52c6f0ea3b48a4c736b34a4d953e59d5b6c8  wasm32-wasip1 debug command
c3e9d72e96bebeda1cc13acfe8a3d154690c457e6f26b2610a504400451bb92f  wasm32-wasip2 debug command
```

No per-case timing or performance measurement is claimed.

## Appended behavior

Every body uses existing public APIs with independent literal fixtures and no
network, clock, OS thread, executor, device or credential access.

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
- The catalog control first proves the unmodified literal table passes, flips the
  last constant's integer, and requires the same comparison to report
  `catalog integer differs` for that name. The evidence case then checks all 365
  constants' exact names, integers, domain rules, template bytes, lookup and
  non-empty rendering, plus the three count/uniqueness guards. Mismatches become
  case errors rather than guest panics, so the control and every later case are
  preserved.

## Fuel accounting

The private interpreter still grants one pooled allowance per entry with the
existing rates: 15,000,000 per ordinary case and 5,000,000 per crypto unit. The
four publication groups and two catalog groups are pure deterministic
projection/parsing work with no crypto, so they add only the ordinary-case
allowance. The exact current total is **15,280,000,000**:

- 458 ordinary groups × 15,000,000 = 6,870,000,000
- 257 authenticated transactions + 772 message crypto calls = 5,145,000,000
- 573 MWA backend entries = 2,865,000,000
- 80 appended parity-completion crypto units = 400,000,000

No rate increased, and the appended crypto ledger is unchanged. This is finite
source-owned accounting, not a measured minimum.

## Final local gates

Workspace tests passed 2,055 tests across 297 result groups, with no failures.
Workspace Clippy and Rustdoc passed with warnings denied; formatting, dprint,
Monochange and the repository style gate passed. Style scored 2,313 files, with
every repository and per-file category at least 90. The interpreted host's four
fuel tests and Clippy passed. Rust 1.89 passed the conformance tests and checks
for all three Wasm targets. The inventory tool's 18 tests also pass with the
expanded maintained report.

## Limits

This is off-chain, host-neutral projection and error-catalog evidence. It adds no
live wallet, validator, registry publication, store submission, external upload,
installed device or network RPC claim. The suite count and receipt do not mark the
whole SDK, the errors package or any phase complete. Actual command outputs and
statuses were retained outside build targets by the implementation unit. Those
disposable local artifacts are not committed. The final logs, runtime outputs,
generator comparison and artifact hashes are retained at
`/tmp/solana-kit-error-parent-final/` outside build targets.
