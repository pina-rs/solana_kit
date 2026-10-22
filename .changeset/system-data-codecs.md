---
solana_kit_system: minor
---

# Add System instruction data codecs

Provide bounded encode and strict decode for all fourteen pinned System
instruction-data layouts, including account creation with prefunding. Use exact
u64 amounts and u64 UTF-8 byte-length prefixes. This initial off-chain client
slice provides data codecs; account builders, nonce account decoding, PDAs and
program errors remain separate work. Publication remains disabled.
