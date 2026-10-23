# Account snapshots and shared diagnostics

The public client now exposes standalone account parsing, borrowed decoding,
existence assertions and four typed fetch helpers through `accounts`. A standalone
`BaseAccount` needs no address or payload. Its numeric strings have checked `u64`
bounds, and optional parsed metadata preserves exact opaque JSON. Node response
parsing retains its stricter metadata and numeric requirements.

Core `errors` owns stable numeric definitions, domains, bounded public context and
retained diagnostic causes. Client `diagnostics` converts strict RPC and execution
values through those same owners. Unknown integer codes and execution payloads
survive without truncation. Debug and Display redact payloads; public message
rendering is explicit and bounded.

The maintained suite appends 31 account and 17 error contract bodies to its exact
259-case prefix, including the accepted collection bodies. Native Rust, Chrome,
Wasmtime Preview 1 and Preview 2, and the pinned LeptosKit interpreter execute all
307 cases. Each observes the sole corrupt hash failure as 306 of 307.
The interpreter also traps a one-fuel allowance before
any commit. Separate ordinary consumers execute the same 48 bodies with Rust
1.99.1 and the 1.89 MSRV, outside repository Cargo configuration.

The original collection boolean-prefix control retains its invalid byte `2` and
exact rejection assertion. It now expects `InvalidBoolean { value: 2 }`, matching
the diagnostic API's precise error rather than the former discriminator category.
All 54 collection bodies and 281 ordered predicates remain present. The original
259 ordered report names and outcomes are unchanged. Suite identity is now `v32`.

The private interpreter harness explicitly enables Wasmi 2.0's `portable-dispatch`
backend. Its ordinary main thread passes with debug assertions and overflow checks.
The optimized automatic tail-call backend overflows the native stack under the
checked Rust 1.96 host profile, including with an older accepted guest. This
verification setting does not establish the default backend of generated apps.
Installed desktop, Android and iOS application journeys remain separate work.

The [account decisions](../parity/account-snapshot-contracts.tsv) retain 65
idiomatic and 19 bounded equivalents. The
[error decisions](../parity/shared-errors.tsv) retain all 438 bounded rows.
No canonical declaration status or implementation phase is promoted by this
runtime receipt. Pinned Dart fixtures and source notices remain intact.

Actual metadata-boundary restoration and Context depth-relaxation mutations fail
the original public assertions. Catalog generation freshly reproduces all 365
definitions, and receipt audits validate exact membership, source digests, license
bytes, and missing, duplicate and changed-digest controls.

This SDK is exclusively for off-chain application clients. On-chain programs
should use Pina or the official Solana program libraries.
