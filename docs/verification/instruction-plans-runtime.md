# Portable asynchronous instruction planning

The maintained suite executes 161 canonical cases and observes only the intended
SHA-256 corruption in its 160/161 negative run on native Rust, Chrome 155,
Wasmtime 50 Preview 1 and Preview 2, and the pinned LeptosKit `WasmHost` interpreter.
The previous 118 case labels, order and outcomes are unchanged. The actual reports
are [canonical](../../verification/conformance/evidence/native-success.txt) and
[corrupt](../../verification/conformance/evidence/native-corruption.txt).

Executed code has source tree `a569576979420bbb76dd8e0dba94c60ffeee898e`.
Later changes update this receipt and the declaration auditor only. The source
retains the current SIWS camelCase correction and native artifact-capture tooling.

## Preserved contracts

The [runtime map](../../verification/instruction-plans/runtime-map.tsv) classifies
all 48 original bodies. All 43 host-neutral bodies execute on each host, preserving
184 ordered assertion sites and their literal predicates. These cover exact wire,
lookup, signature, lifetime and account order; 1232/1233 and 4096/4097 packets;
64/65 instructions; bounded no-progress; original errors; consumed proposals;
transactional trees; repeated Pending polls; registered wakes; and ownership on drop.

The polling fixture requires a registered wake before repolling and has a finite
1024-poll bound. Independently driven callbacks remain Pending through repeated
polls without reacquisition. A later nested failure retains its historical test
name containing `update`; the sixth callback is construction in that topology.
The mapped case accurately says `plans-later-nested-create-error`. Separate
consumed-proposal controls establish updater failures.

Five native-only bodies execute real Tokio spawn and Send checks. Two original
tests use Ready operations inside actual spawned tasks; the other three exercise
Pending operations. They are excluded from the guest adapters.

## Execution and sensitivity

| Producer or host                                            | Executed result                                                      |
| ----------------------------------------------------------- | -------------------------------------------------------------------- |
| Rust 1.99.1 and actual 1.89.0, default                      | 307 ordinary tests plus 5 docs per compiler                          |
| Rust 1.99.1 and actual 1.89.0, all features                 | 328 ordinary tests plus 7 docs per compiler                          |
| Ordinary outside consumer, both compilers and feature modes | 27 tests per cell                                                    |
| Native, Chrome, both Wasmtime previews and actual LeptosKit | 161/161 canonical; sole SHA corruption 160/161                       |
| Host fuel accounting                                        | All 3 tests pass; one-fuel interpreter trap before any report commit |

Three separate source mutations actually recompile changed core code in distinct
targets. Removing no-progress yields 9 passes and its intended failure. Bypassing
the awaited updater yields 5 passes and 5 failures. Removing consumed-proposal
retry protection yields 3 passes and the intended `consumed proposal was retried`
failure. The unmutated suites pass. Compiler errors and Cargo Fresh artifacts are
excluded as mutation sensitivity.

Actual browser, plain guest and both WASI byte streams identify rustc 1.99.1
`58cf8f9c3`; actual child compiler paths, versions and hashes bind those bytes to
the full producer commit. The plain guest imports exactly LeptosKit `commit`,
`node_create` and `node_set_props`. Native and three Wasm normal/build graphs pass
the closed core, System and client policies. Valid injected forbidden imports
and dependencies reject. All-target Clippy and docs deny warnings; formatting,
Monochange and every authored/generated style floor pass.

The existing checked fuel policy retains 573 MWA, 257 transaction and 772 message
crypto units and its rates. The 43 additional ordinary cases add 645,000,000 to
the previous allowance, yielding 10,380,000,000. This is fixed-fixture host
accounting, not a production performance or minimum-fuel guarantee.

## Limits

All [212 declaration rows](../../verification/instruction-plans/canonical.tsv)
remain partial. This proof does not supply implicit network or resource factories,
timers, automatic cancellation, dynamic Dart coercions, full package parity,
validator coverage or installed desktop/mobile journeys. Callbacks own external
effects already performed. Depth 512 and width 512 are bounded probes; recursive
Plan operations and drop do not promise arbitrary-depth stack safety.

This SDK is exclusively for off-chain application clients. On-chain programs
should use Pina or official Solana program libraries.
