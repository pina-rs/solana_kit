---
solana_kit_native: minor
solana_kit_surfpool: patch
---

Add optional native Surfpool deployment from borrowed program bytes or an explicit
regular file path, with finite caller-selected read limits. Reuse sequential portable
program writes before opening the optional IDL file, preserve exact JSON fields, and
retain partial writes and typed causes on later failures or cancellation.
Preserve literal unknown JSON keys, including serde_json's internal token names,
without changing the parser's existing nesting ceiling.
Allow the permanent portable crate in SDK dependency manifests while keeping
normal implementation publishing disabled in Monochange configuration.
