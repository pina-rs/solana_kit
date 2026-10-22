# solana_kit_client

Executor-neutral typed Solana RPC for off-chain application code. This crate is
not for Solana on-chain programs; use Pina or official Solana program libraries.
It uses std and owned Kit values without a runtime, network, JavaScript or ambient
randomness dependency. Native and local futures share one Client protocol owner.

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

Address history defaults to signature pages capped at 1000 entries. Explicit full
pages retain their 100-entry cap and strict version negotiation. Transaction and
block reads preserve historical missing-metadata version omission. Submission
accepts opaque bytes without a global legacy packet cap, automatic retry or
confirmation. Full result graphs keep exact quantities and preserve opaque JSON
only for program-defined data and unknown future transaction failures.

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
