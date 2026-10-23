# Portable instruction planning and packer progress

The maintained suite executes 162 canonical cases and observes only the intended
SHA-256 corruption in its 161/162 negative run on native Rust, Chrome 155,
Wasmtime 50 Preview 1 and Preview 2, and the pinned LeptosKit `WasmHost` interpreter.
All previous 161 labels, their order and outcomes remain unchanged. The actual
reports are [canonical](../../verification/conformance/evidence/native-success.txt)
and [corrupt](../../verification/conformance/evidence/native-corruption.txt).

Executed corrected code has source tree `d0995f298cf3e9d8d3aad9949fba92bbe783f7ec`.
The later receipt update changes no Rust source, fixtures, dependencies or pins.

## Successful packing requires appended work

A custom packer must append at least one instruction on every successful call,
including its final call. The shared validator returns `NoProgress` when the
instruction list is unchanged, even if the packer becomes done. This prevents
an empty transaction or silently omitted packed work. Prefix and metadata
violations retain `InvalidPackerOutput`; invalid compiled output retains its
non-retryable `PackedMessage` error.

One new portable body checks synchronous planning, asynchronous planning and
append, both with an existing instruction and with an empty candidate. Its
controls retain unchanged unfinished output, metadata and prefix rejection,
valid completed appends, and zero calls to already completed packers. Public
results preserve the expected instructions and lifetime. The exact body and its
helpers match their native source after the explicit import and report shims.

## Preserved contracts

The [runtime map](../../verification/instruction-plans/runtime-map.tsv) retains
all 48 original bodies. All 43 original host-neutral bodies execute on each host,
preserving 184 ordered assertion sites and their literal predicates. The new body
adds 12 assertion sites, bringing portable planning coverage to 44 bodies.

The existing contracts cover exact wire, lookup, signature, lifetime and account
order; 1232/1233 and 4096/4097 packets; 64/65 instructions; ordinary capacity retry;
original errors; consumed proposals; transactional trees; repeated Pending polls;
registered wakes; and ownership on drop. Local fixture repolling requires an
actual registered wake and has a finite 1024-poll bound.

Five native-only bodies execute actual Tokio spawn and Send checks in every
compiler and feature cell. Two use Ready operations inside spawned tasks; three
exercise Pending operations. They remain excluded from portable adapters.

## Executed verification

| Producer or host                                            | Executed result                                                      |
| ----------------------------------------------------------- | -------------------------------------------------------------------- |
| Rust 1.99.1 and actual 1.89.0, default                      | 308 ordinary tests plus 5 docs per compiler                          |
| Rust 1.99.1 and actual 1.89.0, all features                 | 329 ordinary tests plus 7 docs per compiler                          |
| Ordinary outside consumer, both compilers and feature modes | 27 tests per cell                                                    |
| Native, Chrome, both Wasmtime previews and actual LeptosKit | 162/162 canonical; sole SHA corruption 161/162                       |
| Host fuel accounting                                        | All 3 tests pass; one-fuel interpreter trap before any report commit |

The exact old production source accepts a 136-byte transaction candidate with no
instructions when a custom packer reports completion without appended work. Its
new public contract test fails on incorrect success. A fresh source mutation
that restores the old completion exception also compiles and fails the new test.
The fixed source passes. Compiler failures and Cargo Fresh proxies do not count
as mutation sensitivity. Earlier progress-removal, updater-bypass and
consumed-proposal controls retain their historical receipts; this unit does not
claim new executions of those mutations.

The final namespace correction uses an explicit `Vec` import. Native algorithms
and tests are unchanged by that correction; their sole accompanying core change
clarifies the `NoProgress` documentation. Corrected workspace and private-host
Clippy and docs pass with warnings denied. Formatting, ordinary read-only
Monochange checks, import/dependency gates, and all authored/generated style
floors pass. Earlier failed style and ambiguous-import checks remain retained.

Actual browser, plain guest and both WASI byte streams use private rustc 1.99.1
`58cf8f9c3`. Child commands and preserved artifact bytes bind the producer to the
executed source. The plain guest retains exactly LeptosKit `commit`,
`node_create` and `node_set_props` imports. No MSRV Wasm execution is claimed.

The checked allowance is 10,395,000,000. The 44 ordinary planning groups contribute
660,000,000. Existing 573 MWA, 257 transaction and 772 message crypto units remain unchanged.
Rates remain 15,000,000 per ordinary group and 5,000,000 per crypto unit. This is fixed-fixture
host accounting, not a production performance or minimum-fuel guarantee.

## Limits

All [212 declaration rows](../../verification/instruction-plans/canonical.tsv)
remain partial. This proof does not supply implicit network or resource factories,
timers, automatic cancellation, dynamic Dart coercions, full package parity,
validator coverage or installed desktop/mobile journeys. Callbacks own external
effects already performed. Depth 512 and width 512 remain bounded probes;
recursive Plan operations and drop do not promise arbitrary-depth stack safety.

This SDK is exclusively for off-chain application clients. On-chain programs
should use Pina or official Solana program libraries.
