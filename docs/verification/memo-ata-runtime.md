# Memo and Associated Token Account runtime integration

Signed executable `2af1c02cc404e1d1ee6c3c5f298d3cfe898a9f5b` integrates accepted
runtime parent `92a20937d0b7c5866ec072e7dff7d9b273204263` and exact accepted
handwritten clients `5616186f62a795cf5b756fb9b677705f35dae3c9`. Public clients,
original tests, source pins, notices and the 72-ID declaration receipt remain
unchanged. The finite generator, all 54 RPC methods, nine subscription families,
WASI adapter and existing core/System/Compute Budget owners remain unchanged.

The suite `solana-kit-core-v19` appends `memo-utf8-signers` and
`associated-token-metadata-pda` after the preserved first 81 groups. Both accepted
function bodies remain unchanged. They execute raw UTF-8/null bytes and signer
parsing, ATA construction/strict parsing and an independent literal PDA.

## Executed proof

On 2026-10-22, `CARGO_NET_OFFLINE=true cargo +1.99.1 xtask all` exits 0.
The run uses Rust 1.99.1, Chrome 155.0.8059.85, ChromeDriver 155.0.8059.84,
Wasmtime 50.0.0 and actual pinned LeptosKit
`fecf8628d326ff7b03f295523e6bb584c3fdf88d` WasmHost.

| Actual host        | Canonical                         | Corruption control                 |
| ------------------ | --------------------------------- | ---------------------------------- |
| Native Rust        | 83/83, exit 0                     | 82/83, exit 1                      |
| Chrome             | Complete 83/83 report checked     | Complete 82/83 report checked      |
| Wasmtime Preview 1 | 83/83, exit 0                     | 82/83, exit 1                      |
| Wasmtime Preview 2 | 83/83, exit 0                     | 82/83, exit 1                      |
| Actual LeptosKit   | Complete 83/83 JSON, first commit | Complete 82/83 JSON, second commit |

Only `sha256-fips-abc` fails under corruption. Both actual native first-81
reports compare byte-for-byte with the accepted parent's raw reports. Every host
checks the complete suite identity, case count and outcomes. The actual
interpreter separately traps OutOfFuel with one fuel before any commit.
The checked pooled allowance remains 15,000,000 times the case count, now
1,245,000,000 per entry, including lazy translation. No fuel policy changed.
Near-completion exhaustion may follow a report commit; it does not imply rollback.

Compiled guest imports remain exactly `leptoskit::{node_create,node_set_props,commit}`.
Selected core/client/System graph policies and forbidden import/dependency
controls pass. All five extracted packaged-core consumer groups pass.
Executed artifact SHA-256 values are relative to this checkout:

```text
66b60416d868708f18c4d672fc235e6fefa83d62d8dffcd0bd810dd91aad6f4d  target/wasm32-wasip1/debug/solana_kit_conformance.wasm
4a9715c5c0e4502326c0f99432480dab87b1651916254460c5fc5bcdbcc59a02  target/wasm32-wasip2/debug/solana_kit_conformance.wasm
e0bc0d83e3ad4e0e7684f22b7d6a271dc18867fe8779f551eaee3bdb6d4de956  target/wasm32-unknown-unknown/release/solana_kit_conformance_guest.wasm
```

## Independent source and caller evidence

All 32 untouched original Dart tests pass with the retained offline package
configuration. The fresh independent Dart oracle exactly reproduces the accepted
literal output, including historical Memo identities/BOM behavior, repeated
signers, exact ATA positions and seven independently chosen PDA results.
Unchanged ordinary Rust 1.89 consumers, with only dependency paths adapted,
pass inherited crypto/default 6/5 tests and novel crypto/default 5/4 tests.
Their source, assertions, oracle references and locks remain unchanged.
These overlapping counts are tests, not additional public operations.

The independent audit retains exactly 72 unique declaration IDs, ten complete
source hashes and 9 implemented/63 partial classifications. Both program and
Dart notices compare exactly with their pinned sources. See the
[bounded client contract](memo-associated-token.md) for full pins and deliberate
strict-parser differences. These are handwritten clients; the finite generator
and general renderer are unchanged. Full package/declaration parity is incomplete.

Previously executed WASI HTTP 20 embedded/13 CLI/six reactor and safe-consumer
proofs carry solely by complete executable source equality to the accepted
parent. This unit does not claim a new HTTP replay. Actual Rust 1.89 Preview 2
CI remains required before merge; missing local std is not execution evidence.

The SDK is exclusively for off-chain application clients. On-chain users use
Pina or official Solana program libraries. Memory fixtures establish no
validator, device, wallet, transport-network, release or registry journey.
Component publication remains disabled.
