---
solana_kit_helius: minor
---

# Estimate exact compute units from transaction wire

Add `transactions::get_compute_units` for one bounded simulation of caller-owned
transaction bytes with explicit read constraints. Preserve exact `u64` units and
evaluation context, including a valid zero report. Surface execution failure,
missing or null units, stale context and shared RPC errors through typed,
redacted errors.
