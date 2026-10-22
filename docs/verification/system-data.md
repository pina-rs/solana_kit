# System instruction data generation

This unit provides off-chain application data codecs for all fourteen System
instructions in the pinned IDL. On-chain authors must use Pina or official Solana
program libraries. Account builders, nonce accounts, PDAs, errors, instruction
identification/parsing and validator execution remain separate work.

The generated `solana_kit_system` crate is `no_std + alloc` and depends only on
`solana_kit_core`, whose default graph contains `bs58`. It has no official Solana,
serde, bincode, Tokio, JavaScript or random dependency. The private generator's
native test graph pins `solana-system-interface =3.3.0`, `solana-address =2.7.0`
and `bincode =1.3.3` as independent serialization oracles. They never enter an SDK
normal/build graph. The exact registry checksums are in its private lockfile.

## Design choice

Two shapes were considered: a single generated enum with one codec, or individual
data structs and codecs with an enum dispatcher. The latter lets callers express
a concrete instruction's fields without matching a large enum while preserving
strict runtime tag selection when decoding unknown instruction data. Both paths
use the same generated per-type codecs. A single validated IR supplies names,
tags and fields to both the old private proof and public emitter; no schema
parser or per-instruction handwritten wire algorithm is duplicated.

The parser supports u64 little-endian numbers, public keys and u64 little-endian
UTF-8 byte-length framing. It validates actual nested node kinds, widths,
endianness, numeric discriminator/default ranges, unique names/types/tags and
selected default strategies. Unknown wire properties, including range
constraints, are rejected. Presentation metadata does not alter wire encoding.
The emitter accepts this concrete fourteen-tag System set; this is not generic
Codama, linked-type, enum, account or renderer parity.

Each codec implements the core `Encoder` and `Decoder` contracts. Encoding checks
exact sizes and caller byte limits before allocation; decoding shares explicit
byte/item budgets and rejects malformed, truncated, trailing or unknown-tag data.
Tags are fixed by their types. IDL optional defaults remain explicit caller fields,
so mutable Dart discriminator and default-constructor behavior is only partially
mapped. No float amounts or implicit SOL conversion exists. Structural string
encoding does not claim seed length or account permission validation.

Validation and in-memory rustfmt finish before opening a create-new pending file;
replacement uses a single rename. Invalid schema and pending collisions preserve
existing destination/pending sentinels. This proves the tested single-writer,
single-file lifecycle, not concurrent writers or multi-file crash durability.

## Sources and checks

System repository revision `e9cb05138ea802b01f1ddf79ea140dcd765c9db6`, IDL
SHA-256 `bb1a83194457a7fc735f9e5a806ecaced94a17276fc7b45f356171810f15b63a`.
The unmodified input and Apache-2.0 notice remain in the private fixtures; the
notice also ships in the public crate. Dart source is pinned at
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`.

The private tests serialize all fourteen official enum variants, including
`CreateAccountAllowPrefund`, with exact integer maxima and four string fixtures.
They check encoded length, bytes, official decode, strict local decode/re-encode,
every truncated prefix, trailing bytes, limits, invalid UTF-8 and unsupported tags.
The original literal52/94-byte independent proofs and discriminator-kind
regressions remain. Additional tests mutate every instruction discriminator kind,
selected layouts/defaults, names, duplicates, unsupported shapes and ranges
through actual atomic replacement. CI regenerates public output twice and rejects
drift; it also runs the private tests and all three Wasm compile targets.

Three shared functions in `verification/conformance/src/system.rs` execute the
literal52/94 proofs and malformed-input rejection in the combined v6 suite.
The [runtime receipt](runtimes.md) records actual native, Chrome, WASI Preview1/2
and pinned LeptosKit execution, including oversized u64 string-prefix and
truncated-string rejection in 32-bit guests. Four separate default normal/build
target queries require exactly System, core and base58; core policies stay separate.
MSRV1.89, warning-denied documentation, Clippy, formatting,
Monochange preview and generated-inclusive per-file monostyle floors are checked.

`docs/parity/system-data.tsv` preserves every canonical System declaration ID and
its bounded disposition; no global capability entry is marked complete. Regenerate
source and the scoped receipt from the repository root:

```sh
cargo run --locked --manifest-path verification/codegen-design/Cargo.toml -- system
cargo run --locked --manifest-path verification/codegen-design/Cargo.toml -- system-receipt
cargo test --locked --manifest-path verification/codegen-design/Cargo.toml
```

Publication is disabled. The changeset records only the implemented data slice.
The source consumer uses no Cargo patches. Archive assembly with an unpublished
core dependency does not prove clean-registry readiness; dependency publication
order and a clean packaged System consumer remain release work.
