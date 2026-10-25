---
solana_kit_core: minor
solana_kit_native: minor
solana_kit_browser: minor
---

Add an explicit exportable key owner with zeroizing backups and caller-selected entropy.
Native applications can opt into secure-provider entropy and private, atomic backups that never overwrite an existing destination.
Browser applications can opt into caller-owned Web Crypto entropy without a fallback random source.
Ordinary signing keys retain their non-exporting API.
