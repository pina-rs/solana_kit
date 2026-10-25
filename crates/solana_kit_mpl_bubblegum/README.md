# Solana Kit Bubblegum

Typed Bubblegum instruction/state codecs, ordered privilege checks, bounded proof
accounts, leaf-canonical metadata, PDA derivation and optional Keccak/Merkle helpers.
The default dependency is Kit core. Enable `pda` for derivation and `hashing` for
Keccak-256. Callers own RPC, proof data, signing, authorization and execution.
The `tree` feature reuses Kit Compression geometry for initialization instructions.
The `das` feature composes Kit's typed RPC client with a caller-owned transport.
DAS response binding checks supplied identities and optional hash paths. It does
not authenticate the provider's root against bank state.

This is exclusively an off-chain application client. For Solana on-chain programs
use Pina or the official Solana program libraries.

The committed module is maintained through `bubblegum-generate OUTPUT` in the
private Rust generator. Account headers participate explicitly in both codec
directions. Normal publishing is disabled. Placeholder eligibility requires a
separate review of settled APIs, real callers and the complete publishing artifact.

The complete instruction model corrects incompatible Dart layouts rather than
emitting missing mandatory values. See the discrepancy table and third-party
notices. In particular, V1 metadata updates require a root, nonce, index, current
metadata and update arguments. Collection operations require their full ordered
authority account list. Hash-version bytes differ from account Borsh tags.

```rust
use solana_kit_mpl_bubblegum::can_transfer;
assert!(can_transfer(false, false));
assert!(!can_transfer(true, false));
```
