---
"solana_kit_core": patch
---

Match the pinned Dart compiler's address collation for static accounts, lookup
table precedence, loaded accounts and lookup descriptors. Mixed-case addresses
now produce the same frozen legacy, v0 and v1 message bytes.
