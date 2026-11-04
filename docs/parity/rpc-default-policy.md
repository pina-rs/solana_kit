# RPC default-policy differences

Pinned Dart source: `0138bb956d2d81ccc0c56edc602401c0df2f2b46`.

The source's `solana_kit_rpc/src/rpc_default_config.dart` selects `confirmed` and
`rpc_transformers/src/request_transformer_default_commitment.dart` inserts it
when the caller omitted a commitment. Rust keeps the existing explicit typed
request contract: `None` omits the field and leaves the node's policy in charge.
This is an observable deliberate difference; transformer-default parity is not
claimed.

For the source factory's behavior, provide
`ReadConfig { commitment: Some(Commitment::Confirmed), ..Default::default() }`
to each read, and set `SendConfig::preflight_commitment` explicitly for
submission. Account, history, simulation and subscription configurations own
their respective commitment fields. An explicit `Finalized` or `Processed`
remains exactly that choice. The client does not sample or infer finality.

The source's `rpc_request_coalescer.dart` also shares identical requests. Rust
retains independent exchanges and strict per-call response IDs. Automatically
sharing responses would require a separate ownership and correlation contract,
including cancellation by one waiter and methods that cause effects. This
optimization is not silently added to existing calls, and no deduplication or
automatic retry is promised.

The source's subscription autoping sends an application JSON `ping` notification
after idle intervals. Rust does not inject this optional application message or
own an idle timer. Applications select their socket, clock and readiness driver;
the documented subscription deadlines and cancellation guarantees still apply.
Protocol Ping/Pong handling in a concrete WebSocket adapter is separate from
that application notification. Idle keepalive and reconnect are not inferred
from ordinary subscription execution.

These policies preserve existing Rust request bytes and cancellation semantics.
They do not alter canonical declaration statuses or claim exact source defaults.
