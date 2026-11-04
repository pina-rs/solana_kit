# Migration example checks

This private workspace compiles the maintained
[Dart-to-Rust migration guide](../../docs/migration/from-dart.md) as crate
documentation. The guide's Rust blocks are doctests; the browser wallet recipe is
a separate example target because it requires `wasm32-unknown-unknown`.

```sh
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 \
  cargo +1.99.1 test --manifest-path verification/migration-examples/Cargo.toml --locked --offline
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 \
  cargo +1.99.1 clippy --manifest-path verification/migration-examples/Cargo.toml \
  --all-targets --locked --offline -- -D warnings
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 \
  cargo +1.99.1 check --manifest-path verification/migration-examples/Cargo.toml \
  --target wasm32-unknown-unknown --example browser_wallet --locked --offline
```

The package is never published. It performs no network, signing, device or
publication work; see the guide for the exact compiled inventory and limits.
