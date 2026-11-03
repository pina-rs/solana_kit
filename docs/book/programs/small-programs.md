# Compute Budget, Memo, and ATA

These three small clients show up in almost every transaction. `solana_kit_compute_budget` sets compute limits and priority fees, `solana_kit_memo` attaches and reads memos, and `solana_kit_associated_token_account` creates and derives associated token accounts. All three are `no_std + alloc` and depend only on `solana_kit_core`.

## Compute Budget

The crate has strict codecs, builders, and parsers for all five pinned layouts: `RequestUnits`, `RequestHeapFrame`, `SetComputeUnitLimit`, `SetComputeUnitPrice`, and `SetLoadedAccountsDataSizeLimit`. Compute Budget instructions take no accounts, so `instruction` only needs limits.

```rust
use solana_kit_core::codecs::{Error, Limits};
use solana_kit_compute_budget::{
    ComputeBudgetInstructionData, SetComputeUnitLimit, SetComputeUnitPrice, parse_instruction,
};

fn budget() -> Result<(), Error> {
    let limits = Limits::new(64, 0);
    let limit = SetComputeUnitLimit { units: 200_000 }.instruction(limits)?;
    let price = SetComputeUnitPrice { micro_lamports: 10_000 }.instruction(limits)?;

    assert!(limit.accounts.is_empty());
    assert!(matches!(
        parse_instruction(&price, limits)?,
        ComputeBudgetInstructionData::SetComputeUnitPrice(_)
    ));
    Ok(())
}
```

<Callout kind="caution" title="RequestUnits is historical">

`RequestUnits` is a deprecated layout, and current validators reject it. The client can still encode it, but encoding does not mean a node will accept it. Heap alignment and the network's compute limits are validator policy, and the builders do not clamp values.

</Callout>

### Editing an instruction list

The helpers work on an existing `&[Instruction]` and return a new `Vec<Instruction>`. They follow the pinned Dart message helpers:

| Function                               | Behavior                                                                                  |
| -------------------------------------- | ----------------------------------------------------------------------------------------- |
| `find_compute_unit_limit`              | Returns `(index, units)` of the first limit instruction, if any.                          |
| `find_compute_unit_price`              | Returns `(index, micro_lamports)` of the first price instruction, if any.                 |
| `update_or_append_compute_unit_limit`  | Replaces the first limit with `update(old)`, or appends one. Later duplicates stay as is. |
| `update_or_append_compute_unit_price`  | The same for the price, keeping full `u64` precision.                                     |
| `append_compute_unit_price`            | Always appends a price, even if one exists.                                               |
| `fill_provisory_compute_unit_limit`    | Adds a limit of `PROVISORY_COMPUTE_UNIT_LIMIT` (0) only when none exists.                 |

```rust
use solana_kit_core::{Instruction, codecs::Error};
use solana_kit_compute_budget::{fill_provisory_compute_unit_limit, update_or_append_compute_unit_price};

fn with_priority_fee(instructions: &[Instruction]) -> Result<Vec<Instruction>, Error> {
    let instructions = update_or_append_compute_unit_price(instructions, |_| 25_000)?;
    fill_provisory_compute_unit_limit(&instructions)
}
```

The provisory limit is a placeholder inserted before resource estimation sets a real value. See [Build a transaction](../getting-started/build-a-transaction.md). `MAX_COMPUTE_UNIT_LIMIT` (1,400,000) is the documented validator maximum. The find helpers ignore trailing bytes and account metadata, as the Dart helpers do. They return an error if they reach an unknown Compute Budget tag before finding a match.

## Memo

The Memo client is handwritten. `add_memo` builds a raw UTF-8 memo with any number of readonly signers. `add_memo_with_program` targets one of the three supported identities in `SUPPORTED_PROGRAMS`: `PROGRAM` (current v4, `Memo4c2pN8afCj432Lb7RMVKi9PbQnnW7ewFFaV3oAH`), `LEGACY_PROGRAM` (v1), and `LEGACY_PROGRAM_V3`. Any other program is rejected.

```rust
use solana_kit_core::{Address, codecs::{Error, Limits}};
use solana_kit_memo::{add_memo, memos_from_instructions, parse_instruction};

fn memo(signer: Address) -> Result<(), Error> {
    let limits = Limits::new(566, 566);
    let instruction = add_memo("invoice #42", &[signer], limits)?;

    let parsed = parse_instruction(&instruction, limits)?;
    assert_eq!(parsed.memo, "invoice #42");
    assert_eq!(parsed.signers, [signer]);

    let found = memos_from_instructions(&[instruction], limits)?;
    assert_eq!(found[0].index, 0);
    Ok(())
}
```

- `parse_instruction` returns `ParsedMemo { memo, signers }`. It checks the program identity and requires every account to be a signer. The text and the signer accounts share one item budget, so set `max_items` to at least the memo length plus the signer count.
- `memos_from_instructions` scans a list and returns an `ExtractedMemo` for each memo-program instruction: the text, the raw bytes, the program address, and the index. It ignores account roles, as the source helper does.
- `decode_memo` decodes the bytes alone.

UTF-8 is strict. NUL characters are kept, and decoding removes a leading BOM, as the Dart codec does. The client sets no transaction-size or compute limit of its own. Your `Limits` are the only bound.

## Associated Token Account

The ATA client is also handwritten. `AssociatedTokenInstruction` has three variants, `Create`, `CreateIdempotent`, and `RecoverNested`, each holding a typed accounts struct. You pass the System and token program addresses yourself. The constants `SYSTEM_PROGRAM`, `TOKEN_PROGRAM`, and `TOKEN_2022_PROGRAM` are exported for convenience.

Enable the `crypto` feature to derive the address with `AssociatedTokenSeeds`:

```toml
[dependencies]
solana_kit_associated_token_account = { git = "https://github.com/pina-rs/solana_kit", features = ["crypto"] }
```

```rust
use solana_kit_core::{Address, codecs::Limits};
use solana_kit_associated_token_account::{
    AssociatedTokenInstruction, AssociatedTokenSeeds, CreateAccounts, SYSTEM_PROGRAM, TOKEN_PROGRAM,
    parse_instruction,
};

fn create_ata(payer: Address, owner: Address, mint: Address) -> Result<(), Box<dyn std::error::Error>> {
    let associated_token = AssociatedTokenSeeds { owner, token_program: TOKEN_PROGRAM, mint }
        .find()?
        .address();

    let instruction = AssociatedTokenInstruction::CreateIdempotent(CreateAccounts {
        payer,
        associated_token,
        owner,
        mint,
        system_program: SYSTEM_PROGRAM,
        token_program: TOKEN_PROGRAM,
    })
    .instruction(Limits::new(1, 6))?;

    assert!(matches!(
        parse_instruction(&instruction, Limits::new(1, 6))?,
        AssociatedTokenInstruction::CreateIdempotent(_)
    ));
    Ok(())
}
```

For a Token-2022 mint, use `TOKEN_2022_PROGRAM` in both the seeds and the accounts. `find_with_program` derives against a non-canonical ATA program. The derivation accepts any token program, and it does not require the owner to be on the curve.

`parse_instruction` checks the ATA program identity, the one-byte tag, the exact account count, and each account's role. It does not accept the mutable discriminators the Dart low-level parsers allow. `error_message(ERROR_INVALID_OWNER)` returns the program's single custom error text.

## Status

All three crates run their contract cases on native Rust, Chrome, both WASI previews, and the LeptosKit interpreter. The Compute Budget builders were compared byte for byte against the five pinned Dart constructors. Still pending:

- Validator acceptance.
- Account-existence checks.
- Token account codecs in the ATA crate. Those live in [Token and Token-2022](token.md).

See [`docs/verification/compute-budget.md`](https://github.com/pina-rs/solana_kit/tree/main/docs/verification/compute-budget.md) and [`docs/verification/memo-ata-runtime.md`](https://github.com/pina-rs/solana_kit/tree/main/docs/verification/memo-ata-runtime.md).
