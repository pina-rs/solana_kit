---
solana_kit_privacy_rings: patch
---

# Add isolated CI coverage for privacy rings

Append feature-isolation, MSRV, 32-bit, and WASM build jobs to
`.github/workflows/privacy-rings.yml` while preserving the checksum-pinned
local prover job. Document the resulting coverage and its limits in the crate
README and service docs.
