---
solana_kit_dapp_publisher: minor
solana_kit_dapp_publisher_cli: patch
---

Add deterministic normalized publication models, typed portal projections,
checkpoint and status precedence, and progress messages that omit provider text.
Require caller-supplied timestamp fallbacks and retain unknown backend values for
explicit inspection. Fail publication identity generation when system entropy is
unavailable instead of using zero-filled identifiers.
