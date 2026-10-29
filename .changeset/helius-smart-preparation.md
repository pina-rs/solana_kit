---
solana_kit_helius: minor
---

# Prepare unsigned Helius smart transactions

Add `transactions::create_smart_transaction` with caller-owned instructions,
payer identities, simulation lifetime and lookup snapshots. Simulate before
recommended account-key pricing and final blockhash refresh; reuse exact buffer
and fee caps, then compile unsigned v0 price/limit instructions with the real
refreshed expiry. Preserve typed failures and cancellation without signing or
submitting the transaction.

Reconcile the retained producer against current client/signup/TLS interfaces;
preserve its shared behavioral predicates through fallible source bodies and
append one group without changing the accepted prefix. Native-only capture emits
actual executed outcomes for root-owned golden regeneration; normal verification
continues to require exact goldens. All new execution remains pending Root/CI.
