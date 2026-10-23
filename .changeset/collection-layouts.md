---
solana_kit_core: minor
---

# Add collection termination and configurable optional layouts

Decode collection markers at item boundaries with explicit required, optional,
or omitted policies. Configure absent-value bytes and numeric boolean storage
using ordinary Rust Option values. Permit exhausted count prefixes explicitly
while retaining errors for partial prefixes and shared allocation limits.
