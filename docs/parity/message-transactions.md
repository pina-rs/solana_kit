# Message and transaction contracts

This SDK is exclusively for off-chain client application code. Its portable
`no_std + alloc` core does not promise support for on-chain programs; use Pina
or official Solana program libraries for those.

The legacy/v0 slice preserves immutable signed message bytes and ordered signer
slots. Local construction requires explicit blockhash expiry or durable nonce
metadata. External wire decoding cannot recover that metadata and must retain an
unresolved token.
Incoming messages have a separate protocol validation path: local compiler
restrictions must not reject otherwise valid received messages.

The pinned Dart source is commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. Its compiler sorts addresses by
base58 text within privilege groups using a per-character rank of digits, then
`a, A, b, B` through `z, Z`, omitting characters outside the base58 alphabet.
An equal prefix sorts by length. It places the payer first, retains duplicate
instruction positions, and keeps invoked programs readonly and static.

The adjacent TSV contains all phase 3 source symbols, copied from the canonical
capability inventory. Pending rows are deliberate follow-ups, including v1's
message-first envelope, durable nonces, introspection, off-chain messages and
modifying, partial and sending signers. A legacy/v0 vertical slice does not
complete phase 3. No implementation or runtime proof is claimed by a pending row.

## Wire and signature evidence

`Message::compile`, `Message::decode`, and `Transaction` expose the vertical
slice. Inputs to compilation are owned instructions and lookup snapshots; returned
messages expose only immutable slices and copied metadata. `with_signature`
checks slot ownership and exact signed bytes. Cryptographic verification is an
explicit `crypto` feature operation; structural decoding alone does not establish
signature authenticity.

Table snapshots are ordered by base58 table address; when an account appears in
several snapshots the first table in that order wins, and the first matching index
within that table wins. Signers and invoked programs stay static. Account role
groups and loaded partitions use the pinned Dart compiler's base58 ordering.

The compiler uses this explicit collation independently of `Address::Ord`, whose
byte ordering is unchanged. Static privilege groups, competing table selection,
loaded table/account groups and lookup descriptors share the same rule. No host
locale or ICU dependency is required. The nine literal full compiler outputs in
`crates/solana_kit_core/tests/fixtures/address_collation.tsv` were produced by the
actual pinned Dart legacy/v0/v1 compiler. They cover mixed digits/case, multiple
programs, repeated instruction positions, privilege upgrades, the payer reused
as an authority, duplicate table entries and competing tables. Reversing snapshot
input preserves the result. See the adjacent fixture provenance. This correction
adds native evidence only; combined portable host execution remains separate.

Fixtures follow the pinned Dart message codec field layout and protocol-valid
constraints from cached official `solana-message` 4.4.1 (Apache-2.0). No official
implementation code was copied into the SDK. The deterministic signature fixture
uses RFC 8032 test 1's public seed, signing the explicit legacy message through
Node's OpenSSL-backed `crypto.sign` independently of Rust's Ed25519 implementation.
All fixtures are synthetic public data. The private oracle and its dependencies
are excluded from application dependency graphs.

The integrated wire parser and writer reuse `codecs::ShortU16`; compact writes
use a three-byte stack buffer and decoding uses the shared bounded cursor.
Codec truncation remains transaction `Error::Truncated`; noncanonical and
overflow counts remain `Error::InvalidLength`. Received wire acceptance and
packet/trailing-byte policy are unchanged. The four transaction cases execute
with all 31 primitive/codec cases as `solana-kit-core-v3` on every host in the
[runtime receipt](../verification/runtimes.md).

## Version-one client wire

`Message::compile_v1` uses the same immutable message and ordered signature owners.
`V1Config` derives its mask from optional typed fields, keeping the two priority
fee bits atomic. `HeapSize::new` validates the inclusive 32..256 KiB range and
one-KiB alignment before a configuration can contain a heap request. Priority fees
are whole lamports in a full-width u64. Absent compute and loaded-data requests
mean zero; absent heap means 32 KiB. Explicit zero remains distinguishable from
absence in received bytes. Unknown mask bits and partial priority pairs reject.

Version one has no lookup tables. The complete envelope is the signed message
prefix followed by exactly one 64-byte signature per required signer, with no
signature count prefix or padding. Its inclusive limit is 4096 bytes, including
reserved signature slots; legacy/v0 retain 1232. Version one also bounds required
signatures to 12, distinct addresses to 64, instructions to 64, and account
positions per instruction to 255. Received addresses cannot repeat. Compiler
privilege ordering and received protocol checks retain their separate contracts.

Three pinned Dart signed vectors and an independent literal envelope match the
public decode/encode API, strict crypto and official solana-message 4.4.1. The
[local validator proof](../verification/v1-wire.md) now constructs v1 messages and
signatures through the public SDK. It checks successful simulation, submission,
confirmation and version-one readback, signature rejection, exact 4096 success,
and the specific -32602/4097/max4096 error. A disabled-feature genesis accepts the
legacy control and explicitly rejects v1 as UnsupportedVersion. `getVersion`
alone does not advertise feature support. These are local fixture-node results.

The three version-one groups execute in the shared 48-case suite on native Rust,
real Chrome, Wasmtime Preview 1/2 and the pinned actual LeptosKit WasmHost. They
check the literal envelope, inline configuration, exact 4096/4097-byte bounds and
signature tampering; see the [runtime receipt](../verification/runtimes.md).
A 32-byte lifetime wire token does not prove
durable nonce creation, advance, expiry or rollback. All 746 canonical source IDs
remain present; bounded mappings stay partial and Phase 3 remains incomplete.

## Durable nonce construction

`Lifetime::DurableNonce` carries explicit token, nonce account and authority.
`DurableNonce::prepare_instructions` deterministically retains, replaces or
prepends the prerequisite advance. Compilation checks exact first-instruction
identities and roles. Received lifetimes remain unresolved.
`Message::nonce_advance` reports structural evidence without authenticating state.
The [nonce receipt](../../verification/durable-nonce/README.md) records actual
Dart literals, strict construction guarantees and remaining lookup, validator
and host proof. All 30 instruction meta sequences and compiled message literals
match after the shared pinned address comparator integration. Every literal
comparison requires exact byte equality. The original three collation failures
remain in local evidence.
