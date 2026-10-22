# Typed composite codecs

This is a bounded capability slice for off-chain application clients. On-chain
programs should use Pina or official Solana program libraries. The existing
Encoder, Decoder, Reader, Writer and shared allocation budget remain unchanged.
No dependencies, Cargo features or host APIs were added.

| Public capability | Contract                                                               |
| ----------------- | ---------------------------------------------------------------------- |
| Mapped            | Fallible deterministic input/output value conversion                   |
| Paired            | Independent concrete encoder and decoder implementations               |
| Checked           | Fallible value observers before writing and after reading              |
| Padded            | Zero-filled left/right storage; input padding is skipped               |
| Sentinel          | Exact delimited frames with bounded scratch and payload length         |
| BitArray          | Fixed byte width, MSB-first or reversed byte/LSB-first order           |
| Unit              | Zero-width typed unit value                                            |
| Literal           | Ordered Copy-valued choices with a configurable u32-valued index codec |
| Choice and Either | Two statically typed alternatives with checked zero/one byte tags      |
| Counted           | Configurable u32-valued element counts reusing Sequence claims         |
| Dependent         | A typed pair whose second codec depends on the first value             |
| OwnedSequence     | Sequence adaptation for owned Vec record fields                        |

Mapped callbacks can construct ordinary Rust structs from typed Tuple values.
Dependent can nest to build larger typed records. Callback-owned allocations must
be bounded by the callback author. Built-in allocating decoders claim elements
before allocation. The budget measures elements, not exact heap bytes.

Counted reuses Sequence with a resolved fixed count, including bounded zero-width
elements. Existing array-to-slice inference remains intact. Adding an owned Vec
implementation directly to Sequence was rejected after the unchanged workspace
exposed an inference regression. OwnedSequence makes that representation explicit.

## Evidence

Actual immutable Dart commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46` executed its
original packages locally with an isolated four-package config and no dependency
resolution or network. The checked-in oracle script reproduces 3,840 golden rows:
2,560 bit cases across byte widths 0..4, both directions and 256 input patterns;
1,280 scalar cases covering padding, mapping, sentinel, count and literal choices.
Rust executes every row, including both encoding and decoding where applicable.
The original MIT notice is retained beside fixtures and inside the core archive.

Independent literals and failure probes exercise exact frames, nonzero offsets,
truncation, invalid tags, full-buffer trailing bytes, cumulative allocation
claims, callback failures, zero-progress remainder reads and count overflow.
Another 434 small binary payload/delimiter pairs compare accepted encoding against
the first delimiter position and verify roundtrips. Three isolated portable
functions originally executed natively without changing the shared registry.
The [v12 runtime integration](runtimes.md#historical-v12-composite-codec-receipt)
now appends those exact functions and executes them in native Rust, Chrome,
Wasmtime Preview 1 and Preview 2, and the pinned actual LeptosKit host.

A clean Rust 1.89 consumer without patches or repository flags executes typed
record conversion, dependent Vec fields with exact limits, and unchanged array
coercion. Workspace, default-feature and MSRV tests, warning-denied Clippy/docs,
formatting and Monochange checks pass. Core compiles for wasm32-unknown-unknown,
wasm32-wasip1 and wasm32-wasip2. The later v12 receipt records actual runtime
integration. Device journeys remain unverified.

## Exact scope and differences

[The scoped table](../parity/composite-codecs.tsv) retains all 244 original IDs,
113 core and 131 data-structure entries, including every original source field.
Source digests were checked against 37 defining files. Nine IDs are implemented
with bounded contracts, 24 are partial, and 211 are not covered by this unit.
[The exact remaining set](../parity/composite-codecs-remaining.tsv) has 235 IDs.
The prior core-codec receipt and canonical inventory are unchanged.

Explicit differences keep unsupported behavior visible:

- Paired does not infer fixed/maximum-size metadata or validate compatibility at
  construction. Each operation retains its exact existing wire contract.
- Mapped exposes typed value conversion without Dart's entire-buffer and original
  offset callback context. Arbitrary callback allocations need their own bound.
- Padded zeroes caller-owned padding. Dart leaves existing destination bytes there.
- Sentinel rejects empty delimiters or delimiters wider than 256 bytes, ambiguous overlaps at
  the appended delimiter, and unread bytes in the payload frame. It uses an
  explicit maximum payload and at most that much temporary encoding storage.
- Literal outputs Copy values and uses u32-valued index adapters. Owned dynamic
  literals and arbitrary numeric index domains remain partial.
- Dependent provides typed pairs, not the dynamic named-field builder and frozen
  map introspection. Counted omits element-boundary sentinel strategies, missing
  prefix leniency and integer count domains wider than u32.
- Choice adds a byte tag. It is not credited as a complete reference union or
  discriminated-union implementation.

General maps/sets, arbitrary named records, nullable configurations, predicate
and pattern choices, full unions, offset rewinding/wrapping, resize, fixed-size
truncation/padding, reversal, byte observers, size introspection, Keccak and byte
utility facilities remain unimplemented by this unit. Earlier bounded constants,
bytes, tuples and options retain their previous evidence and limitations.
Neither package parity nor Phase 2 completion is claimed.
