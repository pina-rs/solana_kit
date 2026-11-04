# Numeric and string codecs

This is exclusively an off-chain client SDK. On-chain Solana programs should use
Pina or official Solana program libraries.

The scoped receipt accounts for exactly 88 canonical IDs from Dart source
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`: 59 numeric and 29 string IDs.
[The exact mapping](numeric-string-codecs.tsv) preserves each source digest and
records 81 implemented bounded equivalents and
[seven partial capabilities](numeric-string-codecs-remaining.tsv). The checked
[manifest adjudication](numeric-adjudication.md) records those seven rows and
their executed evidence. The canonical ledger changes only status, evidence
and notes for those seven rows and five fixed-point rows; declaration identities
and older historical receipts remain unchanged. Phase 2 and whole package parity
are not complete.

`Number<T>` preserves the existing exact primitive and 256-bit wire types. Its
explicit constructor, default LE selection and byte-order accessor replace the
Dart config object. Rust integer types and checked conversions replace coercing
fractional `num` inputs. `ensure_number_range` supplies the two inclusive range
assertions through `PartialOrd`, retaining the reference's behavior for NaN.

`Encoder`/`Decoder` traits replace callback factories. A clean external consumer
implements a checked three-byte numeric codec and runs its success, overflow and
truncation paths. `BigNumber<BigUint>` and `BigNumber<BigInt>` in
`crates/solana_kit_core/src/codecs/big_number.rs` accept a nonzero width and
LE/BE order with range and trailing-byte checks. The four BigInt rows stay
partial after adjudication because the dynamic helpers truncate or no-op on
sizes and values the typed Rust codec rejects; the exact residual is in
[numeric-adjudication.md](numeric-adjudication.md). The `core_parity` BigInt
tests run 2-, 3- and 33-byte cases with `--features bigint-codecs`.

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
behavior. The explicit `Utf16BaseX` codec represents Dart's UTF-16 code-unit
alphabets, including surrogate halves and lone code units, as `&[u16]` and
`Vec<u16>`; it is the executable substitute for non-BMP alphabets. The three
BaseX rows stay partial only for the 4096-symbol and 65536-entry bounds and the
code-unit result representation, as recorded in
[numeric-adjudication.md](numeric-adjudication.md).

The generic bit-reslicing codec validates a power-of-two alphabet and 1 through
16 bits per symbol. It discards incomplete trailing input bits and zero-pads the
final decoded symbol.

`Utf8Config` defaults to strict decode, stripping the first BOM and preserving
nulls. All eight option combinations match the executed Dart fixtures, including
its repeated-BOM restoration quirk. Rust `str` is always well formed; lone
surrogate input is excluded by the type. Lossy decoding inserts U+FFFD for
malformed sequences. Existing unit `Utf8` remains strict and BOM preserving.
The standalone `remove_null_characters` helper removes only NUL and preserves
every BOM. Its byte budget is checked before output allocation.
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
executes on MSRV 1.89. An unchanged independent consumer replays 19,158 actual
Dart oracle rows against the integrated public package with zero mismatches.

Three portable groups entered the then-current shared 59-case suite natively, in
Chrome, in Wasmtime Preview 1 and Preview 2, and in the pinned actual LeptosKit
host. They remain in the current v58/461 suite. The
[runtime receipt](../verification/runtimes.md) records the executed source and
artifact hashes. This bounded evidence does not complete Phase 2,
installed-app journeys, or the seven partial capabilities documented in
[numeric-adjudication.md](numeric-adjudication.md).
The original MIT license is retained beside the generated fixtures. Component
publication remains disabled.
