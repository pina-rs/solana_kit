---
solana_kit_core: minor
---

# Add portable Solana primitives and deterministic cryptography

Provide exact-length base58 addresses, hashes, signatures, account roles, and
owned ordered instructions without host dependencies. The optional crypto
feature adds SHA-256, seed/PDA derivation with curve and seed validation, and
deterministic Ed25519 signing with strict verification.

The initial implementation has native behavioral evidence and a shared,
host-neutral 25-case conformance suite. Other runtime execution is verified in
separate portability units. This changeset records release intent and does not
authorize publication.
