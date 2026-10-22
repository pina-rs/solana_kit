# Isolated confirmation verification

These off-chain client fixtures retain the original public confirmation policy
and native ownership checks. The shared host suite separately appends three real
policy, Pending cleanup and exact submission groups after its prior 105 groups.
See the [runtime receipt](../../docs/verification/runtimes.md) for 108-case execution.

Run from the SDK checkout:

```sh
cargo test --manifest-path verification/transaction-confirmation/Cargo.toml --locked --offline
cargo +1.89.0 test --manifest-path verification/transaction-confirmation/Cargo.toml --locked --offline
```

The ownership tests execute one generic algorithm on two Tokio workers and with
local `Rc` state across observed Pending/wake boundaries. They verify cancellation,
drop, explicit awaited cleanup, competing readiness, closed streams and error
preservation. Subscription cleanup drives the actual `Connection` owner through
true and false acknowledgements and generation replacement. It uses in-memory
frames and does not establish network or validator confirmation.

The nonce adapter uses the existing System decoder before emitting a trusted
nonce assertion. Its controls reject wrong owner, truncated/trailing layout,
legacy/uninitialized state and executable accounts. Built-in nonce RPC polling
and asynchronous subscription factories remain pending.

`oracle.dart` produces the committed `dart-events.tsv` with immutable Dart
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. Run it from an owned evidence directory
with an explicit package map redirected to an owned archive; do not run pub
resolution or write reference/user caches. `DART-LICENSE` preserves the reference
MIT notice.

The `receipt` binary accepts SDK checkout, owned pinned Dart archive and read-only
live Dart checkout paths. It verifies exact defining hashes, source/test/license
identity and every external registry tuple against the accepted root lock. It
writes the 59-declaration receipt and source inventory; it never marks partial
capabilities complete.
