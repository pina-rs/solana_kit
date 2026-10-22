---
"solana_kit_signers": minor
---

# Add portable signer capability contracts

Add local asynchronous message, partial, modifying, and sending signer contracts
with immutable byte snapshots and deterministic keypair support. Keep submission caller controlled.

Pipeline callers create a `SignerCapability` handle for each logical resource.
Repeating one handle deduplicates, while distinct handles claiming one address
conflict even when their backends are zero-sized.

Automatically select composite roles across explicitly registered heterogeneous
backends, with local and native Send adapters. Add bounded ordered transaction
batches while preserving logical handle identity and signature snapshot checks.
