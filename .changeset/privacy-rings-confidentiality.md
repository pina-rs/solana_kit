---
solana_kit_privacy_rings: minor
---

# Harden Privacy Rings prover confidentiality

`PrivacyProverClient` now refuses any prover base URL that carries a query, so an API key can never survive in the origin or a request target, and host, port, and bracket tails must parse before the loopback decision. `seal_prover_request` draws a fresh HPKE ephemeral key from a caller-supplied `rand_core` generator instead of accepting a reusable seed, the DHKEM key schedule rejects an all-zero X25519 shared secret in both the seal and open paths, and the `Debug` output of witness and sealed-request types no longer prints nullifier secrets, note values, blindings, plaintexts, or response keys.
