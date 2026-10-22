# Portable core native verification

Executed on 2026-10-22, macOS ARM64, with Rust/Cargo stable 1.99.1. The
`feat/portable-core` implementation is based on
`fafc32fe09e87d5b51b41670a53dbafde34a22ea`.

This receipt establishes native behavior for the initial deterministic primitive
slice. Plain Wasm library compilation is additional compile-only evidence.
Browser, wasmi, and WASI execution are separate verification units and are not
established by this receipt.

## Implemented behavior

`solana_kit_core` owns exact-byte addresses, hashes, and signatures; bounded,
strict base58 parsing/display; ordered owned instructions; account roles and
privilege merging. It uses `no_std` plus `alloc` and forbids unsafe code.

The optional `crypto` feature adds SHA-256, direct and canonical-bump PDA
derivation with seed/curve validation, seeded-address derivation with owner-domain
validation, and seed-based Ed25519 signing and strict verification. The key type
exposes no secret export or debug representation, and its dependency's `zeroize`
feature erases owned key material on drop. Caller-supplied seed storage remains
the caller's responsibility. No entropy capability is required or invoked.

The `solana_kit` umbrella remains its dependency-free reservation. Nothing in
this unit was published to a registry.

## Shared conformance contract

Private package: `verification/conformance`, `publish = false`.

```rust,ignore
use solana_kit_conformance::{run, FixtureMode, Report, SUITE_ID, EXPECTED_CASE_COUNT};

let report: Report = run(FixtureMode::Canonical);
assert_eq!(SUITE_ID, "solana-kit-core-v1");
assert_eq!(EXPECTED_CASE_COUNT, 25);
assert!(report.is_success());
```

`run(FixtureMode) -> Report` is synchronous and host neutral. Its library uses
`no_std` plus `alloc` and has no imports from a runtime or host integration.
`Report` exposes `suite_id: &'static str` and `cases: Vec<CaseResult>`.
Each case exposes `name: &'static str` and
`outcome: Result<(), &'static str>`. `passed_count()` counts successful cases;
`is_success()` checks suite identity, the exact expected count, and every outcome.

All cases execute even after a failure. Hosts must preserve failure results and
check the exact identity and count. A changed fixture/assertion contract needs a
new suite ID. [Fixture provenance](../../verification/conformance/fixtures/PROVENANCE.md)
pins exact source revisions, paths, checksums, and primary standards.

The 25 cases cover fixed independent base58/PDA/seeded-address oracles; SHA-256
with both contiguous and split input; RFC 8032 Ed25519 tests 1 and 2; altered
messages/signatures; weak-key rejection; invalid base58 and exact-byte lengths;
bounded text; 15/16/17 seed-count boundaries; 32/33-byte seed boundaries; on-curve
rejection; forbidden seeded-address owners; all 16 privilege merges; ordered
duplicate instruction accounts; and independent owned instruction data.

## Executed commands and outcomes

All commands were run at the repository root with the checked-in configuration:

Monostyle 0.3.26 reports readability 100 and complexity 100 for the repository,
changed code, and every individual file, with the unchanged scoring rules.

| Command                                                                               | Observed result                                                           |
| ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `cargo +stable fmt --all --check`                                                     | Passed                                                                    |
| `cargo +stable test --workspace --all-features`                                       | Passed three suite/report tests and one core README doctest; none ignored |
| `cargo +stable clippy --workspace --all-targets --all-features -- -D warnings`        | Passed                                                                    |
| `cargo +stable check -p solana_kit_core --no-default-features`                        | Passed                                                                    |
| `cargo +stable check -p solana_kit_core --no-default-features --features crypto`      | Passed                                                                    |
| `cargo +stable check -p solana_kit_conformance --lib --target wasm32-unknown-unknown` | Passed; compile-only                                                      |
| `cargo +stable run -p solana_kit_conformance`                                         | Exit 0; `solana-kit-core-v1: 25/25 cases passed`                          |
| `cargo +stable run -p solana_kit_conformance -- --corrupt-fixture`                    | Exit 1; `solana-kit-core-v1: 24/25 cases passed`                          |
| `monostyle check . --fail-under 90`                                                   | Passed; every file meets both floors                                      |
| `monostyle check crates/solana_kit_core verification/conformance --fail-under 90`     | Passed; every changed code file meets both floors                         |

The [canonical output](../../verification/conformance/evidence/native-success.txt)
and [corruption output](../../verification/conformance/evidence/native-corruption.txt)
preserve every case result. Corruption changes the expected hash's first bit and
produces the actual comparison failure:

```text
sha256-fips-abc: Err("SHA-256 digest mismatch")
solana-kit-core-v1: 24/25 cases passed
```

The suite tests also remove a case and replace the suite identifier, proving
that these incomplete or mismatched reports fail `is_success()`.

## Dependency inspection and remaining integration

`cargo +stable tree -p solana_kit_core --no-default-features` reports only
`solana_kit_core -> bs58 0.5.1`. The `crypto` normal/build closure selects
`sha2 0.10.9`, `curve25519-dalek 4.1.3`, `ed25519-dalek 2.2.0`, and their small
support dependencies. It contains no Solana crate, Tokio, JavaScript binding,
HTTP stack, OS RNG, or filesystem library. Native SHA-256 CPU detection selects
`cpufeatures` and `libc`; the inspected `wasm32-unknown-unknown` closure excludes
both. `Cargo.lock` pins the resolved graph, including build dependencies.

The integration unit must register `solana_kit_core` and the private conformance
package in monochange and add CI/release-preview handling. In this scoped unit,
`monochange step validate` fails explicitly because the new changeset's
`solana_kit_core` target has not yet been registered; this worker does not edit
`monochange.toml`. The changeset records a minor release intent, not publication
authorization. Transactions, codecs beyond fixed base58 values, RPC, transports,
wallets, program clients, official System Program reuse, async host capabilities,
and actual non-native runtime execution remain outside this unit.
