# solana_kit_core

Portable off-chain client types for Solana addresses, hashes, signatures,
ordered instructions, bounded binary codecs, and immutable legacy/v0 transactions.
The default dependency graph contains only `bs58`. The optional `crypto` feature
adds SHA-256, validated program-derived addresses, and deterministic Ed25519
signing and strict verification through RustCrypto implementations.

This crate is only for off-chain application code. Do not use it in Solana
on-chain programs; use Pina or official Solana program libraries instead.
`no_std + alloc` enables portable clients and does not imply on-chain support.

The crate uses `no_std` plus `alloc`, with no executor, JavaScript, filesystem,
network, or random-number source. Callers supply signing seeds and own the
security and erasure of their seed storage. Signing keys erase their owned
secret material on drop through `ed25519-dalek`'s `zeroize` feature.

The `transaction` module compiles and strictly decodes immutable legacy/v0
messages, retains ordered signature slots, and encodes packets within 1232 bytes.
Local compilation requires explicit blockhash expiry metadata. Received wire
preserves its exact signed bytes and records the lifetime token as unresolved,
because expiry height is absent from the wire. Optional `crypto` signs and
verifies those frozen bytes; structural decoding alone does not authenticate
signatures. V1, durable nonces and general signer capabilities remain pending.

```rust
use solana_kit_core::{AccountMeta, AccountRole, Address, Instruction};

let program = Address::from_bytes([0; 32]);
let account = AccountMeta {
    address: Address::from_bytes([1; 32]),
    role: AccountRole::WritableSigner,
};
let instruction = Instruction {
    program_address: program,
    accounts: vec![account],
    data: vec![1, 2, 3, 4],
};
assert!(instruction.accounts[0].role.is_signer());
```

Parsing validates exact decoded byte lengths. An address is a 32-byte value;
parsing alone does not assert that it is a usable Ed25519 verification key.
Program-derived address creation rejects points on the Ed25519 curve and enforces
Solana's 16-seed, 32-byte-per-seed limits. Bump search reserves one seed slot and
tries every bump from 255 through 0.

The `codecs` module encodes exact-width integers through 256 bits, IEEE 754
floats, canonical short-u16, bytes, strict UTF-8, booleans, sequences, tuples,
options, constants, and byte-length frames. Integers use explicitly selected
little or big endian order; the 256-bit types hold exact wire bytes and do not
provide arithmetic. Floats preserve NaN payloads, infinities, and signed zero.

`Decoder::decode` consumes the full input. `decode_prefix` accepts an absolute
offset and returns the next offset. Every decoding operation takes explicit
input-byte and total allocated-element limits, shared by nested frames. Errors
can leave cursors advanced or output partly written. See the codec module docs
for error and composition contracts. `no_std + alloc` supports portable clients;
it is not an on-chain deployment promise.

```rust
use solana_kit_core::codecs::{Decoder, Encoder, Limits, SizePrefix, Utf8};

let codec = SizePrefix(Utf8);
let bytes = codec.encode("hé", 7)?;
assert_eq!(bytes, [3, 0, 0, 0, b'h', 0xc3, 0xa9]);
assert_eq!(codec.decode(&bytes, Limits::new(7, 3))?, "hé");
# Ok::<(), solana_kit_core::codecs::Error>(())
```
