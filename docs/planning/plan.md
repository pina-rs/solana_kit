# Implementation plan

## Outcome

Provide an exclusively off-chain client SDK for application code, covering the
capabilities of the pinned Dart Solana Kit ecosystem through idiomatic Rust APIs
usable in native applications, browsers, WASI hosts, and LeptosKit's interpreted
Wasm guests on desktop, Android, and iOS. Use small
dependency graphs, audited cryptography, and monochange for releases.

Do not use Solana Kit in Solana on-chain programs; use Pina or official Solana
program libraries instead. `no_std + alloc` supports portable clients, not
on-chain deployment. Every implementation phase and runtime target below stays
within this off-chain client scope.

The reference audit covers 70 Dart packages and one Codama renderer. It includes
17 program clients, six external integrations, two wallet protocol packages,
three Flutter adapters/UI packages, developer tooling, and the core SDK.
[The ledger](parity.tsv) accounts for every top-level package. Source file counts
measure scope; they do not establish behavioral coverage.

The Rust SDK is complete when every package capability has an implemented,
behaviorally evidenced Rust equivalent, every public operation has behavioral evidence,
all promised runtime journeys execute successfully, and registry release
preparation succeeds. A compiled placeholder or a skipped test cannot satisfy
an entry. Flutter widgets become an appropriate Rust/LeptosKit experience;
publishing tools remain native tools. An existing Rust facility may replace a
reference capability only when executable evidence demonstrates the equivalent
behavior; documentation alone cannot complete a ledger entry.

## Delivery method

Use small PRs that each end with an independently observable result. Merge only
after their checks and review pass. Record deviations and evidence in the parity
ledger and decision trail. Keep prospective tasks here, rather than in changesets.

The initial, uncalibrated estimate is 20–35 implementation/review units across the
phases below. This is a planning estimate, not an audited workload count or a time
or delivery commitment. Symbol-level
inventory and the host transport experiments will refine it. Generated clients
reduce repeated work; wallet and mobile host integration are the largest unknowns.

Verification rigor is high because malformed encodings, signing mistakes, integer
loss, or incorrect expiry can produce valid-looking but incorrect transactions.
Protocol vectors, negative cases, actual runtime execution, and downstream
dependency checks are required in addition to ordinary unit tests.

## Phase 0: grounded design

Deliver immutable source pins, a package disposition ledger, alternative crate
designs, caller-facing usage sketches, and a host capability matrix. Identify
which behavior is reusable and which platform glue must change.

Acceptance: every top-level reference package appears once; current source is
distinguished from stale checkouts; mobile networking gaps are visible; no SDK
support or parity claim is inferred from reference documentation.

## Phase 1: workspace, fixtures, and portability experiments

Create the Cargo workspace, stable-toolchain policy, formatting/lint gates,
reference-fixture extraction, and a Rust `xtask` verification tool. Enforce
monostyle readability and complexity scores of at least 90 without disabling
scoring rules. Establish
monochange package ownership, release groups, changesets, and dry-run release
checks before adding multiple published crates.

Prove the riskiest boundaries through narrow, runnable vertical experiments
first. Later phases complete the APIs and behavior catalogs; an experiment is
not broad capability completion:

1. Run identical address/hash/signature and instruction fixtures natively,
   in a browser, in a no-JavaScript wasmi guest, and in a WASI host.
2. Compile and execute a native `tokio::spawn` usage and a local future that holds
   `Rc` across an await. Keep these guarantees available through generic clients.
3. Exercise official standalone System Program account creation on each target
   and inspect its full normal/build dependency closure before selecting reuse.
4. Verify async guest entropy, injected timers, and cancellation wakeups without
   assuming browser globals or OS APIs.
5. Compile a downstream crate without repository-specific rustflags or patches.

The expanded 45-case portable-core/program suite now executes natively, in real Chrome,
in Wasmtime 50.0.0 Preview 1/2 commands, and in the pinned actual LeptosKit
headless WasmHost. Each host observes the corrupt-hash negative control.
`cargo xtask all` reproduces those runs, binary import/dependency gates, and a
clean external consumer. See the [runtime receipt](../verification/runtimes.md)
for exact versions, artifact hashes, commands and limits. Async contracts,
System Program reuse, host capabilities and device journeys remain open.

Start WASI core checks with Preview 1 and Preview 2; implement the named Preview 2
HTTP host contract first. Pin the host and WASI interface versions. Execute its
clock, entropy, cancellation, and outbound HTTP operations. Choose and prove a
TLS-capable WebSocket backend or explicit host bridge before advertising WASI
subscriptions. An HTTP-only component host does not imply socket availability.

Acceptance: repeatable execution logs, import/dependency allowlists, and a
release preview for a real initial package. Record compile-only cells honestly.

## Phase 2: deterministic primitives and codecs

Deliver addresses, seeded/PDA derivation, hashes, signatures, exact integer types,
errors, binary codecs, fixed points, options, account models, and instructions.
Use audited Rust implementations for base58, hashing, curve operations, and
Ed25519. Keep entropy and key-file persistence outside deterministic crypto.

Collapse related Dart packages into Rust modules where that removes packaging
overhead without adding unwanted dependencies. Optional crypto/serialization
features must leave simple instruction users with a small graph.

Acceptance: cross-language golden vectors plus malformed lengths, seeds, curve
points, overflow, truncated data, invalid discriminators, and trailing-data
policy. Execute the portable layer across all four environment classes.

The bounded codec slice implements 53 wire operations and partially implements
21 factories from the 1,387-ID Phase 2 inventory. Its per-unit receipt leaves
1,313 other IDs uncovered; Phase 2 remains incomplete. See the
[codec receipt](../verification/core-codecs.md) for exact supported configurations,
remaining IDs, provenance, and tests.

## Phase 3: messages, transactions, and signing

Deliver legacy, v0, and v1 compilation, decoding, signatures, lookup tables,
durable nonces, transaction introspection, off-chain messages, and signer
capabilities. Preserve immutable signed message bytes and lifetime metadata.

The pinned Dart source already includes v1's message-first envelope. Audit its
vectors against the pinned upstream implementation and actual validator support;
do not assume older legacy/v0 size limits apply to v1. Reject unsupported node
capabilities explicitly.

Acceptance: byte-for-byte independent oracles, deterministic signatures,
privilege merging/account ordering, fee-payer/signature ordering, exact packet
limits per version, malformed inputs, and sign/decode/re-encode behavior. Separate
construction requirements from validation of externally received transactions.

The immutable legacy/v0 slice compiles owned instructions and lookup snapshots,
preserves exact signed bytes, validates received protocol wire independently of
local compiler restrictions, and signs/verifies through optional deterministic
crypto. Its four transaction cases remain in the shared 45-case suite. The
[transaction ledger](../parity/message-transactions.md) retains all 746 source
IDs: 22 partial and 724 pending, none complete. V1, durable nonces, full
introspection, off-chain messages, general signer capabilities, confirmation,
and instruction plans remain outstanding; Phase 3 is incomplete.

## Phase 4: RPC and host transports

Port all 54 reference RPC catalog entries to typed request/response pairs, including
context and configuration. Improve on the reference's 14 convenience methods:
no untyped escape hatch should be required for a cataloged standard operation.

Build native Tokio, browser, and WASI HTTP adapters. Add the optional LeptosKit
adapter with a reference host harness. Keep HTTP semantics, JSON-RPC errors,
integer precision, request correlation, deadlines, body limits, cancellation,
and caller-owned authentication explicit. Never automatically retry transaction
submission as an ordinary idempotent read.

LeptosKit needs a cancellable async guest bridge for HTTP, sockets, and timers.
The current direct native/browser plugins cannot satisfy this contract alone.
Keep host integration in its owning project or an adapter crate; the Solana core
must not depend on a UI framework. Installed app hosts grant capabilities and
destinations; guest code cannot grant itself access.

Acceptance: typed catalog coverage, loopback success/error/timeout/cancellation
tests, browser execution, WASI execution, and interpreted guest-to-host execution.
Capture permission and capability failures as errors. No fallback returns an
empty account, zero balance, or successful-looking result.

## Phase 5: subscriptions and orchestration

Deliver nine reference subscription families, including separately marked
unstable methods; lifetime-aware transaction confirmation; resource estimation;
and sequential/parallel instruction plans.

Use one connection owner with bounded dispatch queues, explicit overflow,
request/notification correlation, subscribe/unsubscribe acknowledgements,
cancellation, and cleanup. Cover notification-before-ack races and connection
replacement. Reconnection must state gaps; a new subscription cannot promise
replay of missed events without a source of historical data.

Acceptance: stalled/slow readers do not retain unbounded history; missing or
rejected acknowledgements terminate; unsubscribe false is an error; failed
transactions and expired blockhashes/nonces are distinguished. Test host unload,
late replies, and quiet-stream cancellation across transports.

## Phase 6: program clients and generation

Evaluate the existing Rust Codama renderer against Kit-owned types. Reuse its
infrastructure if adaptable; otherwise implement a focused native Rust generator
over the pinned Codama schema. Keep Node tooling outside runtime libraries if
needed for upstream normalization; project-owned generator logic belongs in Rust.

Reuse pinned IDLs, corrected layouts, discriminator facts, normalization rules,
and fixtures. Generate account/instruction codecs, builders, parsers, PDAs, and
errors reproducibly. Resolve and validate all output before replacing files.
Preserve handwritten behavior where the reference program is not generated.

Deliver System, Compute Budget, Memo, ATA, Token, Token-2022, ALT, Stake, Config,
Loader, Account Compression, Bubblegum, Token Metadata, MPL Core, Squads,
Attestation Service, and the recurring-payments Subscriptions program. The last
is distinct from RPC subscriptions.

Acceptance: generator golden output and drift check, negative IDLs/security cases,
byte oracles, and pinned validator integration. No program client pulls an Agave
client or validator into application normal/build dependencies.

## Phase 7: wallets and application integration

Port Wallet Standard and Mobile Wallet Adapter protocols; local, modifying,
partial, and sending signer capabilities; browser discovery; host-mediated
mobile interactions; and wallet adapter/UI equivalents suited to LeptosKit.
Keep key-file persistence and dApp publishing separate from guest libraries.

Acceptance: deterministic protocol tests, rejection/disconnect/cancel paths,
user-gesture handling where required, and guest unload/replacement. Run a real
generated LeptosKit desktop app, Android emulator, and iOS simulator through
balance fetch, signing, send/confirm, subscribe/unsubscribe, and pending-I/O unload.
Record wallet/platform availability instead of assuming Android MWA exists on iOS.

## Phase 8: integrations, tooling, and release readiness

Deliver Anchor, Helius, Jupiter, Pyth, SNS, Surfpool, the dApp publishing CLI,
Rust test support, documentation, migration examples, and equivalent lint policy.
Keep paid-service tests credential-optional but retain deterministic replay tests;
missing credentials must not masquerade as end-to-end verification.

Monochange previews must write the intended Cargo manifest/version fields,
propagate dependency changes, produce changelogs, and respect publish order.
Rehearse first publication from a clean downstream context. Use registry trusted
publishing once configured; do not use local credentials for an unrequested release.

Acceptance: the capability ledger is complete, runtime evidence is linked,
dependency allowlists pass, clean-consumer packaging succeeds, API/docs agree,
and release readiness succeeds. Publish only implemented crates.

## Runtime evidence policy

These environments host off-chain application code. Solana on-chain program
execution is outside the SDK's supported targets.

| Environment              | Required evidence                                                                                |
| ------------------------ | ------------------------------------------------------------------------------------------------ |
| Native Tokio             | Linux/macOS/Windows checks; multi-thread spawn; real HTTP/WS; cancellation                       |
| Browser Wasm             | Actual browser vectors and fetch/WS; wallet/entropy; local futures                               |
| Plain core Wasm          | Import allowlist; wasmi instantiation; vectors; async host wake/cancel                           |
| WASI Preview 1           | Core compile and execution in a named pinned host                                                |
| WASI Preview 2           | Component compile and execution; named HTTP/clock/random capabilities; explicit socket/TLS proof |
| LeptosKit desktop/mobile | Generated app journeys through the real guest bridge and installed host                          |

Keep unsupported, compiled, executed, and device-verified states distinct. Do not
infer a runtime from `target_arch = "wasm32"`; browser and LeptosKit guests share
the same Rust target. See [the architecture](../architecture/design.md).
