# Durable nonce portable runtime receipt

Signed executable `b77156be8e4d0adf290a45b62720a09244b3564a` executes the complete
v23 suite through `cargo +1.99.1 xtask all`. Native Rust, Chrome 155.0.8059.85,
Wasmtime 50.0.0 Preview 1 and Preview 2, and the actual pinned LeptosKit
`fecf8628d326ff7b03f295523e6bb584c3fdf88d` WasmHost/Wasmi 2.0.0 all verify
101 canonical passes and 100/101 under sole SHA-256 fixture corruption.
ChromeDriver is 155.0.8059.84. Each host checks the entire ordered report.

The existing 97 groups, functions, IDs and ordered expected rows remain unchanged.
Four groups call the accepted isolated nonce functions directly: construction,
wire, negatives and signatures. Public nonce/compiler source, 30 nonce byte/meta
literals, nine collation literals and all 746 declaration IDs remain exact accepted
nonce integration. The global address ordering is unchanged; compiler collation
uses its accepted private comparator. Received wire lifetime remains `Unresolved`.

The private interpreter uses the unchanged checked workload rates:

```text
15,000,000 × (101 - 1) + 5,000,000 × (256 + 1)
= 2,785,000,000 fuel per entry
```

All 256 batch successes, 257-item preflight rejection, immutable signed bytes and
ordered early-stop assertions remain intact. Invalid metadata and multiplication
and final-add overflow reject. The separate one-fuel execution traps before any
report commit. Near-minimum historical exhaustion can retain a commit; this
receipt promises no transactional rollback or newly measured minimum. Production
host defaults, imports, timeouts and source pins remain unchanged.

Exact actual artifacts from the complete successful run:

| Artifact                                                                  | SHA-256                                                            |
| ------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `target/wasm32-wasip1/debug/solana_kit_conformance.wasm`                  | `2a6ce73c4179c487370edf1b87d3131af9d2c5bc1913abf36cba4061cb7932b4` |
| `target/wasm32-wasip2/debug/solana_kit_conformance.wasm`                  | `4683e4fc0ba5230c697dc79e8caa1ec0ef2024b3fcb282a552949389e767e007` |
| `target/wasm32-unknown-unknown/release/solana_kit_conformance_guest.wasm` | `9a5f25a3d94009d93efb87ca5f86387ebe0925ae27b1c359d615b4efa8f37ee4` |
| `verification/leptoskit/target/release/solana_kit_leptoskit_verification` | `b14e348052b46a1b7f9213a6e302f83e71a82a9adae8199869ac4e312987900c` |

Only `leptoskit::{node_create,node_set_props,commit}` are imported. Import and
normal/build dependency negative controls pass. All five groups in the freshly
extracted packaged-core consumer execute successfully. Later ancestry integration
and documentation preserve the entire executable source; no redundant replay is
claimed for those changes.

These are exclusively off-chain application fixtures. Nonce metadata and
structural inspection do not authenticate account owner, state, balance,
authority, stored token, expiry or validator acceptance. The declaration receipt
retains 46 partial and 700 pending IDs, with none complete. Installed desktop or
mobile journeys, real wallets, network submission and full parity remain outside
this proof. On-chain programs use Pina or official Solana program libraries.
