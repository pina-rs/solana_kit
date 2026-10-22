# Portable transaction introspection

The maintained `cargo xtask all` suite executes all 30 existing public
introspection contracts through three bounded groups. Native Rust, real Chrome,
Wasmtime Preview 1 and Preview 2, and the pinned actual LeptosKit `WasmHost` each
check the complete ordered 111-case report. The corrupt-hash control changes only
its original hash comparison and yields 110/111. The previous 108 report entries,
assertions, order, and source bodies remain unchanged.

`introspection-wire-metadata` preserves literal legacy/v0/v1 bytes, exact message
and signature slots, unresolved lifetimes, lookup descriptors, wire-only decoding
without execution metadata, and required metadata for complete account resolution.
`introspection-accounts-traces` checks raw, parsed and partial account roles,
borrowed opaque full-width JSON, original metadata errors, missing/null/numeric
depth, and interleaved outer/inner traces. `introspection-boundary-validation`
checks malformed headers, signatures, indices, counts, versions, trailing bytes,
lookup contradictions, top-level payer/static bounds, and positive writable and
readonly loaded inner programs.

The portable fixtures mechanically adapt the unchanged public test bodies.
Comparison predicates, literals, and operation order within every contract are
preserved. Assertions return report errors; impossible fixture-shape branches
return errors. Existing unwrap failures still fail execution. Only test wrappers,
visibility, imports and Result propagation differ. The existing `base64`, `bs58`
and `serde_json` packages gain direct edges solely in private conformance tooling;
production SDK dependencies and package versions remain unchanged.

The suite keeps 15,000,000 fuel per ordinary group and 5,000,000 per existing crypto
unit. These three groups perform no signing, signature verification or key
creation. Checked allowance is 6,765,000,000 per guest entry. One fuel still traps
before any report commit. This is bounded fixture accounting, without a measured
performance, minimum fuel, default timeout or production-host allowance claim.

All 42 declaration rows remain partial. Rust Eq/Hash/Debug replace Dart value
wrappers; exact Dart hash numbers and display strings remain outside the contract.
Structural wire validation does not authenticate signatures or establish validator
acceptance. Traces describe attempted execution; callers must inspect `meta.err`.
No network, validator, installed device or full package-parity result is implied.
This SDK serves off-chain applications. On-chain programs should use Pina or
Solana's official program libraries.
