# Composed slot-tracking store

`reactive::slot_tracking::SlotTrackingStore` combines fresh initial-future and
subscription-stream factories behind one caller-driven connection.
`LocalSlotTrackingStore::new_local` accepts local factories, fallible maps and
listener captures. Neither constructor starts sources. `connect()` publishes
loading, starts subscription then initial factories, and returns the sole driver.
The caller polls or explicitly spawns that driver. No executor, timer, reconnect
task or new normal dependency is started. Caller-owned error enums can unify
source and mapper failures.

Each poll advances at most one subscription item and the initial future, in that
order. Both factories receive the same composed `CancellationToken<E>`. Reset,
dispose, supersession and unfinished-driver drop cancel both tokens and destroy
the actual retained inputs. Cancellation during a child poll is immediately
readable; destruction waits until that poll returns. Callbacks, mappers, polls,
user destruction and waker operations occur outside library mutexes.

Cached `Arc<Contextual<T>>` data supplies the watermark. Lower slots are ignored
before mapping; equal slots are accepted. Mapping succeeds before a new contextual
value can advance that watermark. Standalone `SlotTrackingStream` instead
advances before mapping and keeps errors nonterminal. The composed store ends
both children on the first source or mapper failure. `CancellationReason::Failure`
preserves the exact `Arc<ReactiveFailure<E>>` shared by the child tokens, outer
error snapshot and driver result. No E: Clone requirement or swallowed mapper
error is introduced. Rust panics propagate.

Reconnect retains the complete context/value and previous error while loading.
A lower-slot reply settles loading by re-emitting the retained Arc once, clears
the error, and leaves later lower replies quiet. This requires no T: Clone.
Reset clears the context and watermark. Source completion leaves a quiet pending
window until cancellation, matching the composed Dart controller lifetime.

Listener leases and weak registration/work edges use the existing Core ownership
model. `observe` and `observe_filtered` share the existing bounded observation
bridge, including coherent snapshot admission and caller-owned connection lifetime.

## Pinned source and held verification

Dart commit `0138bb956d2d81ccc0c56edc602401c0df2f2b46`:

| Source                                                            | SHA-256                                                            |
| ----------------------------------------------------------------- | ------------------------------------------------------------------ |
| `packages/solana_kit/lib/src/slot_tracking_reactive_store.dart`   | `b0d92630f2a34ff9cedd187d0a28139d0f2805138c67d9532d665ff0f273e002` |
| `packages/solana_kit/test/slot_tracking_reactive_store_test.dart` | `fc6fe114867c163dd5806b0ffee14b50ceaaa5b946c6a666ffb40a95fd13dfa5` |

Seven shared bodies are registered in source suite v52, increasing its source
count from 432 to 439. Three native controls cover idle inference, conditional
Send and reentrant wakeup. All ten new bodies are **UNCOMPILED and UNEXECUTED**
in this worker. Root owns independent source review, real dependency resolution,
compilation, native/browser/WASI/production wasmi runs and authentic golden
refresh. Existing runtime reports remain unchanged. Formatting, Monochange and
source-quality checks establish neither compilation nor behavior.
