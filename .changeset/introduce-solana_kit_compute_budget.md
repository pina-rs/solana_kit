---
solana_kit_compute_budget: major
---

# solana_kit_compute_budget: typed compute-budget instructions

Strict bounded codecs, owned Kit instruction builders, parsers, and instruction
list helpers for all five pinned Compute Budget layouts. Every builder returns a
normal `solana_kit_core::Instruction` and every parser enforces the exact wire
shape.

`RequestUnits` remains available as the deprecated historical layout, but
building its bytes does not imply validator acceptance. Heap alignment and
network limits remain validator policy.
