---
solana_kit_core: major
---

# solana_kit_core: portable Solana types and wire formats

The foundational off-chain crate: validated addresses, hashes, signatures,
ordered instructions, account roles, bounded binary codecs, account snapshots,
sysvar layouts, program errors, exact fixed points, immutable legacy/v0/v1
transactions, and off-chain application messages.

The default graph depends only on `bs58`. Optional features add SHA-256,
validated program-derived addresses, deterministic Ed25519 signing and strict
verification, original Keccak-256, arbitrary-width integer codecs, and text
codecs. The crate is `no_std + alloc`, owns no executor or entropy source, and
never claims on-chain support.
