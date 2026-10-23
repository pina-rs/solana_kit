# Portable Reader transaction controls

This private workspace executes five complete accepted control bodies against
the public `Reader` API. Three come from the core codec transaction tests. Two
come from the independent data-layout controls. The function bodies preserve all
original literals and ordered assertions. `contracts::run` calls every body.

The native and WASI command prints `reader-transactions: 5/5` only after every
body returns. The browser test invokes the same function through the real
wasm-bindgen test harness. The plain guest publishes its report through the
actual pinned LeptosKit rendering ABI after those bodies return. The separate
host validates that report and requires one-fuel exhaustion before any commit.
Any assertion failure aborts the run and cannot produce a passing report.

The controls cover failed cursor and nested allocation rollback, successful
commit, nested transactions, external closure effects and ordinary consuming
frames. This workspace owns no shared conformance registry or golden report.
Its native host adapter uses LeptosKit commit
`fecf8628d326ff7b03f295523e6bb584c3fdf88d`. Browser dependencies are test-only.
The Reader library depends only on core without default features.

These are headless off-chain client checks. They do not establish validator,
generated desktop, Android or iOS application support. Use Pina or official
Solana program libraries for on-chain programs.
