# Parallel allocation verification

The error contracts measure allocations made by one synchronous Context insertion.
The observer belongs to that measuring thread. Other test threads can allocate or
measure their own insertions concurrently without changing the result.

A constant-initialized thread-local `Cell<Option<usize>>` represents an inactive
observer or its current count. A guard restores inactivity during normal return
and unwinding. Allocator callbacks use allocation-free `try_with` access, count
allocation and reallocation attempts, and preserve System's original ownership
and layout behavior. They tolerate thread-local teardown without panicking.
Nested measurements on one thread fail before changing the active observer.

The original process-wide counter fails the maintained cross-thread contamination
control. The corrected observer passes unrelated allocation exclusion, two
simultaneous measurements and panic cleanup. The original Context limits,
small-versus-large equality checks and all 17 original error bodies are unchanged.
Removing the production object-cardinality precheck still fails the allocation
limit. Relaxing depth by one still fails the original exact boundary assertion.

The maintained projector copies this observer unchanged into the portable
contracts. All 48 original account and error bodies retain their registry order.
The combined suite executes 307 canonical cases and observes exactly one corrupt
hash failure across native Rust, Chrome, both WASI previews and the pinned
LeptosKit interpreter. The plain guest retains only its three rendering imports.

Run the workspace under the test runner's default parallelism.

```sh
cargo test --workspace --all-features --locked
```

This is verification support for the off-chain application client SDK. On-chain
programs use Pina or official Solana program libraries.
