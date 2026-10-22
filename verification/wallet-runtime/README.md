# Portable wallet assertions

This private crate exports five deterministic function groups. The central
conformance suite calls these exact functions through a path dependency on every
host. It has no wallet discovery, authorization UI, network or crypto backend.

The metadata and capability assertions derive from the frozen wallet's original
public API tests. The registry assertions preserve the independent Unit148
consumer's positive invariants,21 mutation combinations, nested dispatch,
destructor and disposal cases,31/32-pass boundaries, exhausted-reference counts
and recovery. Adaptations remove diagnostic printing and expose test functions.
The original author and independent consumer sources remain separately retained.

Run `cargo test --manifest-path verification/wallet-runtime/Cargo.toml --offline`.
Run `CARGO_NET_OFFLINE=true cargo xtask all` with the named host executables for
actual native, Chrome, both Wasmtime previews and pinned LeptosKit execution.

Assertions panic with their concrete mismatch. The host receives a successful
case only after every assertion in that group has executed. Wallet contracts
remain203 partial declarations. Per-asset upstream license provenance remains
pending. This SDK is exclusively for off-chain applications; on-chain programs
use Pina or official Solana libraries.
