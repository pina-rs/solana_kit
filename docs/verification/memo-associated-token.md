# Memo and Associated Token Account clients

These handwritten off-chain clients use Kit-owned types and `no_std + alloc`.
They are not generated. On-chain programs should use Pina or official Solana
program libraries. Component publication remains disabled.

## Pinned contracts

Dart source is `0138bb956d2d81ccc0c56edc602401c0df2f2b46`. Memo's Codama IDL is
from `4a487432a7a57cc3739db43c6e7ac12fc96ff338`, SHA256
`13e1cd43277ec47113900e964087cc3627b9e83f3ca8eab9c8c5bf02114b571c`.
ATA's handwritten official interface is from
`0b867b5340cd001e5980d8ca7928effc4e10015c`, SHA256
`033191cb1316fe2370fb9f694b0a0f9f140f647ca44661f07cb661ed4aa8c56d`.
Both crates retain the original Dart MIT and program Apache notices.

Memo emits raw UTF-8 without a discriminator or length prefix. All valid suffixes
belong to the memo. Strict decoding rejects malformed UTF-8, removes one leading
BOM and preserves null characters. Extraction retains original bytes, historical
identity and source index. Builders retain duplicate readonly signer positions.
Full parsing accepts readonly and writable signers. Extraction ignores metadata.
The actual Dart low-level builder accepts arbitrary program identities. Rust
builders accept the three supported Memo identities and full parsing checks them.
Rust strings exclude the source's possible unpaired surrogate inputs.

ATA canonical data is exactly one byte, with create tag 0, idempotent create tag 1
and nested recovery tag 2. Metadata contains six or seven exact ordered accounts.
Canonical parsing checks identity, count, roles, tag and trailing bytes. Source
low-level parsers decode only data and permit mutable discriminator values.
Those permissive standalone codecs are not implemented. System and token account
identities remain caller-supplied. Associated addresses use owner, token program
and mint seeds under the selected ATA program. Optional `crypto` reuses the
existing core implementation without new runtime dependencies.

## Reproducible receipt and evidence

The [declaration receipt](../parity/memo-associated-token.tsv) contains every
canonical source ID for these two packages exactly once. Its Rust tool validates
each complete source hash against the canonical ledger before writing.

```sh
rustc --edition 2024 -Dwarnings tools/memo_ata_receipt.rs -o target/memo_ata_receipt
target/memo_ata_receipt "$DART_SOURCE"
cargo test -p solana_kit_memo -p solana_kit_associated_token_account --all-features --offline --locked
cargo test -p solana_kit_conformance --test memo_associated_token --offline --locked
```

The receipt distinguishes actual bounded behavior from missing source wrappers,
mutable data classes and wider capabilities. The global ledger remains unchanged.
The independent source tests and literal probes are retained by the coordinator
in ignored Unit115 evidence. All 32 original Dart Memo and ATA tests execute.
Public Rust tests cover literal payloads, nulls, BOMs, malformed UTF-8, metadata,
all three ATA tags, every wrong role, all unknown tags, trailing and empty data,
foreign programs, budgets and both Token and Token-2022 PDA literals.

Two isolated conformance functions execute natively. They are not registered in
the central suite, and its identity and case counts remain unchanged. Wasm checks
are compilation evidence. No browser, WASI host, interpreted guest, validator,
RPC, wallet, device or registry-consumer runtime proof is claimed for this unit.

## Remaining finite renderer work

The finite generator is unchanged. It accepts tagged numeric and linked layouts
with no account metadata. Actual Memo's `remainingAccounts` property rejects
before output creation. Removing that property still leaves an untagged UTF-8
argument outside its discriminator and numeric/link contracts. ATA's pinned
artifact is Rust source rather than a Codama IDL.

A future bounded extension needs a typed `Discriminator` with `U8` and `None`
variants, a remainder field with explicit terminal placement, and an account
contract with ordered required accounts and typed remaining signer accounts.
Validation must prevent ambiguous untagged dispatch, nonterminal remainder
fields, unknown metadata properties and unsupported optional/default/PDA forms
before the existing atomic writer runs. ATA normalization must be explicit and
proven against the handwritten source. No second generator or invented IDL is
part of this unit. Full renderer, Token, Token-2022 and package parity remain open.
