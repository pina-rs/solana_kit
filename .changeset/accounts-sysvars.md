---
solana_kit_core: minor
---

# Add accounts, sysvars and typed program failures

Decode account snapshots with explicit missing states, owner validation and
metadata preservation. Read and write all nine pinned sysvar layouts with
bounded allocations and strict full decoding, including historical blockhash
fees. Preserve program error codes, instruction positions and original log facts.
These deterministic APIs serve off-chain application clients and perform no RPC.
