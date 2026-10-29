# Caller-driven reactive stores

The optional `solana_kit_client/reactive` feature adds action and streaming stores.
It enables only `futures-core`; the default client dependency graph is unchanged.
Factories, futures and streams remain concrete. No executor, background task,
timer, network adapter or reconnection policy is started by this module.

`ReactiveActionStore::new(factory)` accepts a plain `Fn(signal, args)` returning a
future of `Result<T, E>`. `dispatch(args)` eagerly publishes running state and
returns the sole driver to await or explicitly spawn. `dispatch_with_signal`
adds caller cancellation. Dropping unfinished work cancels its actual future and
publishes a cancellation error while preserving cached data if still current.

`ReactiveStreamStore::new(factory)` accepts a plain `Fn(signal)` returning a future
of `Result<ReactiveStreamConnection<D, R>, E>`. Its data lane yields `Result<T, E>`;
its error lane yields `Option<E>`, ignoring `None` items. `connect()` eagerly
publishes loading state and returns the sole driver. `connect_with_signal` adds
caller cancellation. Each poll reads at most one item per lane, data before error
when both are ready. Sources obey normal future/stream wakeup contracts.

Shared immutable snapshots retain values/errors in `Arc` without user `Clone`
bounds. Running/loading preserve previous data and error. Success/loaded clears
error. Failure retains cached data. Stream errors remain recoverable: the first
error wins until data arrives or another connection begins. Ending both lanes
finishes their driver and leaves the snapshot unchanged. Reset clears cached
fields. Dispose permanently cancels work and clears listeners while preserving
the last readable snapshot.

Use `LocalReactiveActionStore::new_local` or
`LocalReactiveStreamStore::new_local` for callbacks or bridge payloads using
`Rc`/`RefCell`. Default callbacks require `Send + Sync`; factories/futures have no
universal `Send` bound. Native driver/store auto traits follow concrete inputs.
Constructors infer future and stream types; applications needing a named field
type can define an alias.

Keep the `ListenerLease` returned by `subscribe(Arc::new(callback))`. Drop or
`unsubscribe()` stops that independent registration idempotently. Stores hold
weak registrations and weak work-slot references, avoiding a reverse strong edge
when a callback or future captures its store. Commands extract old ownership
before user destruction, polling, callback invocation or waker operations.
Supersession/reset/dispose synchronously destroy retained work. Cancellation
during an executing poll prevents restoration/commit and destroys the work when
that poll returns. Caller cancellation is immediately readable and wakes drivers;
their next poll publishes the cancellation. Cancellation wins ready completion.

`observe(optional_token)` observes current and subsequent loaded values with one
latest accepted-value slot. It never connects, resets or disposes the store.
`observe_filtered(token, predicate)` runs a fallible predicate for every loaded
notification even while unpolled. Rejected values preserve an older accepted
value. A retained terminal error beats buffered data and later store recovery.
Predicate failure affects only the bridge. Cancellation ends it cleanly ahead of
initial data/errors; Drop unsubscribes while parked. Source end and store dispose
do not end observation. Snapshot coalescing does not promise lossless network
delivery or add a network queue.

## Pinned source and adaptations

Reference: Dart commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46`, package
`solana_kit_subscribable`. SHA-256:

| Source                                | SHA-256                                                            |
| ------------------------------------- | ------------------------------------------------------------------ |
| `reactive_action_store.dart`          | `82cec0ef132092286a4443d9a7619e40dcf930c45f3eefb9e46b034a8fac9df1` |
| `reactive_stream_store.dart`          | `c3d774968b2a113701c59771eb45b6e60fa57f6f8fee32024de282bb8646f28d` |
| `bridge_store_to_async_iterable.dart` | `f3ab1cca5a28a6a8648bada03e4d6c8791d3ddd73ea316792aae0030999eba5f` |

Rust drivers replace executor-backed fire-and-forget calls; typed arguments,
snapshots and failures replace dynamic objects. Listeners have independent leases.
A running callback that resets/replaces the store prevents action-factory I/O,
matching Dart's stream safeguard. Deactivated captured listeners are skipped.
Snapshot identity validation and admission use the explicit Core-to-buffer lock
order, changing only metadata while both guards exist. Predicate execution,
buffered-value replacement, user destruction and waker operations follow after
both guards are released. Delayed old captures and reentrant predicates cannot
overwrite a newer snapshot. These deliberate
hardenings differ from Dart's unguarded post-running action invocation, copied
listener set and predicate reentry behavior.

## Verification checkpoint

All twenty reactive bodies pass natively. Sixteen reactive groups join the six
preserved resource groups in shared suite v49: all 422 cases pass, while deliberate
corruption passes 421 and exits one. Both golden reports were copied from those
actual executions. Four additional native tests cover conditional `Send`,
idle-constructor inference, adversarial waker clone/drop/wake reentry, and the
barrier-controlled stale-observation race. Local stores accept plain callbacks
through `subscribe_local`, including callbacks that capture `Rc`.

Strict all-target/all-feature client Clippy and API documentation pass. Controlled
futures and streams use direct polling without sleeps. Current-source browser,
both WASI previews, production LeptosKit/wasmi and installed-app CI remain required.
The separate fuel-test build stopped at the cache cleanup threshold before its
tests ran; it requires fresh verification and is not a passing result.
