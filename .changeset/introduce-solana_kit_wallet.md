---
solana_kit_wallet: major
---

# solana_kit_wallet: Wallet Standard and MWA

Portable Wallet Standard contracts for off-chain applications: typed feature
registries, authorization and signing capabilities, account selection, and
caller-owned cancellation without an executor.

The optional `mwa` feature adds portable association URIs, P-256 association
signing, ES256 JWS, HELLO negotiation, HKDF-SHA256 session keys, AES-128-GCM
frames, ordered sessions, JSON-RPC contracts, and SIWS messages. Returned
signatures remain raw bytes; verification stays the caller's responsibility.
