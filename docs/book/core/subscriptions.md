# Subscriptions

`solana_kit_client::subscriptions` implements Solana's WebSocket subscription protocol as a bounded, synchronous state machine. You own the socket, the clock and the connection lifecycle. The `Connection` owner turns typed requests into outbound frames, routes inbound frames to the right handle and decodes notifications into Kit types. `subscriptions::Session` drives the same owner over a socket adapter. The `confirmation` module builds on top to wait for a transaction to reach a commitment level.

## Subscription families

Nine typed families are available. Each request type implements `SubscriptionMethod`.

| Request | RPC method | Stability |
| --- | --- | --- |
| `Account(address, config)` | `accountSubscribe` | Stable |
| `Program(program, config)` | `programSubscribe` | Stable |
| `Logs(filter, config)` | `logsSubscribe` | Stable |
| `SignatureNotifications(signature, config)` | `signatureSubscribe` | Stable. Finishes after its execution notification |
| `Root` | `rootSubscribe` | Stable |
| `Slot` | `slotSubscribe` | Stable |
| `Block(filter, config)` | `blockSubscribe` | Unstable on the node |
| `Vote` | `voteSubscribe` | Unstable on the node |
| `SlotsUpdates` | `slotsUpdatesSubscribe` | Unstable on the node |

## Driving a connection

`Connection::new(generation, budget)` creates an owner for one socket generation. The protocol loop is:

1. `subscribe(request, deadline)` registers a handle. Routing is installed before any frame exists, so an early notification cannot arrive ahead of its route.
2. `outbound()` returns the next frame to send. Send each one on your socket.
3. `ingest(generation, bytes)` passes in one complete received text frame. You must bound the socket's receive size before this call.
4. `next(&handle)` drains the next notification as `Drain::Notification(value)`, `Drain::Pending` or `Drain::Closed`.
5. `tick(now, cleanup_deadline)` enforces acknowledgement deadlines and starts cleanup for cancelled handles. The tick values come from your own monotonic clock.

This example runs the whole cycle with in-memory frames:

```rust
use solana_kit_client::subscriptions::{Budget, Connection, Drain, Root};

fn example() -> Result<(), solana_kit_client::subscriptions::Failure> {
    let mut connection = Connection::new(1, Budget::default());
    let root = connection.subscribe(Root, 20)?;

    // Send this frame on your socket: {"jsonrpc":"2.0","id":1,"method":"rootSubscribe","params":[]}
    let _frame = connection.outbound().expect("subscribe frame");

    // Feed complete frames received on generation 1.
    connection.ingest(1, br#"{"jsonrpc":"2.0","id":1,"result":7}"#)?;
    connection.ingest(
        1,
        br#"{"jsonrpc":"2.0","method":"rootNotification","params":{"subscription":7,"result":42}}"#,
    )?;
    assert!(matches!(connection.next(&root)?, Drain::Notification(42)));

    // Unsubscribe and wait for the server's true acknowledgement.
    connection.unsubscribe(&root, 30)?;
    let _frame = connection.outbound();
    connection.ingest(1, br#"{"jsonrpc":"2.0","id":2,"result":true}"#)?;
    assert!(matches!(connection.next(&root)?, Drain::Closed));
    connection.forget(root)?;
    Ok(())
}
```

Closing a subscription:

- `cancel(&handle)` stops delivery immediately. A later `tick` sends the unsubscribe. It works even when the outbound queue is full.
- `unsubscribe(&handle, deadline)` waits for an explicit `true` acknowledgement. A `false` reply surfaces as `Failure::UnsubscribeFalse`, and a missed deadline as `Failure::Deadline`.
- `forget(handle)` releases the bounded slot of an ended handle.
- `terminate()` and `Drop` release local state. They do no network I/O.

## Budgets and failures

`Budget` bounds one generation. `Budget::default()` uses default frame `Limits`, 64 subscriptions, 256 queued messages and 1 MiB of retained bytes. `subscribe_with_budget` adds a per-handle `QueueBudget`.

Nothing is dropped silently. A full queue fails with `Failure::Overflow`. Other failures are `Capacity`, `Deadline`, `Correlation`, `Method`, `Closing`, `UnsubscribeFalse`, `Closed` and `Protocol(Error)`, which wraps the shared RPC error. If `ingest` returns an error for the current generation, close that socket.

## Reconnecting

The SDK never reconnects or replays on its own. When you open a new socket, call `connection.replace(new_generation)`. It ends every old handle and returns a `Gap { previous, replacement }`. Notifications sent during the gap are lost, so re-subscribe and reconcile state yourself, for example with a fresh RPC read. Frames still arriving from the old generation return `Failure::Closed` and do not disturb the new one.

## Sessions and socket adapters

`Session<S>` wraps a `Connection` around one socket that implements `TextSocket`. It exposes poll-style operations (`poll_flush`, `poll_next`, `poll_shutdown`) that you drive from your own executor. It creates no task, channel or reconnect loop. Socket adapters exist per host:

| Host | Adapter |
| --- | --- |
| Native (Tokio) | `NativeWebSocket` for use with a `Connection`, and `NativeSubscriptionSocket` for `Session` |
| Browser | `BrowserSubscriptionSocket::from(BrowserWebSocket)` |
| WASI | `WasiWebSocket` implements `TextSocket` directly |
| LeptosKit | `GuestSubscriptionSocket` |

The adapters are at different stages. Some socket paths are still marked source-only or unexecuted in their own READMEs. Check the [Native](../runtimes/native.md), [Browser](../runtimes/browser.md), [WASI](../runtimes/wasi.md) and [LeptosKit](../runtimes/leptoskit.md) pages before you depend on one.

## Transaction confirmation

`solana_kit_client::confirmation` separates the confirmation *policy* from how you *observe* events.

- `Confirmation::new(signature, lifetime, policy)` creates a single-use reducer. It rejects `Lifetime::Unresolved`, because received wire cannot establish expiry. `Policy` sets the target `commitment` (default `Confirmed`) and a `min_context_slot`.
- `observe(Event)` feeds it one event: a `Status` lookup, a `Signature` notification, an `Epoch` block height, a validated `Nonce` state, or a `Timeout`. The result is `None` while undecided, `Some(Ok(()))` when confirmed, or `Some(Err(failure))`.
- Failures stay distinct. They include `Transaction(..)` (execution failed), `BlockHeightExceeded`, `NonceInvalidated`, `NonceAccountMissing`, `Timeout`, `Cancelled`, `Rpc(..)` and `Subscription(..)`.

```rust
use solana_kit_client::{
    confirmation::{Confirmation, Event, Failure, Policy},
    methods::{Commitment, node::EpochInfo},
};
use solana_kit_core::{Hash, Signature, transaction::Lifetime};

fn example() -> Result<(), Failure> {
    let mut confirmation = Confirmation::new(
        Signature::from_bytes([7; 64]),
        Lifetime::Blockhash { blockhash: Hash::from_bytes([3; 32]), last_valid_block_height: 41 },
        Policy { commitment: Commitment::Confirmed, min_context_slot: 0 },
    )?;
    let epoch = |height| Event::Epoch(EpochInfo {
        absolute_slot: 0,
        block_height: height,
        epoch: 0,
        slot_index: 0,
        slots_in_epoch: 100,
        transaction_count: None,
    });
    assert!(confirmation.observe(epoch(41)).is_none()); // the height is inclusive
    assert!(matches!(
        confirmation.observe(epoch(42)),
        Some(Err(Failure::BlockHeightExceeded { current: 42, last_valid: 41 }))
    ));
    Ok(())
}
```

To run a reducer against live events, implement `Observer`, which has a cancellable `next` and a bounded `close`. Then use:

- `wait(confirmation, observer, cancellation)` races observation against your cancellation future. Cancellation wins a tie. `close` is always awaited, and a cleanup failure is reported as `WaitError::Cleanup` together with the original outcome.
- `send_and_confirm(client, &transaction, config, policy, observe, cancellation)` encodes the fully signed transaction once, submits those exact bytes, checks the returned signature, and only then builds your observer. It never retries. A submission cancelled mid-flight can leave node acceptance uncertain.

The optional `confirmation-factories` feature adds ready-made strategies over a `Session`: `confirm_signature`, `confirm_signature_shared` with `SessionAccess`, `submit_and_confirm`, `reconcile` and `ConfirmationWatch`. Each takes an injected `Clock`, a deadline (`FactoryConfig`) and cancellation. See [Sign and send](../getting-started/sign-and-send.md) for an end-to-end flow.

> [!CAUTION]
> A `Nonce` event is a trusted assertion from your adapter. Emit one only after checking System ownership, the exact layout and the initialized state. Nonce events can invalidate a transaction but never confirm one. Built-in RPC nonce observation is still pending.

## Slot tracking

With the `reactive` feature, `reactive::slot_tracking::SlotTrackingStream` and `SlotTrackingStore` combine an initial RPC read with a subscription stream. They reject updates from slots older than the last value. See the client README for their current verification state.

## Status

> [!NOTE]
> The [subscription receipt](https://github.com/pina-rs/solana_kit/tree/main/docs/parity/subscriptions.md) lists the reference declarations as partial or pending, and none as complete. The [portable protocol receipt](https://github.com/pina-rs/solana_kit/tree/main/docs/verification/subscription-runtime.md) runs all nine families with in-memory frames on native Rust, Chrome, WASI and LeptosKit. It does not test a WebSocket transport. Pooling, coalescing, automatic pings, reconnect and historical reconciliation are not provided. The [confirmation receipt](https://github.com/pina-rs/solana_kit/tree/main/docs/verification/transaction-confirmation.md) covers the policy, cancellation and cleanup guarantees. It does not claim validator or device execution.
