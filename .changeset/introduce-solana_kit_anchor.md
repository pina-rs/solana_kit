---
solana_kit_anchor: major
---

# solana_kit_anchor: runtime Anchor IDL coding

Encode and decode Anchor IDL 0.30 instructions, accounts, and events at runtime.
Exact-width dynamic values, explicit IDL discriminators, field-attributed encode
failures, and standard or program-defined error resolution are supported.

The only dependency graph is `no_std + alloc` and it starts no executor or
network work. Every operation takes explicit byte, item, and depth budgets, and
an unsupported generic field fails only when used rather than making the whole
IDL unreadable.
