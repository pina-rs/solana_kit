# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## codama_renderers_rust_solana_kit [0.1.1](https://github.com/pina-rs/solana_kit/releases/tag/codama_renderers_rust_solana_kit/v0.1.1) (2026-11-05)

### Features

- **Preserve numeric widths and byte orders.** Add i32, u128, i128 and f32, retain scalar, prefix and u32 instruction discriminator byte order, and use existing Kit codecs for checked big-endian array, size and option prefixes. Float-bearing clients provide PartialEq and Display without impossible Eq or Hash implementations. Normal registry publishing remains disabled.
  _Owner:_ Ifiok Jr. · _Introduced in:_ [bdbe26d](https://github.com/pina-rs/solana_kit/commit/bdbe26d430364f9d1bdfda060d84718e5b2c4be4)

#### Add a Codama renderer that uses one shared Rust engine to generate off-chain

_Owner:_ Ifiok Jr. · _Introduced in:_ [bc6c913](https://github.com/pina-rs/solana_kit/commit/bc6c9130c54b7337ccd45a2f0f1e505401e9b8d3)

Solana Kit program clients. The NPM package bundles a maintainer-built WASI engine
and supports repeated generation while preserving unrelated output files.
Publication remains disabled until packed-package and downstream verification.
