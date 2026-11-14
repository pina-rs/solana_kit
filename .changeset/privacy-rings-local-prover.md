---
solana_kit_privacy_rings: patch
---

# Verify ignored local-prover tests against pinned binary

`local_prover_accepts_a_loopback_witness` and `local_prover_proves_the_published_witness` now fail when `PRIVACY_RINGS_LOCAL_PROVER_URL` is unset instead of silently passing, and the invalid-witness case expects the pinned prover's HTTP 400. `.github/workflows/privacy-rings.yml` downloads and checksum-verifies the pinned `prover-linux-x64-cli-v0.10.0` binary and both proving keys, starts an owned loopback prover, and runs both ignored tests. No hosted Helius endpoint or secret is used.
