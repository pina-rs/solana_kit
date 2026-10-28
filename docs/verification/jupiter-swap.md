# Jupiter Swap API v2 contracts

The source contract is the MIT-licensed Dart Jupiter package at
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. The package retains its MIT notice;
original Rust implementation uses the Unlicense. No official Solana crate is used.

| Pinned Dart declaration | Rust public surface |
| --- | --- |
| `JupiterOrderRequest`, `toQueryParameters` | `OrderRequest`, `to_query_parameters` |
| `JupiterSwapMode`, `JupiterOrderMode` | `SwapMode`, `OrderMode` |
| `JupiterOrderResponse` | `OrderResponse`, `decode_order` |
| `JupiterExecutionResponse` | `ExecutionResponse`, `decode_execution` |
| `JupiterInstructionPayload` | `InstructionPayload`, `to_instruction` |
| `JupiterAccountMetaPayload` | `AccountMetaPayload`, `to_account_meta` |
| `JupiterBuildResponse` | `BuildResponse`, `decode_build` |
| `JupiterBlockhashWithMetadata` | `BlockhashWithMetadata`, `to_lifetime` |
| `JupiterSwapClient.getOrder` | `Client::get_order`, `GET /swap/v2/order` |
| `JupiterSwapClient.buildSwap` | `Client::build_swap`, `GET /swap/v2/build` |
| `JupiterSwapClient.executeOrder` | `Client::execute_order`, `POST /swap/v2/execute` |
| `decodeBase64SwapTransaction` | `decode_base64_swap_transaction` |

The unused Dart `userPublicKey` execution argument has no Rust counterpart:
the signed transaction already carries wallet identity. Execution requires the
accepted order's nonblank request ID and emits an exact string expiry when
available. A returned signature never implies successful execution.

`Integer` retains arbitrary-width signed decimal integers. Integral numbers and
decimal strings decode without binary floating point. Leading zeroes normalize;
fractional, exponent and hexadecimal forms reject. Timestamp, basis-point and
execution-code fields use checked `i64`; conversion of a lifetime explicitly
checks Kit's `u64` height domain. Malformed known fields reject, including values
that Dart's permissive scalar helpers would turn into absent optionals.

Missing and null optional fields remain absent. Unknown string order modes stay
absent, and nonstring execution errors stay in raw JSON. Present blockhash metadata
requires both fields. Raw instruction payloads may be incomplete until explicit
Kit conversion; missing privilege flags never become false. Lookup maps retain
JSON key order and address index order. Only an absent or null lookup-address list
falls back to map keys; an explicit list, including an empty one, wins.

All standalone decoders admit bounded JSON before typed decoding and retain the
exact response bytes, including unknown fields. Ordinary record and error
diagnostics exclude response bodies. Adapter causes remain explicitly available
through the error source chain. Transport adapters continue to own approved
origins, authentication, receiving limits, redirects, deadlines and cancellation.

The ten `jupiter-swap-*`, `jupiter-order-*`, `jupiter-execute-*`,
`jupiter-build-*`, `jupiter-instruction-*` and `jupiter-lookup-*` conformance groups
run through the existing `solana_kit_conformance` registry. They cover query
defaults and overrides, request bodies, response precision and optionality,
malformed fields, duplicate keys, admission budgets, conversion failure paths,
independent v0 wire bytes, single-exchange errors and owned-future cancellation.
The suite identity is `solana-kit-core-v45`, with 388 canonical cases and 387
passing cases under the sole-hash-corruption negative control.

Root-owned execution uses the existing commands and harnesses:

```sh
cargo test -p solana_kit_jupiter --locked
cargo test -p solana_kit_conformance jupiter_public_contracts_pass --locked
cargo run -p solana_kit_conformance --locked
cargo run -p solana_kit_conformance --locked -- --corrupt-fixture
cargo clippy -p solana_kit_jupiter -p solana_kit_conformance --all-targets --locked -- -D warnings
```

Browser execution uses `verification/browser`; WASI uses the conformance binary
for its maintained WASI targets. `verification/guest` and `verification/leptoskit`
execute the same registry in the pinned production wasmi host. The ten extra
ordinary groups use the existing finite fuel-accounting formula. The source retains
the independent explicit `13_785_000_000` fuel oracle for 388 cases and
all fixed crypto, MWA and overflow controls. The host test still requires execution.
The fixtures use a manual local peer and require no credentials or live Jupiter access.
Generated desktop, Android and iOS application runs and actual host HTTP exchanges
require separate execution receipts; compiling or registering these cases does
not establish those results.
