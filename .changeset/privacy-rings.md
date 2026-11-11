---
solana_kit_privacy_rings: minor
---

# Add a Privacy Rings client on Kit types

Deposit, confidential transfer, and withdraw build shielded-pool instructions with Kit addresses. The crypto feature decrypts viewing-key payloads and derives the published PDAs. Typed methods cover the published Privacy Rings indexer RPC. `PrivacyProverClient` posts a witness to the remote prover and attaches the compressed proof.
