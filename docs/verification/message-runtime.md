# Portable message signer integration

Signed executable `2bf9e21b16a4f10c62aab191557914d550fa61df` integrates the
accepted message module and assertions above the current browser/nonce stack.
It preserves every old 101 assertion module, ordered case and expected result.
Two isolated groups append exact bare-message snapshots and full bounded bulk
signing, producing `solana-kit-core-v24` with 103 cases.

The full `cargo +1.99.1 xtask all` passes on native Rust, Chrome 155.0.8059.85,
Wasmtime 50.0.0 Preview 1 and Preview 2, and actual pinned LeptosKit
`fecf8628d326ff7b03f295523e6bb584c3fdf88d` using Wasmi 2.0.0. Every host checks
103 canonical successes and 102/103 under sole `sha256-fips-abc` corruption.
The actual interpreter checks complete ordered JSON, errors, success/pass counts,
exact first/second commit counts and a separate one-fuel zero-commit trap.
Imports remain exactly `leptoskit::{commit,node_create,node_set_props}`.
Closed graphs, forbidden dependency/import controls and all five freshly
packaged-core consumer groups pass.

## Full fixed workload

`message_batches` signs all 256 distinct one-byte messages through the public
whole-slice backend. Its comparison oracle signs each message again. Public merge
validation performs 256 strict Ed25519 verifications. The snapshot assertion
signs twice; the two groups each derive one deterministic key. Thus these groups
execute 514 signing calls, 256 strict verification calls and two key derivations,
772 crypto calls in total. The 257-message input rejects before backend signing.
The original assertion bodies are unchanged; their existing private 256 constant
is merely exported for source-owned accounting.

The old transaction batch retains all 256 successful backend effects, immutable
signed bytes, 257-item preflight rejection and ordered early stop. It authenticates
257 transactions through 514 strict verifications while its cached backend derives
one key and signs one fixed message. That work does not perform 256 backend
signatures.

The checked private allowance is `15,000,000 × (103 - 3) +
5,000,000 × (257 + 772) = 6,645,000,000` fuel per entry. The three separate groups
are the transaction batch and both message groups. Every message crypto call
conservatively receives the entire existing five-million crypto envelope.
Ordinary/crypto rates, production host defaults, timeouts and imports remain
unchanged. Invalid/oversized metadata and arithmetic overflow reject; guest JSON
cannot grant fuel. The [private model](../../verification/leptoskit/README.md)
retains historical 92-case measurements and their near-minimum commit caveat.
No 103-case minimum or timing claim is made.

## Actual artifacts

```text
2e8e7cf931cf7b79713da468e8d72307ae7992fe646f3ead2fa9f32ea31f5f99  native
0a87f9515e98bbf430a3b0b20e6f6432250c066e2969d97915bb708d829991c2  chrome-wasm
d92197761512171eee46b63f5c14f2d271c63d6397dea50d702217c99d1abc38  wasm32-wasip1
b7a2b4e6e4b4ce0d800bca94051b59e59583d52d267a5fa9de026ebe2008689d  wasm32-wasip2
acaaf766e664e99b14d2fc65266da19f62b469caadb47849f393c39e6e99d409  interpreted-guest
a4e399f8f4d88af6f765291a06c59832c0b1e6415927f47e11f5c65212ec4177  actual-release-verifier
```

The complete public message source, original transaction source/tests, 85 IDs,
39 partial/46 pending/zero complete statuses, 17 defining hashes and MIT notice
remain accepted. This proof covers off-chain fixtures, not full signer parity,
dynamic discovery, attachment, configuration, persistence, real wallet submission,
validator acceptance or installed-device support. On-chain programs use Pina or
official Solana program libraries. Trusted-positive browser WSS remains pending.
