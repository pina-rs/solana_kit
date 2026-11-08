---
solana_kit_client: major
---

# solana_kit_client: executor-neutral typed RPC

Typed balance, blockhash, account, token, node metadata, economics, history,
submission, and simulation operations through one executor-neutral protocol
owner. Native and local futures share the same client without a runtime,
network, JavaScript, or ambient-randomness dependency.

Optional features add version-aware resource estimation, caller-driven reactive
stores, slot-tracking streams, and typed subscription families. Callers own the
transport, deadlines, cancellation, and any spawned driver.
