# Compute Budget client and second-program generator

This crate is exclusively an off-chain application client. Do not use it in
Solana on-chain programs. Use Pina or official Solana program libraries there.
`no_std + alloc` supports portable clients.

## Implemented behavior

All five pinned instructions have concrete data types, bounded Encoder/Decoder
implementations, Kit-owned instruction builders, canonical parsers and tagged
sum-type dispatch. Every builder uses the schema-selected trusted program address
and an empty account list. Parsers reject foreign programs, accounts, mismatched
or unknown tags, truncated/trailing data and exceeded byte budgets. Values stay
u32/u64 throughout. Heap alignment and current validator resource policy are not
wire constraints imposed by these codecs.

RequestUnits tag0 preserves the historical source layout, including its two u32
fields. Current solana-compute-budget-interface3.0.0 reserves that enum position
as Unused. Modern validators reject this historical instruction. Construction
and parsing support historical bytes, not a promise of node acceptance.

Instruction-list helpers preserve first-match updates, append ordering, later
duplicates, full unsigned prices and callbacks receiving None when absent.
Setting a price appends even when prices already exist. Filling a provisory limit
preserves an existing limit and inserts zero only when absent. As in the pinned
Dart source, both discovery helpers ignore trailing bytes and account metadata.
Price discovery decodes the nine-byte prefix and retains all u64 bits. Unknown
Compute Budget tags reject when they reach the lookup's minimum payload length,
even before a later valid match. Foreign programs and shorter payloads skip.
Each scan stops at its first selected match, so later malformed data cannot
invalidate that result. Updates propagate errors before calling the updater.
`find_compute_unit_limit` now returns `Result<Option<ComputeUnitLimitDetails>, Error>`.
Callers must handle this unreleased API correction. These are
pre-compilation slice operations. They do not modify a compiled signed message.

[Exact declaration accounting](../parity/compute-budget.tsv) retains all84
canonical IDs once. It records61 implemented and23 partial. Fixed tags replace
mutable Dart discriminator/default constructors. A standalone source instruction
kind enum and full transaction-message wrappers remain unfinished. Global program
and phase ledgers are not promoted to complete.

## Generator design

Compared extending the System-specific emitter with a separate template engine.
Chose a compact separate typed IR sharing the actual existing identifier guard
and in-memory formatter/staged replacement implementation. This leaves the
reviewed fixed System schema, its semantic bindings and program identity check
byte-identical. No second file replacement algorithm or runtime template engine
was introduced.

The private generic module models a selected Program with trusted address policy,
concrete Instructions, Fields, u8/u32/u64 little endian numbers and named linked
Definitions. Supported definitions are finite structs and explicit contiguous
u8 enums with unit variants. A generated real consumer executes a struct linked
to an enum, byte-compares the nested encoding and rejects unknown enum tags.
Every link resolves before emission. Duplicate generated/imported/codec names,
unresolved links, cycles and more than64 definitions reject. Unknown selected
semantic properties and unsupported nonempty accounts/PDAs/errors/additional
programs fail before any output write. Display/docs metadata is not wire input.
The typed IR is private and is not a general Codama renderer contract.

An explicit fixture varies both valid program name and public key together with
its trusted policy. Rendered aggregate names and address bytes follow that
validated input. Foreign identity under the original policy rejects. Names
cannot supply paths or source syntax. All code is rendered and formatted before
opening the create-new pending file. Invalid schemas preserve absent/existing
output and pending sentinels in180 executed combinations. Valid pending-file
collision also preserves both sentinels. Two valid generations match committed
bytes. Existing single-file filesystem limits remain. Multi-file atomicity,
crash durability and concurrent writers are not established.

The55 canonical renderer IDs remain pending in the global ledger. This slice
proves a second concrete program plus linked finite types. Dynamic layouts,
account/PDA/error generation, tuple/data-bearing enums, arbitrary discriminators,
normalization and the remaining program clients are still outstanding.

## Source provenance and executed checks

Compute Budget repository revision is e189b8ca25ecc283354b96f12cdcd6d7f4246921.
The unmodified idl.json SHA256 is
1a2e18158513353622cb6e39c075dc06731af76975af33df1468966d24e1ca95.
The upstream Apache-2.0 notice is included in the crate. Authored code is Unlicense.
Dart reference pin is0138bb956d2d81ccc0c56edc602401c0df2f2b46, with its MIT notice
retained beside the golden fixtures.

Actual Dart3.13.3 executed the pinned original libraries using an isolated local
package configuration without pub resolution, source changes or networking.
It produced21 complete instruction fixtures covering all five tags, zero,
u32::MAX and u64::MAX, plus a multi-step duplicate/update/append ordering vector.
The committed fixture SHA256 is
66cfe1e05c716839b9fe3a9e4f34a5de881b6673f6eebf839e16b10744731fc1.
An independent private native oracle executes16 current official constructor
comparisons, checking bytes, program identity and empty metas. Official interface
3.0.0 registry checksum is
8292c436b269ad23cecc8b24f7da3ab07ca111661e25e00ce0e1d22771951ab9.
Official dependencies exist only in the private native dev graph.

All26 private generator tests and6 public client tests pass. Public malformed
controls exercise every truncated prefix, tag mismatch, foreign program,
nonempty metas, trailing input and byte budgets. Two portable functions follow
the first75 cases in the shared `solana-kit-core-v17` suite. Canonical execution
requires77 passing cases. The corrupt-hash control requires76 passing cases and
exactly the original hash failure. Native Rust, Chrome, Wasmtime50.0.0
Preview1 and Preview2, and the actual pinned LeptosKit WasmHost execute these
outcomes. The client and conformance crate compile on
wasm32-unknown-unknown, wasm32-wasip1 and wasm32-wasip2.

Rust1.89 executes the public tests and a clean external no_std source consumer
without patches or repository Rustflags. Its full-width price parsing and
first-only duplicate update pass. Each target's normal/build graph is exactly
Compute Budget -> Kit core -> bs58 0.5.1. No SDK, host, executor, JS, entropy or
serialization dependency enters that graph.

Workspace all-feature tests, warning-denied Clippy/Rustdoc, private generator
Clippy, root/private rustfmt, dprint, actionlint, locked LeptosKit metadata,
package listing and Monochange check/validation/preview pass. Publishing remains
disabled and only the existing umbrella0.0.0 reservation remains published.
The preview owns the program independently. Archive listing includes its licenses;
this is not a clean registry/archive install while core is unpublished.

Pinned monostyle0.3.26 scores generated source and every Git-inventoried authored
Rust file. Every file must reach90 in readability and complexity. Rules,
coverage and thresholds are unchanged.

The correction executes6,656 original Dart helper cases and the matching Rust
matrix over all u8 tags, lengths0 through12, both program identities and both
positions relative to a valid match. Actual Dart accepts trailing price bytes.
The earlier receipt's price-rejection claim was incorrect. Canonical instruction
parsers and codecs still reject whole-input trailing bytes. Accepted finite
generator boundaries now execute all256 unit enum variants, all256 instruction
tags, sparse tags0/93/255 and a zero-field instruction under warning-denied
Rust1.89 compilation. Complete u8 domains omit unreachable wildcards. Sparse
domains retain invalid-tag errors, and generated imports follow the emitted
field codecs. The private generated consumers use ordinary Cargo with isolated
targets, so Cargo selects compiler-compatible dependencies.

Regenerate and verify from the repository root:

```sh
cargo run --locked --manifest-path verification/codegen-design/Cargo.toml -- compute-budget
cargo run --locked --manifest-path verification/codegen-design/Cargo.toml -- compute-budget-receipt
cargo test --locked --manifest-path verification/codegen-design/Cargo.toml
cargo test --locked -p solana_kit_compute_budget
cargo test --locked -p solana_kit_conformance --test compute_budget
```

No validator, RPC, wallet, real generated app, mobile device or registry release
journey is established by this slice. Coordinator-owned independent review
and remote CI remain separate gates.

The linked enum fixture is the exact selected accountState defined-type JSON node
from the pinned Token IDL at31646dce3df0a03d468a14913f01b9fd3c3adad0.
That full IDL SHA256 is33add106ed36fead9222bfd882f1b364f3c4714346becefd5fa6068652b0cc33.
Its u8 enum is linked through a synthetic finite settings struct and used by a
real generated instruction codec consumer. This does not advertise a Token
client or support for its other layouts. The source notice matches the retained
Apache-2.0 notice.
