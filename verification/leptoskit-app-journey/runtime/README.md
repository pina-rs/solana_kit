# Generated application runtime

This private verification package composes the SDK's real wallet, HTTP and
WebSocket owners with bounded application timers. The public LeptosKit generator
selects its `from_app_dirs` factory through ordinary Cargo runtime metadata.
It has one interpreter routing owner and returns unrelated platform work intact.
It is never published or reserved on crates.io.

Generate a workspace as described in the sibling wallet fixture, then select
the composed runtime:

```sh
cargo run --locked --manifest-path "$SDK/Cargo.toml" --package solana_kit_xtask \
  -- prepare-wallet-rpc "$PROJECT"
```

Before launching the app, the test controller writes `journey.json` into its
native application data directory:

```json
{"http":"http://127.0.0.1:8899","socket":"ws://127.0.0.1:8900"}
```

`quiet` and `quiet_socket` optionally select separate pending-response peers for
cancellation and unload journeys. All grants require explicit nonzero ports on
numeric loopback addresses. Android additionally permits its emulator's
`10.0.2.2` host gateway. Hostnames, public nodes, userinfo, URL paths, queries and
fragments are rejected. The SDK transports remain general-purpose; these bounds
belong only to this test fixture. No ambient Solana configuration or credentials
are used.

The generated native UI thread activates, polls and closes the service outside
Tokio. The installer remains inert until activation. Two owned Tokio workers run
native I/O. Close aborts and joins HTTP, socket and timer jobs and the wallet
worker, then drops the runtime outside its entered context. The actual
interpreter remains owned by LeptosKit's closing application throughout cleanup.

The fixture's generated guest offers balance reads and an explicitly approved
local-validator SOL transfer through the typed SDK client. Compilation and
headless service tests do not establish an installed application journey.
This runtime selection does not install Android's external wallet
Intent capability; the original `prepare-wallet` selection continues to exercise
that capability. Subscription controls and installed desktop,
Android and iOS journeys remain separate acceptance work. A native runtime test
or a generated guest build does not prove those journeys.

This is exclusively off-chain client verification. On-chain programs use Pina or
official Solana program libraries.

The `generated-wallet-tests` feature enables a separate protocol integration
test. It requires `GENERATED_WALLET_WASM` to name the actual compiled public-generator
guest and fails when that artifact is absent. It drives real guest controls
through production wasmi and the same native service installer, using default
fuel limits. A bounded loopback peer checks rejection, stale approval, actual TCP
cancellation, RPC error recovery and a single approved submission. Its legacy
wire oracle and signature verifier are independent of the SDK's transaction
builder. The peer's acknowledgements, status and balance effects are explicit
protocol fixtures. They do not establish validator execution or installed OS UI
acceptance.
