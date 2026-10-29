---
solana_kit_helius: minor
---

# Resolve exact priority fees and buffered compute units

Add pure `transactions::resolve_priority_fee` with exact decimal flooring,
consistent rate and total caps, checked rounded-up lamports and typed wire-range
errors. Add `transactions::buffer_compute_units` with the source binary ceiling,
positive floor and runtime maximum, rejecting non-finite buffer arithmetic.
