# Current Phase 2 and Phase 3 reconciliation

This additive receipt originally reconciled the source audit with the
implementation at `65c502f9044d320ec809fc0b45670b7860a8b4e3` on 2026-11-04.
The Dart reference remains `0138bb956d2d81ccc0c56edc602401c0df2f2b46`.
Historical slice receipts retain their original counts and limits; their
outstanding lists are not a current implementation inventory. The execution
section below is updated for the later v56/455 integration while keeping the
original v55/448 result as historical evidence.

## Implemented families and executable evidence

| Family                                      | Current Rust contract                                                                                             | Evidence in this repository                                                                                                       |
| ------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| Addresses, constants and PDA                | Validated `Address`, collation, seeded derivation, typed PDA ownership and optional audited curve operations      | `core_parity`, `address_collation`, shared address cases                                                                          |
| Errors, account models and sysvars          | Typed error catalog, account decoding and sysvar layouts                                                          | `accounts_sysvars`, shared core parity and sysvar cases                                                                           |
| Numeric and string codecs                   | Exact bounded integers, optional arbitrary widths, byte/text/radix codecs including UTF-16 symbol handling        | `codecs`, `text_codecs`, `core_parity`                                                                                            |
| Composite codecs                            | Collections, records, tuples, selected unions, framing, offsets, transformations and shared allocation accounting | `composite_codecs`, `codec_framing`, `codec_cursors`, `codec_size`                                                                |
| Fixed points                                | Bounded typed shapes, exact arithmetic and codecs, explicitly lossy finite presentation                           | `fixed_points`, `fixed_point_numbers`, `core_parity`; the [receipt](fixed-points.md) retains its five unresolved declaration rows |
| Instructions and program-client composition | Owned typed account roles, instruction data and explicit client composition                                       | Core instruction/program tests and shared portable cases                                                                          |
| Legacy, v0 and v1 messages                  | Compilation, received-wire decoding, lookup provenance, inline v1 configuration and exact packet limits           | `codec_transactions`, `lookup_origin`, `v1_transactions`, shared message cases                                                    |
| Transactions and lifetimes                  | Immutable signed bytes, signer slots, explicit unresolved/expiry/nonce metadata and structural nonce inspection   | Core transaction tests and shared transaction/nonce cases                                                                         |
| Offchain messages                           | Typed domain/version formats and bounded message preparation                                                      | `offchain_golden`, `offchain_boundaries`, `offchain_malformed`                                                                    |
| Introspection                               | Typed RPC/wire decoding, loaded account roles, inner instructions and walking order                               | Client `introspection` tests and shared introspection cases                                                                       |
| Confirmation                                | Reducers, polling and composed observation factories, timeout/cancellation and cleanup                            | Client `confirmation`, `confirmation_factory`, `confirmation_watch`, `confirmation_races`                                         |
| Instruction plans                           | Typed plans, packing, planning and execution with ordered results and failure boundaries                          | Core/client `plans`, client `async_planning`, shared plan cases                                                                   |

These are test-target names under `crates/solana_kit_core/tests` and
`crates/solana_kit_client/tests`, or modules under `verification/conformance/src`.
They identify executable behavior rather than asserting that every source
declaration has been individually adjudicated.

The three concrete API gaps identified by the audit were subsequently addressed:

- [Bounded key grinding, constant-time equality and signer wrappers](key-grinding-and-decompilation.md).
- [Transaction-message decompilation](key-grinding-and-decompilation.md), retaining
  lookup provenance and explicit lifetime metadata without authenticating it.
- [Per-call signer configuration](signer-call-config.md) throughout message and
  transaction capabilities, local/native composition and browser wallet adapters.

Rust `Option`, trait implementations, explicit typed composition, codec mapping,
and existing account-role or transaction operations replace several named Dart
wrappers. A missing matching function name alone does not establish absent
behavior. Deliberate bounds, error differences and non-invertible privilege
merging remain documented in the owning receipts.

## Current execution and limits

At the original `65c502f9` checkpoint, the then-current `solana-kit-core-v55`
shared suite passed 448/448 natively, in real Chrome, in both Wasmtime previews
and in the actual pinned LeptosKit interpreter. Each corrupt-fixture run returned
447/448 and exit 1. Import/dependency negative controls, the interpreter's
one-fuel control and a fresh packaged-core consumer passed through
`cargo xtask all` at that checkpoint. Those counts are historical evidence for
that source, not the current suite.

The current shared suite is `solana-kit-core-v59` with 476 cases. It passes
476/476 natively, in actual Chrome, in both Wasmtime 50 previews and in the
pinned LeptosKit interpreter. The corruption control is 475/476 and exits 1,
with only `sha256-fips-abc` failing. The first 461 report rows remain
byte-identical to the v58 report, which itself preserves the captured 448-row
v55 prefix. The last fifteen groups are the error-contract controls and
evidence; before them are the error-code catalog negative control and
the exact 365-constant comparison. The four preceding appended groups cover
normalized publication bundle/source projection, diagnostic redaction,
ingestion translation, and session checkpoint precedence. The earlier seven
groups cover grinding and cancellation, legacy/v0/v1 message decompilation,
explicit signer configuration, Token plans, typed parsed-account families,
fixed-point composition, and test-support exchanges. Import/dependency controls,
the one-fuel control and a fresh packaged-core consumer passed in the same
invocation. See the [publication runtime receipt](../verification/publication-runtime.md) and the [current runtime receipt](../verification/runtimes.md).

At the earlier checkpoint the grinding/decompilation tests had only native
execution and Wasm compilation evidence, and the explicit configuration tests
had real Chrome wallet fixture execution but were not in the shared suite. The
v56 integration subsequently appended all seven groups, including those three,
and executed them on every named host.

The [installed wallet receipt](../verification/installed-wallet-current.md)
records two complete current-source iOS controller runs, including native wallet
recovery and real local-validator effects. The macOS functional journey passed through computer use,
with its separate [receipt](../verification/installed-wallet-macos-current.md); the Mac2 controller
route remains unavailable. The current Android controller passed twice
on an arm64 emulator, recorded in its
[execution receipt](../verification/installed-wallet-android-current.md).

No canonical capability ID, hash, status, checker, threshold, shared golden or
suite version changes in this reconciliation. Declaration-level evidence
adjudication and bounded differences remain explicit; this receipt does not
declare either phase or the whole ecosystem complete.
