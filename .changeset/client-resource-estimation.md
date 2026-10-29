---
solana_kit_client: minor
solana_kit_core: minor
---

# Estimate and tune transaction resource limits

Add optional client resource estimation over caller-owned RPC transports. Preserve
version-specific budgets, nonce lifetimes, explicit limits, observed usage, and
original errors. Add pure removal of version-one resource requests.
