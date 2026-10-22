---
solana_kit_core: minor
---

# Add immutable version-one client transactions

Compile and decode version-one messages with typed inline configuration, exact
message-first signature envelopes, and the inclusive 4096-byte packet limit.
Preserve received signed bytes and keep legacy/v0's 1232-byte contracts. Optional
crypto signs and verifies the frozen message. Local validator feature support is
explicit evidence, not inferred from a node version. This SDK is exclusively for
off-chain application code; use Pina or official Solana program libraries on chain.

Durable nonce lifecycle, general signer capabilities and complete Phase 3 remain
outstanding. Publication stays disabled.
