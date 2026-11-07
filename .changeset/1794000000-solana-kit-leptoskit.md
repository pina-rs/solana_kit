---
solana_kit_leptoskit: patch
---

# Depend on LeptosKit 0.3.0 from crates.io

Replace the unpublished Git pin with the published `=0.3.0` release for the
bridge, FFI, native, wasm and verification dependencies. The crate now declares
Rust 1.95, the minimum LeptosKit 0.3.0 supports. The public transport, socket,
REST and wallet APIs are unchanged; the upgrade adapts only the dependency
source and the generated-wallet verification harness.
