# Token handwritten instruction plans

This page records the Rust port of the three pinned handwritten SPL Token plan
modules. It covers one crate-level optional feature and five public helper
behaviors. It does not claim whole-SDK or validator completion.

## Source

Pinned Dart source at commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`:

| Source file                                                      | Git blob                                   | Rust owner                             |
| ---------------------------------------------------------------- | ------------------------------------------ | -------------------------------------- |
| `packages/solana_kit_token/lib/src/create_mint.dart`             | `cb3fafc34a2757e295e4bab8287a5651046830a4` | `crates/solana_kit_token/src/plans.rs` |
| `packages/solana_kit_token/lib/src/mint_to_ata.dart`             | `f590f53057e8d5eba1f787744987461bedd7f349` | `crates/solana_kit_token/src/plans.rs` |
| `packages/solana_kit_token/lib/src/transfer_to_ata.dart`         | `42f850b7e51b19e1f2551e88145dd096daf7b4e4` | `crates/solana_kit_token/src/plans.rs` |
| `packages/solana_kit_token/lib/src/generated/accounts/mint.dart` | `96f3423f808b3458f855be1eb978335c9081d14f` | mint size 82                           |

The Rust layer composes the existing typed builders:

- `solana_kit_system::CreateAccount`
- `solana_kit_associated_token_account::AssociatedTokenInstruction::CreateIdempotent`
- `solana_kit_token::{InitializeMint2, MintToChecked, TransferChecked}`
- `solana_kit_core::plans::Plan<Instruction>`
- `solana_kit_associated_token_account::AssociatedTokenSeeds::find_with_program`

No generated file, core planner, canonical capability ledger, status, ID or
shared suite was changed.

## Feature

`solana_kit_token` keeps its default dependency closure unchanged:

```text
default: solana_kit_token -> solana_kit_core -> bs58
```

The explicit `plans` feature adds:

```text
plans: solana_kit_token -> solana_kit_system -> solana_kit_core
       solana_kit_token -> solana_kit_associated_token_account
       solana_kit_associated_token_account/crypto -> solana_kit_core/crypto
```

ATA crypto derivation is therefore available only when `plans` is enabled. The
feature is opt-in; default builds still expose only the generated Token surface.

## API

| Source behavior                        | Rust function                                                                          |
| -------------------------------------- | -------------------------------------------------------------------------------------- |
| `getCreateMintInstructionPlan`         | `create_mint_plan(CreateMintInput, CreateMintConfig, Limits)`                          |
| `getMintToAtaInstructionPlan`          | `mint_to_ata_plan(MintToAtaInput, MintToAtaConfig, Limits)`                            |
| `getMintToAtaInstructionPlanAsync`     | `mint_to_ata_derived_plan(MintToAtaDerivedInput, MintToAtaConfig, Limits)`             |
| `getTransferToAtaInstructionPlan`      | `transfer_to_ata_plan(TransferToAtaInput, TransferToAtaConfig, Limits)`                |
| `getTransferToAtaInstructionPlanAsync` | `transfer_to_ata_derived_plan(TransferToAtaDerivedInput, TransferToAtaConfig, Limits)` |

All functions return `Result<Plan<Instruction>, PlanError>`. `PlanError`
retains `codecs::Error`, `solana_kit_system::ClientError` and
`solana_kit_core::crypto::PdaError` causes without string or boxed-erasure
fallbacks.

`CreateMintConfig`, `MintToAtaConfig` and `TransferToAtaConfig` implement
`Default`; absent program fields select the canonical Token, System and
associated-token-account identities. Input structs mirror the source field
names and requiredness. Decimals are `u8`; amounts and lamports are `u64`.

## Behavior

### Create mint

`create_mint_plan` returns `Plan::indivisible` with exactly two leaves:

1. System `CreateAccount` for the new mint.
2. Token `InitializeMint2`.

The `CreateAccount` payload is:

- lamports: `mint_account_lamports`, else `1_461_600`
- space: `82`
- account owner: selected token program

The first instruction's program identity is the selected System program. The
second instruction's program identity is the selected token program. Its only
account is the new mint, writable and non-signing. `freeze_authority` is passed
through unchanged: `None` encodes the absent COption and `Some` encodes the
authority.

The default `1_461_600` lamports is the source's pinned rent-exempt constant,
not a live `getMinimumBalanceForRentExemption` result. Callers must pass
`mint_account_lamports` when the target cluster's current rent-exempt minimum
differs.

### Mint to associated account

`mint_to_ata_plan` returns `Plan::sequential` with exactly two leaves:

1. ATA `CreateIdempotent`
2. Token `MintToChecked`

ATA account positions are payer (writable signer), associated account
(writable), owner, mint, selected System program and selected token program.
`MintToChecked` positions are mint (writable), associated account (writable)
and mint authority (readonly signer). The amount is passed through as written, including zero.

### Transfer to associated account

`transfer_to_ata_plan` returns `Plan::sequential` with exactly two leaves:

1. ATA `CreateIdempotent` for the destination
2. Token `TransferChecked`

ATA account positions are payer (writable signer), destination (writable),
recipient, mint, selected System program and selected token program.
`TransferChecked` positions are source (writable), mint, destination
(writable) and authority (readonly signer). The amount is passed through as written, including
zero.

### Synchronous derivation

The source async variants derive addresses. Rust keeps the same operation
synchronous:

- Mint helper seeds: `owner`, selected token program, `mint`.
- Transfer destination seeds: `recipient`, selected token program, `mint`.
- Transfer source seeds: `authority`, selected token program, `mint`.
- All seeds use the selected associated-token-account program as the deriving
  program.

The derived functions delegate to the explicit input functions, so both plans
share one instruction-building path.

## Override precedence

Each configuration field is optional. `None` selects the canonical program;
`Some` wins. The selected token program is used for both instruction identity
and account metadata where the token program appears. The selected System
program is used for the System instruction identity and the ATA metadata
System position. The selected associated-token-account program is used for the
ATA instruction identity and for derivation.

No implicit extra validation is added. In particular the helpers do not check
whether accounts exist, whether the payer is funded, whether an authority
signed, whether an amount is non-zero or whether the source account owns
enough tokens. They do not fetch rent, a recent blockhash, a signer, a network
response or an executor.

## Limits and failure behavior

`Limits` are passed to every builder. A too-small byte or item budget returns
the original typed failure and never exposes a partial plan. The create-mint
pair is never split into independent leaves; callers that compile plans must
respect its indivisible boundary.

## Differences from Dart

- `BigInt` inputs become `u64`. Negative values and values above `u64::MAX`
  must be rejected by the caller before constructing the input; Rust cannot
  represent them.
- The async Dart derivation becomes synchronous because Rust's PDA hashing is
  pure and needs no future.
- The source's live rent responsibility remains explicit rather than being
  hidden behind a default lookup.
- Rust returns the shared core `Plan<Instruction>` tree rather than a
  Dart-specific `InstructionPlan` object.

## Local evidence

All Cargo commands are offline. Crate checks are scoped to
`solana_kit_token`; style is repository-level. `CARGO_INCREMENTAL=0` and
`CARGO_BUILD_JOBS=4` were used.

```sh
cargo +1.99.1 test -p solana_kit_token --locked --offline
cargo +1.99.1 test -p solana_kit_token --all-features --locked --offline
cargo +1.99.1 test -p solana_kit_token --features plans --locked --offline --test plans
cargo +1.99.1 clippy -p solana_kit_token --all-targets --all-features --locked --offline -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo +1.99.1 doc -p solana_kit_token --all-features --locked --offline --no-deps
cargo +1.89.0 test -p solana_kit_token --features plans --locked --offline --test plans
cargo +1.99.1 check -p solana_kit_token --features plans --target wasm32-unknown-unknown --locked --offline
cargo +1.99.1 check -p solana_kit_token --features plans --target wasm32-wasip1 --locked --offline
cargo +1.99.1 check -p solana_kit_token --features plans --target wasm32-wasip2 --locked --offline
rustfmt +1.99.1 --edition 2024 --check crates/solana_kit_token/src/lib.rs \
  crates/solana_kit_token/src/plans.rs crates/solana_kit_token/tests/plans.rs
dprint check
monochange check
cargo xtask style
```

Local results:

- Default Token tests: 93 oracle + 6 strict passed; the plans target is
  skipped unless `plans` is enabled.
- All-feature Token tests: 93 oracle + 15 plans + 6 strict passed.
- Rust 1.89 plans target: 15 passed.
- Clippy all-targets/all-features with `-D warnings`: passed.
- Rustdoc with `-D warnings`: passed.
- All three Wasm target checks: passed.
- Scoped `rustfmt`, `dprint check` and `monochange check`: passed.
- Repository style and formatting are verified in the isolated Token commit,
  excluding unfinished concurrent changes.

The tests use independent literal wire bytes, ordered accounts and roles for
all five behaviors. Derived-address literals were produced with the installed
Agave `solana find-program-derived-address` command, not with the code under
test.

Parent review corrected the authority roles to match the pinned source's
`readonlySigner`. A regression first failed because a compiled ATA plan required
only its payer's signature. It now requires the distinct mint/transfer authority
as well. The same roles flow through both derived helpers.

## Limits of this receipt

This receipt proves local instruction construction and address derivation. It
does not prove validator acceptance, live rent values, account existence,
signing, execution, publication or complete Token
package parity.
