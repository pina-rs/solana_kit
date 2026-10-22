# Message and transaction contracts

This SDK is exclusively for off-chain client application code. Its portable
`no_std + alloc` core does not promise support for on-chain programs; use Pina
or official Solana program libraries for those.

The legacy/v0 slice preserves immutable signed message bytes and ordered signer
slots. Local construction requires explicit blockhash expiry metadata. External
wire decoding cannot recover that metadata and must retain an unresolved token.
Incoming messages have a separate protocol validation path: local compiler
restrictions must not reject otherwise valid received messages.

The pinned Dart source is commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. Its compiler sorts addresses by
base58 text within privilege groups, places the payer first, retains duplicate
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
