---
"solana_kit_client": patch
---

Reject payer and loaded account indices as top-level raw JSON invoked programs,
matching core wire decoding while preserving loaded programs in inner calls.
