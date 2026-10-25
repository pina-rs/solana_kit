# Ordered client collections

`collections::{OrderedMap, OrderedSet}` owns distinct keys or values in first
insertion order. `codecs::{OrderedMapCodec, OrderedSetCodec}` adapts the existing
sequence contracts to those values. These are portable `no_std + alloc` client
APIs. On-chain programs must use Pina or the official Solana program libraries.

```rust
use solana_kit_core::{
    codecs::{Count, Decoder, Encoder, Limits, Number, OrderedMapCodec, Sequence, Tuple},
    collections::OrderedMap,
};

let map = OrderedMap::from_entries(vec![(2u8, 20u8), (1, 10), (2, 99)]);
let codec = OrderedMapCodec(Sequence {
    item: Tuple(Number::<u8>::LE, Number::<u8>::LE),
    count: Count::U32,
});
let bytes = codec.encode(&map, 64)?;
assert_eq!(bytes, [2, 0, 0, 0, 2, 99, 1, 10]);
let decoded = codec.decode(&bytes, Limits::new(64, 2))?;
assert_eq!(decoded.as_slice(), &[(2, 99), (1, 10)]);
# Ok::<(), solana_kit_core::codecs::Error>(())
```

The adapters borrow the values for encoding. They require no `Clone`, `Ord`, hash
implementation, host API or new dependency. Choose a `Sequence` with `Count::U32`,
`Count::Fixed` or `Count::Remainder`; use `Counted` for another checked numeric
prefix, `OptionalCount` for exact-exhaustion leniency, or `ItemSentinel` for
explicit termination. The selected sequence owns framing, errors, value-free
size declarations and whole-operation overrides.

Map replacement retains the first equal key object and position, while replacing
its value. Removing and reinserting a key appends it. Sets retain the first equal
value object. Uniqueness uses Rust `Eq`. Unlike Dart's default object identity,
Rust equality may compare application-defined fields. Equality must remain stable
while an item is stored. Collection equality compares associations or membership
independently of order; `as_slice` exposes the order used by encoding.
Lookup and removal accept borrowed keys through `Borrow`, so a `String` key can
be queried with `str` without allocating another key.

Decoders read and claim every original wire entry before collapsing duplicates.
Normalization removes duplicates from that owned vector without another
allocation. Duplicate entries do not refund claims, skip malformed later values,
or change the final input offset. Lookup and insertion are linear; normalizing a
vector is quadratic. Callers select item budgets appropriate for their inputs.

The behavior is grounded in the pinned Dart
[map](https://github.com/openbudgetfun/solana_kit/blob/0138bb956d2d81ccc0c56edc602401c0df2f2b46/packages/solana_kit_codecs_data_structures/lib/src/map.dart)
and
[set](https://github.com/openbudgetfun/solana_kit/blob/0138bb956d2d81ccc0c56edc602401c0df2f2b46/packages/solana_kit_codecs_data_structures/lib/src/set.dart)
implementations and their tests. The Rust implementation composes existing Kit
codecs and independently implements owned collection normalization.

Eight maintained public-core bodies cover object retention, framing choices,
duplicate claims, offsets, malformed and nested input, borrowed non-Clone values,
selected-operation forwarding and all small wire orders up to four entries over
three keys. The same bodies are registered for portable conformance. All eight execute in the integrated `solana-kit-core-v37` suite: 354/354 canonical
cases and 353/354 with the single SHA-256 corruption on native Rust, sandboxed
Chrome, both WASI previews and the production LeptosKit interpreter. Native
ordinary callers also pass on Rust 1.89. These results cover the stated contracts;
installed application journeys and registry releases remain separate.
