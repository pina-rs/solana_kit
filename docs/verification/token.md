# Generated Token client

This bounded client covers the pinned Token program's 28 instruction layouts,
Mint (82 bytes), Token (165 bytes), Multisig (355 bytes), two linked unit enums,
and 20 program errors. It is exclusively for off-chain application clients.
Use Pina or official Solana program libraries for on-chain programs.

The shared private generator normalizes anonymous records, validates the complete
selected input, resolves acyclic links and checked layouts, then emits concrete
Kit-owned types and codecs through the existing single-file atomic writer.
There is no Token wire emitter, runtime schema interpreter, official SDK runtime
dependency or generated-source lint exclusion. System and Compute Budget output
remain unchanged. The original full 256-tag and atomic rejection regressions remain
required tests.

## Source and reproduction

The original Token revision is `31646dce3df0a03d468a14913f01b9fd3c3adad0`.
The exact original input SHA256 is
`33add106ed36fead9222bfd882f1b364f3c4714346becefd5fa6068652b0cc33`;
the source archive is
`e69baa42895defdaadff4b9535c93d7a1bf773671e3421b1302ea02572e75f31`.
The immutable original fixture is retained with its Apache-2.0 notice. Dart
revision `0138bb956d2d81ccc0c56edc602401c0df2f2b46` and its MIT notice supply
independent constructor/account fixtures. Both notices accompany the client.

The pinned original root contains an additional Associated Token owner. Input
selection checks the full original hash and explicitly excludes that exact owner;
its separately accepted client remains the owner of ATA behavior. No additional
program is silently accepted by the generic parser.

The frozen normalized input is `fixtures/token-normalized.json`, SHA256
`c3c0beb64fd0e2b0e9c848319d7a5eb8b8a8ebf3fde723448059f1485e7c0efc`.
The shared normalizer hoists `BatchDataItem` from original JSON pointer
`/program/instructions/27/arguments/1/type/item`; all field order remains intact.
`token-normalize` refreshes this reviewed input from the exact original pin.
Ordinary `token` generation rejects snapshot drift before publication.

```sh
cargo run --manifest-path verification/codegen-design/Cargo.toml --offline --locked -- token
cargo test --manifest-path verification/codegen-design/Cargo.toml --offline --locked
cargo test -p solana_kit_token --offline --locked
```

The source-only `token_fixtures` tool writes Dart constructor calls and ordinary
Rust consumers. Expected bytes come from executing the actual original Dart
archive sources with an existing local package configuration; no pub resolution
is needed. The committed TSV contains 93 literal rows: three value cases for all
28 constructors and all three accounts. A separate Dart table retains all 20
error messages and three unknown-code controls.

## Observable contracts

The caller matrix compares every encoded byte, program identity, address and
ordered account role. Values include zero, nonzero, full-width integers, absent
and present options, Unicode text, fixed signer arrays, and batch records with
empty and maximum u8 byte-length payloads. Ordinary external Rust 1.89 consumers
execute these cases with warnings denied and without repository compiler flags.

Public codecs reject malformed booleans, options, enums, discriminators, account
lengths, UTF8, incomplete frames and byte/allocation budgets. Every fixed account
truncation is rejected. Instruction prefix tests distinguish incomplete values
from complete shorter raw-text or remainder-batch frames. Strict invocation
parsers also validate the expected identity, ordered privileges and remaining
roles. Duplicate accounts retain their positions.

Account COptions use explicit four-byte tags and fixed absent payload storage;
instruction options use variable one-byte tags. Nonzero ignored absent payload
bytes decode as absent and re-encode as zero padding. Account sizes derive from
checked wire layouts, never host ABI or account-size guesses. Fixed arrays use
owned vectors whose codecs require exactly the declared count.

Instruction builders accept explicit ordered metas. Caller-selected authorities
can remain nonsigners while ordered remaining signers apply. Initialization
remaining addresses stay nonsigners. `SyncNative` permits trailing rent omission.
Custom program identities are supported by the explicitly named builder/parser
methods. Defaults are caller-supplied addresses. No amount, authority or signer
count restrictions are invented. Encoders enforce byte limits; decoding shares
allocation claims through nested Reader scopes, and builders separately cap
account positions against the caller's item budget.

## Boundaries and residual work

The [415-ID receipt](../parity/token.tsv) retains every original canonical field,
source digest and provenance, with per-ID behavior and residuals. Its source audit
verifies all 38 defining-file hashes against the original archive. No renderer ID
or whole package contract becomes complete through this client.

Original Dart builders force authority signatures and omit remaining multisigner
lists. This client implements the IDL's caller-selected roles and remaining lists;
the difference is explicit. Source mutable discriminator fields, data-only parser
behavior, wrapper equality/Display/hashCode, fetching, PDA and handwritten helper
contracts remain partial or pending. Program error codes preserve unknown values
and exact source messages, including the original spelling.

Rust raw UTF8 preserves an initial BOM; the original Dart default decoder removes
it. Both preserve NUL and produce the same encoding bytes. The receipt records
this decoder difference. The pinned IDL and Dart Batch builder declare empty
accounts; assembling arbitrary batch account lists remains a separate contract.

The original generator unit's three Wasm compilations are portability checks.
Subsequent [portable runtime execution](token-runtime.md) adds the literal matrix
and strict groups on all five hosts. Validator, device, wallet submission, network
and registry release execution remain separate. Atomic publication covers one
generated module; it does not claim atomic replacement of a directory tree.
