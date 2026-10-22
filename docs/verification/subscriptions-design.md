# Subscription ownership design

The caller owns one `Connection`, the socket, monotonic time, and notification
polling. The SDK starts no task and requires no executor. This protocol is for
off-chain application clients. It does not establish a host networking capability.

`tests/subscription_designs.rs` executes two ownership sketches. The event-enum
candidate decodes before dispatch and exposes a variant for every family. The
selected typed-handle candidate keeps one registry of bounded raw notifications.
Each handle retains the exact request and its associated result decoder. Adding
a family does not add branches in connection lifecycle code.

The Model the Domain principle chose an enum for subscribe, active, unsubscribe,
and ended states. The Prove It Works principle requires public-owner sequences
with independent expected frames, including negative controls.

The shared JSON owner checks bytes, nesting, duplicate keys, and response framing.
The shared RPC account and block owners decode representation-specific results.
No subscription duplicates account, transaction, address, hash, or signature
validation algorithms. Notification results remain exact JSON until typed drain.
A drain validation failure terminates the connection and returns its cause.

`SignatureInfo` selects its receipt or processed schema from raw JSON before
decoding transaction failures. Serde's untagged enum buffering cannot supply the
raw fragment required by the shared transaction-error decoder. The explicit
selection preserves future error integers and decimal spelling. Known malformed
errors still fail validation. A failed transaction completes its signature
registration and keeps unrelated registrations active. The focused regression
checks direct decoding and actual connection delivery. The
[correction receipt](../../verification/subscriptions-design/signature-correction.tsv)
records the frozen external consumer's failing and passing runs.

The throughput checkpoint has four items.

1. Blocking first steps. Read pinned source and compare executable ownership sketches.
2. Independent workstreams. One worker owns this coupled connection implementation.
3. Shared mutable state. One mutable owner serializes all protocol transitions.
4. Smallest safe decomposition. Request schemas, notification schemas, and lifecycle
   state have separate modules. The owner is transport-neutral.

The caller passes a generation and must route each socket's frames with that
generation. Replacement requires a larger generation. Private owner identity
rejects handles from another connection. Replacement closes old handles and returns a gap. It does not
replay history. Cancellation stops local delivery immediately. Explicit tick
starts remote cleanup and expires missing acknowledgements. An ended handle must
be forgotten to reclaim its bounded registration slot. Dropping the connection
releases local state. The caller closes its socket.
Stale-generation ingestion returns `Failure::Closed` without terminating the
replacement connection. The caller keeps the replacement socket open.
