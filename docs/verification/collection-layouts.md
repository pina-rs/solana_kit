# Collection counts and optional layouts

These codecs serve exclusively off-chain application clients. On-chain programs
use Pina or official Solana program libraries. Core stays `no_std + alloc`.

`ItemSentinel` compares the marker only between elements. Required and optional
policies write the marker. Omitted policy writes only items. Optional and omitted
reads permit the exact end of input. All policies consume a marker when present.
The caller declares a positive minimum item size. Optional and omitted markers
cannot exceed that size. Truncated items fail, and nonprogressing codecs fail.
Items cannot begin with the marker, as in the pinned source.

`Nullable<C, P>` encodes Rust `Option<T>`. Its optional prefix stores a boolean.
`Absence` selects omitted bytes, an explicit zero-filled width, or a constant
pattern. Tagged zero storage ignores old payload contents when reading. Constant
storage is always checked. Without a prefix, the absent pattern is reserved and
cannot also represent a present value. Explicit zero width replaces Dart fixed
size metadata. Present values must occupy that width.

`Boolean<P>` validates a caller-selected u32-valued numeric codec as zero or one.
`Mapped` supplies checked conversions for other numeric storage. `OptionalCount`
adds exact-end leniency to existing `Counted` prefixes. Partial prefixes still
fail. Existing `Sequence`, `Counted`, `Bool`, `OptionCodec`, and byte-oriented
`Sentinel` retain their contracts. Counted's resolved count remains u32. Wider
wire storage is supported with checked mappings; larger resolved domains remain
outside this bounded slice.

The pinned Dart source is `0138bb956d2d81ccc0c56edc602401c0df2f2b46`.
The original selected tests execute 144 cases. A fresh source oracle emits 35
literal rows covering all three terminator policies, tagged and untagged omit,
zero and constant patterns for both nullable and Option, and u32 boolean storage.
Public Rust tests replay every row and independently exercise truncation, dirty
output, invalid tags and constants, shared limits, wide prefixes and overflow.
A freshly compiled altered marker fails the public boundary test.

The [exact 73-ID table](../parity/collection-layouts.tsv) preserves every original
canonical field. It contains 21 array, six boolean, 12 nullable and 34 options
IDs. Seventeen Option value and unwrap declarations map to tested native Rust
constructors, patterns, equality, nullable identity and lazy fallback operations.
Fifty-six rows retain partial contracts. Five value residuals name numeric Dart
hash identity, generic formatting, runtime type inspection and heterogeneous
recursive traversal. Runtime metadata, arbitrary callback semantics, full error
taxonomy and remaining count domains are separate wire and configuration gaps.

Fresh native tests and ordinary consumers run on Rust 1.99.1 and 1.89.0. The
[portable runtime receipt](collection-runtime.md) executes all 54 ordinary
consumer contracts on native Rust, Chrome, both WASI previews and the pinned
LeptosKit interpreter. All 73 bounded dispositions retain their stated limits.
Installed devices, validators and Phase 2 completion remain outside this proof.
