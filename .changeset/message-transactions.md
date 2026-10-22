---
solana_kit_core: minor
---

# Add immutable legacy and v0 transaction construction

Compile instructions with explicit blockhash expiry metadata, deterministic
privilege merging and lookup tables. Decode bounded external wire, preserve exact
signed bytes, keep ordered signature slots immutable, and encode packets within
the legacy/v0 packet limit. Optional deterministic crypto signs and verifies
those frozen bytes. This is an off-chain client SDK.

The phase 3 ledger keeps v1, durable nonces, off-chain messages, introspection and
signer capabilities explicitly pending. This slice does not claim full phase 3
parity. Publication remains disabled.
