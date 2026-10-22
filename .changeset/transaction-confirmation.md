---
solana_kit_client: minor
---

Add explicit transaction confirmation policy and caller-owned observation with
bounded awaited cleanup. Preserve exact transaction bytes and reported signature
checks when submitting. Subscription factories and authenticated nonce RPC
observations remain pending; no clock, executor or retry policy is implicit.
