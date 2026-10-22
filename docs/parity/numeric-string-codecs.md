# Numeric and string codecs

This is exclusively an off-chain client SDK. On-chain Solana programs should use
Pina or official Solana program libraries.

The scoped receipt accounts for exactly 88 canonical IDs from Dart source
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`: 59 numeric and 29 string IDs.
[The exact mapping](numeric-string-codecs.tsv) preserves each source digest and
records 81 implemented bounded equivalents and
[seven partial capabilities](numeric-string-codecs-remaining.tsv). The canonical
global ledger and older historical receipts remain unchanged. Phase 2 and whole
package parity are not complete.

`Number<T>` preserves the existing exact primitive and 256-bit wire types. Its
explicit constructor, default LE selection and byte-order accessor replace the
Dart config object. Rust integer types and checked conversions replace coercing
fractional `num` inputs. `ensure_number_range` supplies the two inclusive range
assertions through `PartialOrd`, retaining the reference's behavior for NaN.
`Encoder`/`Decoder` traits replace callback factories; a clean external consumer
implements an arbitrary checked three-byte numeric codec and executes its
success, overflow and truncation paths. Four dynamically sized BigInt helpers
remain partial: typed 64/128/256-bit codecs exist, but arbitrary caller-selected
multiword widths are not a public generic helper.

`Uint256` and `Int256` parse radix 2 through 36 and decimal `FromStr` without
floating point, reject overflow and invalid syntax, and format exact canonical
decimal text through fixed-size integer long division. Text input is bounded to
4096 digit bytes; no allocation is required for parsing or formatting scratch.
Signed zero canonicalizes to zero. Exact LE/BE byte conversion remains owned by
Kit. Arithmetic is separate from these wire values.

String encoders convert alphabet text **to binary bytes**; decoders render binary
bytes as text. Hex accepts uppercase and lowercase and puts an odd final digit
in the final byte's low nibble (`AbC` becomes `ab 0c`). Base10 and Base58 preserve
one zero byte per leading zero symbol. BaseX uses fixed scratch storage for exact
size calculation, accepts at most 4096 input symbols, and supports BMP alphabets
of 2 through 65536 symbols, including the reference's first-duplicate lookup
behavior. Non-BMP alphabets remain partial because the Dart implementation
indexes UTF-16 code units, which may produce lone surrogates that Rust strings
cannot represent. The generic bit-reslicing codec validates a power-of-two
alphabet and 1 through 16 bits per symbol; it discards incomplete trailing input
bits and zero-pads the final decoded symbol.

`Utf8Config` defaults to strict decode, stripping the first BOM and preserving
nulls. All eight option combinations match the executed Dart fixtures, including
its repeated-BOM restoration quirk. Rust `str` is always well formed; lone
surrogate input is excluded by the type. Lossy decoding inserts U+FFFD for
malformed sequences. Existing unit `Utf8` remains strict and BOM preserving.
Null padding counts UTF-16 units, including two units for non-BMP characters.
All output allocation is claimed before reservation; BaseX also claims temporary
digit slots against the shared element budget. Limits count elements and UTF-8
bytes, not exact heap allocation sizes.

The optional `text-codecs` feature adds only `base64` 0.22.1 without default
features or host imports. Input ignores '=' anywhere, accepts missing padding,
drops one incomplete trailing symbol and rejects invalid symbols or nonzero
unused bits, matching the pinned Dart oracle. Decode emits standard padded
Base64. Exact encoded size uses no heap allocation. Default, crypto and
fixed-point graphs retain their previous dependencies. The gate queries all
32 feature/target selections plus 16 normal-only numeric selections and rejects
Tokio under all eight policies and Base64 leakage under the four old policies.

Evidence executes 1,536 actual Dart text fixtures, 88 UTF-8 configurations,
512 signed/unsigned 256-bit decimal fixtures, malformed literals, wide alphabets,
and 17,920 additional BigInt radix comparisons. The clean external consumer also
executes on MSRV 1.89. Three isolated portable cases are supplied for later shared
runtime integration. This unit executes native Rust and target compilation;
it does not claim browser, WASI or LeptosKit execution of these new operations.
The original MIT license is retained beside the generated fixtures. Component
publication remains disabled.
