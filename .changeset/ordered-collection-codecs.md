---
solana_kit_core: minor
---

# Preserve insertion order in map and set codecs

Encode owned ordered maps and sets through existing sequence framing. Decoded
duplicate keys replace values in their original position; duplicate set values
retain their first appearance. Original wire items retain their shared allocation
claims, and normalization reuses the decoded storage without cloning values.
