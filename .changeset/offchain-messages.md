---
solana_kit_core: minor
---

# Add bounded off-chain application message signing

Construct and strictly decode immutable V0 and V1 application messages with
explicit byte and allocation budgets. Preserve exact message bytes, encode
ordered optional signature slots, and sign or verify deterministically with the
optional crypto feature. V0 content limits remain distinct from transaction
packet limits. Publication remains disabled.
