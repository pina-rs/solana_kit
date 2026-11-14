---
solana_kit_privacy_rings: minor
---

# Verify note ownership before private balances

`open_output` remains an unauthenticated inspection helper. `verify_note` (`crypto` + `poseidon`) checks an expected owner and recomputes the commitment using the pinned UTXO domain, admitting only data-free confidential and proofless notes; ring- or data-bearing shapes fail closed. `private_balances` now takes `VerifiedNote` values and sums by verified commitment, so one commitment counts once across leaf relabels and a spent commitment stays excluded. `ProoflessNote` retains trailing metadata. `TrackedNote` is no longer a balance input.

Reject conflicting leaf and commitment representations before spent filtering, including cross-rail and different-mint aliases. Require a trusted mapping for the exact proofless mint. Preserve raw proofless tails for inspection while admitting only the six canonical empty option tags. Migrate nullifier maps to commitment keys so leaf relabels cannot bypass filtering. A hash match still requires trusted commitment/tree context and is not an inclusion proof.
