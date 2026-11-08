---
solana_kit_signers: major
---

# solana_kit_signers: executor-neutral signing capabilities

Portable asynchronous signing contracts for message, transaction, modifying,
partial, and sending signers. The crate is `no_std + alloc` and has no runtime,
entropy, filesystem, or network dependency.

Each logical resource gets a `SignerCapability`; `TransactionSigners`
deduplicates repeated handles, rejects two handles claiming one address, and
preserves local or native futures. Every call receives its own configuration,
including `aborted` and `min_context_slot`, unchanged across composed pipelines.
