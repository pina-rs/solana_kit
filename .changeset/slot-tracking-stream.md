---
solana_kit_client: minor
---

# Combine initial and subscribed RPC values by slot

Add an optional reactive stream adapter that preserves complete RPC contexts,
accepts equal slots, rejects older values, and owns both source lifetimes. Retain
nonterminal typed failures with bounded pending outputs and conditional Send.
