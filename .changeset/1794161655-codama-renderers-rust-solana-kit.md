---
codama_renderers_rust_solana_kit: patch
---

# Publish the Codama renderer with the SDK group.

Add `codama_renderers_rust_solana_kit` to the `sdk` MonoChange group so it
shares the primary `v*` version and release tag. Qualify and upload the package
from `.github/workflows/publish.yml` beside the crates.io publish path.
