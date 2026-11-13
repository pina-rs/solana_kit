---
solana_kit_privacy_rings: minor
---

# Build witnesses for the pinned shapes and the indexed prover

`confidential_transfer_witness` now sizes the public signer vector at the pinned shape width `min(n, 64 - 4 - n) + 1` instead of input count plus one, so the pinned 32/40/48/49-input confidential shapes and the 48x8 and 48x16 confidential EdDSA shapes build, and it refuses rail and shape pairs no pinned proving key serves. `indexed_proof_request` now takes the `TransferWitness` itself and derives the unresolved prepared witness and the public-input prefix the indexed prover accepts, validating trees, lookups, and slots against the pinned envelope before transport, and `prove_indexed` polls a queued job at `/prove/{circuit}/status` instead of an unregistered route. A tree slot beyond the word width is rejected before any indexing on every target.
