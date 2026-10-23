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

The public aggregate source is `f9be1648741e497eda98cd2331d71834d82b0ac0`,
based on `77dbbd3cfaf94745a1a1666bc65eeff230275a8c`. Its Anchor files and complete
body adapters match the independently accepted source. Current core, client,
planning and completed-packer assertions retain their exact base bytes. The
following SHA-256 digests identify retained aggregate runtime artifacts.

| Artifact              | SHA-256                                                            |
| --------------------- | ------------------------------------------------------------------ |
| Native executable     | `975232ce3e58ed9e822b1a3d0c959ae484d62e462bd296735be42bde351090cb` |
| Browser input Wasm    | `4792cdc6a1cd96a6317f7d5c041cb006e06a3ee1280f570d1cd5f0d484779424` |
| Browser executed Wasm | `e4e6e52307bd0971d6312688f02c9d4011bc5e8812d30ef52fccda0b7252719e` |
| WASI Preview 1        | `bfe9d3f29b63554edb622d7bbd2712965051441d19a8c18eb943c7eed6f49d9b` |
| WASI Preview 2        | `1cfe752c1d01881d1306918d2b0e58c04c0849704c541f6911e1ec4ffc66d50f` |
| Plain guest           | `1da198fc75f1f538e7f39849ce51c89df4ac0af7047834019fe0e82572e458da` |
| LeptosKit host        | `169f56b889dbfd21f48965f1c0cec7d8a6a7df27f3bc310e78948098daa42870` |

The browser runner transforms the input Wasm before execution. A second real
Chrome run retains that transformed Wasm and its generated JavaScript. The
native, WASI and interpreter logs retain both complete ordered reports. Every
host observes the sole SHA corruption; insufficient fuel prevents a guest commit.

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
