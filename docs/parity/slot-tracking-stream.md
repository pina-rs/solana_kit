# Standalone slot tracking

`solana_kit_client::reactive::slot_tracking::SlotTrackingStream` combines an owned
initial future and a subscription stream. The existing optional `reactive` feature
provides this module without another dependency. The composed reactive slot store
is a separate unit.

`SlotTrackingStream::new` takes both sources and two fallible mappers. Initial and
subscription payload types may differ. The mappers produce the same item and
error types. Concrete inputs determine Send, and local `Rc` values remain valid.
Neither input requires Unpin. The adapter starts source polling when its caller
polls it, without creating an executor or task.

## Ordering and ownership

Each polling round polls the subscription and then the initial future, including
when the first input is immediately ready. Equal slots are accepted. Older slots
do not invoke a mapper. The complete RPC `Context`, including API version presence,
moves unchanged into the output.

The watermark advances before mapping an accepted value. A mapper failure
therefore excludes later older responses. Source failures have no slot and leave
the watermark unchanged. Both failures remain stream items, allowing the other
input and later subscription events to continue.

Each source has one pending output. Outputs drain before another polling round,
with subscription output first. An initial output cannot starve behind a ready
subscription, and equal-slot updates remain separate outputs. When a stale item
is consumed without yielding, the adapter wakes its caller for another round.

Completed inputs are fused separately. The adapter ends only after both inputs
and pending outputs finish. `FusedStream::is_terminated` reflects that condition.
Drop destroys the actual owned inputs and pending values. Underlying source types
retain their own drop and cancellation policies.

## Pinned source and deliberate differences

The Dart revision is `0138bb956d2d81ccc0c56edc602401c0df2f2b46`. The retained
archive SHA-256 is `8d2578053f9a679f87f9c6cce601f3db4c0049583a632f684d925bc9e785980a`.
The source path is `packages/solana_kit/lib/src/slot_tracking_stream.dart`.

| Source anchor | Contract                                                                |
| ------------- | ----------------------------------------------------------------------- |
| Line 15       | Initial future and subscription accept independent payload types        |
| Lines 30–34   | Both inputs must finish before output closes                            |
| Lines 37–48   | Reject older slots, accept equal slots, advance watermark before mapper |
| Lines 51–68   | Attach subscription and initial result handlers together                |
| Lines 70–73   | Cancel the subscription and reject late output                          |
| Lines 79–98   | Alias delegates to the same implementation                              |

The source file SHA-256 is
`740482f958a163f0c56e0842d38de461b8fb948ca8d81776789112a87a504776`.
Its test file, `packages/solana_kit/test/slot_tracking_stream_test.dart`, has SHA-256
`c651693b8259522daa18142f82606a200a7ba3bc368bc08e204d01d5fc80f19d`.

Dart forwards an RPC mapper exception to its error stream, but a subscription
mapper exception escapes its listener. Rust forwards typed `Result` errors from
both mappers. This is deliberate hardening of that subscription exception path.
Rust panics remain panics. The shared error type can be a caller-defined enum.

Dart's supplied future may already be running before listening and continues when
the output is cancelled. Rust owns its lazy future and drops it with the adapter.
The bounded pending slots provide backpressure instead of an unbounded controller
queue. Subscription-before-initial is Rust's explicit simultaneous-readiness rule.

## Source handoff verification status

Five original contract groups are registered from the client's native test bodies
into the maintained portable suite. A separate Tokio test owns and consumes the
concrete stream across awaits inside `tokio::spawn` to witness conditional Send.
The new bodies cover ordering, context preservation, both initial polls, equal-slot
buffer draining, nonterminal errors, mapper watermark behavior, both completion
orders, source fusion, owned Drop, local payloads, and stale-value wakeups.

All new test bodies are **UNEXECUTED** at this source handoff. The combined suite
signature is `solana-kit-core-v50` with count 427, adding five groups to the verified
parent's 422. Existing golden reports remain unchanged from that parent's actual
422-group run. The coordinating gate must record the 427-group results before
refreshing those reports. Source formatting and style checks do not prove runtime
correctness.
