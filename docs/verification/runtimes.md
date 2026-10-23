# Portable core runtime verification

`cargo xtask all` executes the same `solana_kit_conformance::run` suite natively,
in Chrome, in Wasmtime Preview 1 and Preview 2 commands, and in the actual
LeptosKit headless `WasmHost`. It then checks the guest imports, selected core
dependency closures, forbidden fixtures, and an external packaged-core consumer.
Each journey is also available as `native`, `browser`, `wasi`, `leptoskit`,
`gates`, or `consumer`. Run from this checkout; no secrets are needed.

All arguments are validated before a journey starts or writes evidence. Commands
accept only these positional arguments; no Cargo flags are forwarded:

| Command                                                                                   | Arguments                                           |
| ----------------------------------------------------------------------------------------- | --------------------------------------------------- |
| `native`, `browser`, `wasi`, `leptoskit`, `gates`, `dependency-graphs`, `consumer`, `all` | None                                                |
| `style`                                                                                   | Optional repository root; defaults to this checkout |
| `style-report`                                                                            | Required report JSON path                           |
| `imports`                                                                                 | Required Wasm artifact path                         |
| `dependency-fixture`                                                                      | Required dependency tree text path                  |

Paths resolve from this checkout. Prefix a path beginning with `-` with `./`.
Unsupported options and trailing arguments fail with usage before child execution.
To keep nested Cargo commands offline, set the inherited environment explicitly:

```sh
CARGO_NET_OFFLINE=true cargo xtask consumer
```

`cargo xtask consumer --offline` is rejected; xtask does not implement Cargo flag
propagation. Offline execution requires the dependencies to be cached already.

## Reproduce

The checked-in toolchain selects Rust 1.99.1, rustfmt, Clippy and all three Wasm
targets. Install `wasm-bindgen-cli` exactly 0.2.128 with `cargo install
wasm-bindgen-cli --version 0.2.128 --locked`. Set these executable paths:

```sh
export CHROME=/path/to/chrome
export CHROMEDRIVER=/path/to/matching/chromedriver
export WASMTIME=/path/to/wasmtime
export WASM_BINDGEN_TEST_DRIVER_TIMEOUT=30
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

## Current v27 introspection integration

The [introspection runtime receipt](introspection-runtime.md) records three
appended groups preserving all 30 public contracts. Every named host executes
111/111 canonical and 110/111 with sole hash corruption. Existing 108 bodies and
ordered report prefixes remain exact. The checked allowance is 6,765,000,000 per
entry, preserving the prior ordinary and crypto rates with zero added crypto
calls. The one-fuel zero-commit control, exact imports, graph negatives and five
packaged-core groups pass at signed executable
`051dd3a8ea739abe1fa00e2ea6a836520ba24709`.

## Historical v26 confirmation integration

The 108-case suite preserves every previous 105 assertion body and ordered row.
Three groups exercise public confirmation policy and event ordering, local Rc
across an observed Pending receive with finite awaited cleanup, and exact
submission bytes and effect exclusion. Actual native Rust 1.99.1, Chrome 155,
Wasmtime 50 Preview1 and Preview2, and pinned LeptosKit WasmHost execute 108/108
canonical and 107/108 under sole hash corruption. The one-fuel zero-commit
control, exact imports, closed dependency graphs and all five fresh packaged-core
groups pass at signed executable `6baff658d68997638c383b84ce85d0057c9a2778`.

The checked private allowance is 105 ordinary groups times 15,000,000 plus 257
authenticated transactions and 772 message crypto calls times 5,000,000, totalling
6,720,000,000 per entry. The three new groups add no crypto loop. Existing 256
distinct message and 256 cached transaction assertions remain unchanged. This
is finite source-owned accounting, not a measured minimum or speed claim.
The [confirmation receipt](transaction-confirmation.md) records the caller-owned
observation and bounded-cleanup contract. No actual timer backend, authenticated
nonce RPC adapter, subscription factory, validator or device journey is proved.

## Historical v25 Token integration

The 105-case suite preserves all 103 previous assertion groups and appends the
93-row Token literal matrix and six strict Token groups. Actual native Rust,
Chrome, both Wasmtime previews and the pinned LeptosKit host execute 105/105 and
104/105 under sole hash corruption. The one-fuel zero-commit control, closed
import/dependency checks and all five freshly packaged-core consumer groups pass.
The [Token runtime receipt](token-runtime.md) records the frozen executable and
source-owned 6,675,000,000-fuel allowance. Existing 256 distinct message and 256
cached transaction cases remain intact. The source authority-role, BOM and Batch
account differences remain explicit. This is off-chain fixture execution.

## Historical v24 message signer integration

The 103-case suite preserves every previous 101 function, ordered case and result.
Two isolated groups execute the accepted bare-message snapshots and the entire
256-distinct-message bulk signing and comparison oracle. Native Rust, Chrome,
both Wasmtime previews and the pinned LeptosKit host check 103 canonical passes
and 102/103 under sole hash corruption. Import/dependency negatives, the one-fuel
zero-commit control and five freshly packaged-core groups pass.
The [message runtime receipt](message-runtime.md) records the signed executable,
actual artifact hashes and finite source-owned crypto workload. This does not
complete signer parity or establish device, wallet submission or validator proof.

## Historical v23 durable nonce integration

The 101-case suite retains all 97 wallet and signer groups unchanged and appends
four accepted durable nonce groups. Native Rust, Chrome, both Wasmtime previews
and the actual pinned LeptosKit host execute 101 canonical passes and 100/101
under sole hash corruption. Exact imports, graph negatives, one-fuel zero-commit
and all five fresh packaged-core consumer groups pass.
The [nonce runtime receipt](nonce-runtime.md) records the signed executable,
artifact hashes and the checked 2,785,000,000-fuel workload. These are off-chain
construction and structural inspection fixtures; account authentication, nonce
expiry, validator acceptance and installed device journeys remain unproved.

## Historical v22 wallet and signer integration

The 97-case suite preserves all 90 wallet-frontier cases and appends the seven
accepted signer assertions unchanged. The checked private interpreter workload is
15,000,000 times 96 ordinary groups plus 5,000,000 times 257 authenticated batch
transactions: 2,725,000,000 per entry. The original 256 successful signatures,
257-item preflight rejection and ordered early stop remain intact.
The [signer runtime receipt](signer-runtime.md) separates historical 92-case
measurements from current combined execution.

Signed executable `66b90da63acd1efd90ce16853d71bc0fd1e55dde` passes a complete
fresh `cargo +1.99.1 xtask all`: native Rust, Chrome, both Wasmtime previews and
the actual pinned LeptosKit host check 97 canonical cases and 96/97 under sole
hash corruption. Exact imports, graph negatives, one-fuel zero-commit control
and five freshly extracted packaged-core consumer groups pass. Existing host,
wallet and public signer algorithms remain unchanged. This is off-chain fixture
execution, with no installed-device or real-wallet claim.

## Historical v21 wallet receipt

The wallet assertions are also integrated onto the current native WebSocket
frontier `5ce9e806906af2bdb1e3f16a7d2e8c78b525f3fd`. Every existing SDK, host,
guest and strict verification algorithm remains byte-identical to that frontier;
the wallet assertion sources and all 90 expected fixture rows remain identical
to the accepted wallet receipt below. Fresh native execution checks the combined
source. The earlier five-host result carries by exact executable source identity;
it is not a new combined five-host execution or installed-device proof.

Signed executable `e0cd5c16a2f011fe7db6f94c4e39afff2f7f3290` integrates frozen
wallet contracts ending at `2a7b7dc277d6062a7bc202bb83d70912f16efa88` into exact
parent `6bd0b0df9cc848c106d680f1980fc7b7f7494a0b`. All 85 previous case IDs,
order, outcomes and function sources remain unchanged. Five public assertion
groups from the isolated wallet verification crate append to the registry.

The full offline five-host run passes 90/90 canonical cases and 89/90 under sole
`sha256-fips-abc` corruption. Exact imports, graph negatives, five extracted
packaged-core consumer groups and the one-fuel zero-commit trap pass. The pooled
formula remains 15,000,000 times the case count, now 1,350,000,000 per entry.
The [wallet runtime receipt](wallet-runtime.md) records actual artifacts and
bounded contracts. All 203 wallet declarations remain partial. Per-asset upstream
license provenance remains pending. No real wallet, device or cryptographic
backend is inferred from deterministic fixtures.

## Historical v20 codec framing receipt

Signed executable `b7a449cfa84ad9379c568f6613abc71c1e618153` integrates exact
accepted final Memo/ATA parent `a976ca97eed9ff2b3a94d3d20313c8a50d0b05ef`
and the independently accepted codec framing delta ending at
`19e2c90b425707f560a9a54e0c147d56e5fc8c96`. The suite
`solana-kit-core-v20` preserves all first 83 case IDs, order and outcomes, then
appends the two unchanged accepted framing functions through their owning array.

The full offline five-host run passes 85/85 canonical cases and observes 84/85
under sole `sha256-fips-abc` corruption. Exact imports, graph negatives, all five
packaged-core consumer groups and the one-fuel zero-commit trap pass. The checked
pooled allowance remains 15,000,000 times the case count, now 1,275,000,000 per
entry. No individual-case or transactional rollback guarantee is inferred. The
[framing runtime receipt](codec-framing-runtime.md) records actual artifacts and
the accepted padding, resize-prefix and reversed-context differences. All 113
framing declaration IDs retain their partial or uncovered status and residuals.

## Historical v19 Memo and ATA receipt

Signed executable `2af1c02cc404e1d1ee6c3c5f298d3cfe898a9f5b` integrates exact
accepted parent `92a20937d0b7c5866ec072e7dff7d9b273204263` and handwritten
Memo/ATA clients from `5616186f62a795cf5b756fb9b677705f35dae3c9`. The suite
`solana-kit-core-v19` preserves all first 81 case IDs, order and outcomes, then
appends the two unchanged accepted portable functions in an owning array.

The full offline five-host run passes 83/83 canonical cases and observes 82/83
under sole `sha256-fips-abc` corruption. Exact imports, graph negatives, all five
packaged-core consumer groups and the independent one-fuel trap also pass. The
checked pooled allowance remains 15,000,000 times the case count, now
1,245,000,000 per entry. No individual-case or transactional rollback guarantee
is inferred. See the [Memo/ATA runtime receipt](memo-ata-runtime.md) for tools,
hashes, ordinary consumers, provenance and bounded handwritten-client limits.

## Historical v18 subscription receipt

Signed executable source `eff549b358c2e0f1ae635884b9cb67dc94b6c4ba` integrates
accepted final Compute Budget parent `87d04bc42af263d135ea078be30865511335f52e`,
corrected subscription source `24f8b8a11b5f0b449c0b7695abf136fd71f20fe7`, and
the independently accepted private fuel correction
`6e2e59e1af03e3bea2c3d2351a2d20a240f190bb`. The suite is
`solana-kit-core-v18`. Both reports preserve the first 77 case IDs, order and
result lines from the final Compute Budget parent, then append four
[subscription protocol groups](subscription-runtime.md).

On 2026-10-22, the full command with `CARGO_NET_OFFLINE=true` and
`WASM_BINDGEN_TEST_DRIVER_TIMEOUT=30` passed from that integrated source.
Runtime tools were Rust 1.99.1, Chrome 155.0.8059.85, ChromeDriver 155.0.8059.84,
Wasmtime 50.0.0, and the pinned actual LeptosKit host above.

| Actual host               | Canonical                   | Corruption control             |
| ------------------------- | --------------------------- | ------------------------------ |
| Native command            | 81/81, exit 0               | 80/81, exit 1                  |
| Real Chrome               | Full 81/81 report checked   | Full 80/81 report checked      |
| Wasmtime Preview 1        | 81/81, exit 0               | 80/81, exit 1                  |
| Wasmtime Preview 2        | 81/81, exit 0               | 80/81, exit 1                  |
| Actual LeptosKit WasmHost | Full 81/81 JSON, one commit | Full 80/81 JSON, second commit |

Only `sha256-fips-abc` fails under corruption. The actual interpreter also
requires a fresh one-fuel host to trap before any report commit. Its normal
finite allowance is checked `15,000,000 × EXPECTED_CASE_COUNT`, currently
1,215,000,000 per entry. This pooled allowance includes lazy translation and
the entire report. It is not an individual-case maximum. The measured final
guest requires 1,019,911,869 fuel. One less traps after a report commit, so a
near-completion trap does not establish rollback. See the
[private host fuel evidence](../../verification/leptoskit/README.md).

All assertions and the 32-cycle lifecycle fixture remain unchanged. The private
host alone uses an optimized build. No SDK default or acknowledgement deadline
changed. All nine subscription families execute complete in-memory text-frame
I/O. These results establish no WebSocket, validator, installed-app or device
networking support. The 173-ID subscription receipt remains 68 partial and
105 pending, with transport, pooling, ping, reconnect and confirmation work open.

The same command passes import/dependency policies, all forbidden controls and
the extracted packaged-core consumer's five groups. Guest imports remain
exactly `leptoskit::{node_create,node_set_props,commit}`. Executed artifact hashes
are relative to the owned checkout.

```text
546750535be05157773eecb2984ac5e6120de238dd814c90ece524504cf52cf5  target/wasm32-wasip1/debug/solana_kit_conformance.wasm
629a2a8207c452fb14fed23ff146ed9b82e2cdf2c49dc6bde62349f9a085a38d  target/wasm32-wasip2/debug/solana_kit_conformance.wasm
2e35bfc7f379c33f76eadc8adf85fb669b14c13a75e7fe66afbcbfc08aec0a44  target/wasm32-unknown-unknown/release/solana_kit_conformance_guest.wasm
```

## Historical v17 Compute Budget receipt

Signed executable source `cedc862fa559cb5430c10e8fbc9bb4c7a4d6aedf` merges the
accepted offchain parent `87b1816163fa4c7b0e6fe82e1619a5d1d962c462` and executes
`solana-kit-core-v17`. Its first 75 case IDs, order and canonical/corruption
report bytes match that final parent. It appends
`compute-budget-full-width-price` and `compute-budget-ordering-historical`.
The fixture owner returns these two entries as an ordered array.

On 2026-10-22, `CARGO_NET_OFFLINE=true cargo +1.99.1 xtask all` passed using warm
exclusive targets after the final parent merge. Rust 1.99.1, Chrome
155.0.8059.85, ChromeDriver 155.0.8059.84, Wasmtime 50.0.0 and the pinned actual
LeptosKit host above executed the suite. Only the driver startup timeout was
set to 30 seconds.

| Actual host               | Canonical                   | Corruption control             |
| ------------------------- | --------------------------- | ------------------------------ |
| Native command            | 77/77, exit 0               | 76/77, exit 1                  |
| Real Chrome               | Full 77/77 report checked   | Full 76/77 report checked      |
| Wasmtime Preview 1        | 77/77, exit 0               | 76/77, exit 1                  |
| Wasmtime Preview 2        | 77/77, exit 0               | 76/77, exit 1                  |
| Actual LeptosKit WasmHost | Full 77/77 JSON, one commit | Full 76/77 JSON, second commit |

Only `sha256-fips-abc` fails in corruption mode. The two appended assertion
functions retain their accepted bodies. All five concrete Compute Budget
codecs, builders, parsers and corrected instruction-list helpers remain exact
to the accepted source. Historical RequestUnits tag0 is wire support and does
not establish modern validator acceptance. The
[Compute Budget receipt](compute-budget.md) retains all 84 source IDs with
61 implemented and 23 partial. All 55 general renderer IDs remain pending.

All 26 private generator tests pass. Unchanged independent Rust 1.89 consumers
execute 8,192 literal helper cases, 34 original Dart vectors, complete and sparse
dispatch, full enums, empty/numeric layouts and linked types. The original
6,656-case Dart matrix remains applicable through exact helper-source equality.
The generator remains bounded to the documented finite layouts.

The full host command also passes compiled-import and selected dependency
policies, their negative controls, and all five extracted packaged-core consumer
groups. Guest imports remain exactly `leptoskit::{node_create,node_set_props,commit}`.
The independent Compute Budget normal/build closure is program, Kit core and
bs58 0.5.1 on native and all three Wasm targets.

Executed artifact SHA-256 values follow. Paths are relative to the owned checkout.

```text
6031c7c8a154a7ac428f811edfa64532b4b91da792a78e8b89c4c8023555bcc5  target/wasm32-wasip1/debug/solana_kit_conformance.wasm
dada23b048f8c7a1cea4b0d646b85861bd852053b181c215e6c609f8f3eb8703  target/wasm32-wasip2/debug/solana_kit_conformance.wasm
1aed9322ffc081acb0990fae564b8a32b0b8ea1b4b234ec73fcae73941331753  target/wasm32-unknown-unknown/release/solana_kit_conformance_guest.wasm
```

This is off-chain application-client evidence. It establishes no complete phase,
validator, wallet, installed app, mobile device or registry release readiness.
The following receipts retain their historical counts and artifacts.

## Historical v16 offchain message receipt

Signed executable source `1fd7a56f94b0589517c7586c012b23fedff0756b` executes
`solana-kit-core-v16`. It preserves the corrected history parent's first 72 case
IDs, order and report bytes and appends `offchain-valid-wire`,
`offchain-malformed-bounds`, and `offchain-signature-tamper`. The accepted final
history parent `a0c2520b27db369f9c5f2e65afbbf15de755d6bd` adds documentation and
ancestry only over its executed source. No executable source or manifest changes
were needed for that final additive merge.

On 2026-10-22, `cargo +1.99.1 xtask all` passed with fresh owned targets using
Rust 1.99.1, Chrome 155.0.8059.85, ChromeDriver 155.0.8059.84, Wasmtime 50.0.0,
and the pinned actual LeptosKit host above. Only the driver startup timeout was
set to 30 seconds.

| Actual host               | Canonical                   | Corruption control             |
| ------------------------- | --------------------------- | ------------------------------ |
| Native command            | 75/75, exit 0               | 74/75, exit 1                  |
| Real Chrome               | Full 75/75 report checked   | Full 74/75 report checked      |
| Wasmtime Preview 1        | 75/75, exit 0               | 74/75, exit 1                  |
| Wasmtime Preview 2        | 75/75, exit 0               | 74/75, exit 1                  |
| Actual LeptosKit WasmHost | Full 75/75 JSON, one commit | Full 74/75 JSON, second commit |

Only `sha256-fips-abc` fails in corruption mode. The three appended functions
retain their accepted isolated bodies without algorithm changes. The
[offchain receipt](../parity/offchain-messages.md) records immutable V0/V1
messages, envelopes, exact signing bytes and both documented V0 differences.
All 83 declaration IDs remain partial, with none complete.

The unchanged independent Rust 1.89 consumer passes with default features and
crypto after a dependency manifest path adaptation only. Every source, lock and
original Dart oracle byte remains exact. All four message, unsigned-envelope,
signed-envelope and deterministic signature fixtures match. The default core
normal/build closure remains core plus bs58. The full host command also passes
selected dependency/import policies, negative controls and all five packaged-core
consumer groups. Guest imports remain exactly `leptoskit::{node_create,node_set_props,commit}`.

Executed artifact SHA-256 values follow. Paths are relative to the owned checkout.

```text
2ada3e01d30b284d70865eb41db88079a8818d711383cb9bd62d8e7edc5f8dfa  target/wasm32-wasip1/debug/solana_kit_conformance.wasm
a4daefce416082e4a646aa18daf707d1608b87ae9b037033c2b6283d234bb217  target/wasm32-wasip2/debug/solana_kit_conformance.wasm
9f8946aa4b2c7353107568de367c535931b6aafacfc0ff67fcd744b6b3ef9ae9  target/wasm32-unknown-unknown/release/solana_kit_conformance_guest.wasm
```

This proves synchronous offchain-client execution. It establishes no complete
phase, validator, installed app, mobile device or registry release readiness.
The following receipts retain their historical counts and artifacts.

## Historical v15 complete RPC catalog receipt

The `solana-kit-core-v15` suite preserves the first 68 case lines and appends
four history/submission/full-result/address/simulation groups. Clean signed source
`f4e0364bbd900912a677277e9ee5853d3994c6d9` executes all 54 catalog methods through
the public Client. Native Rust, Chrome, both Wasmtime previews and the actual
pinned LeptosKit host each check 72 canonical passes and the sole 71/72
corrupt-hash result. `cargo +1.99.1 xtask all` also passes its existing import,
dependency and five packaged-core consumer gates.

The [complete catalog runtime receipt](rpc-history.md) records artifact hashes,
literal requests, typed values, default caps and failure controls. The 146-ID
history ledger remains 130 implemented and 16 pending, and the account/token
100-ID ledger is unchanged. All 398 canonical RPC declaration IDs remain intact.
Method coverage does not establish complete declaration, network, validator,
device, registry or Phase 4 support.

## Historical v14 account and token RPC receipt

The `solana-kit-core-v14` suite preserves the first 65 case lines and appends
three account/token groups. Clean signed source
`f7f1107a8770bba12544aac10586ebefcdd33438` executes all nine new methods and richer
accountInfo through the public Client and existing memory fixture. The public
catalog at that checkpoint implemented 39 of 54 methods, leaving 15 pending.

The [account/token receipt](rpc-account-token.md) records actual host results,
full-width values, literal requests, malformed controls and representation limits.
The exact 100-ID account/token ledger retains 88 typed, four Debug partial,
four equality pending and four Hash pending entries. These are protocol
executions, with no new networking, device, validator, registry or complete-phase claim.

## Historical v13 economics RPC receipt

The `solana-kit-core-v13` suite preserves the first 62 case lines and appends
three economics groups. Clean signed source
`42d89dedcd5bed621fecf9dba440d499b1c1e95a` executed
`cargo +1.99.1 xtask all` on 2026-10-22. All eight economics methods execute
literal requests and typed responses through the public client and the existing
memory transport fixture. The client now implements 30 of the 54 catalog methods.

Native Rust, Chrome, Wasmtime Preview 1 and Preview 2, and the pinned actual
LeptosKit host each check all 65 canonical passes and the 64/65 corruption result.
Only `sha256-fips-abc` fails. Graph and compiled-import controls remain unchanged
and pass, as do all five packaged-core consumer tests. The unchanged independent
economics consumer also passes all five groups on Rust 1.89 without repository
flags or patches. The [economics receipt](rpc-economics.md) records exact
requests, malformed/config boundaries, versions, artifact hashes and limits.
All 42 economics declaration IDs retain their 39 implemented, two partial and
one pending statuses. This protocol execution adds no networking, device,
validator, registry or complete-phase claim.

## Historical v12 composite codec receipt

The `solana-kit-core-v12` suite preserves the first 59 case lines and appends
three existing composite groups. Clean signed source
`1be097b26d0c389b5f710c428d3fc5e93d0e3bfa` executed `cargo +1.99.1 xtask all`
on 2026-10-22. The added groups check typed padded composition, backward bits,
allocation exhaustion, invalid choice tags and sentinel overlap rejection.
The [composite receipt](composite-codecs.md) retains the bounded contracts and
exact 235-ID residual set. Existing numeric, transaction, System and RPC groups
remain in their original order.

| Actual host               | Canonical                   | Corruption control             |
| ------------------------- | --------------------------- | ------------------------------ |
| Native command            | 62/62, exit 0               | 61/62, exit 1                  |
| Real Chrome               | Full 62/62 report checked   | Full 61/62 report checked      |
| Wasmtime Preview 1        | 62/62, exit 0               | 61/62, exit 1                  |
| Wasmtime Preview 2        | 62/62, exit 0               | 61/62, exit 1                  |
| Actual LeptosKit WasmHost | Full 62/62 JSON, one commit | Full 61/62 JSON, second commit |

Rust 1.99.1, Chrome 155.0.8059.85, ChromeDriver 155.0.8059.84, Wasmtime 50.0.0
and the pinned actual LeptosKit host above performed these runs. Only
`sha256-fips-abc` fails in corruption mode. All unchanged 32 core normal/build
selections, 16 numeric normal-only queries, four System queries, four client
queries and their negative controls pass. The compiled guest still imports
exactly `leptoskit::{node_create,node_set_props,commit}`. All five extracted
packaged-core consumer tests pass. No manifest, feature, lock, dependency policy
or package owner changed.

```text
8b266b31d49f0a93366bd90c19c6a9d6714f5d219dd1a2d02b8c3e2762a93a6a  wasm32-wasip1
d20728c8f50bce853452192ca867613babf132a102bdd3a50bbf35cb6af2af3c  wasm32-wasip2
b45f81aa806cd69697ef509b2c40bdf381e7e589f7c22bc856f90a5d81b40786  leptoskit-guest
```

An ordinary external Rust 1.89 consumer replays the unchanged independent five
groups. They exercise 64,386 binary payload/delimiter pairs, 1,061 freshly
reproduced original-Dart rows and allocation rejection guards. Its lock and
assertions remain unchanged. The independent accounting auditor rechecks all
244 IDs, nine implemented-bounded, 24 partial and 211 uncovered, exact 235-ID
residual membership and all 37 defining-source digests. This execution proves
bounded off-chain client behavior, not complete composite parity, networking,
validator support or installed-device journeys.

## Historical v11 numeric and text codec receipt

The `solana-kit-core-v11` suite preserves the first 56 case lines and appends
three numeric and text groups. Clean signed source
`370c60218f924655b5b2da13d9d13b060bc31f46` executed `cargo xtask all` on 2026-10-22.
The groups check leading-zero radix conversion, odd hexadecimal tails, configured
UTF-8, exact signed and unsigned 256-bit text, invalid radix and symbol rejection,
allocation limits, tolerant Base64 padding, and invalid unused-bit rejection.
The [numeric and string receipt](../parity/numeric-string-codecs.md) retains all
88 canonical IDs, including seven partial capabilities.

| Actual host               | Canonical                   | Corruption control             |
| ------------------------- | --------------------------- | ------------------------------ |
| Native command            | 59/59, exit 0               | 58/59, exit 1                  |
| Real Chrome               | Full 59/59 report checked   | Full 58/59 report checked      |
| Wasmtime Preview 1        | 59/59, exit 0               | 58/59, exit 1                  |
| Wasmtime Preview 2        | 59/59, exit 0               | 58/59, exit 1                  |
| Actual LeptosKit WasmHost | Full 59/59 JSON, one commit | Full 58/59 JSON, second commit |

Rust 1.99.1, Chrome 155.0.8059.85, ChromeDriver 155.0.8059.84, Wasmtime 50.0.0,
and the pinned actual LeptosKit host above performed these runs. Only
`sha256-fips-abc` fails in corruption mode. All 32 core normal/build selections,
16 numeric normal-only queries, four System queries, four client queries and
their negative controls pass. The core controls reject Tokio under all eight
feature policies and Base64 leakage under all four policies without text codecs.
Default core still selects only `bs58`. Optional `text-codecs` adds `base64`.
The separate System and client policies retain their reviewed closures. Compiled
guest imports remain exactly `leptoskit::{node_create,node_set_props,commit}`.
All five extracted packaged-core consumer tests pass.

```text
7e56e1a969a705c64b76af2d6238dd4cab3dba6ff8b4573e5c74e12cc9e4ce40  wasm32-wasip1
6bd6ae7d162a4e1f78582b2d726a0e3ae022f84ccf997ef1083cc7227389ecbf  wasm32-wasip2
218b6b0918a8355c8c7d8469a655d72e991224f6ce49d14a3ae6e008fa6a44a1  leptoskit-guest
```

An ordinary external Rust 1.89 consumer also executes the unchanged independent
19,158-row Dart oracle with zero mismatches. The separately reviewed standalone
NUL-removal correction preserves every other scalar, including repeated BOMs.
This receipt establishes bounded off-chain client behavior. It does not establish
installed-device journeys, complete numeric or string parity, or on-chain support.

## Historical v10 node RPC receipt

The `solana-kit-core-v10` suite preserves the first 53 case lines and appends
three public-client node protocol groups. Clean signed source
`96494782927020a2708732abd31c56570139663e` executed `cargo xtask all` on 2026-10-22.
Every one of the nineteen node methods executes with a literal expected request
and an asserted typed response. The [node receipt](rpc-node.md) records scope,
source identity and the twelve remaining Dart display/hash declarations.

| Actual host               | Canonical                   | Corruption control             |
| ------------------------- | --------------------------- | ------------------------------ |
| Native command            | 56/56, exit 0               | 55/56, exit 1                  |
| Real Chrome               | Full 56/56 report checked   | Full 55/56 report checked      |
| Wasmtime Preview 1        | 56/56, exit 0               | 55/56, exit 1                  |
| Wasmtime Preview 2        | 56/56, exit 0               | 55/56, exit 1                  |
| Actual LeptosKit WasmHost | Full 56/56 JSON, one commit | Full 55/56 JSON, second commit |

Rust 1.99.1, Chrome 155.0.8059.85, ChromeDriver 155.0.8059.84, Wasmtime 50.0.0,
and the pinned actual LeptosKit host above performed these runs. Only
`sha256-fips-abc` fails in corruption mode. All 16 core, eight numeric normal-only,
four System and four client graph queries and their negative controls pass.
Client retains exactly thirteen reviewed external package/version pairs. Compiled
guest imports remain exactly `leptoskit::{node_create,node_set_props,commit}`.
All five extracted packaged-core consumer tests pass.

```text
da12d010d4abf9fe538cddb334dc05b0cfdc3ed880da4e576148a44eb8b0b419  wasm32-wasip1
9b20a390f9acc7e8b7fec4705af963db295a98380a3948846d85ff8f8505e29c  wasm32-wasip2
1a2fed08f01535285acf851725e71fe0c447d319b4f99e1bc4af89bb1a9a8524  leptoskit-guest
```

These are memory-transport protocol fixtures. Network adapters, installed-device
journeys, BLS certificate verification and complete catalog parity are separate.
All code remains exclusively for off-chain application clients.

## Historical v9 System client receipt

The `solana-kit-core-v9` suite preserves the first 51 case lines and appends the
existing System builder/parser and nonce snapshot groups. Clean signed source
`ef8c553bcd1a349ee937a45644d1fa6e8fc31558` executed `cargo xtask all` on 2026-10-22
with Rust 1.99.1, Chrome 155.0.8059.85, ChromeDriver 155.0.8059.84, Wasmtime 50.0.0,
and the same pinned actual LeptosKit host. System production and private generator
source equal reviewed `034ec2bcf2e4724982157b7b2ef09da5e80ab8be`.
The native adapter from signed parent `92ab00b9f2fb3cabba75dd573338012a05faff43`
is present without changing the portable client or guest graphs.

| Actual host               | Canonical                   | Corruption control             |
| ------------------------- | --------------------------- | ------------------------------ |
| Native command            | 53/53, exit 0               | 52/53, exit 1                  |
| Real Chrome               | Full 53/53 report checked   | Full 52/53 report checked      |
| Wasmtime Preview 1        | 53/53, exit 0               | 52/53, exit 1                  |
| Wasmtime Preview 2        | 53/53, exit 0               | 52/53, exit 1                  |
| Actual LeptosKit WasmHost | Full 53/53 JSON, one commit | Full 52/53 JSON, second commit |

Only `sha256-fips-abc` fails in the control. The two appended groups execute
duplicate positional accounts, the strict signer-role parser, exact initialized
nonce fields and full-width fees, and rejection of an invalid nonce state.
All preceding core and RPC assertions still execute. The native adapter's real
HTTP and TLS proof remains in its [separate receipt](native-http.md).
This portable suite uses memory RPC transports and performs no network calls.

All 16 core graph queries, eight numeric normal-only queries, four System queries,
four client queries, and their dependency and import negative controls pass.
The System graph remains System, core, and bs58. Compiled guest imports remain
exactly `leptoskit::node_create`, `leptoskit::node_set_props`, and
`leptoskit::commit`. All five extracted packaged-core consumer tests pass.

Executed artifact SHA-256 values:

```text
b9807e35a80afb495ee04cfedddae13febfa0ec659a2cec955539f2eed733019  wasm32-wasip1
d5c5ac663f5e4b100621d1e0feb625d566c536f8ed5141197eda8db5bafbd21d  wasm32-wasip2
7a27798cb44a6a66e2f72e612dbbddd75ae5dc369813133038e6e4af82147d74  leptoskit-guest
```

The [System client receipt](system-client.md) retains all 68 partial or pending
System declarations. Validator, durable nonce transaction, wallet, generated app,
Android, iOS, and registry-release verification remain separate work.
All crates are exclusively for off-chain application clients.

## Historical v8 public RPC protocol receipt

The `solana-kit-core-v8` suite preserves the first48 case lines and appends three
public-client protocol groups. Signed clean source
`709ec9588865df8dd55362412891c234295a72a7` executed `cargo xtask all` on2026-10-22
using Rust 1.99.1, Chrome 155.0.8059.85/driver155.0.8059.84, Wasmtime 50.0.0 and the
same pinned actual LeptosKit WasmHost. Public client source/tests equal accepted
`6dc82ec165293b8a61007357d16d24f2e06ed510`; core production equals the V1 parent.

| Actual host               | Canonical                  | Corruption control            |
| ------------------------- | -------------------------- | ----------------------------- |
| Native command            | 51/51, exit0               | 50/51, exit1                  |
| Real Chrome               | Full51/51 report checked   | Full50/51 report checked      |
| Wasmtime Preview1         | 51/51, exit0               | 50/51, exit1                  |
| Wasmtime Preview2         | 51/51, exit0               | 50/51, exit1                  |
| Actual LeptosKit WasmHost | Full51/51 JSON, one commit | Full50/51 JSON, second commit |

Only `sha256-fips-abc` fails in the control. The three appended groups execute
exactgetBalance request/context/u64MAX, bounded opaque1e400/error and strict
escaped-duplicate/schema checks, and concrete localRc use after observedPending
and wake, abandonment and correct correlation. One shared fixture owner serves
private/native and all guest runners. These are pure protocol/memory transports;
this receipt does not prove HTTP networking, browser fetch, host I/O or devices.

All16 core graph queries,8 numeric normal-only queries,4 System queries and their
actual Tokio controls remain. Four additional native/Wasm normal+build queries
use a separate exact-version closed client policy of13 external packages plus
owned client/core. Its actual Tokio negative fails. No core/System allowlist was
widened. Actual compiled guest imports remain exactly LeptosKit node_create,
node_set_props and commit. All5 clean packaged-core consumer tests pass.

Executed artifact SHA-256 values:

```text
1a4c70f8199f7ec4adb0c6916178269c0ad3e22be09696fb2e5b5d6bf06a9523  wasm32-wasip1
0a20813ff27fb347e4e8246e5da3f42f15c3913f660b8a3ef96e8645503287f3  wasm32-wasip2
2ab453111ab245d1578502ffeef61b86dc8425cec13af3259f7dba956196260e  leptoskit-guest
```

Raw per-operation stdout/stderr/status stay in ignored target/runtime-evidence.
The client intentionally uses std; core still supports no_std+alloc. This is
exclusively off-chain client code. On-chain programs use Pina or official Solana
program libraries. Remaining51 RPC operations, production adapters, subscriptions,
wallets, devices and registry-release preparation are not completed by this proof.

## Historical refreshed v7 executed receipt

After the fixed-point presentation correction and signed integration of merged
main `b74b8f893ba38c10cb208dfeadebb49ed7473d5e`, the unchanged 48-case
`solana-kit-core-v7` suite was freshly executed on 2026-10-22 at signed source
checkpoint `d3a3f5373dfce86d05f0431abddd25431af7fdff`. Rust 1.99.1,
Chrome 155.0.8059.85, ChromeDriver 155.0.8059.84, Wasmtime 50.0.0 and the
actual pinned LeptosKit host all executed through `cargo xtask all`, which exited 0.

| Execution                 | Canonical                      | Corruption control                                     |
| ------------------------- | ------------------------------ | ------------------------------------------------------ |
| Native command            | 48/48, exit 0                  | 47/48, exit 1                                          |
| Real headless Chrome      | Full 48/48 report checked      | Full 47/48 report checked; both browser tests passed   |
| Wasmtime Preview 1        | 48/48, exit 0                  | 47/48, exit 1                                          |
| Wasmtime Preview 2        | 48/48, exit 0                  | 47/48, exit 1                                          |
| Actual LeptosKit WasmHost | Full 48/48 JSON and one commit | Press event produced full 47/48 JSON and second commit |

Only `sha256-fips-abc` fails in corruption mode. All case assertions, their
order, goldens and V1 production source are unchanged from the initial v7
receipt below. Core fixed-point production matches corrected parent
`ce571db4ae71ce22910d5cf3f4c822a215308431`; its five finite-presentation
regressions execute separately natively and on MSRV 1.89, not as new portable cases.
The V1 packet limits remain 4096 bytes inclusive, with legacy/v0 limited to 1232.

All 16 core normal/build feature-target queries, eight numeric normal-only
queries, four separate System graph queries and their concrete negative controls
pass unchanged. The guest imports exactly `node_create`, `node_set_props` and
`commit` from LeptosKit. All five extracted packaged-core consumer tests pass.

Fresh executed artifact SHA-256 values:

```text
2fcb4eba2dd1c4448075dfe9c9b7da708b51e48fdee5a4a0f9f95443faa6799f  wasm32-wasip1
57867865b48f96ac137437ce3229754da6337b6652f976c803e0850cda9f13c7  wasm32-wasip2
30a217c710408551effed354fde26c50bd5c51ad28f97ffd75d76df0e9776c45  leptoskit-guest
```

This remains bounded synchronous off-chain client evidence. On-chain programs
must use Pina or official Solana program libraries. The initial v7 limits and
separate local-validator evidence below remain applicable; no validator was
restarted merely to repeat unchanged V1 source.

The preceding corrected-math run at signed checkpoint
`5d9bf3e4c1bc5fa06e051745737ec1e65d0c30ad` also executed all 48/47 outcomes
and gates successfully, producing the same three artifact hashes. The fresh
run above incorporates the merged complete-coverage style tooling and TLS/WS
fixture without changing the V1 or portable conformance source. Its earlier
receipt remains in the signed history.

## Historical initial v7 executed receipt

The integrated `solana-kit-core-v7` suite preserves all preceding 45 cases and
appends three V1 groups. Execution on 2026-10-22 at signed source checkpoint
`3d04ed4daa814b9907dbcaccbd4eb0393229a3bb` used Rust 1.99.1,
Chrome 155.0.8059.85, ChromeDriver 155.0.8059.84, Wasmtime 50.0.0 and the actual
pinned LeptosKit host. `cargo xtask all` exited 0.

| Execution                 | Canonical                      | Corruption control                                     |
| ------------------------- | ------------------------------ | ------------------------------------------------------ |
| Native command            | 48/48, exit 0                  | 47/48, exit 1                                          |
| Real headless Chrome      | Full 48/48 report checked      | Full 47/48 report checked; both browser tests passed   |
| Wasmtime Preview 1        | 48/48, exit 0                  | 47/48, exit 1                                          |
| Wasmtime Preview 2        | 48/48, exit 0                  | 47/48, exit 1                                          |
| Actual LeptosKit WasmHost | Full 48/48 JSON and one commit | Press event produced full 47/48 JSON and second commit |

Only `sha256-fips-abc` fails in corruption mode. V1 fixtures execute literal
message-first encoding, full-width inline priority fees, half-mask rejection,
inclusive 4096-byte envelope success, 4097-byte rejection and actual signature
tampering inside every host, including the 32-bit guests. Legacy/v0 retain their
1232-byte cap. Production V1 source is unchanged from the separately reviewed
official-oracle and local-validator proof; integration only extends fixture bounds.

All 16 core normal/build feature-target queries, eight numeric normal-only
queries, four separate System graph queries and their concrete negative controls
pass unchanged. The guest imports exactly `node_create`, `node_set_props` and
`commit` from LeptosKit. All five extracted packaged-core consumer tests pass.

Executed v7 artifact SHA-256 values:

```text
c43c52d33533eece95b27215010cb18b1a8cc60fc682e8cb90c7a28d7f399eb0  wasm32-wasip1
19158ac6f403ec4f351e2c82ab8625261d13fad7c408996bf46a0f781706da49  wasm32-wasip2
ec95a1904fd66a87760b26a2fd3f450cafc510f42a5029d875555930cb17dcc7  leptoskit-guest
```

This is bounded synchronous off-chain client behavior. It does not prove
durable-nonce lifecycle, public networking adapters, wallets, generated mobile
applications, remote node feature activation or complete Phase 3 parity. The
[local validator receipt](v1-wire.md) separately records simulation, submission,
confirmation, exact packet boundaries and its actual RPC-listener limitation.

## Historical v6 executed receipt

The integrated `solana-kit-core-v6` suite preserves all preceding 42 cases and
appends three generated System data cases. Local macOS ARM64 execution on
2026-10-22 at signed checkpoint `4fa22d04fd0a00e115e4ea45ec1d09ca3cc1cb57`
used Rust 1.99.1, Chrome 155.0.8059.85, ChromeDriver 155.0.8059.84,
Wasmtime 50.0.0 and the pinned actual LeptosKit host. Clean source and native
goldens were used for `cargo xtask all`, which exited 0.

| Execution                 | Canonical                      | Corruption control                                     |
| ------------------------- | ------------------------------ | ------------------------------------------------------ |
| Native command            | 45/45, exit 0                  | 44/45, exit 1                                          |
| Real headless Chrome      | Full 45/45 report checked      | Full 44/45 report checked; both browser tests passed   |
| Wasmtime Preview 1        | 45/45, exit 0                  | 44/45, exit 1                                          |
| Wasmtime Preview 2        | 45/45, exit 0                  | 44/45, exit 1                                          |
| Actual LeptosKit WasmHost | Full 45/45 JSON and one commit | Press event produced full 44/45 JSON and second commit |

Only `sha256-fips-abc` fails in corruption mode, with `SHA-256 digest mismatch`.
The System cases execute independent 52/94-byte literals, UTF-8 byte framing,
unknown tags, trailing bytes, insufficient budgets, oversized u64 string lengths
and truncated strings. These negative cases execute inside the 32-bit guests.

All 16 core normal/build selections and eight numeric normal-only queries remain
passing. Four additional System default normal/build queries independently
require exactly `solana_kit_system`, `solana_kit_core` and `bs58`, across native
and all three Wasm targets. Its separate production checker rejects the Tokio
fixture; no core allowlist was widened. The compiled guest imports exactly the
three pinned LeptosKit functions, and the existing forbidden-import and four
core dependency controls pass. All five packaged-core consumer tests pass.

Executed v6 artifact SHA-256 values (local debug WASI artifacts, release guest):

```text
93e9f7d7e0223ce02b59aac1be3527035876a37a358f47f6351b554e1fe59eab  wasm32-wasip1
acfdb71cce02272d2bf018fd7434e4474a27a19814f3c3529654489ec9f0230f  wasm32-wasip2
6c0e1542a29ff9a44ab99baa8d41052f759bc61f56fa5168a647df310e3601fb  leptoskit-guest
```

Core production matches the corrected fixed-point parent
`ce571db4ae71ce22910d5cf3f4c822a215308431`. Its five finite-presentation
regression tests also execute natively and on MSRV 1.89; they are separate from
the 45 portable cases. The reviewed generated System source/schema/emitter and
all portable case assertions are unchanged. The [System receipt](system-data.md)
retains all 241 canonical IDs: 83 implemented data mappings, 28 partial and 130 pending.
This proves bounded synchronous off-chain data behavior; it does not complete
System builders, validator execution, the full program client or device journeys.
System source-consumer checks use no patches; clean-registry packaging remains
unproved while its core dependency is unpublished. Earlier receipts stay historical.

## Historical v5 executed receipt

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
are inspected for native, plain Wasm, Preview 1, and Preview 2, with and without
crypto. A closed crate-name allowlist rejects Tokio and other new dependencies;
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
