# Anchor portable runtime evidence

The bounded Anchor contracts execute in native Rust, Chrome, both WASI previews,
and the pinned LeptosKit `WasmHost`. This is an off-chain application SDK.
Solana on-chain programs use Pina or official Solana program libraries.

## Source and complete bodies

The [source receipt](../parity/anchor.md) retains the pinned Dart package's five
families and all 120 declaration IDs. The source contains no provider, program
builder, asynchronous network operation, or PDA resolver. This receipt does not
add those APIs or complete an ecosystem phase.

The [portable body map](../../verification/conformance/src/anchor_cases/runtime-map.tsv)
accounts for 43 complete bodies and 169 ordered assertion sites. It includes all
35 original public Rust tests, three distinct outside-consumer bodies, and five
independent Unicode, floating-point, recursive-schema, and collection-depth
controls. The remaining 11 outside bodies register the same complete wire tests.
There are no native-only Anchor exclusions. Adaptation replaces host assertions
with reportable requirements and imports `alloc` and `core`; original fixture
bytes, ordering, predicates, and expected errors remain intact.

The suite preserves the prior 162 cases and the planning slice's 43 portable
bodies, 184 predicates, and five separate native-only bodies. Anchor appends 43
cases. The completed-packer planning control remains intact. Suite
`solana-kit-core-v30` therefore has 205 cases. Each named host passes
205 cases and rejects exactly the SHA-256 case in the 204-case corruption control.
The LeptosKit host also rejects insufficient fuel before committing a result.
These counts describe this bounded suite, not the 120 source declarations or
full source parity.

## Independent defect control

Independent review found that encoding an empty vector or zero-length array
could bypass collection-depth admission. Its own decoder rejected the encoded
value under the same policy. The correction checks the collection depth before
encoding, matching the decoder. A distinct compiled producer with that guard
removed passes three new controls and fails the two intended empty-collection
controls. The corrected producer passes all five, including exact-depth,
nonempty, Unicode, signed-zero, infinity, and finite-recursion checks.

The original Dart tests and current/MSRV public tests remain separate evidence.
They retain 62 Dart bodies, 35 native bodies and one runnable doctest per Rust
compiler, and 14 outside-consumer bodies per compiler. Rust intentionally uses
explicit enum/option values, bounded schema traversal, strict malformed numeric
admission, and explicit prefix-consumption results. The source receipt records
these representation and validation differences.

## Execution and boundaries

The native producer is Rust 1.99.1. Chrome is 155.0.8059.85. WASI Preview 1 and
Preview 2 execute in Wasmtime 50.0.0. The actual interpreter is LeptosKit revision
`fecf8628d326ff7b03f295523e6bb584c3fdf88d`. The host and guest use the same core
suite. Real compiler commands, actual test bodies, executable bytes, Wasm bytes,
source identities, and the guard mutation are retained in ignored verification
evidence. Current and Rust 1.89.0 producers are identified separately.

The isolated Anchor consumer links on all three Wasm targets. Its normal and
build dependency graph contains exactly 17 allowed packages. Actual root imports
are checked separately from component-internal wiring. Injected Tokio and
forbidden core/component imports are rejected. These checks establish dependency
and capability boundaries; they do not grant a guest HTTP, entropy, wallet, or
platform capability.

All authored SDK files and active verification helpers retain readability and
complexity scores of at least 90 with unchanged Monostyle 0.3.26 rules. No ignored
body, skipped host, compile-only result, or aggregate score substitutes for the
reported runtime and per-file checks.

## Remaining work

The declaration ledger retains bounded source-parity limits. Validator tests,
installed desktop/Android/iOS applications, live wallet journeys, and provider
integrations remain separate requirements. Independent acceptance and public
delivery belong to the coordinator. The aggregate proof retains the accepted
completed-packer correction and all 43 Anchor bodies.
