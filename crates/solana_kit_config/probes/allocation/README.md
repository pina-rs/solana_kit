# Config allocation regression probe

This private standalone consumer observes the public Config codec and Store
builders. It is exclusively off-chain verification code. It is not a dependency
of the library and is never published.

The source has not been compiled or executed in unit400. No allocation number,
speedup, parity acceptance or quality-gate success follows from this source.

Each invocation creates 4096 signer keys and a 2 MiB opaque tail before enabling
the allocator observer. The `write` case also creates its complete destination
before observation. Assertions, argument parsing and printing happen afterward.
The consumer starts no threads. Run one operation per process.

The observer sums requested allocation sizes and retains the largest request.
It observes `alloc`, `alloc_zeroed` and `realloc`, including failed requests.
A reallocation contributes its full requested replacement size. Deallocation
does not subtract anything. These metrics are neither live heap bytes, physical
memory, allocation-call counts, latency nor throughput. Compiler optimizations
can change observed requests, so the original-source control is mandatory.

Both metrics must remain at most 1024 bytes for each operation. This ceiling
permits constant runtime overhead while rejecting proportional input copies.
The operation result and wire properties are checked separately.

| Argument              | Public observation                                       |
| --------------------- | -------------------------------------------------------- |
| `length`              | Exact encoded size from already-owned input              |
| `write`               | Complete write into caller-owned storage                 |
| `encode-reject`       | Byte limit rejects before proportional temporary storage |
| `store-reject`        | Explicit Store rejects its byte limit                    |
| `derived-byte-reject` | Derived Store rejects its byte limit                     |
| `derived-item-reject` | Derived Store rejects the combined item budget           |

## Later executable control plan

Every command below is deferred until the root grants a frozen helper and its
exclusive resource admission. Do not launch ambient Cargo or toolchain proxies.

1. Freeze the positive source, this consumer, manifests and the existing core
   source. Run the benchmark-checklist skill before measuring. Bind the actual
   assigned Rust 1.99.1 and MSRV 1.89 compiler binaries, owned Cargo home,
   network-denied outer sandbox, package graph and distinct targets inside the
   executing helper. Resolve the standalone lockfile offline before freezing
   the measured builds. Do not install or repair a toolchain or registry.
2. Compile a sanity build first, then the positive standalone binary separately
   for each producer. Select its actual Cargo compiler-artifact target named
   `config_allocation_probe`. Bind child compiler identity, source hashes and
   actual linked Config/core artifact bytes. Preserve each executable before
   any target reuse. A parent toolchain label or Cargo Fresh line is insufficient.
3. Build an original-source control in a separately owned source copy and a
   target distinct from every positive target. Use the exact same consumer and
   core source. Restore only Config `src/lib.rs` from the preserved signed base
   `85be2849ff881e789bebdb59eb5edafcc9e48b2c` before freezing that control.
   Unit400 retains those original bytes in its primary ignored evidence store.
   Bind the source difference and actual Config/core extern bytes. Do not edit
   the original349 or active397 worktrees.
4. Execute all six arguments separately on each actual positive binary. Require
   success, exact public results and both requested-byte ceilings. Run each
   argument on the original binary with the same ceiling and input. Require
   that the public result check passes before its resource assertion fails.
   Retain the failing output and exit status. A renamed helper, source-only
   mutation or stale binary does not satisfy this control.
5. Repeat at least five times per argument and producer, alternating positive
   and original binaries. Record the host-load qualification, raw observations,
   median and range. Preserve failures. Report only requested-byte observations
   within the named interval. This probe establishes no end-to-end speedup.
6. Run all maintained Config tests on both compilers, including every unchanged
   literal test. Run the exact generator comparison to prove `generated.rs`
   remains unchanged. Execute the relevant native, browser, WASI Preview 1,
   WASI Preview 2 and pinned interpreter behavior paths through maintained
   registration. Confirm production-host and installed application limits
   separately. The standalone allocator probe itself is native only.
7. Run formatter admission and SDK Monostyle 0.3.26 on all supported shipping
   and helper files with width100, generated-ignore=false and no ignore bypass.
   Both readability and complexity floors are 90 per file. Run static analysis,
   Clippy, docs, graph/import gates and ordinary guarded readonly Monochange
   checks. None has run in unit400. Normal publication remains disabled.
