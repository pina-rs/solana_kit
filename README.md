# solana_kit

An off-chain Rust Solana client SDK for native applications, browser WebAssembly,
WASI, and embedded WebAssembly applications such as LeptosKit.

Solana Kit is only for off-chain application code. Do not use it in Solana
on-chain programs; use Pina or official Solana program libraries instead.
`no_std + alloc` enables portable clients and does not imply on-chain support.

The first implemented slice is `solana_kit_core` 0.1.0: portable addresses,
hashes, signatures, ordered instructions, account roles, and optional deterministic
cryptography. Its shared 25-case suite has [native execution evidence](docs/verification/core-native.md).
The core component is unreleased; the published `solana_kit` 0.0.0 umbrella remains
a dependency-free name reservation with no SDK APIs. Future runtime support will
be claimed only after the corresponding execution tests pass.

- [Delivery plan](docs/planning/plan.md)
- [Architecture and API proposal](docs/architecture/design.md)
- [Reference findings and reuse decisions](docs/planning/reference-findings.md)
- [Package parity ledger](docs/planning/parity.tsv)
- [Pinned reference sources](docs/planning/references.toml)
- [Tasks](docs/planning/tasks.md) and [decision trail](docs/planning/decisions.tsv)
- [Reproducible planning checks](docs/planning/verification.md)
- [Core API and feature guide](crates/solana_kit_core/README.md)

Crates will use underscore names, including `solana_kit` for the umbrella crate
and `solana_kit_*` for its components. The target is the entire Dart Solana Kit
ecosystem, delivered in independently verified phases. Package boundaries and
APIs will follow Rust's ownership model and dependency needs.

Released under [Unlicense](LICENSE). Reused third-party material retains its
required license and notices.

Monostyle enforces readability and complexity scores of at least 90. Run
`monostyle check . --fail-under 90` before sharing changes. Normal scoring rules
apply to authored and generated code.
