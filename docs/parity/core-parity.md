# Deterministic core contract additions

This source layer adds client codec and value contracts from Dart Solana Kit
commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46`. The integrated v37 suite executes all 354 cases on native Rust, sandboxed Chrome,
WASI Preview 1 and 2, and the pinned production LeptosKit WasmHost. Its corruption
control reports exactly 353/354. Rust 1.89 native callers and Rustdoc also pass.
Registry release and installed application journeys remain separate acceptance.

These APIs are exclusively for off-chain applications. Use Pina or official
Solana program libraries for on-chain programs.

## Composition contracts

`UnionEncoder` selects an encoder from the value. `UnionDecoder` selects a decoder
from the enclosing bytes and initial offset. Neither writes or consumes an
implicit discriminator. Invalid selections fail before the payload operation.
No failed alternative is retried. Caller-defined enums preserve heterogeneous
values without unchecked casts or an untyped JSON value domain.

`Contextual` maps a successfully decoded value with its original bytes and start
offset. An explicit frame supplies its own bytes and frame-relative offset.
Mapping errors retain the inner decoder's consumed bytes and allocation claims.
Callbacks must bound their own allocations and keep size declarations stable.

`TupleEncoder` and `TupleDecoder` use borrowed runtime schemas. Tuple lengths must
match exactly. `RecordEncoder` and `RecordDecoder` use borrowed field names and
Kit-owned ordered maps. Fields follow schema order. Extra input keys are ignored.
Missing fields and duplicate schema names are rejected. Callers represent an
absent value explicitly in their chosen Rust value type. This replaces Dart's
implicit missing-key null and ambiguous repeated field names with explicit types.
Decoders claim output entries before allocation and share claims with children.

## Integer and text contracts

The optional `bigint-codecs` feature reuses `num-bigint` independently of fixed
points. `BigNumber<BigUint>` encodes unsigned integers. `BigNumber<BigInt>` uses
signed two's complement. Widths are positive byte counts with checked bit-width
arithmetic. Overflow is rejected. Both byte orders are explicit. Decoding claims
the wire width before allocating limbs. Encoding temporary storage follows the
supplied value's size and remains the caller's responsibility.

`Utf16BaseX` indexes alphabets by u16 code unit, preserving supplementary-character
positions and unpaired surrogates. The alphabet has 2 through 65536 positions.
Encoding accepts at most 4096 units. Decoding claims scratch digits and output
units against one shared budget. Duplicate symbols use their first index and
need not round-trip a later duplicate index. Leading zero units preserve leading
zero bytes. The existing scalar-string BaseX codec remains available.

`Utf16BaseXReslice` shares bit arithmetic with the scalar-string reslicing codec.
Its alphabet has exactly 2^bits positions, with 1 through 16 bits per symbol.
Encoding discards incomplete trailing bytes. Decoding zero-pads the last symbol.
Both UTF-16 APIs expose code units directly instead of silently replacing invalid
surrogate sequences during conversion to Rust strings.

Existing `Mapped<Number<f32>, ...>` composition provides the reference's f64-to-f32
presentation conversion, including IEEE overflow, underflow and signed zero.
Existing fixed-point presentation scales the exact value before converting to
f64. A finite tiny negative value can become negative zero. Non-finite output is
an explicit overflow error. This avoids the reference's intermediate infinity
division for large decimal scales. Exact fixed-point arithmetic stays unchanged.

## Address and hash contracts

`compare_addresses` exposes the existing compiler collation for canonical base58
text. Digits precede letters, and each lowercase letter precedes its uppercase
variant. This differs from the byte ordering supplied by `Address::Ord`.

`AccountLookupMeta` owns table provenance with an explicit u8 index and a
non-signer role. `InstructionWithAccounts<InstructionAccount>` can mix ordinary account
metadata with that provenance. `Message::compile_with_lookup_accounts` uses the
existing compiler and validates the exact table, index and address against its
supplied snapshots. Invalid explicit hints never silently become static accounts
or switch tables. Conflicting origins and hints on a merged signer, payer or
invoked program are errors. Ordinary `Instruction` and `Message::compile` calls,
including unannotated empty account vectors, retain their existing entry point.

This is a deliberate strict Rust adaptation. Pinned Dart `compile_accounts.dart`
selects the address-comparator-minimum of different supplied table origins,
retains the first index for the same table, and promotes signers/programs to
static accounts. Rust requires the caller to resolve those ambiguous explicit
assertions. Ordinary snapshot-only selection retains its existing deterministic
first-table behavior. Wire decoding retains the chosen lookup descriptor and
index; concrete loaded addresses still require authenticated caller snapshots.

Public sysvar fetching already composes `Client::fetch_encoded_account`,
`MaybeEncodedAccount::require` and `EncodedAccount::decode` with
`SysvarCodec<T>`, `OWNER_ADDRESS` and explicit decode limits. Retain the returned
`Contextual` evaluation context alongside the typed account. No network stack or
one-caller sysvar wrapper is added. Ordinary Clock and variable-length SlotHashes
callers exercise this exact composition with in-memory protocol fixtures; they
do not claim network execution.

The address constant module adds 36 program and mint literals and reexports the
13 existing sysvar literals. These account identities do not imply deployment on
every cluster. The pinned Dart native-loader literal decodes to 30 bytes and
cannot inhabit a validated address. Its corrected canonical literal is
`NativeLoader1111111111111111111111111111111`, from `solana-sdk-ids` 3.1.0.
No runtime dependency on that Solana crate is introduced.

The independent optional `keccak` feature uses RustCrypto Keccak-256 with original
0x01 padding. It differs from SHA3-256. The default core graph remains unchanged.

## Verification boundary

Fifteen new shared assertion groups cover selected operations, byte context,
runtime schemas, nested budgets, UTF-16 units and reslicing, literal integers,
lookup metadata, address display and collation, floating conversions, fixed-point
extremes, constants and original Keccak vectors. Unit 463's eight ordered
collection groups remain intact. The prior W-only proposal was v36 with 344 cases.
Ten additional source groups now cover lookup-origin compilation and public
sysvar fetch/decode composition. The actual integrated suite is `solana-kit-core-v37` with 354 cases, retaining
the original 321-case prefix. All 33 appended groups execute on every named host.
Current and Rust 1.89 native tests pass, including the compile-fail signer contract.
The maintained reports contain actual 354/354 and 353/354 execution output.
Independent unmerged wallet additions are outside this count.
