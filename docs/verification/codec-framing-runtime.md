# Codec framing runtime receipt

The signed executable source is `b7a449cfa84ad9379c568f6613abc71c1e618153`.
It integrates accepted final Memo/ATA parent
`a976ca97eed9ff2b3a94d3d20313c8a50d0b05ef` and only the independently accepted
framing delta from `788606066ba94aaa1e298c776f75ff3a8de51c87` through
`19e2c90b425707f560a9a54e0c147d56e5fc8c96`. Both complete accepted portable
functions remain unchanged. Their own two-entry array appends
`codec-framing-transforms` and `codec-framing-boundaries` after the preserved
first 83 cases. The suite is `solana-kit-core-v20`.

This SDK serves off-chain applications. On-chain programs use Pina or official
Solana program libraries. This receipt does not complete the codec package,
Phase 2, validator, device, wallet, networking or registry work.

## Actual host execution

On 2026-10-22, the full command completed successfully after the exact final
parent relay, with the required tools already cached:

```sh
CARGO_NET_OFFLINE=true cargo +1.99.1 xtask all
```

Tools were Chrome 155.0.8059.85, ChromeDriver 155.0.8059.84, Wasmtime 50.0.0
and actual LeptosKit `WasmHost` at
`fecf8628d326ff7b03f295523e6bb584c3fdf88d`. Driver startup timeout was 30
seconds. The [runtime instructions](runtimes.md) describe tool-path selection.

| Actual host        | Canonical                          | Corruption control                 |
| ------------------ | ---------------------------------- | ---------------------------------- |
| Native Rust        | 85/85, exit 0                      | 84/85, exit 1                      |
| Chrome             | Complete ordered 85 report checked | Complete ordered 84 report checked |
| Wasmtime Preview 1 | 85/85, exit 0                      | 84/85, exit 1                      |
| Wasmtime Preview 2 | 85/85, exit 0                      | 84/85, exit 1                      |
| Pinned LeptosKit   | Complete JSON, first commit        | Complete JSON, second commit       |

Only `sha256-fips-abc` fails, with `SHA-256 digest mismatch`. Actual first-83
canonical and corruption report rows compare byte-for-byte with the accepted
parent. The actual interpreter separately traps with one fuel before any
commit. Its unchanged checked pooled allowance is 15,000,000 times 85, or
1,275,000,000 per entry. This includes lazy translation and the entire report;
it establishes no individual-case limit or transactional rollback guarantee.

The guest imports exactly `leptoskit::{commit,node_create,node_set_props}`.
Core/client/System graph policies, forbidden dependency/import controls and all
five extracted packaged-core consumer groups pass in the same full invocation.
Default native and three-Wasm core closures remain core plus bs58.

```text
cca54516d5b5665f20cc26c6e12a369e65cd29a81dd417e116e09bf6ef7834d6  target/wasm32-wasip1/debug/solana_kit_conformance.wasm
cc792210f22cb620213c9e61470daa21ee5d3135e61e444954fd3a0513d65feb  target/wasm32-wasip2/debug/solana_kit_conformance.wasm
46c5c16fa2a16e775c757a25d246e3049b1a9677242f4ba0065730ff1757d0f2  target/wasm32-unknown-unknown/release/solana_kit_conformance_guest.wasm
```

## Ordinary callers, provenance and limits

The unchanged independent eight-group consumer passes on actual Rust 1.89,
outside repository Cargo configuration and without inherited compiler flags or
wrappers. Only its public dependency path changes. A fresh original Dart oracle
emits the same 20 independent rows; the original author oracle emits the same
299 rows. All 71 original Dart tests and all 17 public Rust groups pass.

The audit retains all 113 canonical IDs, original 14 inventory fields, 16 defining
source hashes and the exact MIT notice. Status remains 39 partial and 74
uncovered, with all 113 residuals open. No receipt is promoted by host execution.
The [framing contracts](codec-framing.md) retain their accepted differences:
FixedSize and Padded clear dirty padding; nested metadata-only resizing can leave
unused bytes that a ByteSizePrefix Writer frame rejects; Reverse exposes only
the isolated reversed field to nested offset callbacks. The fresh original
context observation is Dart `4:9:1`, Rust `2:2:0`, both ending at outer position 3.

Current workspace all/default/no-default checks, Rust 1.89 checks and tests,
three-Wasm compilation, warning-denied Clippy/docs, formatting, actionlint,
Monochange check/validate/read-only preview and complete per-file style floors
pass. Scoped core API validation retains the partial, medium-confidence major
advisory for Sentinel's private policy field. No threshold, ignore, budget,
feature or assertion changed. Existing WASI HTTP evidence carries only through
complete source equality; this unit adds no WASI HTTP, device or network replay.
