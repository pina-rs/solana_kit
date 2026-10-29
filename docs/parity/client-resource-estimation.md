# Client resource estimation

The optional `solana_kit_client/resource-estimation` feature exposes `resources`.
`ResourceMessage` holds unsigned instructions, a payer, explicit lifetime metadata,
and version-specific compilation inputs. `VersionContext` binds lookup snapshots
to version zero and inline configuration to version one.

`estimate` compiles a separate unsigned transaction and makes one typed
`simulateTransaction` call through the caller's `Client<T>`. The adapter retains
its endpoint, credentials, response budget, deadline, and cancellation ownership.
Dropping an estimation or tuning future drops its active exchange. These helpers
never sign, submit, spawn a task, fetch a lifetime, or retry a request.

## Source behavior

The source is the retained Dart archive at revision `0138bb95`. The contracts are
`packages/solana_kit/lib/src/resource_limit_estimation.dart` and
`packages/solana_kit_transaction_messages/lib/src/resource_limit_estimation.dart`,
with their original package tests. This port covers standard Kit estimation.
Helius estimation and fee policy are separate contracts.

| Operation                       | Legacy and version zero             | Version one                                              |
| ------------------------------- | ----------------------------------- | -------------------------------------------------------- |
| Compute getter and setter       | First matching Compute Budget tag 2 | Inline compute configuration                             |
| Loaded bytes getter and setter  | First matching Compute Budget tag 4 | Inline loaded configuration                              |
| Set a missing instruction limit | Append after existing instructions  | Update configuration                                     |
| Remove an instruction limit     | Remove only the first match         | Clear only the named configuration field                 |
| Fill provisional limits         | Fill absent compute with zero       | Fill absent compute and loaded bytes with zero           |
| Simulate for estimation         | Set compute to 1,400,000            | Set compute to 1,400,000 and loaded bytes to 67,108,864  |
| Tune compute                    | Replace absent, zero, or 1,400,000  | Replace absent, zero, or 1,400,000                       |
| Tune loaded bytes               | Preserve existing instructions      | Replace absent or zero if the estimator supplies a value |

All applicable explicit limits bypass the estimator. Equal instruction values
retain existing account metadata and trailing bytes. Other tags, later duplicate
limits, instruction order, priority fees, and heap requests survive resource edits.
A nonce advance remains instruction zero. Compilation validates the caller's nonce
identities. Blockhash lifetimes request RPC replacement, while nonce lifetimes keep
the exact token and disable replacement.

The estimator checks compute usage first. Absent or null compute usage produces
`MissingComputeUsage`. Version one then requires loaded usage. Only after both
checks does a transaction failure produce `ResourceError::Transaction` with its
typed original cause. Compute usage caps at `u32::MAX`. Applying an estimate above
1,400,000 fails with `ComputeUnitLimitOutOfRange`, matching the Dart compute setter.
No safety margin, multiplier, priority fee, or additional default is inferred.

## Rust boundary differences

The existing typed `SimulateTransaction` method admits responses before resource
validation. Its required nullable metadata fields and replacement-blockhash
consistency rules also apply here. The Dart estimator accepts looser response
maps. Malformed or incomplete typed RPC responses therefore retain a
`ResourceError::Rpc` cause before resource-specific checks run.

Rust preserves typed compilation, instruction encoding, and RPC failures in
separate `ResourceError` variants. Dart wraps these causes in a generic compute
estimation error. `TuningError<E>` retains the exact caller estimator error type.
Unsigned `u32` inputs exclude negative and noninteger resource values.

## Verification status

Six original resource contract groups are registered in the shared conformance
suite as `solana-kit-core-v48`, with 406 total groups. The maintained native,
browser, both WASI previews, and production wasmi runners all call this suite.
The new bodies cover actual RPC wire for all versions, lookup snapshots, both
lifetimes, maximum simulation budgets, source ordering, provisional and explicit
limits, missing and null usage, metric precedence, u64 capping, typed failures,
and cancellation of a suspended adapter exchange. Native and WASI builds also
assert that concrete Send transports retain Send estimation and tuning futures.

At source handoff, the new test bodies are **UNEXECUTED**. Runtime verification and
Cargo resolution belong to the coordinating gate. Formatting and the direct
pinned style analyzer are source checks and do not prove runtime correctness.
