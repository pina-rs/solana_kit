# Fixed-point composition and presentation differences

This note supplies current evidence for the five rows in
[fixed-points-remaining.tsv](fixed-points-remaining.tsv): the two
`*FixedPointToNumber` presentation functions and the three custom callback
constructors in `fixed_point_codec.dart`. The accompanying reconciliation
refreshes only those rows' reason/notes text in that file and the canonical
126-entry scoped [receipt](fixed-points.tsv); no ID, status, count, threshold, golden or
source hash changes. The source pin is Dart
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`, package
`solana_kit_fixed_points` 0.11.0. Package and Phase 2 completion are not claimed.

## Constructor substitutes

The reference constructors store a `fixedSize` value and a caller callback.
Rust callers implement the public `Encoder`/`Decoder` traits in their own crate;
no callback-factory API was added to production. The consumer-level
[composition test](../../crates/solana_kit_core/tests/fixed_point_composition.rs)
implements a two-byte sign-magnitude fixed-point codec and composes it through
`Paired::try_new`, with `Mapped` capturing typed conversion callbacks.

| Remaining row           | Typed substitute                                                                                                                                  | Declared bounds                                                                                                                                                                         | Executed evidence                                                                                                                                                 |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `FixedPointEncoder.new` | Test-local `Encoder<FixedPoint<RADIX>>`; `write` receives a `Writer` already at the caller's offset.                                              | `encode_size` is `Size::Fixed(2)`; `encoded_len` is exactly two bytes; `encode` rejects max-buffer and final-position mismatches; shapes must match.                                    | `custom_encoder_is_a_public_encoder_with_offsets_and_callback_errors`                                                                                             |
| `FixedPointDecoder.new` | Test-local `Decoder<FixedPoint<RADIX>>`; `read` receives a `Reader` at the caller's offset, and `decode_prefix` reports the next absolute offset. | `decode_size` is `Size::Fixed(2)`; one-byte input is `Truncated`; trailing bytes are `TrailingBytes`; an oversized byte budget is `LimitExceeded` before the callback.                  | `custom_decoder_is_a_public_decoder_with_offsets_and_callback_errors`                                                                                             |
| `FixedPointCodec.new`   | `Paired::try_new` over the two declared halves; `Mapped` around the built-in fixed-point codec converts checked domain values.                    | Pairing requires equal fixed declarations; encoder width 2 versus decoder width 4 is `PairError::FixedSizeMismatch`. Callback policy rejects emit `InvalidValue` or `NumberOutOfRange`. | `paired_try_new_composes_declared_custom_halves`; `mapped_callbacks_convert_checked_domain_values`; `mapped_callbacks_reject_policy_errors_truncation_and_budget` |

The meaningful callback errors are part of the wire contract, not hidden
panics: the sign-magnitude encoder rejects the signed minimum at `-32768`
because its separate sign bit leaves only 15 magnitude bits; the decoder rejects
the reserved negative-zero wire value `80 00`. The same codec is instantiated
for decimal `Shape<10>` and binary `Shape<2>`, matching the generic Dart
constructors.

The Dart and Rust contracts differ deliberately where the callback return value
is ignored:

- Dart `FixedPointEncoder.encode` allocates `fixedSize` bytes and ignores the
  callback's returned offset. The scratch-source oracle returned
  `encode(7) => [7, 0] fixedSize=2` even though its callback wrote one byte.
  Rust `Encoder::encode` requires the writer to finish at `encoded_len`; the
  test's `ShortFixedPoint` writes one byte and is rejected with
  `Error::InvalidValue`.
- Dart `FixedPointDecoder.decode` ignores the callback's next offset and accepts
  trailing bytes; the oracle returned `decode([7, 9]) => 7`. Rust `decode`
  rejects trailing bytes, while `decode_prefix` returns the next offset
  (`Ok((value, 3))` with one prefix byte).
- Dart `FixedPointCodec` never compares encoder and decoder widths. Rust
  `Paired::try_new` does. The built-in `FixedPointCodec` now declares its validated
  byte width through both traits, so its halves pair directly and mismatched
  widths return `PairError::FixedSizeMismatch`. A regression first failed with
  `UnknownSize` before the missing declarations were added. `Mapped` propagates
  the underlying declaration, so the checked `Cents` view also pairs directly
  without calling conversion callbacks during construction.

## Presentation differences

`to_number` remains an explicitly lossy presentation API; exact parsing,
formatting, arithmetic and codecs never call it. Rust scales the exact base-10
scientific representation once and returns typed `Error::Overflow` for a
nonfinite result. The pinned Dart source instead computes
`raw.toDouble() / iterated_pow2_or_pow10`, which loses exactness before the
division and has no typed nonfinite outcome.

The following values are literal output from the pinned-source oracle (Dart SDK
3.13.3, macOS arm64), compared with the executed Rust assertion in
[`fixed_point_numbers.rs`](../../crates/solana_kit_core/tests/fixed_point_numbers.rs).

| Case                 | Mathematical value | Pinned Dart output                              | Rust `to_number`                                |
| -------------------- | ------------------ | ----------------------------------------------- | ----------------------------------------------- |
| Decimal normal       | `125 / 10^2`       | `1.25` (`0x3ff4000000000000`)                   | `Ok(1.25)`                                      |
| Binary normal        | `3 / 2^2`          | `0.75` (`0x3fe8000000000000`)                   | `Ok(0.75)`                                      |
| Decimal extreme      | `2^1024 / 10^0`    | `Infinity` (`0x7ff0000000000000`)               | `Err(Error::Overflow)`                          |
| Binary extreme       | `2^1024 / 2^0`     | `Infinity` (`0x7ff0000000000000`)               | `Err(Error::Overflow)`                          |
| Decimal cancellation | `10^400 / 10^400`  | `NaN` (`0x7ff8000000000000`)                    | `Ok(1.0)`                                       |
| Binary cancellation  | `2^1024 / 2^2048`  | `NaN` (`0x7ff8000000000000`)                    | `Ok(2^-1024)` (`0x0004000000000000`)            |
| Decimal underflow    | `1 / 10^309`       | `0.0` (`0x0000000000000000`)                    | `Ok(1e-309)` (`0x0000b8157268fdaf`)             |
| Binary underflow     | `1 / 2^1024`       | `0.0` (`0x0000000000000000`)                    | `Ok(2^-1024)` (`0x0004000000000000`)            |
| Decimal rounding     | `1 / 10^25`        | `1.0000000000000002e-25` (`0x3abef2d0f5da7dda`) | `Ok(1e-25)` (`0x3abef2d0f5da7dd9`)              |
| Binary rounding      | `(2^54 + 3) / 2`   | `9007199254740992.0` (`0x4340000000000000`)     | `Ok(9007199254740994.0)` (`0x4340000000000001`) |

The Dart decimal cases explain two distinct failures in the source path. For
`10^400 / 10^400`, both `BigInt.toDouble()` values are `Infinity`, so the
division produces `NaN` instead of `1.0`. For `1 / 10^25`, the iterated
`_pow10Double` denominator differs from the correctly rounded `10^25` by one
ULP, so the quotient rounds up; `1 / 10^309` first produces `Infinity` and then
`0.0`. For binary `2^1024 / 2^2048`, the integer part is zero while both the
fraction numerator and the iterated `2^2048` denominator are `Infinity`, again
producing `NaN` for the exact finite value `2^-1024`. The binary rounding case
separates the integer and fractional parts before conversion:
`(2^54 + 3) / 2` computes `(2^53 + 1).toDouble() + 0.5`, and the rounded
integer part makes the sum land on `2^53` instead of the exact rounding
`2^53 + 2`.

These are deliberate deviations, not oracle mismatches to repair. Reproducing
them would introduce `NaN`, infinities, premature underflow to zero and double
rounding, contradicting the documented finite-output contract and typed
`Overflow` behavior. Finite results, including subnormals and signed underflow
to zero, remain covered by the existing conversion tests.

## Reproduction

The source tree was not modified. The oracle used an owned scratch copy from
`git archive`, removed only the scratch copy's `resolution: workspace` stanza so
the path package resolved offline, and ran the conversion and codec scripts:

```bash
git -C "$DART_SOURCE" archive \
  0138bb956d2d81ccc0c56edc602401c0df2f2b46 packages/solana_kit_fixed_points \
  | tar -x -C "$scratch/src"
cd "$scratch/oracle" && dart pub get --offline && dart run bin/oracle.dart
```

The conversion script constructed `DecimalFixedPoint(raw, decimals)` and
`BinaryFixedPoint(raw, fractionalBits, totalBits)` for the ten rows above and
printed IEEE754 bits with `ByteData.setFloat64`; the codec script used
`FixedPointEncoder<int>`, `FixedPointDecoder<int>` and `FixedPointCodec<int>`
with callbacks that report a two-byte advance after writing one byte and that
ignore a trailing byte. Dart package tests were not executed and no network was
used. The Rust assertions execute under the acceptance command for
`--test fixed_point_numbers`; the constructor evidence executes under
`--test fixed_point_composition`.
