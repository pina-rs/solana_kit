# solana_kit

An off-chain Rust Solana client SDK for native applications, browser WebAssembly,
WASI, and embedded WebAssembly applications such as LeptosKit.

Solana Kit is only for off-chain application code. Do not use it in Solana
on-chain programs; use Pina or official Solana program libraries instead.
`no_std + alloc` enables portable clients and does not imply on-chain support.

The first implemented slice is `solana_kit_core` 0.1.0: portable addresses,
hashes, signatures, ordered instructions, account roles, bounded binary codecs,
immutable legacy/v0/v1 transactions, account snapshots, sysvar layouts, program
errors, optional exact fixed points, and deterministic cryptography. The generated
`solana_kit_system` slice adds bounded data codecs, typed account builders and strict
parsers for all fourteen pinned System instructions, nonce account snapshots,
and numeric custom errors. `solana_kit_client` adds typed balance, latest-blockhash and full-base64
account reads and all nineteen node metadata operations through one executor-neutral
protocol owner. The shared 59-case suite has
[runtime execution evidence](docs/verification/runtimes.md).
The core component is unreleased; the published `solana_kit` 0.0.0 umbrella remains
a dependency-free name reservation with no SDK APIs. This bounded codec slice
leaves Phase 2 incomplete; see its [capability receipt](docs/verification/core-codecs.md).
The [transaction ledger](docs/parity/message-transactions.md) records 33 partial
and 713 pending capabilities; Phase 3 remains incomplete.
The [account/sysvar receipt](docs/verification/accounts-sysvars.md) maps 151
declarations and retains all 78 unresolved declarations explicitly.
The [fixed-point receipt](docs/parity/fixed-points.md) maps 121 bounded equivalents
and retains five remaining declarations out of 126 canonical IDs.
The [System client receipt](docs/verification/system-client.md) records 173
implemented mappings and 68 partial or pending declarations. Validator execution
and durable nonce transaction journeys remain unfinished.

Six portable RPC protocol groups execute with memory transports on every core
host. The native adapter provides bounded HTTP/1 and TLS with caller-owned
Tokio execution, deadlines, cancellation and authentication. Its
[native networking receipt](docs/verification/native-http.md) records actual local
server journeys. The explicit `solana_kit_browser` adapter provides bounded Window
Fetch HTTP with local futures, deadlines, cancellation, and caller authentication.
Its [integration receipt](docs/verification/browser-http-integration.md) records
actual Chrome journeys. Guest networking, browser workers, remaining RPC operations,
subscriptions and installed application journeys remain separate work; see the
[RPC receipt](docs/parity/rpc-foundation.md) and
[node operation receipt](docs/verification/rpc-node.md).

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
