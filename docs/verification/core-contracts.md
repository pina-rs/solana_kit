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
205-case prefix. Native Rust, Chrome, Wasmtime Preview 1 and Preview 2, and the
pinned LeptosKit interpreter execute all 253 cases. Each observes the sole corrupt
hash failure as 252 of 253. The interpreter also traps a one-fuel allowance before
any commit. Separate ordinary consumers execute the same 48 bodies with Rust
1.99.1 and the 1.89 MSRV, outside repository Cargo configuration.

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
