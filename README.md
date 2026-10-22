# solana_kit

A Rust Solana toolkit for native applications, browser WebAssembly, WASI, and
embedded WebAssembly applications such as LeptosKit.

The project is at the design and reference-audit stage. The only current package
is an empty `solana_kit` 0.0.0 name-reservation placeholder, with no SDK APIs.
Runtime support will be claimed only after the corresponding execution tests pass.

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

Released under [Unlicense](LICENSE). Reused third-party material retains its
required license and notices.

Monostyle enforces readability and complexity scores of at least 90. Run
`monostyle check . --fail-under 90` before sharing changes. Normal scoring rules
apply to authored and generated code.
A lightweight Rust Solana Kit ecosystem for native, browser WebAssembly, and embedded Wasm runtimes.
