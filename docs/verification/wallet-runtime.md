# Wallet runtime receipt

Signed executable source `e0cd5c16a2f011fe7db6f94c4e39afff2f7f3290` integrates
only the wallet delta from `bb5055e2feb9775bfee127b0bbb5316e472040e1` through
`2a7b7dc277d6062a7bc202bb83d70912f16efa88` into exact parent
`6bd0b0df9cc848c106d680f1980fc7b7f7494a0b`. The public crate, original tests,
notices, 37 assets and 203-ID source receipt remain byte exact to the correction.
Current SDK owners, guest/private-host algorithms, fuel policy, strict CLI and
narrow TSV LF policy remain unchanged.

The suite `solana-kit-core-v21` preserves the first 85 case IDs, order, outcomes
and function sources. A dedicated isolated crate exports the same five functions
called by every host. They check literal metadata, versions, typed features and
failures; concrete capability inputs and ordered partial outputs; registry
revision and identity; reentrant callbacks, destructors and disposal; and finite
exhaustion, reference counts and recovery. The 21 independent mutation scenarios
and 31/32-pass boundaries execute in those functions. Assertions panic on a
mismatch before a case can return success.

## Actual execution

On 2026-10-22, `CARGO_NET_OFFLINE=true cargo +1.99.1 xtask all` exits 0 with the
configured Chrome, driver and Wasmtime paths. Tools are Chrome 155.0.8059.85,
ChromeDriver 155.0.8059.84, Wasmtime 50.0.0 and actual LeptosKit `WasmHost` pinned
at `fecf8628d326ff7b03f295523e6bb584c3fdf88d`.

| Host               | Canonical                          | Sole hash corruption               |
| ------------------ | ---------------------------------- | ---------------------------------- |
| Native Rust        | 90/90, exit0                       | 89/90, exit1                       |
| Chrome             | Complete ordered 90 report checked | Complete ordered 89 report checked |
| Wasmtime Preview 1 | 90/90, exit0                       | 89/90, exit1                       |
| Wasmtime Preview 2 | 90/90, exit0                       | 89/90, exit1                       |
| Pinned LeptosKit   | Complete JSON, first commit        | Complete JSON, second commit       |

Only `sha256-fips-abc` fails under corruption. Actual LeptosKit traps at one fuel
before any report commit. Its unchanged pooled formula is 15,000,000 multiplied by
90 cases, or 1,350,000,000 per entry. This is neither an individual-case budget nor
a transactional rollback promise. Guest imports remain exactly
`leptoskit::{commit,node_create,node_set_props}`. Selected dependency graphs,
forbidden import/dependency controls and all five packaged-core consumer groups
pass with unchanged policy and budgets.

Actual artifact SHA256 values are:

```text
b7ec93067b85f8be8fd6f5ab818caac9147917fafa9616e7b082e329c3b8afe6  target/wasm32-wasip1/debug/solana_kit_conformance.wasm
d563f127f547e8488601aa96ddd7afad3127b929ce29841f4b9cf4dcdef8e6e7  target/wasm32-wasip2/debug/solana_kit_conformance.wasm
d69f03baabd5f14c90447e6c1af23ac5a4ff96fb66b2c2fa67bc6468c2337a53  target/wasm32-unknown-unknown/release/solana_kit_conformance_guest.wasm
1bfe393a858f74c408d83870a9e7bb117fef7bdf8933cf3c2a06ac8cec081f98  verification/leptoskit/target/release/solana_kit_leptoskit_verification
```

## Ordinary evidence and limits

Current workspace all/default/no-default tests, actual Rust 1.89 all-feature
workspace check and 37 original wallet tests pass. All three Wasm targets compile.
Clippy and rustdoc pass with warnings denied. The original ordinary consumer
executes real Tokio spawn and local Rc Pending/wake/drop. The unchanged Unit148
consumer passes its positive invariants, 21 mutation cases, exhaustion and drop
controls. The unchanged original Unit138 consumer intentionally exits 101 at its
historical defect assertion against corrected behavior. That exit is not a clean
contract pass. Its old-source positive failures remain independently preserved.
The original Dart 20-test suite and Unit138 literal/identity oracle pass without
package resolution or reference edits.

The read-only source audit confirms 203 exact unique partial IDs, seven source
hashes, 37 exact assets, package license and notices. Wallet normal/build closure
remains wallet plus base64 0.22.1; all five dependency-policy controls are rejected.
Per-asset upstream license provenance remains pending. All declarations remain
partial. Mutable getters without `changed` retain the documented observational
contract, which does not promise simultaneous metadata snapshots.

This SDK serves off-chain applications. On-chain programs use Pina or official
Solana libraries. Deterministic backends establish no browser discovery, MWA,
real authorization UI, live cryptography, transaction network submission,
validator acceptance or installed desktop/mobile journey. Actual Rust 1.89
Preview 2 CI remains a separate root gate. Independent integration review remains
required.
