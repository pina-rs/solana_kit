# Bounded off-chain client codec receipt

The public `solana_kit_core::codecs` API adds explicit cursors, byte and shared
allocation budgets, fixed-width signed/unsigned integers from 8 through 256
bits, both byte orders, bit-preserving IEEE 754 floats, canonical short-u16,
bytes, strict UTF-8, strict booleans, homogeneous sequences, typed pairs,
zero/one-tagged options, C-layout options, trailing options, constants, affixes,
and exact u32/u64 byte-length frames. It is a client SDK, with `no_std + alloc`
for portability. On-chain programs should use Pina or official Solana libraries.

256-bit types are owned wire representations without arithmetic. `Utf8`
preserves a leading BOM and nulls. Remainder codecs must be final fields.
`Limits::max_items` counts allocated entries and bytes, including nested output;
it is not a heap-byte quota for arbitrary custom types. Custom allocating
decoders must claim their elements. Errors may leave cursors advanced or output
partly written. Full decoding rejects trailing bytes; prefix decoding returns
the next absolute offset. Numeric input types enforce range at compile time.

## Capability accounting

[The receipt](../parity/core-codecs.tsv) contains all 1,387 canonical Phase 2
IDs from the independent capability inventory, preserving their source digest
and immutable commit/path provenance. It records 53 `implemented-wire`
operations, 21 `partial` factories, and 1,313 `not-covered-by-unit05` IDs.
These statuses describe this unit alone, not completion of earlier primitive
work. They do not modify the canonical inventory worker's ledger.

[The remaining list](../parity/core-codecs-remaining.tsv) enumerates all 1,334
IDs not completed by this slice, including the 21 partial factories. Partial
factories expose supported configurations only: arrays lack sentinel/custom
count codecs; booleans use one byte; affixes accept exact bytes; options omit
arbitrary prefix and absent-value codecs; UTF-8 omits lossy/BOM/null options;
length frames support explicit LE u32/u64 prefixes. Every partial remains open.

Unfinished families include fixed-point arithmetic, account/maybe-account
models, program/error/sysvar helpers, base encodings beyond existing fixed
value parsing, maps/sets, general structs/unions, sentinels, padding, arbitrary
offset/size/reverse/transform codecs, and general codec introspection. The
receipt is not a Phase 2 completion claim.

## Behavioral evidence

Executed locally on 2026-10-22 with Rust 1.99.1 and MSRV 1.89.0. The preserved
implementation was checkpointed as signed, locally verified commit
`18cdffea5131f97ead31e6e3cad65d3e6dba929f`; subsequent changes complete these
receipts and add cursor-contract tests.

| Check | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo test -p solana_kit_core` | Passed default codecs and doctests |
| `cargo test -p solana_kit_core --features crypto` | Passed codecs and crypto doctests |
| `cargo test -p solana_kit_core --all-features --tests` | Passed 18 tests |
| `cargo +1.89.0 test -p solana_kit_core --all-features --tests` | Passed 18 tests |
| `cargo test -p solana_kit_conformance` | Passed canonical, corruption, missing-case/wrong-suite checks |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Passed |
| Core all-feature checks for `wasm32-unknown-unknown`, `wasm32-wasip1`, `wasm32-wasip2` | Passed compilation only |
| Default normal/build dependency closure | Only `bs58 0.5.1`; codecs add no dependency |
| Monostyle 0.3.26 with generated files scored | Every file and repository category at least 90 |

The 18 codec tests include exhaustive short-u16 round trips for all 65,536
values, independently fixed byte vectors, signed boundaries, 256-bit extremes,
NaN payload/signed-zero/infinity behavior, exact cursor offsets, malformed
lengths/tags/constants/UTF-8, shared allocation exhaustion, stalled remainder
decoders, failed frames, and inconsistent custom writers. Source/provenance
details are in [the fixture record](../../verification/conformance/fixtures/PROVENANCE.md).
No Dart execution is claimed.

The shared host-neutral suite is `solana-kit-core-v2`, 31 cases. Native
[success](../../verification/conformance/evidence/native-success.txt) reports
31/31; native [corruption](../../verification/conformance/evidence/native-corruption.txt)
reports 30/31 and exits 1 after failing the real SHA-256 comparison. The new six
cases exercise numeric bytes, compact integers, string prefix widths, bounded
sequences, option layouts, and exact framing. Browser, WASI and wasmi execution
of v2 remains an integration task for the runtime harness. Target compilation
does not establish those runtime results. Serde is not an existing feature;
this unit neither implements nor claims a serde matrix.

The component remains unpublished. The minor changeset records additive API
intent; it does not authorize registry publication.
