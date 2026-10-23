---
solana_kit_core: minor
---

# Add opt-in transactional codec reads

Allow a decoder to commit cursor movement and allocation claims only when its
closure succeeds. Failed reads restore both values while external closure effects
remain visible. Ordinary frame semantics remain unchanged.

The private shared generator now represents tagged, framed and TLV data layouts
from complete pinned inputs. Exact generation goldens and independent malformed
encoding controls retain their bounded scope. This is off-chain client code.
