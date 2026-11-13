---
solana_kit_privacy_rings: minor
---

# Require ownership and commitment verification before private balances

`open_output` remains an unauthenticated inspection helper. `verify_note` (poseidon) checks an expected owner and recomputes the commitment using the pinned UTXO domain, admitting only data-free confidential and proofless notes; ring- or data-bearing shapes fail closed. `private_balances` now takes `VerifiedNote` values and sums by verified commitment, so one commitment counts once across leaf relabels and a spent commitment stays excluded. `ProoflessNote` retains trailing metadata. `TrackedNote` is no longer a balance input.