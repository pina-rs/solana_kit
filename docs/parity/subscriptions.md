# Bounded subscription protocol receipt

The public client implements nine typed subscription families through one mutable
`subscriptions::Connection`. The caller owns text-frame transport, tick values,
and generation identities. This is exclusively an off-chain client protocol.
No native, browser, WASI, LeptosKit, validator, or device networking is proved here.

`docs/parity/subscriptions.tsv` extracts all 173 declaration IDs from the three
pinned subscription packages. There are 68 partial and 105 pending entries, and
zero complete entries. The canonical ledger is unchanged. Typed Rust request
schemas replace Dart parameter factories. Their differing API shapes remain
partial. Pooling, coalescing, automatic pinging, endpoint validation, WebSocket
adapters, convenience clients, and abortable futures remain pending.

The reference is Dart commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46`.
The request names and field schemas were inspected from its subscription API,
subscription execution, channel, and WebSocket channel sources. Its license is
retained at `verification/subscriptions-design/DART-LICENSE`. The Rust state
machine and independent fixtures are newly authored. Shared RPC framing, account,
transaction, and scalar validators retain their existing provenance.

Regenerate the exact ID receipt locally.

```sh
rustc --edition 2024 verification/subscriptions-design/receipt.rs -o target/subscription-receipt
target/subscription-receipt docs/parity/capabilities.tsv docs/parity/subscriptions.tsv
```

Run the independent public-owner behavior checks.

```sh
cargo test -p solana_kit_client --test subscriptions --test subscription_families --test subscription_designs --test signature_notifications
```

The isolated cases are registered in `verification/subscriptions-design/cases.tsv`.
They have not been added to the shared portable runtime suite. Three Wasm target
checks establish compilation only. Native in-memory tests establish actual public
protocol execution, not network transport execution.

The original pinned Dart API and protocol tests execute separately. They are an
oracle for reference behavior, not proof of identical behavior. Rust deliberately
adds bounded early-notification retention, explicit unsubscribe acknowledgements,
and generation replacement gaps. Dart silently ignores some unrelated/malformed
messages and treats cancellation cleanup as best effort. Rust reports terminal
framing, correlation, wrong-family, and overflow errors to the caller.

Resource limits cover registration count, queued frame count, retained raw bytes,
and per-frame byte/depth limits. The caller must bound socket reception before
passing a complete frame. Typed decoding occurs at drain. Quiet cancellation
stops delivery immediately and a caller tick starts unsubscribe cleanup.
Missing acknowledgement deadlines terminate the registration. The caller must
close the current socket when `ingest` returns an error for its generation.
Stale-generation frames return `Failure::Closed` without terminating the
replacement connection. The caller keeps the replacement socket open.
`terminate` and Drop release local state. Dropping the owner does not perform network I/O. Signature execution
notifications finish their server one-shot subscription without an unsubscribe.

Signature execution failures use the shared typed transaction-error decoder.
Known errors retain their validated fields. Future error objects retain exact
raw JSON, including integers larger than `u64` and decimal spelling. A failed
transaction completes only its signature registration. Other registrations
continue to deliver notifications.

The caller supplies monotonically increasing generations within one connection.
A private owner identity rejects handles from another connection. Replacement rejects old handles and old-generation ingestion, returns
a gap, and starts an empty owner. No retry, reconnect, historical reconciliation,
or replay is implicit. An ended handle retains a bounded slot until `forget`.

Four [portable public protocol groups](../verification/subscription-runtime.md)
extend the shared conformance registry. They execute complete memory-frame I/O.
They do not establish WebSocket transport or device networking support.
