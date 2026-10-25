# Public Preview2 text socket verification

This private harness consumes `solana_kit_wasi` with its public `websocket`
feature. It does not implement a second socket codec or expose a publishable crate.
Build `guest` for wasm32-wasip2 and `runner` for the native host. Run the latter
with the pinned Wasmtime50 CLI executable and actual guest component path, or
with `--embedded` and the actual component path. The embedded host requires
Rust1.97 or newer; guest and SDK compiler identities are recorded separately.

Independent loopback peers test WS, trusted WSS, pending-receive duplex writes,
fragmented text, corrupt-value sensitivity, trust/name/upgrade rejection, frame
bounds, binary/UTF8/masking/abrupt-EOF errors, quiet cancellation, retained-owner
control and process destruction. The embedded mode separately executes the
public API with exact per-destination policy, actual pending Store destruction
and a fresh replacement Store. Process destruction is not Store destruction.

The embedded host allows only wildcard ephemeral TCP bind and its exact loopback
TCP destination. It denies listening, accepting and UDP. Its application owns
the explicit fixture trust anchors. Fixtures are deliberately public disposable
test certificates from the earlier `wasi-sockets` experiment; never deploy them.
The host waits for socket EOF and restored runtime task counts after Store drop.
The receipt distinguishes these bounded cases from uncancellable operating-system
DNS callbacks, arbitrary hosts, installed applications and untested schedules.

This entire SDK is off-chain application code. Use Pina or official Solana program
libraries for on-chain programs.
