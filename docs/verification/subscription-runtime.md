# Portable subscription protocol fixtures

Solana Kit is exclusively an off-chain application client SDK. These fixtures
exercise the public `subscriptions::Connection` with complete in-memory text
frames. They do not open a WebSocket, contact an RPC node, or establish device
or production networking support.

The shared conformance owner appends four groups after the existing 77 groups.
The suite identity is `solana-kit-core-v18`. Canonical output requires 81/81.
The corrupt-hash control requires exactly the original hash failure and 80/81.
Every prior case name, order, and result line is unchanged.

| Group                            | Public behavior                                                                                                                                                                                  |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `subscriptions-nine-families`    | Literal account/program/logs/root/slot/vote requests, acknowledgements and typed results, full-width values, byte accounts, both vote timestamp forms                                            |
| `subscriptions-typed-variants`   | Signature receipt and known/future non-null errors, one-shot completion with surviving Root peer, all seven slot updates, all four block transaction views and both known block update errors    |
| `subscriptions-lifecycle`        | Early notification delivery, 32 cycles reclaiming bounded capacity, quiet cancellation, late acknowledgement, false unsubscribe acknowledgement, replacement gaps and stale-generation isolation |
| `subscriptions-malformed-bounds` | Malformed typed results in all nine families, duplicate/deep/invalid acknowledgements, explicit message/retained-byte/frame limits, quiet subscribe and unsubscribe deadlines                    |

Expected request frames are independent literals. Assertions compare complete
outbound bytes before injecting acknowledgements and notification frames.
Observed typed values establish successful delivery. Subsequent operations prove
that cancellation, one-shot completion and replacement release capacity.

`cargo xtask all` runs the same registry natively, in Chrome, in Wasmtime Preview
1 and Preview 2, and in the pinned actual LeptosKit interpreter. It also checks
dependency closures and binary imports, rejects forbidden dependency/import
controls, and builds five consumers against the packaged core. Only
`WASM_BINDGEN_TEST_DRIVER_TIMEOUT=30` changes the driver startup limit. SDK
acknowledgement deadlines remain explicit caller ticks.

The unchanged external Unit114 Rust 1.89 consumer has 14 groups. Its literal
frames and 208 bounded registrations provide separate native protocol evidence.
The accepted author suite has 23 tests. The pinned Dart API/protocol suite has
87 tests. The [scoped receipt](../parity/subscriptions.md) retains all 173 IDs as
68 partial and 105 pending. Transport adapters, pooling, coalescing, ping,
reconnect, historical reconciliation and confirmation remain open.
