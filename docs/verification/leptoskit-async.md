# Actual LeptosKit async guest experiment

This private experiment concerns off-chain application clients. It introduces no
public SDK adapter and makes no mobile or installed-application support claim.

The upstream host and plugin bridge are pinned to revision
`fecf8628d326ff7b03f295523e6bb584c3fdf88d`. Source inspection confirms that the
real `WasmHost::wake_plugins` invokes the bridge's exported waker callback and
then the application's optional `leptoskit_poll` executor pump. Its plugin ABI
module is `leptoskit_plugins_v1`, version 1. There are no JavaScript or WASI
imports in the fixture design.

The upstream bridge retains running cancellations until the host drains them.
Call IDs are never reused inside one bridge, but a replacement bridge begins
from its initial IDs. The application therefore needs a generation token around
native callbacks; an old numeric call ID alone is insufficient after unload.

The executed guest holds `Rc` across genuine suspension and uses a local executor.
The application acquires the upstream singleton `Events` receiver and routes
unrelated application events separately from the cancellation signal. The
capability futures never acquire that receiver. The host driver explicitly calls
`complete` and `wake_plugins` on later turns, without periodic polling.

| Actual guest journey     | Observable evidence                                                                                        |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- |
| Entropy                  | Initially Pending; complete fixture bytes `[9, 8, 7, 6]` verified in guest after host wake                 |
| Short/unavailable/denied | Exact errors; caller's sentinel slice remains unchanged on failure                                         |
| Timer                    | Initially Pending at injected tick 0; still pending at 9; completes at deadline 10; backward time rejected |
| Quiet cancellation       | Separate routed event wakes it in one pump, drops the upstream Call and removes the owned fixture job      |
| Late reply               | `InvalidCall` both before and after cancellation drain; no old guest progress                              |
| Bounded state            | Two call slots, two events, 16-byte payload limit, 64-byte bridge budget, two owned fixture jobs           |
| Unload/replacement       | Owned fixture resources reach zero; old generation rejected with a new guest using the same numeric ID     |

The bridge keeps a running cancellation's slot until the application drains it.
The probe observes `Busy` before that drain, then drops its fixture resource and
retains only the new unload-pending job. Re-entering the guest after a rejected
late reply does not revive cancelled work. The guest trace is bounded and checked
in full at every milestone.

The same gates reject controls omitting installed bridge registration, host wake,
or cancellation routing. Corrupt full-length entropy traps on the guest's exact
byte assertion. A numeric-ID-only replacement control demonstrates that the raw
new bridge accepts the old ID and wakes the new guest with corrupt bytes, trapping
its exact assertion. Generation validation belongs to the application, and this
private fixture implements and exercises it. After quiet cancellation, the guest
also reacquires and drops the singleton receiver, proving its application router
released that registration.

Run from the repository root:

```sh
cargo build --locked --manifest-path verification/leptoskit-async/Cargo.toml -p solana_kit_leptoskit_async_guest --target wasm32-unknown-unknown
cargo run --locked --manifest-path verification/leptoskit-async/Cargo.toml -p solana_kit_leptoskit_async_host -- verification/leptoskit-async/target/wasm32-unknown-unknown/debug/solana_kit_leptoskit_async_guest.wasm
cargo test --locked --manifest-path verification/leptoskit-async/Cargo.toml --workspace
```

The host parses the actual compiled artifact and requires exactly the eight
plugin imports and three rendering imports, including every integer parameter
and result signature. Neither JavaScript, WASI nor ambient OS imports are allowed.
The private manifest and lock keep interpreter/UI dependencies outside the root
SDK workspace and the guest's normal/build closure.

The [execution receipt](../../verification/leptoskit-async/evidence/execution.txt)
records the actual canonical and negative journeys. The
[artifact hash](../../verification/leptoskit-async/evidence/artifact.sha256)
identifies the locally executed guest; it is not a cross-machine reproducibility
claim. Missing artifacts fail explicitly. Four native ABI gate tests cover the
real compiled guest, missing imports, a novel ambient import, and a wrong bridge
function signature. Native compilation of the guest crate runs no tests and is
not counted as guest execution evidence.

This establishes an interpreted guest capability integration contract with an
explicit deterministic entropy fixture and injected time. It does not establish
OS cryptographic entropy quality, real-time scheduling, real native I/O abortion,
HTTP, sockets, TLS, mobile device behavior, or a public transport. Dropping owned
fixture work proves local teardown, not acknowledged remote cancellation. Dropping
the WasmHost does not run guest destructors or stop application-owned work; the
application owner must perform that cleanup. Linux CI is defined separately but
has not run locally. Production handlers still need capability grants, platform
resource ownership, acknowledged close where applicable, and their own execution
proof through this same bridge.
