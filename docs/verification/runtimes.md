# Portable core runtime verification

`cargo xtask all` executes the same `solana_kit_conformance::run` suite natively,
in Chrome, in Wasmtime Preview 1 and Preview 2 commands, and in the actual
LeptosKit headless `WasmHost`. It then checks the guest imports, selected core
dependency closures, forbidden fixtures, and an external packaged-core consumer.
Each journey is also available as `native`, `browser`, `wasi`, `leptoskit`,
`gates`, or `consumer`. Run from this checkout; no secrets are needed.

## Reproduce

The checked-in toolchain selects Rust 1.99.1, rustfmt, Clippy and all three Wasm
targets. Install `wasm-bindgen-cli` exactly 0.2.128 with `cargo install
wasm-bindgen-cli --version 0.2.128 --locked`. Set these executable paths:

```sh
export CHROME=/path/to/chrome
export CHROMEDRIVER=/path/to/matching/chromedriver
export WASMTIME=/path/to/wasmtime
cargo xtask all
```

Wasmtime must report version 50.0.0. Missing executables, targets, guest files,
unexpected reports, or wrong exit codes are errors. Every executed command saves
stdout, stderr and its actual status under ignored `target/runtime-evidence/`.
The CI workflow pins Chrome and ChromeDriver 155.0.8059.84 and validates all host
download SHA-256 values before extraction. Wasmtime's Linux archive digest comes
from the official v50.0.0 release metadata; Chrome digests were calculated from
the exact official Chrome for Testing downloads.

The private host uses upstream `leptoskit_wasm` at revision
`fecf8628d326ff7b03f295523e6bb584c3fdf88d`, with its own locked workspace. It is
excluded from the SDK workspace and core dependency graph. The actual host runs
the Rust guest, reads every case name/error from committed node properties, then
dispatches a `Press` event and requires the corrupted report and second commit.
The guest's small unsafe FFI wrappers implement that pinned synchronous copy ABI;
core and the host harness forbid unsafe code.

## Current v5 executed receipt

The integrated `solana-kit-core-v5` suite preserves all preceding 39 cases and
adds three exact fixed-point cases. Fresh local macOS ARM64 execution on
2026-10-22 at signed checkpoint `9a8c5a52d1f7d90ade1b8fb7e4648e7ee8990c43`
used Rust 1.99.1, Chrome 155.0.8059.85, ChromeDriver 155.0.8059.84,
Wasmtime 50.0.0 and the pinned actual LeptosKit host. Source and native goldens
were clean for this `cargo xtask all` run, which exited 0.

| Execution                 | Canonical                      | Corruption control                                     |
| ------------------------- | ------------------------------ | ------------------------------------------------------ |
| Native command            | 42/42, exit 0                  | 41/42, exit 1                                          |
| Real headless Chrome      | Full 42/42 report checked      | Full 41/42 report checked; both browser tests passed   |
| Wasmtime Preview 1        | 42/42, exit 0                  | 41/42, exit 1                                          |
| Wasmtime Preview 2        | 42/42, exit 0                  | 41/42, exit 1                                          |
| Actual LeptosKit WasmHost | Full 42/42 JSON and one commit | Press event produced full 41/42 JSON and second commit |

The sole corrupted case remains `sha256-fips-abc`, with `SHA-256 digest mismatch`.
The three new cases cover exact parsing, negative floor arithmetic, scale larger
than storage width, signed codec bytes and truncation rejection. Every host
checks suite identity, case order/count, every outcome and failure state.

All 16 normal/build selections pass: plain, crypto, fixed-points and combined,
each on native, plain Wasm, Preview 1 and Preview 2. Eight additional normal-only
numeric selections prove `autocfg` is build-only. The numeric default features
remain disabled; plain core remains `bs58` only and crypto stays independent.
The actual combined guest imports exactly the three pinned LeptosKit functions.
The production import gate rejects `env::random`; the dependency gate rejects
Tokio under each of the four feature policies. The clean packaged-core consumer
passes all five tests, including actual fixed-point arithmetic and wire bytes.

Executed v5 artifact SHA-256 values (local debug WASI artifacts, release guest):

```text
3a3717685636b73a2aa192b305a05d12babba55034ba85d079725821dbcc3c5c  wasm32-wasip1
c5beea1bc5085d004d77da07e2a00224fa7debe477f5cbf862f759cebac7002a  wasm32-wasip2
6cfd3128dada0eb65a3e2a28a5fac3f73d4c7466060a4fcee58e0e0c2c81b0e8  leptoskit-guest
```

The [fixed-point receipt](../parity/fixed-points.md) retains all 126 canonical IDs:
121 implemented-bounded and five remaining. The production source and original
fixed-point tests are unchanged from their independent review. This bounded
off-chain client slice does not complete the package, Phase 2 or device journeys.
Earlier receipts below retain their historical counts and artifacts.

## Historical v4 executed receipt

The integrated `solana-kit-core-v4` suite preserves all preceding 35 cases and
adds four account/sysvar cases. Fresh local macOS ARM64 execution on 2026-10-22
at signed integration checkpoint `c18863db96bec6239c576c966c05c9dfc899a63f`
used Rust 1.99.1, Chrome 155.0.8059.85, ChromeDriver 155.0.8059.84, Wasmtime
50.0.0, and the pinned actual LeptosKit host. The committed source and native
goldens were clean for this `cargo xtask all` run, which exited 0.

| Execution                 | Canonical                      | Corruption control                                     |
| ------------------------- | ------------------------------ | ------------------------------------------------------ |
| Native command            | 39/39, exit 0                  | 38/39, exit 1                                          |
| Real headless Chrome      | Full 39/39 report checked      | Full 38/39 report checked; both browser tests passed   |
| Wasmtime Preview 1        | 39/39, exit 0                  | 38/39, exit 1                                          |
| Wasmtime Preview 2        | 39/39, exit 0                  | 38/39, exit 1                                          |
| Actual LeptosKit WasmHost | Full 39/39 JSON and one commit | Press event produced full 38/39 JSON and second commit |

The sole corrupted case remains `sha256-fips-abc`, with `SHA-256 digest mismatch`.
The account cases execute the independent signed Clock bytes, account metadata
and missing snapshots, u64 history bounds, and lossless program-error facts.
Every host checks suite identity, exact count, every outcome, and failure state.
All eight default/crypto dependency closures, exact guest imports, both forbidden
fixtures, and all four extracted packaged-core consumer tests passed. No core
dependencies were added; the default graph remains `bs58` only.

Executed v4 artifact SHA-256 values (local debug WASI artifacts, release guest):

```text
fdac326ef0a8c2b6e969ca33489415f3f811ba1addd3746b124af5148f65c2e4  wasm32-wasip1
c698a1970f91cff926ee27d505d4ee69dc93eb2f126ce5537fa6b36182e463af  wasm32-wasip2
1646059fbcf4e86854a95eafeb48a642bdf58bb31597784df3973a1a191093ab  leptoskit-guest
```

The [account/sysvar receipt](accounts-sysvars.md) maps 151 declarations and
retains all 78 unresolved declarations. Account fetching remains pending;
this deterministic client slice does not complete Phase 2, transports, or device
journeys. Earlier receipts below describe their historical artifacts.

## Historical v3 executed receipt

The integrated `solana-kit-core-v3` suite has 35 cases: the preceding 31
primitive/codec cases and four immutable legacy/v0 transaction cases. Fresh
local macOS ARM64 execution on 2026-10-22 at signed integration checkpoint
`31a4eb8c1fcc118eee467e99cb414859394e3e72` used Rust 1.99.1, Chrome
155.0.8059.85, ChromeDriver 155.0.8059.84, Wasmtime 50.0.0, and the pinned actual
LeptosKit host. The committed source and native goldens were clean for this
`cargo xtask all` run, which exited 0.

| Execution                 | Canonical                      | Corruption control                                     |
| ------------------------- | ------------------------------ | ------------------------------------------------------ |
| Native command            | 35/35, exit 0                  | 34/35, exit 1                                          |
| Real headless Chrome      | Full 35/35 report checked      | Full 34/35 report checked; both browser tests passed   |
| Wasmtime Preview 1        | 35/35, exit 0                  | 34/35, exit 1                                          |
| Wasmtime Preview 2        | 35/35, exit 0                  | 34/35, exit 1                                          |
| Actual LeptosKit WasmHost | Full 35/35 JSON and one commit | Press event produced full 34/35 JSON and second commit |

The sole corrupted case remains `sha256-fips-abc`, with `SHA-256 digest mismatch`.
Every host checks suite identity, exact count, every outcome, and failure state.
All eight selected default/crypto dependency closures passed. The default core
graph remains `bs58` only; exact guest imports and both forbidden-import and
forbidden-dependency negative controls passed. The extracted packaged-core
consumer passed four public API tests, including exact legacy message bytes,
signing, received unresolved lifetime metadata, and packet re-encoding.

Executed v3 artifact SHA-256 values (local debug WASI artifacts, release guest):

```text
4a03db4577a99f778358391c1240e4707eac03b473350de427819e8e0131da94  wasm32-wasip1
ee3e18f8317eb432f306c0c8794b5106722b0ae6b63dc07cb77eec092229e754  wasm32-wasip2
b4f477790d5ad1fc24d915cf95c6f2b8180ce4442fddcf4076537ca48023358f  leptoskit-guest
```

The [transaction ledger](../parity/message-transactions.md) retains all 746
Phase 3 IDs: 22 partial and 724 pending, with none complete. This receipt proves
the bounded synchronous legacy/v0 slice across these hosts; v1, durable nonces,
introspection, off-chain messages, general signers, transport, and device work
remain outstanding. Earlier receipts below describe historical artifacts.

## Historical v2 executed receipt

The expanded `solana-kit-core-v2` suite has 31 cases, including six bounded
codec cases. Fresh local macOS ARM64 execution on 2026-10-22 at integration
checkpoint `bde7552e2079` used the same Rust 1.99.1, Chrome 155.0.8059.85,
ChromeDriver 155.0.8059.84, Wasmtime 50.0.0, and pinned actual LeptosKit host
described below. `cargo xtask all` exited 0.

| Execution                 | Canonical                      | Corruption control                                     |
| ------------------------- | ------------------------------ | ------------------------------------------------------ |
| Native command            | 31/31, exit 0                  | 30/31, exit 1                                          |
| Real headless Chrome      | Full 31/31 report checked      | Full 30/31 report checked; both browser tests passed   |
| Wasmtime Preview 1        | 31/31, exit 0                  | 30/31, exit 1                                          |
| Wasmtime Preview 2        | 31/31, exit 0                  | 30/31, exit 1                                          |
| Actual LeptosKit WasmHost | Full 31/31 JSON and one commit | Press event produced full 30/31 JSON and second commit |

The sole corrupted case is `sha256-fips-abc`, with `SHA-256 digest mismatch`.
Each host validates the suite identity, exact case count, outcomes and failure
state. The actual-host display derives its counts from the suite constant.
All eight selected default/crypto dependency closures passed, both import and
dependency negative controls rejected their forbidden fixtures, and the clean
packaged-core consumer passed all three public API tests.

Executed v2 artifact SHA-256 values (local debug WASI artifacts, release guest):

```text
983f30eaa10b04dc39068702b031d1d0fa48b9ce4ce6cb663157d810b442e815  wasm32-wasip1
cd4e1b9d62332e28999a8b2a05cbf5b011caa84dd360f34235b6c4281648084f  wasm32-wasip2
2ecef09163ac6df0a91ec33bcaa79dade89ec26d4ca71127a745d5d4fd9e785b  leptoskit-guest
```

The [codec receipt](core-codecs.md) records the bounded capability scope and
remaining work. This execution does not complete Phase 2 or the transport and
device plans. The following v1 receipt is retained as historical evidence;
its counts and hashes describe those earlier artifacts, not the current suite.

## Historical v1 executed receipt

Local macOS ARM64 execution on 2026-10-22 used Rust 1.99.1, Chrome
155.0.8059.85, ChromeDriver 155.0.8059.84, and Wasmtime 50.0.0
`ebe5e8df1` (2026-10-20). The exact suite is `solana-kit-core-v1`, 25 cases.

| Execution                 | Canonical                      | Corruption control                                            |
| ------------------------- | ------------------------------ | ------------------------------------------------------------- |
| Native command            | 25/25, exit 0                  | 24/25, exit 1                                                 |
| Real headless Chrome      | Full report checked            | Sole failed case `sha256-fips-abc`; both browser tests passed |
| Wasmtime Preview 1        | 25/25, exit 0                  | 24/25, exit 1                                                 |
| Wasmtime Preview 2        | 25/25, exit 0                  | 24/25, exit 1                                                 |
| Actual LeptosKit WasmHost | Full 25/25 JSON and one commit | Press event produced full 24/25 JSON and second commit        |

Executed artifact SHA-256 values (local debug WASI artifacts, release guest):

```text
b86c95b60b2571a9cf1b18e6663d94bcac16bc36b356fbf67a3ee242cbf0a260  wasm32-wasip1
16c19a5f4baea390e8eab79b099c7f760916bd982699edfd84f9a008cb700405  wasm32-wasip2
e6498b45bfa7b6dccc82bc5c51bbc4df3acf6ad12901aafc55fc7975b642e65b  leptoskit-guest
```

The compiled guest imports exactly `leptoskit::node_create`,
`leptoskit::node_set_props`, and `leptoskit::commit`. The binary parser rejects
all other imports and missing expected entries. `xtask/fixtures/forbidden-import.wat`
imports `env::random` and fails the same checker. To exercise the public failing
gate after `cargo xtask gates` creates the binary fixture:

```sh
cargo xtask imports target/runtime-evidence/forbidden-import.wasm
cargo xtask dependency-fixture xtask/fixtures/forbidden-dependency.txt
```

Both commands must exit nonzero. The combined gate instead asserts each expected
rejection and fails if a negative fixture succeeds. Normal/build dependency trees
are inspected for native, plain Wasm, Preview 1, and Preview 2, selecting plain,
crypto, fixed-points and combined features. A closed crate-name allowlist rejects Tokio and other new dependencies;
the lockfile pins versions. Address-only core selects only `bs58`. Native crypto
also selects CPU-detection `libc`; the plain Wasm crypto tree selects neither.
Host/UI/test-runner dependencies remain outside these selected core closures.

The clean consumer extracts the verified core archive to a fresh system temporary
directory, with its own workspace and no repository configuration or patches.
It removes Rust flags and compiler wrappers, then executes public API success
and failure tests. The extracted sources and output path are retained for review.

## Evidence limits

These are executed synchronous core fixtures. They establish no HTTP, sockets,
TLS, entropy, clocks, async wake/cancellation, wallets, validator, generated app,
Android, or iOS behavior. The new Linux CI workflow is checked locally with
actionlint; its remote execution must pass before merge. Browser and headless
LeptosKit execution do not complete the broader transport or device plans.
