---
codama_renderers_rust_solana_kit: patch
---

# Enable npm registry publication for the Codama renderer.

Turn on MonoChange release tags for `codama_renderers_rust_solana_kit`, keep
qualification and upload in `.github/workflows/npm-publish.yml` (external
publish mode), and dispatch that workflow from the release job. The first
registry upload still needs a one-shot `NPM_TOKEN` or an existing trusted
publisher; later uploads use OIDC.
