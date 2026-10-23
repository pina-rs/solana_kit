# Instruction plan evidence

The authority is the complete `solana_kit_instruction_plans` package at pinned
Dart commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46`. Its MIT notice is
preserved in [DART-LICENSE](DART-LICENSE). The canonical inventory contains 212
declarations and 11 defining source files. `canonical.tsv` retains each exact
source hash and maps every declaration to its Rust contract. All remain marked
partial. The [portable runtime receipt](../../docs/verification/instruction-plans-runtime.md)
records the bounded five-host execution; it does not establish every source contract.

`oracle.dart` executes the pinned planner for six independent tree shapes and
records each compiled message's exact bytes. `oracle.tsv` is the actual output.
Core tests compare both structure and every message byte. Client tests compare
the failed/canceled and concurrent-leaf result rows independently. The original
pinned package's preserved reference receipt records 359 tests with an explicit
package configuration from an owned working directory, without pub resolution.

The Rust model uses one owned `Plan<T>` for instruction, transaction and result
trees. Exhaustive matching replaces Dart class predicates and assertions. Typed
`PlanInput<T>` prevents mixed instruction/message inputs. Singleton lists unwrap
recursively, while empty lists remain sequences. `MaxInstructions` validates
1..=64 and defaults to16. Builders reserve their own instructions, payer,
lifetime, lookup snapshots and optional v1 configuration. Planning uses actual
Kit compilation, including reserved signature bytes and version-specific packet
limits. No approximate byte allowance is accepted.

`InstructionBatch` retains rejected instructions. `Linear` tests actual compiled
candidate lengths and advances only after success. Its instruction constructor
must represent the requested linear data span in its payload. `realloc` splits
at10,240 bytes. Custom factories produce fresh independent packers. Custom
packers must leave progress unchanged on error. The planner rejects changed
metadata, removed prefix instructions, non-progress and invalid successful
packer output. An updater failure after packing aborts the entire plan.

`Planner::plan_async` awaits caller-owned `AsyncMessageBuilder` construction and
update futures at each actual candidate decision, before compilation or commitment.
Concrete associated futures support local `Rc` and conditional native `Send`.
Dropping planning drops its active callback and every owned candidate; external
effects already performed remain the caller's responsibility. Pure
`MessageBuilder` policies retain synchronous `plan` through a ready adapter sharing
the same traversal. No implicit RPC, runtime, retry or detached work is selected. The client executor
accepts concrete associated futures owning each message and borrowing only the
external operation owner. Local `Rc` and native `Send` are both supported.
Ordered execution rejects indivisible transaction bundles before starting;
`ConcurrentLeaves` explicitly runs all leaves, including across sequence edges.
Concurrent results retain tree order and divisibility. Failure halts unstarted
ordered work while active siblings settle. Explicit cancellation has the same
settle-started policy; dropping the future drops all active leaf futures.

Successful results require an explicit typed signature. Partial typed context
and original failures survive without deriving signatures from unsigned
transactions. `Boundary` composes caller-provided signing and sending, rejects a
changed planned message and retains signing versus sending errors.
`FailureReport` exposes original typed preflight payloads, zero-based failure
indices, submission markers and bounded last-eight-log presentation. Signing
reports suppress submission markers. Dynamic map merging and deprecated
string/transaction return coercions are replaced by typed `LeafOutcome`.

This is exclusively an off-chain application SDK. On-chain programs should use
Pina or official Solana program libraries. Actual portable host execution is
recorded separately from compilation, validator and installed-device evidence.

The asynchronous policy supplies concrete `Create<'a>` and `Update<'a>` futures
through `create_async` and `update_async`. `Planner::plan_async` borrows the policy
and input tree until completion or drop. `Boundary::run` awaits the planner before
signing. A pending update owns its proposal while earlier candidates remain in the
private arena; no partially planned tree escapes an error.

Two designs were compared: a callback trait on the existing traversal, and an
explicit request/response machine with separate synchronous and asynchronous
drivers. The callback trait is smaller because the caller's future already models
the suspended operation. An owned traversal stack replaces recursive futures,
keeping future types concrete and retaining conditional `Send`. The synchronous
adapter contains only `Ready` futures, so its entry point polls that closed adapter
once. Packing, compilation, retry classification and commitment each have one
implementation shared by both entry points. Traversal frames own their candidate
indices and child iterators, without shared interior mutation or self references.

The original six-tree oracle and assertions remain unchanged. `async_oracle.dart`
executes source-style awaited construction/update hooks and records their exact
order plus two message byte strings. `async_planning.rs` compares those literals
and independently exercises real Pending callbacks with registered wakers, changing
per-proposal context, later failures, no-progress, cancellation by drop, local `Rc`,
and actual native Tokio spawn. The portable adapters execute all 43 mapped
host-neutral bodies, retaining 184 ordered assertion sites, on native Rust, Chrome,
WASIp1, WASIp2 and LeptosKit's actual interpreter. Five Tokio-only bodies retain
separate native execution. All 212 declarations remain partial: typed adaptations,
caller-owned resource factories, timers, cancellation policy and installed-device
journeys still need their own contract adjudication or implementation.
