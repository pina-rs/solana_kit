---
solana_kit_native: minor
---

# Deploy programs from ordered explicit artifact roots

Select the first complete same-root program/keypair pair in caller order and snapshot
only its optional IDL. Verify a bounded keypair JSON file before program reads and
reuse sequential deployment with explicit limits, typed errors and cancellation.
