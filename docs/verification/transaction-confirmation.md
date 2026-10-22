# Transaction confirmation frontier

This work serves off-chain application clients. On-chain programs should use Pina
or official Solana program libraries.

## Grounding and API choice

The client owns `Commitment`, positional `SignatureStatus`, execution failures,
account snapshots and the bounded subscription `Connection`. Transactions retain
`Lifetime`; received wire tokens remain `Unresolved`. `Connection` is a synchronous
protocol owner. The application sends frames, routes generations and drives
acknowledgements and cleanup. There is no portable asynchronous subscription driver.
The System package owns the nonce account codec; the client does not depend on it.

Two shapes were considered against these actual callers:

1. A `Confirmation` reducer accepts typed observations. A generic `Observer` owns
   delivery and explicit asynchronous cleanup. An injected cancellation future
   races observation; there is no task, clock or global `Send` bound.
2. A `Client::confirm` polling loop owns HTTP reads and an injected delay. This is
   convenient for HTTP-only callers, but cannot implement subscription cleanup or
   authenticate nonce layouts without additional contracts.

Choose the first shape for shared strategy policy, then add polling convenience
only where existing domain owners suffice. Signature, receipt, commitment, exact
block-height expiry, nonce replacement and unknown lifetime remain distinct.
Nonce observations are explicitly trusted adapter assertions; they do not
establish on-chain ownership, initialization or authenticity.

## Scope and pending contracts

Subscription factories remain pending until an asynchronous adapter can register
before querying, drive actual acknowledgements, close losing watchers with a
bounded deadline, and report false unsubscribe and generation gaps. RPC nonce
polling remains pending until the client can reuse the System nonce decoder
without introducing an unapproved dependency. No placeholder success implements
those operations. The declaration receipt records these limits individually.

The shared 108-case suite now executes three bounded confirmation groups on all
five portable hosts. The [runtime receipt](runtimes.md) records exact execution
and finite fuel policy. These injected observations do not establish validator,
network, device or actual timer behavior.

## Verified policy and ownership

`Confirmation::new` rejects unresolved received lifetimes. `observe` consumes
existing Kit RPC and subscription models, ignores receipts, checks execution
failure before commitment, uses strict `height > last_valid_height`, and ignores
nonce observations older than their own monotonic watermark. Nonce events never confirm a
transaction; only signature execution at the requested commitment can do so.

`wait` owns the observer. Cancellation wins simultaneous readiness, losing receive
futures are dropped before explicit bounded `close`, and cleanup failure retains
the prior outcome. Dropping the wait promises only the observer's local teardown.
An already settled reducer fails explicitly. An always-ready stream of irrelevant
events yields so local cancellation producers can run.

`send_and_confirm` encodes the fully signed immutable transaction once, submits
those exact bytes, checks the returned primary signature, and only then constructs
an observer. It preserves node-side retry and exact integer options. Cancellation
while submitting can leave node acceptance uncertain; the SDK never retries.

The surrounding client requires `std`. The new strategy module uses `core`
machinery and adds no runtime or normal/build dependency, but this does not make
RPC or the entire client `no_std`. A standalone `no_std` confirmation package is
pending a shared domain owner that avoids duplicating RPC types.

The isolated `verification/transaction-confirmation` crate retains the original
pure policy and native ownership groups. Shared portable fixtures append after
all 105 prior groups and exercise policy, lifecycle and submission directly.
Original pinned Dart tests, literal event rows, ordinary external native current
and Rust 1.89 consumers, actual two-worker `Send` execution and local `Rc` execution
exercise this frontier. The injected nonce adapter tests use the actual System
decoder and reject owner, allocation, layout, legacy, uninitialized and executable
snapshots. Built-in nonce RPC observation and subscription factory wiring remain
pending. Ordinary outside consumers execute the unchanged 15 original and five
independent ordering/lifecycle tests under current Rust and Rust 1.89. The seven
original focused policy tests also pass under both compilers. No validator or
installed application execution is claimed.

The receipt accounts for all 59 declarations: 31 partial, 28 pending, zero complete. Source
and license hashes are in `docs/parity/transaction-confirmation-sources.tsv`.
