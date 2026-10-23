# Token-2022 public runtime contracts

This private workspace executes all 116 original client test bodies and three
independent boundary groups against the actual public Token-2022 crate. It has
no unrelated conformance prefix. The maintained shared suite remains unchanged.
The original 274 assertions and immutable Dart literal expectations remain in
the contract crate. Ordinary assertions trap on failure. A complete report
requires every body to return.

The native command, browser test, both WASI commands and actual pinned LeptosKit
guest execute the same body registry. The interpreter rejects insufficient fuel
before committing. These deterministic client checks do not establish validator
program invariants or installed application support.

This is exclusively off-chain application code. Use Pina or official Solana
program libraries for on-chain programs. This workspace is never published.
