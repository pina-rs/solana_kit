# Portable Token-2022 contracts

This private verification crate executes all 116 original public client test
bodies through the shared native, browser, WASI and LeptosKit conformance suite.
It preserves the original 274 ordered assertion macros and their literal inputs.
Three additional groups exercise the maximum variable account tail, canonical
mint owner/state/allocation boundaries, and multisig threshold failures.

The six adapted modules retain their original predicates and helpers. Their only
code changes remove test-only attributes and expose the body functions to the
registry. Formatting can differ. The registry preserves original declaration
order, case names and duplicate account identities. No original body requires a
native-only substitute.

These bodies retain ordinary Rust assertion failures. A failing contract can
trap a Wasm run before later bodies execute. Successful complete reports require
every body to return. The separate SHA corruption control remains a reportable
failure and must leave every Token-2022 body successful.

This crate is exclusively an off-chain client verification harness. It does not
establish validator transitions or installed desktop, Android or iOS journeys.
On-chain programs should use Pina or official Solana program libraries.
