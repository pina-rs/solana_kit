# Numeric and fixed-point adjudication

This note records the second reviewed use of
[`adjudications.tsv`](adjudications.tsv) for twelve residual declarations in the
numeric/string-codec and fixed-point families. It does not complete either
package, Phase 2, or the SDK.

## Scope and identity

The manifest is keyed by complete canonical ID and the defining source hash from
[`source-files.tsv`](source-files.tsv). The batch spans five pinned files:

| Source                            | SHA256                                                             |
| --------------------------------- | ------------------------------------------------------------------ |
| `lib/src/utils.dart`              | `1f7efa6ea56f6a373ca952cc978b164afa42abc14399f8de753034d033edd7f6` |
| `lib/src/base_x.dart`             | `75a35b0769137b1c2b8999065952bdb2f958bf4f19af6da24a58f3926be2e0f2` |
| `lib/src/binary_formatting.dart`  | `78dd306b5c2d3d10fc632a9524c43e05564e74e7d0b4c78fec77cfaee39acd98` |
| `lib/src/decimal_formatting.dart` | `3f960516178489a365426d8085df123a70408bef317ee93e5ad36363996d7eb2` |
| `lib/src/fixed_point_codec.dart`  | `7dbe80ee270b8347607582473f6b6b6998fe5d1d57be7f568d7570ca415d591d` |

Regeneration leaves `id`, `package`, `path`, `symbol`, `kind`, `line`, `owner`,
`phase`, `unit`, `sha256` and `provenance` byte-identical for all 17,948 rows.
Only `status`, `evidence` and `notes` for these twelve rows differ from the
previous inventory. The twelve address pilot rows and the `mergeRoles` fallback
are preserved.

## Exact mapping

| Canonical ID suffix                                 | Rust equivalent                           | Status  | Shared cases                                                                          |
| --------------------------------------------------- | ----------------------------------------- | ------- | ------------------------------------------------------------------------------------- |
| `utils.dart#writeBigIntUnsigned`                    | `BigNumber<BigUint>` encoder              | partial | `arbitrary-integer-width-literals`, `arbitrary-integer-width-rejections`              |
| `utils.dart#writeBigIntSigned`                      | `BigNumber<BigInt>` encoder               | partial | `arbitrary-integer-width-literals`, `arbitrary-integer-width-rejections`              |
| `utils.dart#readBigIntUnsigned`                     | `BigNumber<BigUint>` decoder              | partial | `arbitrary-integer-width-literals`, `arbitrary-integer-width-rejections`              |
| `utils.dart#readBigIntSigned`                       | `BigNumber<BigInt>` decoder               | partial | `arbitrary-integer-width-literals`, `arbitrary-integer-width-rejections`              |
| `base_x.dart#getBaseXEncoder`                       | `BaseX`/`Utf16BaseX` encoder              | partial | `numeric-text-codecs`, `utf16-radix-surrogate-positions`, `utf16-radix-shared-limits` |
| `base_x.dart#getBaseXDecoder`                       | `BaseX`/`Utf16BaseX` decoder              | partial | `numeric-text-codecs`, `utf16-radix-surrogate-positions`, `utf16-radix-shared-limits` |
| `base_x.dart#getBaseXCodec`                         | `BaseX`/`Utf16BaseX` encoder plus decoder | partial | `numeric-text-codecs`, `utf16-radix-surrogate-positions`, `utf16-radix-shared-limits` |
| `binary_formatting.dart#binaryFixedPointToNumber`   | `FixedPoint::to_number`                   | partial | `fixed-point-extreme-presentation`                                                    |
| `decimal_formatting.dart#decimalFixedPointToNumber` | `FixedPoint::to_number`                   | partial | `fixed-point-extreme-presentation`                                                    |
| `fixed_point_codec.dart#FixedPointEncoder.new`      | test-local public `Encoder`               | partial | `fixed-point-exact-codecs`, `fixed-point-codec-pairing`                               |
| `fixed_point_codec.dart#FixedPointDecoder.new`      | test-local public `Decoder`               | partial | `fixed-point-exact-codecs`, `fixed-point-codec-pairing`                               |
| `fixed_point_codec.dart#FixedPointCodec.new`        | `Paired::try_new`                         | partial | `fixed-point-codec-pairing`                                                           |

The exact IDs and reviewed notes are in the manifest.

## Shared case bodies

Every cited case is an exact successful entry in the maintained
`solana-kit-core-v57` report. The bodies map to source behavior as follows.

| Shared case                          | Body                                                          | Source behavior proved                                                                                                     |
| ------------------------------------ | ------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `arbitrary-integer-width-literals`   | `core_parity/bigints.rs::arbitrary_width_literals`            | Unsigned 3-byte LE encode/decode, signed 3-byte BE two's complement, 33-byte width, signed 2-byte LE.                      |
| `arbitrary-integer-width-rejections` | `core_parity/bigints.rs::arbitrary_width_rejections`          | Zero/oversized width rejection, over-range values, truncation, limit, trailing bytes.                                      |
| `numeric-text-codecs`                | `verification/conformance/src/text_numbers.rs::text`          | BaseX base10 leading-zero encode and Base58 leading-zero decode.                                                           |
| `utf16-radix-surrogate-positions`    | `core_parity/utf16.rs::surrogate_symbols`                     | `Utf16BaseX` surrogate-code-unit encode/decode, duplicate first-index selection, leading-zero decimal units.               |
| `utf16-radix-shared-limits`          | `core_parity/utf16.rs::unicode_limits`                        | Empty/single-unit alphabet rejection, 4096-symbol encode cap, decode element budgets.                                      |
| `fixed-point-exact-codecs`           | `verification/conformance/src/fixed_points.rs::codecs`        | Signed two's-complement fixed-point bytes, exact decode, truncation rejection.                                             |
| `fixed-point-codec-pairing`          | `parity_completion.rs::fixed_point_pairing`                   | Built-in decimal/binary fixed-size halves pair directly, exact bytes, and encoder/decoder width mismatch.                  |
| `fixed-point-extreme-presentation`   | `core_parity/fixed_points.rs::extreme_fixed_point_conversion` | Exact 10^1000 and 2^2048 scaling to finite `f64`, signed underflow to `-0.0`, and typed `Overflow` for a nonfinite result. |

## Why every row stays partial

The four BigInt helpers are dynamic and accept caller-selected sizes and offsets:

- `writeBigIntUnsigned` writes the low `size` bytes of any `BigInt`. A negative
  value is written as two's complement, an over-width value is truncated, and
  `size <= 0` is a no-op. `BigUint` cannot be negative and `BigNumber::new`
  rejects zero width; over-width input returns `NumberOutOfRange`.
- `writeBigIntSigned` adds `2^(size*8)` for negatives, then truncates to `size`
  bytes. Over-width values truncate, `size == 0` writes nothing, and negative
  sizes throw. Rust rejects zero width and over-range values.
- `readBigIntUnsigned` returns `0` for `size <= 0` and reads a caller-selected
  offset slice. Rust rejects zero width and uses a `Reader` cursor with the
  shared byte/element budget.
- `readBigIntSigned` computes `2^(size*8-1)`, so `size <= 0` throws in Dart.
  Rust rejects zero width. Truncation and trailing-byte rules also differ
  between the helper and full `Decoder::decode`.

A narrow oracle against the pinned `utils.dart` copied to scratch confirmed
those edge outcomes with Dart 3.13.3: unsigned writes of `256` and `-1` at
`size = 1` produced `00` and `ff`; signed writes of `128` and `-129` at
`size = 1` produced `80` and `7f`; `size = 0` was a no-op; signed reads and
negative-size signed writes threw `ArgumentError`; offsets wrote and read the
selected byte and an out-of-bounds offset threw `IndexError`.

The three BaseX rows are not limited by string representation. `BaseX` covers
BMP alphabets, and the explicit `Utf16BaseX` codec preserves Dart's UTF-16
code-unit alphabets, including surrogate halves and lone code units, as
`&[u16]`/`Vec<u16>`. The residual is bounded execution: Rust caps encoded input
at 4096 symbols and alphabets at 65536 entries, while the reference has no such
bounds, and the `Utf16BaseX` decoder returns code units rather than a Rust
`String`.
`numeric-text-codecs` proves the ordinary encode/decode pair;
`utf16-radix-surrogate-positions` and `utf16-radix-shared-limits` prove the
code-unit alphabet and bound behavior.

The two presentation functions deliberately deviate where Dart's
`BigInt.toDouble()` and iterated powers lose exactness: nonfinite cancellation,
premature underflow to zero, and one-ULP rounding. The pinned oracle table in
[the composition note](fixed-point-composition.md) records those literals.
`fixed-point-extreme-presentation` proves the Rust finite/overflow substitution.

The three fixed-point constructors are replaced by test-local public `Encoder`
and `Decoder` implementations and `Paired::try_new` for caller callbacks. Dart
ignores the callback's returned offset, accepts trailing decoder input, and
never compares encoder and decoder widths. Rust requires the final write position to equal `encoded_len`,
rejects trailing bytes in `decode`, reports the next offset through
`decode_prefix`, and rejects a declared width mismatch in `Paired::try_new`.
Those are deliberate safety checks, so the constructor rows remain partial.

## Evidence binding and residual work

The extractor accepts executable evidence only from the maintained conformance
success report, binds cited cases to the live registry, and rejects unknown or
duplicate IDs, hash drift, absent or unsafe evidence paths, blank or invented
cases, and malformed manifests. The historical `*-remaining.tsv` lists are the
source of the twelve IDs, not separate execution evidence. No production SDK,
root manifest, lockfile, workflow, shared conformance case, fixture, golden,
fuel ledger, extractor, checker or threshold changed. The remaining 17,924
canonical rows keep their previous status, and no package or phase is claimed
complete.
