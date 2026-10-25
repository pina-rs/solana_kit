# Third-party notices

The renderer's original Rust and TypeScript source uses the Unlicense.

The NPM runtime imports these unmodified packages. Their MIT license notices
remain in their separately installed distributions:

- `@codama/nodes`1.10.2.
- `@codama/visitors-core`1.10.2.
- `@codama/fragments`0.1.5.

The bundled engine is built from `tools/codama-renderer` and the exact Cargo lock.
Its normal dependencies include `serde_json`1.0.145, `sha2`0.10.9,
`prettyplease`0.3.0, `syn`3.0.9 and `solana_kit_core`0.1.0
with its default empty feature set. Core's notices are maintained in
`crates/solana_kit_core/THIRD_PARTY_NOTICES.md`. The private `verification` feature
and its Solana oracle development dependencies are not part of the bundled
engine build.

The actual production WASI build's normal and build packages are recorded in
`licenses/engine-dependencies.json`. Their complete supplied legal files are
preserved under `licenses/engine/`, including the additional Unicode terms.
Rust 1.99.1 library notices and supplied license texts are preserved under
`licenses/rust-1.99.1/`. These library notices include the standard library's
third-party notices; they do not include the compiler's unrelated components.

Package preparation checks the exact built engine, archive members and notices.
Registry publication remains a separate release-owner operation.

The included MIT texts for `prettyplease`0.3.0 and `syn`3.0.9 are under
`licenses/`. They are selected under those packages' MIT OR Apache-2.0 terms.
