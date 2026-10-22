# Exact bounded fixed points

The optional `fixed-points` feature supports portable off-chain clients through
validated `DecimalShape`/`BinaryShape` and owned `DecimalFixedPoint`/
`BinaryFixedPoint` values. `BigInt` is a numeric primitive from `num-bigint`
0.4.8; domain shapes and values remain Kit-owned. Its normal graph consists of
`num-integer` and `num-traits`, with default features disabled and no host APIs.
No custom bigint implementation, unchecked numeric narrowing, floating-point
round trip, or arbitrary-width truncation is used.

Widths are 1 through 4096 bits and scales are 0 through 4096. Binary fractional
bits may exceed the raw storage width. Text is limited to 8192 bytes;
ratio/scalar inputs to 32768 magnitude bits each. Derived intermediates are
bounded by those inputs and scale factors. The bigint allocator can still abort
on process-wide exhaustion; these limits bound work rather than promise a
recoverable allocator for third-party bigint operations.

Shape factories are values with `raw`, `parse`, `ratio` and `codec` methods.
Rust types and private fields prevent the malformed raw constructors accepted
by the reference; `has_shape` and `assert_shape` cover optional constraints.
`compare` permits differing width/signedness at the same scale;
`compare_same_shape` preserves the reference instance comparison rule. Equality
and hashing include the complete shape. Boolean comparisons use the returned
Rust `Ordering` predicates.

Arithmetic methods return explicit errors. Multiplication/division accept
separate typed scalar or fixed-point methods; fixed-point operands require only
matching signedness and return the left operand's shape. Rounding is strict,
floor, ceil, truncation, or nearest with ties away from zero. Strict decimal
parsing rejects excess fractional digits even when zero; binary strict parsing
accepts exact decimal ratios regardless of their textual fractional length.
Whitespace, exponents, plus signs and digit-free input are rejected.

Formatting options rescale exactly before padding or trimming fractional zeros.
`to_base10` preserves all binary precision by multiplying raw by `5^scale`.
`format_with` passes exact scientific notation to caller formatting logic.
`to_number` is intentionally lossy and never participates in exact operations.
It converts the exact base-10 scientific representation through Rust's `f64`
parser, avoiding separate overflowing numerator/denominator conversions. A finite
result succeeds, including subnormal values and signed underflow to zero;
nonfinite results return `Error::Overflow`. Work stays bounded by the validated
width and scale. This presentation contract is not an exact floating arithmetic
API; complete extreme IEEE754 parity with Dart remains open.

Codecs require byte alignment, use core `Encoder`/`Decoder`/`Endian`/cursors,
charge raw output bytes against the shared decoder allocation budget, preserve
two's-complement values and reject mismatching shapes before writing. Full
`decode` rejects trailing bytes; `decode_prefix` provides the reference's prefix
behavior explicitly. Validated storage prevents the reference's invalid raw
write cases from being constructible.

The canonical 126-entry [receipt](fixed-points.tsv) preserves source IDs and
hashes, mapping 121 bounded equivalents and listing [five remaining IDs](fixed-points-remaining.tsv).
Neither the package nor Phase 2 is claimed complete. The source pin is Dart
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`, package `solana_kit_fixed_points`.
Reference vectors are from its decimal/binary, arithmetic, formatting,
conversion, comparison, range, codec and security tests. The retained MIT notice
in the shared fixture provenance applies. No Dart execution is claimed.

Eleven original native tests cover those vectors, 32,340 independently bracketed rational
rounding checks, malformed inputs, budget and signed overflow edges, byte
vectors, and widths beyond 256. The separate shared module adds three cases
under the original isolated suite `solana-kit-core-v3-fixed-points` (34 cases).
The combined `solana-kit-core-v5` suite preserves the 39 preceding cases and adds
these three cases. Its [executed runtime receipt](../verification/runtimes.md)
records native, Chrome, Wasmtime Preview 1/2 and actual LeptosKit proof: canonical
42/42 and corrupted 41/42, with only the SHA-256 negative control failing.
All 16 plain/crypto/fixed-points/combined target closures and the packaged
fixed-point consumer pass. The independent consumer also executes 1,650,000
operation checks and 524,288 byte checks against the integrated core on Rust
1.99.1 and MSRV 1.89.0. These finite checks do not establish complete package
parity. Five additional public conversion regressions cover both radices and
signs, finite wide ratios, nonfinite overflow, subnormals and signed underflow.
Existing historical receipts remain historical.

Reference test digests (SHA-256, paths below the pinned package's `test/`):

```text
40e3b800c642d5ac509f939ee252a8ec6f6724f310c707facf5ad57e203ecd70  binary_arithmetic_test.dart
bb07373538390f795a43c25ebfc15d37abe8c430c1713e4b3120885ea70e498e  binary_fixed_point_test.dart
9a9a71ae20deb9d8341327d821468ee22678c7f7b210c6a9c6c536820d587035  decimal_arithmetic_test.dart
58daef80cecd88becdf8761b8a2c2bfe11c18b0e910dc7f4f2821fb521c31d86  decimal_fixed_point_test.dart
5383b88e8f153f8cd6d18a01c6aa33edb8fb78e664beded560ce2f82903c5e52  fixed_point_codec_test.dart
070bbf4c2d8cadcbc58745fe6d44ebce9401fa0d7d728877f479c183833dbf99  fixed_point_comparisons_test.dart
ad55b16a539924eba9cb85d0b856dfdbcefe5dd192ac6320819b99be43866fc6  fixed_point_conversions_test.dart
d01dd3b9183c74f10ef5dce975956c26e482c94093634c0837bed1752da30a9e  fixed_point_formatting_test.dart
236ef620b4ba9ff3c7686b12ac93b34ee1e1ec1115b3d6a2c2e568dd3a15902b  fixed_point_ranges_test.dart
ffdd25acbd4ac5bb56381b5a7caf4c5ac579af0efa5e5dc0173548e18610a3c9  fixed_point_security_test.dart
```
