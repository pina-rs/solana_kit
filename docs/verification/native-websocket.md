# Native WebSocket execution receipt

The off-chain application SDK provides one bounded caller-owned native text
WebSocket in `solana_kit_native`. Solana on-chain programs use Pina or official
Solana program libraries. Browser, WASI, LeptosKit and device socket execution
remain pending. Native execution does not establish those runtimes.

## Ownership

Two small shapes were compared against the existing public Connection contract.

| Shape                                          | Consequence                                                                                                                         |
| ---------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Socket and Connection combined in a driver     | Requires ownership of ticks, dispatch scheduling and subscription termination policy.                                               |
| NativeWebSocket beside caller-owned Connection | Existing Connection retains all correlation, queues, generation and acknowledgement state. The application schedules I/O and ticks. |

The separate socket is selected. `connect` owns in-progress DNS, TCP, TLS and
upgrade I/O until completion. The resulting owner exclusively borrows for
`send` and `receive`. `close(self)` consumes it. There is no owned task, detached
handle, pool, retry, redirect, proxy, cookie store, ambient authentication,
reconnect, historical replay or global driver.

Cancelling a borrowed receive retains the socket and partial parser state.
Cancelling a borrowed send may leave a partial write. The caller drops the owner
after cancelled send and never retries uncertain delivery automatically.
Dropping the owner or a consuming close future drops its socket. Cancelling
connect drops in-progress networking. OS DNS resolution may finish later without
sending an upgrade. Connection Drop releases only its independent local state.
A current-generation ingest error requires closing the corresponding socket.

## Bounds and diagnostics

WsConfig bounds incoming frames, fragmented messages, read-buffer allocation,
encoded retained write bytes, endpoint plus caller-header bytes, the complete
connect handshake and each operation. Outgoing payload length rejects before
copying. Tungstenite 0.29 supplies its fixed 65,536-byte handshake guard plus one
bounded read chunk, 512-packet limit and tiny-packet defense. No compression
extension is negotiated. Unsolicited extensions reject explicitly because the
pinned dependency does not enforce that check itself.

Certificate chain and destination-name verification use the existing native
TlsRoots implementation. Both explicit WS and WSS are accepted. Userinfo,
fragments, unsupported endpoints, cookie headers and protocol-header overrides
reject. This is a trusted application endpoint policy, without a private-host
filter. Ordinary errors and Debug output omit endpoints and caller header values.
Every upgrade header is marked sensitive without mutating caller-owned values.

An executed negative control proves that sensitive flags cannot redact
Tungstenite's opt-in raw trace. It serializes the whole request into bytes before
logging. Both an Authorization fixture and a URI query fixture appear in that
trace. Do not enable dependency wire-level tracing for credentialed connections.
No claim covers arbitrary consumer logging of raw requests or frames.

## Executed checks

Run these offline from the checkout.

- `cargo test -p solana_kit_native --offline` runs the existing HTTP tests and new socket fixtures.
- `cargo test -p solana_kit_native --test websocket --offline` runs actual loopback WS and trusted fixture WSS. It tests wrong CA, wrong name, TLS against plaintext, forbidden/redirect/malformed upgrade, malformed UTF-8 and frame flags, oversized frame headers and fragmented messages, binary rejection, ping/pong, graceful and abrupt close, outbound limits, authentication, handshake/receive/close deadlines, connect/close cancellation, retained receive cancellation, actual I/O wake and sixteen repeated socket teardown cycles. Every spawned fixture task is awaited.
- `cargo test -p solana_kit_native --test websocket_subscriptions --offline` drives all nine typed families through actual socket I/O and public Connection. Each family checks complete outgoing subscribe and unsubscribe requests, acknowledgements and typed results. Signature completion closes once without unsubscribe. A separate failed-signature case leaves a Root peer active and delivering u64MAX on the same socket.
- `cargo test -p solana_kit_native --test websocket_trace --offline` captures actual dependency trace messages in an isolated test process. It asserts the documented upstream logging limitation and real peer authentication delivery without printing captured credentials.
- Ordinary external Rust 1.89 repeats the public test source outside repository Cargo configuration, with repository flag and wrapper variables removed. The multiworker test moves the real socket through Tokio spawn.
- Workspace tests, no-default-feature checks, MSRV, Clippy with denied warnings, denied-warning docs, formatting, monostyle 0.3.26 and Monochange check/validate/preview are run for this unit. Core/client checks for three Wasm targets establish compilation only.
- The original immutable Dart WebSocket package tests run without modified source or dependency resolution. All 98 pass under the installed Dart VM. This file imports dart:io. No browser execution of those tests is claimed.

Raw execution logs and exact signed source SHAs stay in the primary local Unit133
report. The original native HTTP implementation and tests remain unchanged.
No shared subscription algorithm, core source, central inventory or existing
receipt is changed by the socket implementation.

## Exact declaration scope

Regenerate the scoped receipt without changing canonical inventory.

```sh
rustc --edition 2024 verification/native-websocket/receipt.rs -o target/native-websocket-receipt
target/native-websocket-receipt docs/parity/capabilities.tsv target/native-websocket-declarations.tsv
cmp verification/native-websocket/declarations.tsv target/native-websocket-declarations.tsv
```

`verification/native-websocket/declarations.tsv` retains every exact declaration
ID and source SHA256 for the three source subscription packages. It has 173 IDs,
76 partial and 97 pending, with zero complete. Eight WebSocket declarations gain
bounded native equivalents. The 68 protocol declarations retain their partial
status with actual native network evidence. Partial does not claim identical
Dart configuration, broadcast streams or cancellation behavior. Pooling,
coalescing, automatic ping timers, reconnect, convenience clients and remaining
browser, WASI, LeptosKit and device capabilities stay pending.
