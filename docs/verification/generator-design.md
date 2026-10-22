# Reproducible program-input and generator experiment

Status: private research over two System instruction encoders. This does not
establish full Codama renderer support, program-client parity, decoding support,
validator execution, or runtime portability of generated clients.
All SDK runtime work targets off-chain application clients; `no_std + alloc`
supports portable clients and does not promise Solana on-chain deployment.

## Reproduce

Run from the repository root with its pinned Rust toolchain:

```sh
cargo run --locked --manifest-path verification/codegen-design/Cargo.toml -- inputs
cargo run --locked --manifest-path verification/codegen-design/Cargo.toml -- generate
cargo run --locked --manifest-path verification/codegen-design/Cargo.toml -- proof
cargo run --locked --manifest-path verification/codegen-design/Cargo.toml -- renderer
cargo test --locked --manifest-path verification/codegen-design/Cargo.toml
cargo clippy --locked --manifest-path verification/codegen-design/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path verification/codegen-design/Cargo.toml --check
monostyle check . --include-generated --fail-under 90
```

The first command downloads immutable GitHub archives into the dedicated ignored
`verification/codegen-design/cache/`. It records the bytes actually fetched,
including archive, IDL/source, and license checksums. Subsequent invocations reuse
those archives. No secret, validator, Node installation, or SDK runtime is needed.
The private crate has its own workspace and lockfile and is never published.

The ledger has 18 rows for 17 clients because Loader v3 and v4 have separate
upstream revisions. Counts are raw declared schema nodes, not a parity metric.
`unmodeled` means the source does not explicitly declare that category; it does
not mean zero capabilities. Dart file counts cover `lib/src`, classifying paths
containing `generated` separately. Loader's counts describe the combined Dart
package in both rows and must not be summed. ATA has no pinned IDL: its three
instruction variants are counted in the hashed handwritten Rust interface.

## Authoritative inputs

The authority is [Dart commit 0138bb9](https://github.com/openbudgetfun/solana_kit/tree/0138bb956d2d81ccc0c56edc602401c0df2f2b46),
particularly `config/reference-repos.json`, `config/programs/artifacts.json`,
`scripts/generate_program_packages.mjs`, and actual package sources. Copied JSON
fixtures retain the upstream bytes; `fixtures/DART-LICENSE` retains its notice.
The archive SHA-256 is
`b229acdc682bf9ea4b00e6f215551d7e72d5d1aa85a98ec0ae42ebd71cd3259d`.

[The input ledger](../parity/program-inputs.tsv) records every repository,
immutable revision, input path, checksum, license source, raw schema version,
account/instruction/PDA/error count, and generated/handwritten split. Corrected
paths include Config's `program/idl.json`, Subscriptions' `idl/subscriptions.json`,
and Account Compression's `account-compression/sdk/idl/spl_account_compression.json`.
The Subscriptions client is the recurring-payments program.

The generation script names 12 repository inputs, not all 17 clients. Therefore
its presence does not prove reproducible regeneration for Bubblegum, Account
Compression, Subscriptions, Config, ATA, or Loader v4. Existing generated files
and their banners do not change this fact. The artifact configuration lists
validator program builds; its version labels are not IDL pins. The ledger's raw
schema versions intentionally expose disagreements, such as Account Compression's
IDL 0.2.0 against artifact 0.3.3, and Bubblegum's IDL 0.12.0 at its selected tag.

Licenses are retained beside cached source archives. Attestation has no root
license notice among the inspected conventional paths; its Rust client manifest
declares MIT, but notice/distribution clearance remains unresolved. Metaplex
licenses must be reviewed before redistributing generated sources. The private
fixture notices do not grant additional rights.

## Executed narrow comparison

The [official Codama Rust renderer](https://github.com/codama-idl/renderers-rust/tree/26f0136703d77ea64c77124d055115256a850954)
is pinned at `26f0136703d77ea64c77124d055115256a850954`, package 3.1.3.
Its archive SHA-256 is
`08d2744525b6f57db6f1517b82764233eaf5fb901a22743d3f943a9635aab618`;
`fixtures/RENDERER-LICENSE` preserves the MIT notice. The renderer itself is
TypeScript. Its output uses Borsh and official Solana types. Its `dependencyMap`
option changes import paths, while templates also embed constructors and fields.

The `renderer` command extracts actual CreateAccount constructor expressions from
that revision's generated example and compiles them against Kit types with their
crate path remapped. Compilation is expected to reject `AccountMeta::new` and
`Instruction.program_id`. It checks those specific compiler diagnostics and saves
both probe and diagnostics in `cache/renderer/`. Thus remapping alone is
insufficient. This is a constructor-compatibility experiment, not execution of
Node visitors or a completed fork/adaptation of the renderer. Borsh traits,
account decoding, PDAs, CPI, dependency selection and all templates remain unevaluated.

The focused Rust prototype consumes the **unmodified pinned System IDL** in
`fixtures/system.json`, SHA-256
`bb1a83194457a7fc735f9e5a806ecaced94a17276fc7b45f356171810f15b63a`.
It emits only CreateAccount and CreateAccountWithSeed data structs/encoders.
The explicit builder probe creates an actual Kit-owned Instruction with ordered
WritableSigner accounts. The second encoder proves a non-ASCII UTF-8 variable
field: the pinned System wire uses a **u64 little-endian byte-length prefix**,
not the usual Borsh u32 prefix. Literal 52-byte and 94-byte vectors were checked
against the pinned System Rust interface's enum order and bincode field layout;
they are separate from the emitter and are not computed by the implementation.
They are hand-audited wire oracles, not an executed Dart or validator oracle.

The generator resolves supported types and validates all selected fields before
formatting in memory. Only then does it create a sibling pending file, flush it,
and rename it over the destination. Invalid schemas preserve existing output.
This single-file replacement is atomic on the tested local filesystem; it is not
a multi-file crash-durability protocol. A failed write can leave a pending file,
which is reported by the next create-new operation instead of silently reused.
One writer per output is required.

Recommendation: continue the focused Rust route for the next small vertical unit,
reusing pinned schemas and normalization facts. The executed comparison establishes
that path overrides cannot adapt the official constructor contract, while the
native emitter produces correct bytes with only Kit as a runtime dependency.
It does not prove that a larger renderer fork costs more, or justify claiming
all17 clients are ready. Revisit reuse after testing linked types, enum layouts,
account decoding and a complete normalized Metaplex instruction.

## Core contracts requested

Keep `Address::as_bytes()`, Kit-owned Instruction and ordered AccountMeta roles.
The coordinator's shared codec unit owns `Encoder`/`Decoder`, little-endian
`NumberLE`, `FixedBytes`, UTF-8, `SizePrefix` with u32 and `SizePrefix64` with u64,
and strict `OptionCodec`. Generation needs checked lengths before allocation and
arithmetic before advancing cursors, one-byte option tags rejecting values above1,
explicit discriminator checking, and a documented trailing-data policy. These are
coordination contracts, not APIs implemented by this experiment. The prototype's
byte appends are private research and must be replaced by the agreed core codecs
before client generation. No duplicate public API was added.

## Later generator acceptance map

| Source behavior                           | Required test before full clients                                                                                                                        |
| ----------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Stake `prepareStakeRoot`                  | Pin canonical program name, flatten parameter links, synthesize only missing epoch/timestamp aliases, avoid duplicate account/type output                |
| Token2022 preparation and handwritten TLV | Prove canonical name, stop at Uninitialized, preserve unused extension bytes and two-byte multisig padding                                               |
| MPL Core `prepareMplCoreRoot`             | Rewrite exactly one malformed relationships `crate` link to RelationshipEntry; reject changed match counts                                               |
| Token Metadata preparation                | Rename exactly two updateAuthority option arguments without changing account names or wire order                                                         |
| Squads preparation                        | Rename only MultisigSetConfigAuthorityArgs.configAuthority                                                                                               |
| Attestation preparation                   | Add one u8 account discriminator to each of three accounts; verify actual discriminator values; retype exactly two layout arrays and all26 enum tags     |
| Link/type resolution                      | Reject unresolved links, ambiguous imports, cycles without finite wire layout, duplicate normalized names and category-barrel collisions                 |
| Discriminators                            | Check widths, endian, offsets, literal value range, known tags, and malformed/truncated input                                                            |
| Identifier and path security              | Reject traversal, separators, non-ASCII confusables, reserved words, injected literals and normalized output collisions before replacement               |
| Output lifecycle                          | Render and format in staging; preserve prior tree on any validation/formatter failure; compare independent deterministic output; test interrupted writes |

Current controls execute traversal rejection, duplicate instruction/field names,
unsupported selected types, invalid discriminator values, deterministic source,
and unchanged output on invalid input. The rest of this table is required future
work. The Dart renderer at `packages/codama-renderers-dart/` has
`test/security.test.ts`, `test/render-collisions.test.ts`,
`test/type-manifest-security.test.ts`, `test/instruction-signer-security.test.ts`
and normalization script are source references, not tests executed in this unit.
