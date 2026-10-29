# solana_kit_client

Executor-neutral typed Solana RPC for off-chain application code. This crate is
not for Solana on-chain programs; use Pina or official Solana program libraries.
It uses std and owned Kit values without a runtime, network, JavaScript or ambient
randomness dependency. Native and local futures share one Client protocol owner.

The optional `resource-estimation` feature adds `resources` for version-aware
resource limits. It simulates an unsigned draft with maximum applicable budgets,
preserves nonce lifetimes, and replaces provisional limits while retaining explicit
ones. The caller owns the transport, deadline, and cancellation. See the
[resource estimation contract](../../docs/parity/client-resource-estimation.md).

The optional `reactive` feature provides caller-driven action and streaming stores
with cached snapshots, scoped cancellation, owned listener leases and a
latest-accepted-value stream bridge. Callers poll or spawn each returned driver;
dropping it cancels its actual owned work. See the
[reactive store contract](../../docs/architecture/reactive-stores.md) for source
semantics, local callbacks and the current verification checkpoint.

`rest` owns provider HTTP methods, explicit absent/JSON/form bodies, ordered
decoded path/query pairs, bounded JSON admission and `Transport<D>`. Each provider
owns its destination identity. Concrete adapters map that identity to approved
roots, credentials and deadlines. REST does not pass through RPC envelopes.
See the [REST design](../../docs/architecture/provider-rest.md) for the source
checkpoint, adapter ownership and held runtime gates.

The initial typed operations are getBalance, getLatestBlockhash and full-base64
getAccountInfo, together with all nineteen node metadata operations in `methods::node`
and all eight economic operations in `methods::economics`. The `methods::accounts`
and `methods::tokens` modules add nine operations and richer accountInfo.
The `methods::history`, `methods::submission`, `methods::transaction_results`,
`methods::address_history` and `methods::simulation` modules add the final fifteen
operations. All 54 catalog methods have typed implementations. Account
reads preserve encoding, slices, allocation, metadata and requested null identities.
Compressed bytes remain encoded; parsed program payloads retain exact JSON.
Raw token amounts parse decimal strings directly to u64, independently of display
values. The original full-base64 AccountInfo contract remains available.

Sysvar fetches compose the existing public account fetch and core decoder. This
preserves the requested address, node context and complete account metadata while
checking owner, size, trailing bytes and caller-selected decoding limits:

```rust
use solana_kit_client::{Client, Transport, methods::{Contextual, ReadConfig}};
use solana_kit_core::{accounts::Account, codecs::Limits,
    sysvars::{Clock, SysvarCodec, CLOCK_ADDRESS, OWNER_ADDRESS}};

# async fn example<T: Transport>(client: &Client<T>)
#     -> Result<Contextual<Account<Clock>>, Box<dyn std::error::Error>> {
let Contextual { context, value } = client
    .fetch_encoded_account(CLOCK_ADDRESS, ReadConfig::default()).await?;
let value = value.require()?.decode(
    &SysvarCodec::<Clock>::NEW, OWNER_ADDRESS, Limits::new(40, 0),
)?;
let clock = Contextual { context, value };
# Ok(clock)
# }
```

For variable-length sysvars, select `SysvarCodec::<SlotHashes>::NEW` with
`SLOT_HASHES_ADDRESS` and an explicit entry budget. Missing accounts remain
`AccountError::Missing`, and transport/remote errors retain their existing causes
and data. The maintained Clock/SlotHashes memory-transport callers execute in the 354-case
portable suite on native Rust, sandboxed Chrome, both WASI previews and the
production LeptosKit interpreter. Ordinary native callers also pass on Rust 1.89.

Address history defaults to signature pages capped at 1000 entries. Explicit full
pages retain their 100-entry cap and strict version negotiation. Transaction and
block reads preserve historical missing-metadata version omission. Submission
accepts opaque bytes without a global legacy packet cap, automatic retry or
confirmation. Full result graphs keep exact quantities and preserve opaque JSON
only for program-defined data and unknown future transaction failures.

`introspection::decode_rpc_transaction` resolves those typed transaction results
into ordered account roles and raw or server-parsed instructions. Walk outer calls
with their inner calls while preserving depth and unavailable metadata. Binary
responses retain exact core transactions. JSON responses retain their original
typed messages without invented wire bytes. Direct core-message helpers share the
same lookup resolution rules. See the [introspection receipt](../../docs/parity/transaction-introspection.md)
for native proof and the remaining runtime limits.

Applications implement Transport or select an explicit platform adapter. The adapter
owns its endpoint, caller authentication, response-header budget, deadline and
cancellation. Enforce Request's body limit while receiving; the client checks it
again before parsing. The client never retries, creates a runtime, reads ambient
credentials or promises a network capability the host did not grant.

Method is an open typed extension contract: providers own their parameter and
result schemas while Client retains HTTP status, JSON-RPC framing, correlation,
remote errors and byte/depth budgets. A response decoder receives only the
validated success fragment. Public built-in results use Kit domain values.

```rust
use serde::{Deserialize, Serialize};
use solana_kit_client::{Error, methods::Method};

#[derive(Serialize)]
struct ProviderStatus(Vec<String>);
#[derive(Deserialize)]
struct Status { healthy: bool }
impl Method for ProviderStatus {
    type Response = Status;
    const NAME: &'static str = "providerStatus";
    fn decode(&self, result: &str) -> Result<Status, Error> {
        serde_json::from_str(result).map_err(Error::Schema)
    }
}
```

Errors distinguish transport capability/policy/deadline failures and original
adapter causes from HTTP status, malformed JSON, schema/domain failures,
correlation and remote JSON-RPC failures. Remote error data retains missing versus
explicit null and exact accepted JSON text, including numeric values beyond
floating-point range. Decoded duplicate object keys remain invalid at every
depth; typed integer and domain schemas remain strict. Client IDs are monotonic, exactly browser-representable and fail
before wrapping; a fresh client needs its own adapter correlation namespace.

`TransportError::with_source` retains `Error + Send + Sync` sources on native
and WASI targets. On `wasm32-unknown-unknown` it accepts local `Error` sources.
Errors containing `TransportError` therefore do not promise `Send` or `Sync` on
that target, including JavaScript-free interpreted guests. This changes no
dependency or import graph. Browser adapters can retain their original local
JavaScript values without adding JavaScript to this crate or the portable core.
Explicit source traversal may expose private adapter diagnostics.

The portable protocol executes with memory transports in native Rust, a real
browser, WASI Preview 1 and 2, and the actual pinned LeptosKit interpreted host.
Those fixtures include all 54 catalog methods with typed results and literal requests.
The [complete catalog runtime receipt](../../docs/verification/rpc-history.md)
records the 72-case suite and bounded declaration accounting.
The [economics runtime receipt](../../docs/verification/rpc-economics.md) retains
the exact 42-ID accounting and the three partial or pending declarations.
They do not prove production network adapters or installed browser/mobile journeys.

RPC subscriptions use `subscriptions::Connection`. The caller owns the socket,
clock, and connection lifecycle. Drain `outbound()` frames, pass complete text
frames to `ingest(generation, bytes)`, and drain typed notifications with `next()`.
Call `tick()` to enforce acknowledgement deadlines and remote cancellation cleanup.
Six families are stable. Block, vote, and slot-update methods are explicitly
unstable. This transport-neutral protocol does not establish networking support
for any host. Replacement returns an explicit event gap and never promises replay.

`subscriptions::Session` drives that same protocol owner through transient socket
polls. The optional `confirmation-factories` feature supplies known-signature
confirmation and guarded one-shot submission with an injected clock, deadline and
cancellation. Shared operations take short `SessionAccess` borrows, preserving
unrelated registrations. Applications retain their executor and shared socket
lifetime. See the [factory contract](../../docs/verification/confirmation-factories.md)
for ownership, uncertain cleanup and expiry freshness.

`Clock::Sleep` returns `Result<(), ClockError>`. Successful expiry remains distinct
from capacity, deadline-range and unavailable-clock failures. Confirmation retains
clock failures and their original causes separately from cleanup failures. A clock
failure never authorizes a submission retry. These composed source paths await
current/MSRV, platform and quality verification.

`plans` executes core-owned transaction trees through caller-owned concrete
futures. Parallel branches overlap while results retain plan order. Ordered
failures preserve partial context and cancel unstarted leaves. Signing and
sending capabilities remain explicit. See the
[instruction plan contracts](../../verification/instruction-plans/README.md).
