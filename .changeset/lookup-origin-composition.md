---
solana_kit_core: patch
---

Carry explicit lookup-table account origins through heterogeneous instruction
inputs and the existing message compiler, rejecting inconsistent snapshots,
conflicting hints and static-account privilege conflicts. Keep ordinary compiler
callers source compatible. Maintain public Clock and SlotHashes fetch/decode
composition callers through the existing client transport and core codecs.
