# Pinned numeric and string fixtures

`text-dart.tsv` contains 1,536 executed outputs from the actual Dart checkout at
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`: 256 deterministic four-byte inputs
for decimal, hexadecimal, Base58, Base64, custom radix-10, and octal bit reslicing.
The input is `[0,n,255-n,n^85]`, for `n=0..255`. Each row records its source
family, independent input, decoded text and re-encoded bytes. Octal output pads
its final symbol, so its re-encoded bytes are recorded separately.

`numeric-dart.tsv` contains 88 actual configured UTF-8 results and 512 unsigned
and signed 256-bit decimal results. The UTF-8 fixtures enumerate all eight
options on eleven empty, ASCII, NUL, BOM, repeated-BOM, Unicode, truncated and
malformed inputs. Numeric input byte `i` for trial `n` is
`(n*31+i*59+(n>>3)*(i+1))&255`, with `i=0..31,n=0..511`.

The oracle imported the pinned packages directly through an isolated local
package configuration. No dependency resolution or Dart source changes were
required. The retained original MIT notice is in `DART-LICENSE`. Rust behavior
also uses separately specified literals and the optional independent BigInt
oracle; the fixture counts are finite evidence, not universal parity proofs.

A `-` field denotes an empty byte sequence in the UTF-8 fixtures. This only
normalizes the tabular representation; the original oracle output is retained
in the local ignored evidence store.
