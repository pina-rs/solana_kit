# Original program client contracts

This private workspace imports all74 public test bodies from the Address Lookup
Table, Config, Stake and Loader crates by their original source paths. The
registry calls those functions directly; it does not copy their assertions or
substitute a model implementation. Literal account privileges, discriminators,
optional-account combinations and malformed-input boundaries remain observable.

The native source tool regenerates the registry and expected report:

```sh
cargo run --locked --manifest-path tools/codama-renderer/Cargo.toml \
  --features verification --bin solana_kit_codegen -- program-contracts
```

Regeneration must leave both tracked files byte-identical. The
[program workflow](../../.github/workflows/program-clients.yml) requires native
Rust1.89 and1.99, Chrome, Wasmtime50 Preview1 and Preview2, and the actual pinned
LeptosKit `WasmHost`. Both WASI command streams must match native's complete named
stream. The interpreted guest must commit its complete74/74 report exactly once;
one unit of fuel must fail before commit. Separate mutation builds must expose
intentional corruption in every runtime.

The host selects framework commit `386e1cd81737fd6ddffdd763a1a5d2c96d8c0d6d` and
Wasmi2's portable-dispatch backend. This is an explicit fixture backend choice.
The compiled guest must pass the existing closed import checker. Historical
69-body receipts, including the earlier Config diagnostic failure, do not verify
the current74-body candidate. The Config allocator probe remains native only.

`program_clients_gate` checks native and all three Wasm targets. Default-feature
normal and normal/build closures are exact; ALT's optional PDA feature uses the
same reviewed cryptography policy as Core. All40 actual queries must also reject
an injected Tokio dependency through the same checker.

`program_clients_audit` checks636 immutable Dart declaration rows, each defining
source hash and every retained license against the pinned source export. This
proves provenance; behavioral declaration adjudication remains separate.

These are exclusively off-chain client contracts. They do not prove validator
state transitions or installed desktop/mobile journeys. On-chain programs use
Pina or the official Solana program libraries.
