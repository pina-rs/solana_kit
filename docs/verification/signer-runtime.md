# Signer runtime integration

The combined wallet and signer suite executes all 97 canonical cases and the
96/97 sole-hash-corruption control in native Rust, Chrome, Wasmtime Preview 1
and Preview 2, and the actual pinned LeptosKit host. The signed executable is
`66b90da63acd1efd90ce16853d71bc0fd1e55dde`. It preserves the wallet frontier
`2347a847b923c98eca91ffa0c90df6dadd3c9bee` and imports only the accepted signer
interval ending at `fa69766dc4496c00e57eb2f8139e7f6eb6c3311b`.

The final `CARGO_NET_OFFLINE=true cargo +1.99.1 xtask all` exits zero. Every host
checks the complete ordered report. Only `sha256-fips-abc` fails in corruption
mode. The actual interpreter also traps with one fuel before any report commit;
exact import and dependency negatives and all five packaged-core consumer groups
pass. All 90 wallet-frontier cases precede the seven unchanged signer groups in
`solana-kit-core-v22`. Every 256-success batch assertion executes.
The registry collects foundational cases before RPC and extension cases in two
small phases. Their original order and all 97 assertion bodies remain unchanged.

The checked private allowance uses the accepted workload rates unchanged:
15,000,000 times 96 ordinary groups plus 5,000,000 times 257 authenticated batch
transactions, totaling 2,725,000,000 per entry. Compile-time `MAX_BATCH = 256`
owns the transaction count; guest JSON grants no fuel. Invalid metadata and
overflow reject. Production host defaults, timeouts and guest imports are unchanged.
No minimum fuel measurement is claimed for this 97-case artifact.

Actual artifacts executed at that signed head:

```text
f4c5b7737a7507b7e8ca7e7974724b4889b5643d4c36127659c039e8ab3914a2  target/wasm32-wasip1/debug/solana_kit_conformance.wasm
863b34d50ad7694a695a2475cda8f523a26b7db5b25e429e1746f219ebee641f  target/wasm32-wasip2/debug/solana_kit_conformance.wasm
1bc2e4dffbf80a86e5912261da4386050867c3bb79eab8b002de3137a4bf61e9  target/wasm32-unknown-unknown/release/solana_kit_conformance_guest.wasm
ee2ea51c83e1fec10d28a3d74bf41a209765784e5affdb20ecb2c8a8a2f751d6  verification/leptoskit/target/release/solana_kit_leptoskit_verification
```

## Historical 92-case proof

The original expanded suite was incomplete on the interpreted host. Executable source
`fcf86e1afa372ea1249754a595956390aa0c2ca6` passes all 92 canonical cases and the
91/92 sole hash-corruption control on native Rust, Chrome and Wasmtime Preview 1
and Preview 2. The pinned LeptosKit host exhausts its unchanged 1,380,000,000 fuel
allowance before committing the full canonical report. The accepted workload
correction at `fa69766dc4496c00e57eb2f8139e7f6eb6c3311b` subsequently passes all
five hosts at 2,650,000,000 per entry while preserving every fixture assertion.
Its measured minimum, 2,343,080,723, belongs only to that historical 92-case
artifact. Near-minimum one-less traps can retain one report commit; the separate
one-fuel control requires zero commits. Neither measurement is a 97-case minimum.

The implementation imports exact accepted signer source
`400593afd5d0c4e4ef51bbcc930d80255457dfeb` onto native WebSocket parent
`3dedddb7f5ca7738aa124b4c4553a63754f28283`. The signer crate, original tests,
notices, 85-ID receipt and isolated three-function library remain byte-identical.
The receipt retains 26 partial and 59 pending declarations, with zero complete.
All 17 original source hashes and the original MIT notice match.

## Shared portable behavior

`solana-kit-core-v21` preserves the first 85 function bodies, IDs, order and
outcomes. Seven appended groups call the public signer API through one shared
library on every host:

- The three unchanged isolated functions check deterministic signing, zero
  signatures and changed signed bytes.
- Coincident zero-sized backend storage retains distinct borrowed logical handle
  identities. Repeated references deduplicate, while separate handles conflict.
- Seven different backend types match all 112 single/pair and 698 triple/longer
  literal Dart role expectations without backend effects during role selection.
- Actual `Rc` futures suspend, wake, complete in reverse order and merge in input
  order. Cancellation drops constructed unpolled operations. Dropping a pending
  composition drops its backend operation.
- A full 256-transaction batch succeeds with 256 backend effects and immutable
  signed bytes. A 257-transaction batch rejects before another effect. A
  good/bad/good batch stops after two effects and retains the first operation.

The full 256-success crypto assertion remained intact during the original interpreted
failure. A separate diagnostic guest directly called that same public batch
function. Under the unchanged pooled allowance, the actual host finishes the
batch and then rejects the diagnostic because it intentionally publishes no
suite report. This isolates successful batch execution, not a full-suite pass or
a measured fuel cost. Its one-fuel control traps before any commit.

## Independent and source evidence

Ordinary Rust 1.89 consumers outside repository configuration pass the unchanged
independent 21 crypto and 13 default tests and all 14 original author groups.
These include actual generic multi-thread Tokio execution. The portable library
uses local composition; it makes no Wasm Tokio claim. All 12 intended lifetime
diagnostics and the safe-move positive control pass.

The original 51 Dart tests, 112 and 698 literal role outputs and deterministic
signature fixture execute and compare exactly. Receipt audits reject missing and
duplicate IDs, changed hashes, changed license bytes and a forbidden Tokio
dependency. Default signer normal/build dependencies remain signer, core and
bs58. Crypto adds the existing deterministic core closure.

## Executed artifacts

These are the artifacts used by the four passing hosts and the full interpreted
failure at the signed executable source above:

```text
3466d2ca7ae96f892a644b0c4182a68ffbb05c090cfc271275579b58a164ac44  target/wasm32-wasip1/debug/solana_kit_conformance.wasm
12f60067524d4f000b4ffa45e5a41882562a8e479627cbc1e6b1401e16535a9e  target/wasm32-wasip2/debug/solana_kit_conformance.wasm
cd4c74d1c1a259c1b98b69967e3aa30c784a6b1fedcc386e24f5ed2492bd9bde  target/wasm32-unknown-unknown/release/solana_kit_conformance_guest.wasm
```

Runtime tools are Rust 1.99.1, Chrome 155.0.8059.85, ChromeDriver 155.0.8059.84,
Wasmtime 50.0.0 and LeptosKit revision
`fecf8628d326ff7b03f295523e6bb584c3fdf88d`. Reproduction uses the existing
`CARGO_NET_OFFLINE=true cargo +1.99.1 xtask all` command with the named host paths.
The later accepted checked workload model and current combined receipt above
supersede the historical uniform allowance. No timeout, import allowlist or CLI
policy changed.

This SDK is exclusively for off-chain application clients. On-chain programs use
Pina or official Solana program libraries. Deterministic backends prove
orchestration, not wallet/network submission or confirmation. Message batches,
attachment, configuration, generation, persistence, device and validator journeys
remain open. The separate WASI HTTP correction and remote CI are outside this
unit. No publication or complete signer parity is claimed.
