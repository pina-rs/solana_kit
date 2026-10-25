# Actual interpreted guest HTTP

This optional adapter serves off-chain applications. On-chain programs use Pina
or official Solana program libraries. The separate HTTP application guest requires
three renderer imports and eight `leptoskit_plugins_v1` functions. The compiled
artifact gate checks every import and integer signature. JavaScript, WASI, Tokio
and ambient socket imports are absent from the actual guest artifact.

The public `GuestHttp` transport uses upstream Call registration and wakers. The
application owns its local executor, poll export and singleton Events receiver.
Service names select fixed host endpoints, TLS roots, authentication and deadlines.
Guest bytes contain only JSON and receive budgets.

## Application-owned interpreter

`HttpDriver` owns services, jobs and the native JoinSet. Construction, installation,
draining and delivery take short mutable host borrows. Completion waiting borrows
only the driver, so the application can use or destroy its actual App independently.
`HttpHost` delegates to this same driver and additionally owns its interpreter.
There is one service registry and one owner of task accounting.

The actual owning identity and scoped-access APIs come from public commit
`3c6383a17f786172d696e1e83d8026cba2085fa4`. The adapter and canonical HTTP fixture
pin that revision. The separate portable conformance host and async fixture retain
their earlier immutable pins and behavior.

`HttpCompletion` is opaque, owned, non-Clone and non-Copy. It binds its driver,
actual HostIdentity and BridgeIdentity. Both owner identities are necessary because
the public API permits moving an entire Bridge into another interpreter. Retained
allocation tokens prevent identity reuse without retaining interpreter, Bridge
payload or App callback state. No guest memory pointer leaves the interpreter.

Joining releases job capacity before returning the completion. Dropping that
completion abandons delivery without retaining a job or permit. Late replies to
cancelled calls do not enter the guest. Idle waits remain Pending. Explicit
shutdown closes admission, aborts and joins native work. The borrowed driver leaves
the interpreter alive; the owning wrapper then unloads it. Drop aborts as a fallback
and cannot establish awaited cleanup.

## Execute the actual guest

Execute from the repository root. Select the actual guest from Cargo's JSON
output so the host also works with an absolute external `CARGO_TARGET_DIR`.

```sh
cargo +1.99.1 build --locked --offline \
  --manifest-path verification/leptoskit-http/Cargo.toml \
  -p solana_kit_leptoskit_http_guest --target wasm32-unknown-unknown \
  --message-format=json > /tmp/solana-kit-http-guest.json
component="$(jq -se '
  [.[] | select(.reason == "compiler-artifact" and
    .target.name == "solana_kit_leptoskit_http_guest" and
    (.manifest_path | endswith("/leptoskit-http/guest/Cargo.toml")) and
    (.target.kind | index("cdylib")) != null) | .filenames[] |
    select(endswith(".wasm"))] |
  if length == 1 then .[0] else error("expected one interpreted guest") end
' /tmp/solana-kit-http-guest.json)"
cargo +1.99.1 run --locked --offline \
  --manifest-path verification/leptoskit-http/Cargo.toml \
  -p solana_kit_leptoskit_http_host -- \
  "$component"
cargo +1.99.1 test --locked --offline -p solana_kit_leptoskit --all-features
cargo +1.99.1 test --locked --offline \
  --manifest-path verification/leptoskit-http/Cargo.toml --workspace
```

Missing guest bytes fail. The same assertion bodies execute two instances through
public HttpDriver and two through public HttpHost. Each instance passes authenticated
HTTPS balance and u64::MAX, status rejection, body ceilings, quiet deadlines,
local Rc across observed Pending and wake, retained-call no-EOF control, guest Drop
cancellation, capacity retained until abort termination joins, unrelated Events,
and awaited unload with peer EOF. Successful peers and tasks are awaited.

The maintained REST extension adds actual host grant denial, encoded relative
paths and duplicate query keys, authenticated JSON body delivery and reply limits
through the same `HttpDriver` and `HttpHost` owners. This extension is pending
runtime and CI execution. The historical receipts below do not cover these new
REST bodies or the current public transport checkpoint.

## Historical execution receipts

The earlier recorded Rust 1.99.1 and 1.89.0 producers compile the byte-identical canonical guest source
and execute all four journeys. Verbose producer logs identify each actual child
rustc and rustdoc. The actual local guest artifact hashes are:

| Producer    | SHA-256                                                            |
| ----------- | ------------------------------------------------------------------ |
| Rust 1.99.1 | `18449f3ca7781403c89c4ba270d9236afbc72eb16c4ca290cb01e4f444a35588` |
| Rust 1.89.0 | `c2d152d1f4b893cf5a3e346c7364e0d4a8a4bfe942406984039e0df7fcb2f887` |

These identify local debug builds, without a cross-machine reproducibility promise.
The committed receipt records the actual 1.99.1 stdout and hash.

On both compilers, production tests pass seven existing host controls, ten borrowed
driver controls, four ownership controls and the duplicate-completion compiler
negative. Ordinary external consumers preserve the original four public lifecycle
controls, ten borrowed-driver controls and four ownership controls. The original
assertion bodies remain unchanged. They cover equal IDs across actual interpreters,
both transplant axes across 64 restore cycles, replacement and movement, Pending
Tokio waits while App remains usable or is destroyed, late and dropped completions,
out-of-order replies, retained capacity and real cancellation and shutdown EOF.
Four actual compiled-artifact ABI controls also pass.

Current warnings-denied Clippy passes for every affected host, guest and test target.
Both compilers pass warning-denied rustdoc, default and host tests, and public guest
feature Wasm compilation. Rust 1.89 Clippy is unavailable because that installed
toolchain lacks the component. No component or policy was changed.

This is headless interpreter, lifecycle and network proof. Swift and Kotlin pump
and awaited-close hooks and generated desktop, Android and iOS HTTP journeys remain
future work. Registry publication requires compatible published upstream crates;
component publishing remains disabled. The unchanged portable protocol and
five-host conformance receipts remain separate evidence.
