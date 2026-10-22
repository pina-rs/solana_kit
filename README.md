# solana_kit

A Rust Solana toolkit for native applications, browser WebAssembly, WASI, and
embedded WebAssembly applications such as LeptosKit.

The project is at the design and reference-audit stage. No SDK crates have been
implemented or published yet. Runtime support will be claimed only after the
corresponding execution tests pass.

- [Delivery plan](docs/planning/plan.md)
- [Architecture and API proposal](docs/architecture/design.md)
- [Reference findings and reuse decisions](docs/planning/reference-findings.md)
- [Package parity ledger](docs/planning/parity.tsv)
- [Pinned reference sources](docs/planning/references.toml)
- [Tasks](docs/planning/tasks.md) and [decision trail](docs/planning/decisions.tsv)
- [Reproducible planning checks](docs/planning/verification.md)

Crates will use underscore names, including `solana_kit` for the umbrella crate
and `solana_kit_*` for its components. The target is the entire Dart Solana Kit
ecosystem, delivered in independently verified phases. Package boundaries and
APIs will follow Rust's ownership model and dependency needs.
A lightweight Rust Solana Kit ecosystem for native, browser WebAssembly, and embedded Wasm runtimes.
