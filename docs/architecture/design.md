# Architecture proposal

Status: preimplementation design. The examples and signatures below are proposed
contracts, not available SDK APIs. The first portability experiments may refine
them before they become public. Source grounding is in
[reference findings](../planning/reference-findings.md).

## Caller experience

A native application chooses a Tokio adapter. A browser chooses a browser
adapter. An interpreted guest chooses a host bridge. Transaction construction,
wire encoding, and typed RPC operations use the same Kit-owned values.

Proposed native usage:

```rust,ignore
use solana_kit::{Address, Commitment, Rpc};
use solana_kit_native::NativeHttp;

let rpc = Rpc::new(NativeHttp::new(endpoint, limits)?);
let owner: Address = address_text.parse()?;
let balance = rpc.balance(owner, Commitment::Confirmed).await?;
```

Proposed interpreted-guest usage, after host handlers exist:

```rust,ignore
let io = solana_kit_leptoskit::GuestIo::connect(required_capabilities).await?;
let rpc = Rpc::new(io.http());
let balance = rpc.balance(owner, Commitment::Confirmed).await?;
let seed = io.entropy().seed().await?;
let signer = Keypair::from_seed(seed);
```

Proposed transaction flow:

```rust,ignore
let lifetime = rpc.latest_blockhash(Commitment::Confirmed).await?.value;
let message = Message::v0(payer.address(), lifetime)
    .with_instruction(solana_kit_system::transfer(from, to, lamports))
    .compile()?;
let transaction = message.sign(&payer).await?;
let receipt = rpc.send_and_confirm(transaction, clock, policy).await?;
```

Payer, lifetime, and required signatures must be valid before submission. Incoming
transactions follow a validated decoding path that preserves their original
signed message bytes. Local keys are one signer implementation; wallet signers
have distinct modifying, partial-signing, and sending capabilities.

## Crates and dependency direction

| Crate boundary             | Responsibility                                                                                                                                                                        |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `solana_kit_core`          | Owned addresses, hashes, signatures, amounts, codecs, accounts, instructions, message/transaction wire models, deterministic crypto, signer contracts, off-chain messages, pure plans |
| `solana_kit_client`        | Typed RPC catalog, bounded pubsub, confirmation, account fetching, plan execution, resource estimation, adapter contracts                                                             |
| `solana_kit_native`        | Tokio networking, clocks, entropy, native filesystem helpers where appropriate                                                                                                        |
| `solana_kit_browser`       | Explicit browser HTTP/WS/clock/entropy integration and local futures                                                                                                                  |
| `solana_kit_wasi`          | Named WASI interface/host contracts; no browser assumptions                                                                                                                           |
| `solana_kit_leptoskit`     | Optional interpreted-guest capabilities and installed-host integration                                                                                                                |
| `solana_kit_interop`       | Optional official Solana type conversions                                                                                                                                             |
| `solana_kit_<program>`     | Independently useful program codecs/builders/PDAs/errors, depending on core                                                                                                           |
| Wallet and provider crates | Portable wallet protocols, platform adapters/UI, and optional third-party integrations                                                                                                |
| `solana_kit`               | Discoverable re-exports and useful cross-domain composition                                                                                                                           |

Keep the deterministic layer `no_std + alloc` with optional audited crypto and
serde features. It has no HTTP, Tokio, JavaScript, filesystem, or ambient RNG
dependency. Internal compiler/wire modules remain private. JSON representation
belongs to the client; serde does not define transaction wire format.

The initial client may use `std` collections while remaining executor independent;
do not promise no_std RPC until proved. Programs need core, not the client or
umbrella. Expensive algorithms remain optional. Measure actual address-only,
transaction-only, and full-client consumers before splitting the core further.

Explicit adapter dependencies select environments. Cargo features add capabilities;
they must not globally select one environment. Browser and interpreted guests
share `wasm32-unknown-unknown`, so neither architecture nor OS cfg can distinguish
them. Native/JS/WASI dependencies and imports stay out of the plain guest graph.

## Async contracts

Rust futures are shared protocol machinery; Tokio and browser/guest executors
poll them differently. Core/client operations do not start runtimes or detach
tasks. The public adapter interface permits local futures. Concrete native
adapters retain `Send` where their contents permit it.

Illustrative adapter signatures:

```rust,ignore
pub trait HttpTransport {
    type Exchange<'a>: Future<Output = Result<HttpReply, TransportError>> + 'a
    where
        Self: 'a;

    fn post<'a>(&'a self, request: HttpRequest) -> Self::Exchange<'a>;
}

pub trait Clock {
    type Sleep<'a>: Future<Output = Result<(), ClockError>> + 'a
    where
        Self: 'a;

    fn now(&self) -> MonotonicTick;
    fn sleep_until(&self, deadline: MonotonicTick) -> Self::Sleep<'_>;
}

pub trait Entropy {
    type Fill<'a>: Future<Output = Result<(), EntropyError>> + 'a
    where
        Self: 'a;

    fn fill<'a>(&'a self, bytes: &'a mut [u8]) -> Self::Fill<'a>;
}
```

These are extension contracts for adapter authors. Ordinary callers see typed
domain values, not JSON, reqwest, web-sys, WIT, or official Solana types. Request
budgets, bounded response bodies, cancellation-on-drop, status handling, and
capability failure are part of the implemented adapter contract.

Use ordinary inherent async methods over concrete associated futures where that
is sufficient. If a named state-machine future becomes necessary, it borrows
external transports/clocks and owns its request state; it must not contain a
future borrowing a movable sibling field. Pin projection alone does not make
self-referential construction valid. Phase 1 must compile/run both native
multi-thread `tokio::spawn` and local `Rc`-across-await callers before this API
is accepted. No unsafe Send assertions or universal Send bound.

Minimal signer and entropy contracts live beside the transaction/key types in
core. This lets those types implement signing without a dependency on the client
or a foreign inherent implementation in another crate.

Entropy is asynchronous because embedded hosts can deliver it asynchronously.
Await seed bytes, zeroize temporary secrets, then use deterministic audited key
derivation/signing. Decoding, PDA derivation, and deterministic signing need no
randomness capability. Filesystem persistence remains native-only behavior.

## Subscription and transaction lifecycle

One driver owns each connection, correlation registry, generation, and bounded
dispatch queues. Registration precedes subscribe send. Early notifications and
acknowledgements cannot race past uninstalled routing state. Bound pending calls,
active subscriptions, frame sizes, early notifications, and retained queue bytes.
Overflow is an observable lag/error with cleanup, not silent data loss.

The application or a documented adapter owner holds the driver lifecycle.
Cancellation marks the subscription slot and wakes the driver independently of
command-queue capacity. Explicit close waits for a true unsubscribe acknowledgement
with a deadline; Drop promises local teardown and best-effort remote cleanup.
Dropping a Tokio `JoinHandle` detaches its task, so an owner must explicitly abort
or implement and document cancel-on-drop semantics.

LeptosKit's single guest event receiver remains application owned. The adapter
receives routed socket events or uses cancellable per-socket receive operations.
Guest unload closes sockets, cancels actual native work, retains accounting until
termination, and rejects late completions. Reconnect creates a new generation and
reports a gap requiring reconciliation; it cannot invent lost history.

Confirmation preserves lifetime metadata, checks commitment and node-reported
signature, and distinguishes transaction failure, expiry, cancellation, timeout,
and uncertain submission. Blockhash expiry uses block height; durable nonce expiry
uses nonce state. Retry policy must account for transaction submission uncertainty.

## Reuse and version ownership

Prefer audited small official implementations such as standalone System Program
constructors when their selected graph passes native, browser, WASI, and plain
guest proofs. Keep conversions private and public Kit types stable. If a graph
cannot satisfy a target, evaluate a pinned small source reuse or generation from
the same IDL with its license and byte-level oracle. Broad client/validator/decoder
stacks remain outside normal/build application dependencies.

Monochange owns core/client/adapter lockstep releases while those contracts evolve
together. Each program and provider integration can release independently because
upstream IDL/service updates have a different cadence. Version fields and groups
must have exactly one writer; dependency propagation, archives, changelogs, and
publish order are checked in release previews. Do not add groups or empty crates
for future capabilities before they have implementations.

## Synthesis and alternatives

Two independently drafted designs compared selectable domain crates with a
consolidated core/client design. The consolidated shape is the base: a single
owner for tightly coupled message/account/signature invariants reduces version
edges and public coordination. It keeps host dependencies behind explicit cuts.

Adopt the domain-crate candidate's independent program versioning, explicit driver
ownership, and singleton guest-event routing constraint. Correct its synchronous
entropy sketch, its pubsub/program crate-name collision, and its assumption that
dropping an executor task handle cancels work. Keep `solana_kit_subscriptions`
for the reference on-chain program; RPC pubsub lives in client modules.

The rejected many-domain-crate design offers finer compilation cuts, but creates
more version edges before measurements show a benefit. A 71-crate literal port
adds shallow packages; a single backend selected by `js`/`ssr` flags cannot model
the two different environments sharing one Wasm target. Neither becomes the base.

Both candidates used the available inherited GPT model; the configured Claude/Grok
families were unavailable. An independent GPT-6 Astra review checks the designs,
but this is not cross-family consensus or runtime verification.

## Remaining risks and first implementation

Resolve the exact WASI host/socket/TLS contract and LeptosKit installed-host bridge
in early experiments. Publishing the LeptosKit adapter may need to wait for its
dependencies to become registry-available. Avoid publishing an unusable adapter
with missing host behavior. Keep platform-specific wallet availability explicit.

First implement the workspace/release/fixture foundation and a real four-host
primitive and constructor experiment. The [delivery plan](../planning/plan.md)
defines the evidence required before expanding the SDK.
