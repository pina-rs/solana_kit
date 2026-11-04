---
"solana_kit_core": minor
"solana_kit_signers": minor
---

# Add key grinding, constant-time equality and decompilation

Add caller-owned, finite-budget key grinding with validated base58 prefix and
suffix literals, predicate composition, distinct entropy and exhaustion errors,
and audited constant-time slice equality through `subtle`.

Add typed `Message::decompile` and `DecompiledMessage`: caller-owned lookup
snapshots preserve version, payer, instructions, duplicates, privileges, table
provenance, loaded ordering and V1 configuration without inventing lifetime
metadata. Recompilation reuses the existing compiler; canonical messages roundtrip
to their exact signed bytes. Grind signers consume export capability before they
are returned.
