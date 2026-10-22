# Codec framing receipt

This bounded slice supports off-chain application clients. It does not support
Solana on-chain programs. Those programs use Pina or official Solana libraries.

The [exact declaration table](../parity/codec-framing.tsv) preserves all 113 core
source IDs and defining-file hashes from the pinned Dart inventory. Thirty-nine
IDs in seven framing files are partial. The other 74 IDs receive no new claim.
All 113 remain in the [residual table](../parity/codec-framing-remaining.tsv).
Existing codec receipts remain authoritative for their own work.

## Operations and contracts

`ByteSizePrefix` selects a typed u64-valued prefix. `Mapped` adapts narrower,
compact, signed or floating codecs with explicit domain validation. Prefix units
are encoded payload bytes. `FrameConsumption::Exact` rejects unread payload;
`Skip` consumes the whole frame as Dart does. Prefix truncation retains consumed
prefix bytes. A failed nested read advances across its declared frame and retains
all allocation claims. Existing `SizePrefix` and `SizePrefix64` stay exact.

`FixedSize` writes exactly `size` bytes. It truncates or rejects oversized
encodings before writing, bounds scratch by `max_encoded_size`, and zero-fills
unused destination storage. `decoded_size` explicitly declares the optional
fixed nested decoder width. A larger width zero-extends the field before reading;
a smaller width truncates it. Reads skip unread nested bytes. They advance the
outer cursor across the field even when the nested decoder fails. Scratch claims
and nested output claims share the operation's element budget.

`Resized` changes allocation extent and preserves actual nested cursor movement.
Owned encoding permits unused zero-filled extent. Reducing extent below bytes
actually written returns `Truncated`. Direct writes retain the original codec's
storage requirements. Decoder reads pass through. Size callbacks return checked
`Result<usize, Error>` values. Callback errors are observed during sizing rather
than construction. The trait's encoded length denotes allocation extent for this
wrapper, while direct cursor movement can differ.

`Reverse` reverses one declared fixed width. Encoding rejects a mismatch between
that width and the nested encoded size. Decoding uses a bounded scratch slice and
claims its width before allocating. The nested decoder sees that isolated slice;
whole-enclosing-buffer callback context is not retained. The outer cursor always
advances across the reversed width before a nested failure.

`Offset` applies pre/post callbacks to a borrowed `OffsetScope` containing the
whole enclosing byte buffer and original, selected and nested exit positions.
Before the nested operation, all three positions equal the original position.
Post-write callbacks see modified storage. Callbacks return checked absolute
positions. `wrap` accepts signed offsets, handles negative values and returns zero
for empty buffers. End positions are permitted. Backward seeks preserve claims
and overwritten bytes. Pre-callback or invalid-pre-position failures leave the
cursor unchanged. Nested failures retain nested movement. Post failures retain
the nested exit position. Callback contexts are immutable Rust borrows, unlike
Dart's mutable Uint8List. Default optional callbacks become explicit identities.

`Sentinel::dart` extends the existing type with bounded Dart delimiter rules.
Complete payload collisions reject. Append-boundary overlap is permitted and
reading selects the first delimiter. Empty delimiters reject nonempty encoded
payloads, permit empty encodings, and decode an empty payload only when the
remaining input is nonempty. Delimiters wider than 256 bytes are accepted.
Payload scratch and search remain bounded by the caller. Reads accept unused
nested payload bytes. The existing `Sentinel::new` keeps its exact, nonempty,
1..256-byte, overlap-rejecting contract.

`Padded` already supplies left and right padding. Its read behavior matches Dart.
It deliberately zeroes dirty destination padding. FixedSize makes the same
choice. Dart direct writes preserve old destination padding.

## Evidence and limits

The direct original Dart test runner executes all 71 tests in the seven source
families without pub resolution or source mutation. The checked-in oracle
executes the original pinned implementation and produces 299 rows. Public Rust
tests replay each row and independently test nonzero offsets, literal bytes,
nested frames, shared claims, malformed prefixes, empty/overlapping/wide
delimiters, zero/exact/oversized widths, and failure cursor state.

Two portable functions in `verification/conformance/src/framing.rs` execute
through native tests. Their isolated crate compiles them for the three Wasm
targets. They remain unregistered in the central runtime suite. Wasm compilation
is not browser, WASI-host, LeptosKit or installed-device execution.

Dart runtime metadata classes have no parallel Rust hierarchy. Explicit widths
are caller contracts. Prefix adapters validate their numeric domain. Inner writes
in ByteSizePrefix must fill the declared frame, preserving existing Writer
invariants even if a metadata-only Dart resize could leave bytes unused.
Callbacks own any additional allocation they introduce. Scratch allocation is
charged to the shared element budget, which is not an exact heap-byte quota.
No package, Phase 2 or global codec completion is claimed.

Source pin is 0138bb956d2d81ccc0c56edc602401c0df2f2b46. The existing codec MIT
notice also applies to framing behavior adapted from those sources.
