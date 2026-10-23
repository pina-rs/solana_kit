# Anchor source contract

The portable `solana_kit_anchor` crate implements the pinned Dart Anchor
runtime's five source families. The pin is
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`; all six exported library/source
file hashes match the canonical inventory. The complete
[120-ID receipt](anchor.tsv) retains every declaration, constructor, field,
constant and public operation. These rows describe implementation and source
differences. The [portable runtime receipt](../verification/anchor-runtime.md)
adds bounded execution evidence without claiming full source or ecosystem parity.

The pinned package exports IDL parsing, dynamic Borsh coding, discriminators,
SHA-256 and error resolution. It contains no asynchronous provider, program
builder, transaction submission, account metadata resolver, or PDA operation.
The coordinator corrected the unit brief's initial hypotheses after reading
these actual exports. Existing Kit `Address`, `Instruction`, codecs, client,
message and signer APIs remain the composition points. This crate adds no
network feature, runtime, Solana backend, or speculative provider adapter.

This SDK serves off-chain application clients. On-chain programs use Pina or
Solana's official program libraries. `no_std + alloc` is a portability property,
not an on-chain support promise.

## Value and wire contracts

All 15 source scalar types are represented exactly, including full-width
`u128`/`i128`, signed integers, IEEE floats and Kit-owned addresses. Bytes and
UTF-8 strings have u32 byte lengths; vectors have u32 element counts; fixed
arrays have no count prefix. Struct and instruction fields follow IDL order.
Enums use u8 tags and synthesized `fieldN` tuple names. The Rust value enum
expresses struct, enum and option states directly; there is no magic `__kind`
map key. Nested option tags remain distinguishable.

IDL discriminators are used verbatim, including empty, zero and non-eight-byte
prefixes. Sighash helpers compute the standard eight-byte namespaces separately.
Instruction and account decodes retain the source's prefix-reading behavior and
return consumed offsets; `ValueCodec::decode` is an explicit strict full read.
Unknown names, missing fields, wrong dynamic values, truncation and discriminator
mismatch remain errors. Encoding retains structured field/account attribution.

The source parses unsupported field types lazily. Rust retains each raw field
node and exposes bounded `Field::parsed_type`, `Field::parse`, `Variant::parse`,
`Type::parse`, and `Coder::codec_for`. Generic instantiation, aliases and wide
option prefixes are rejected when used. The named definitions are enums, so
unsupported scalar names cannot be constructed as valid typed primitives.

Every operation has caller-selected byte, aggregate item and nesting limits.
Nesting also has a hard limit of 128 to protect the native stack. Defined-type
cycles, zero-width vectors and malformed length prefixes cannot bypass limits.
Event operation limits cover total log bytes and entries and aggregate decoded
field allocations, in addition to each payload's wire budget.

## Event and error behavior

Event decoding consumes complete transaction logs. Only `Program data:` from the
currently executing IDL program is eligible. Foreign CPI payloads and forged
markers inside message logs are ignored before base64 decoding. Invocation depth,
root resets, matching completion, failed child restoration and missing context
follow the pinned source's rules. Check transaction success separately before
acting on decoded events. Unknown event discriminators are ignored; malformed
eligible base64 and recognized malformed payloads return errors.

The complete 77-entry standard error table preserves code order, names and
messages. IDL errors override the standard table at every code, retain empty
messages and supply the source's default message when absent. Signed 64-bit
unknown codes are retained. Public errors implement `Display` and the standard
error source chain.

## Deliberate source differences

Rust rejects fractional/negative array lengths, fractional error codes, and
fractional/out-of-range discriminator bytes. Dart sometimes truncates numbers or
wraps explicit discriminator bytes through `Uint8List`; the Rust boundary makes
those malformed documents errors. Exact integer-valued documents and the pinned
fixtures retain their bytes. JSON object keys follow
serde_json's last-value behavior; duplicate IDL entry names retain last-write
behavior and event declaration position. Rust type construction replaces the
source's invalid primitive/wide-option constructor states with typed variants.
Account/event enum values retain an explicit variant rather than deleting the
source's `__kind` field. These differences are recorded per family in the receipt.

## Evidence and limits

All 62 original Dart test bodies execute with the frozen unit 242 configuration
from an owned working directory. Fresh source-derived instruction, kitchen-sink,
account/enum and complete error table vectors supplement original literals.
Native Rust executes public wire, malformed-input, IDL, error/hash and event
provenance contracts. Ordinary current and MSRV consumers execute Kit instruction
composition and local dynamic codec behavior without flags or patches. Actual
producer commands and executable bytes are retained under ignored unit 258
captures. The discriminator guard-removal mutation owns a distinct target and
fails the intended mismatch test.

Three current Rust Wasm probes link. Their normal/build closure is checked against
a closed package list; actual core-module and component root imports are inspected
with source-compiled rejection controls. Those probes establish compile, link and
import boundaries. The separate [portable runtime receipt](../verification/anchor-runtime.md)
executes 43 complete bodies on native Rust, Chrome, both WASI previews and the
actual LeptosKit interpreter. Independent collection-depth controls reject the
guardless producer. The declaration ledger still leaves full source parity open.
HTTP, validator, emulator, simulator and installed-device journeys remain separate
requirements. Root owns independent acceptance and public integration.
