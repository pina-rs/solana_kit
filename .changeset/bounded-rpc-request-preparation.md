---
solana_kit_client: patch
---

Bound buffered JSON during RPC parameter and complete-frame serialization. Reject
oversized requests before copying the complete caller value while preserving
exact wire bytes, scalar-parameter validation and existing error distinctions.
