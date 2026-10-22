---
solana_kit_client: patch
---

# Preserve signature execution failures

Deliver valid failed-transaction signature notifications through the shared
typed error decoder. Preserve exact future error JSON and keep other
subscriptions active. Clarify that stale-generation frames do not terminate
the replacement connection or require closing its socket.
