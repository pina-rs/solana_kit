# solana_kit_core

Host-independent Solana addresses, hashes, signatures, and ordered instructions.
The default dependency graph contains only `bs58`. The optional `crypto` feature
adds SHA-256, validated program-derived addresses, and deterministic Ed25519
signing and strict verification through RustCrypto implementations.

The crate uses `no_std` plus `alloc`, with no executor, JavaScript, filesystem,
network, or random-number source. Callers supply signing seeds and own the
security and erasure of their seed storage. Signing keys erase their owned
secret material on drop through `ed25519-dalek`'s `zeroize` feature.

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
