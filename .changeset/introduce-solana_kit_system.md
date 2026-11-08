---
solana_kit_system: major
---

# solana_kit_system: all fourteen System instructions

Bounded data codecs, typed account builders, strict parsers, custom numeric
errors, and nonce account snapshots for the pinned System Program.

Every data type's `instruction` method constructs a fixed System invocation,
supplies Rent and RecentBlockhashes automatically, preserves ordered duplicate
accounts, and makes optional payer and signing-base accounts explicit. Seeded
builders validate UTF-8 byte length and base signatures while leaving PDA
derivation to the caller.
