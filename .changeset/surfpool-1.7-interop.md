---
solana_kit_surfpool: patch
---

Use default deployment chunks that fit the client wire budget and Surfpool
1.7.0's HTTP body cap. Preserve exact caller-selected chunks and typed budget
errors without silently resizing writes.
