# Token and Token-2022

Two crates cover the SPL token programs. `solana_kit_token` is the original SPL Token program: 28 instructions and the Mint, Token, and Multisig accounts. `solana_kit_token_2022` covers Token-2022: 100 typed builders, strict parsers, all three account layouts with their TLV extensions, and 69 custom errors. Both are generated from pinned IDLs and depend only on `solana_kit_core` by default.

## SPL Token

### Build an instruction

Token builders take an ordered slice of `AccountMeta`, not a typed struct. The builder checks each position against the role the IDL declares, so a wrong privilege is an error rather than a failed transaction later. The authority can sign directly, or it can be a non-signing multisig account followed by its signers as extra readonly signer accounts.

```rust
use solana_kit_core::{AccountMeta, AccountRole, Address, codecs::{Error, Limits}};
use solana_kit_token::{PROGRAM, TransferChecked};

fn transfer_checked(
    source: Address,
    mint: Address,
    destination: Address,
    owner: Address,
) -> Result<(), Error> {
    let accounts = [
        AccountMeta { address: source, role: AccountRole::Writable },
        AccountMeta { address: mint, role: AccountRole::Readonly },
        AccountMeta { address: destination, role: AccountRole::Writable },
        AccountMeta { address: owner, role: AccountRole::ReadonlySigner },
    ];
    let instruction = TransferChecked { amount: 1_000_000, decimals: 6 }
        .instruction(&accounts, Limits::new(1232, 32))?;
    assert_eq!(instruction.program_address, PROGRAM);
    Ok(())
}
```

Each instruction type also has `instruction_with_program(program, accounts, limits)` for a non-canonical deployment. The instruction structs are `InitializeMint`, `InitializeAccount`, `InitializeMultisig`, `Transfer`, `Approve`, `Revoke`, `SetAuthority`, `MintTo`, `Burn`, `CloseAccount`, `FreezeAccount`, `ThawAccount`, the `…Checked` variants, `InitializeAccount2`/`3`, `SyncNative`, `InitializeMultisig2`, `InitializeMint2`, `GetAccountDataSize`, `InitializeImmutableOwner`, `AmountToUiAmount`, `UiAmountToAmount`, `WithdrawExcessLamports`, `UnwrapLamports`, and `Batch`.

### Parse an instruction

`parse_instruction` (or `parse_instruction_with_program`) checks the program identity, tag, account roles, and remaining accounts, then returns a `TokenInstructionData` enum. Each type also has its own `parse` and `parse_with_program`.

```rust
use solana_kit_core::{Instruction, codecs::{Error, Limits}};
use solana_kit_token::{TokenInstructionData, parse_instruction};

fn describe(instruction: &Instruction) -> Result<(), Error> {
    match parse_instruction(instruction, Limits::new(1232, 32))? {
        TokenInstructionData::TransferChecked(data) => {
            println!("transfer {} (decimals {})", data.amount, data.decimals);
        }
        other => println!("other Token instruction: {other:?}"),
    }
    Ok(())
}
```

### Decode accounts

Token accounts have no discriminator, so `identify_account` uses the exact length: 82 bytes is a `Mint`, 165 is a `Token`, and 355 is a `Multisig`. Then decode with `MintCodec`, `TokenCodec`, or `MultisigCodec`. Absent `COption` fields come back as `None`, even if the unused payload bytes are not zero.

```rust
use solana_kit_core::codecs::{Decoder, Error, Limits};
use solana_kit_token::{TokenAccountKind, TokenCodec, identify_account};

fn token_balance(data: &[u8]) -> Result<Option<u64>, Error> {
    if identify_account(data)? != TokenAccountKind::Token {
        return Ok(None);
    }
    let token = TokenCodec.decode(data, Limits::new(165, 16))?;
    Ok(Some(token.amount))
}
```

The Token crate has no `decode_*` helper for fetched accounts. Check the owner yourself, or use the generic `EncodedAccount::decode` from [Accounts and sysvars](../core/accounts.md).

### Errors and status

`TokenError::from_code(u32)` covers the pinned program errors and keeps unknown codes as `TokenError::Unknown(code)`. `message()` gives the program's text.

The Token contract groups run on native Rust, Chrome, both WASI previews, and the LeptosKit interpreter. They include byte-for-byte checks of all 28 constructors and the three account layouts. Known gaps:

- Assembling arbitrary account lists for `Batch` is pending. The batch constructor keeps the source's empty account list.
- Fetch helpers and the Dart wrapper contracts are not ported.
- Validator execution is pending.
- Raw UTF-8 decoding keeps a leading BOM, while the Dart decoder removes it.

## Token-2022

### Build an instruction

Token-2022 builders take a typed `…Accounts` struct instead of a slice. The authority is an explicit `Authority { address, is_signer }`, and multisig signers go in `multi_signers`.

```rust
use solana_kit_core::{Address, codecs::{Error, Limits}};
use solana_kit_token_2022::{Authority, TransferChecked, TransferCheckedAccounts};

fn multisig_transfer(multisig: Address, signers: &[Address]) -> Result<(), Error> {
    let instruction = TransferChecked { amount: 1_000_000, decimals: 6 }.instruction(
        TransferCheckedAccounts {
            source: Address::from_bytes([1; 32]),
            mint: Address::from_bytes([2; 32]),
            destination: Address::from_bytes([3; 32]),
            authority: Authority { address: multisig, is_signer: false },
            multi_signers: signers,
        },
        Limits::new(1232, 32),
    )?;
    assert_eq!(instruction.accounts.len(), 4 + signers.len());
    Ok(())
}
```

Builders keep account order and duplicate addresses. For a required sysvar account, pass `None` to use its fixed default address. For an optional account, `None` leaves it out. `instruction_with_program` targets a custom deployment.

### Parse an instruction

`parse_instruction` returns a `ParsedInstruction` with the decoded `data` (a `Token2022InstructionData`) and the original `accounts` list, including any remaining accounts. Where two optional readonly positions cannot be told apart, the parser does not guess names. It keeps the raw account list.

```rust
use solana_kit_core::{Instruction, codecs::{Error, Limits}};
use solana_kit_token_2022::{Token2022InstructionData, parse_instruction};

fn is_transfer(instruction: &Instruction) -> Result<bool, Error> {
    let parsed = parse_instruction(instruction, Limits::new(1232, 32))?;
    Ok(matches!(parsed.data, Token2022InstructionData::TransferChecked(_)))
}
```

### Accounts and extensions

`MintCodec`, `TokenCodec`, and `MultisigCodec` are structural codecs. `Mint` and `Token` carry `extensions: Option<Vec<Extension>>`. The `Extension` enum has 29 variants, including `TransferFeeConfig`, `MintCloseAuthority`, `ConfidentialTransferMint`, `DefaultAccountState`, `ImmutableOwner`, `MemoTransfer`, `NonTransferable`, `InterestBearingConfig`, `CpiGuard`, `PermanentDelegate`, `TransferHook`, `MetadataPointer`, `TokenMetadata`, `GroupPointer`, `TokenGroup`, `ScaledUiAmountConfig`, and `PausableConfig`. `ExtensionsCodec` encodes and decodes the TLV sequence on its own.

For fetched account data, use the `accounts` module. It also checks semantics the structural codecs do not: the account is initialized, no extension appears twice, each extension belongs to that account type, signer counts are in bounds, and invalid mint-extension combinations are rejected.

```rust
use solana_kit_core::{Address, codecs::{Error, Limits}};
use solana_kit_token_2022::accounts::{Token2022Account, decode_account, mint_size};

fn inspect(owner: Address, data: &[u8]) -> Result<(), Error> {
    match decode_account(owner, data, Limits::new(64 * 1024, 4096))? {
        Token2022Account::Mint(mint) => println!("mint, {} extensions", mint.extensions.map_or(0, |e| e.len())),
        Token2022Account::Token(token) => println!("token account, amount {}", token.amount),
        Token2022Account::Multisig(_) => println!("multisig"),
    }
    assert_eq!(mint_size(None)?, 82);
    Ok(())
}
```

`decode_account` rejects an `owner` that is not the Token-2022 program. Use `decode_account_with_program` for a custom deployment. `mint_size` and `token_size` compute the allocation for a set of extensions, including the two padding bytes the program adds when an extended account would otherwise be 355 bytes, the same as a Multisig.

`extensions::pre_initialize_mint_extensions(mint, &extensions, program, limits)` returns the instructions that must run before `InitializeMint`. It covers the 15 extension branches in the source helper, in the order you pass them. Extensions without a pre-initialize step produce no instruction.

These checks do not replace the program's own authority, balance, or cross-account validation. Confidential-transfer proof bytes, keys, and entropy are yours to provide. The crate has no PDA helpers, because neither the IDL nor the original package defines any.

### Fetching with the `client` feature

Enable the `client` feature to add `solana_kit_token_2022::fetch`. It has `fetch_account`, `fetch_maybe_account`, and `fetch_maybe_accounts`, which take a `solana_kit_client::Client`, a `ReadConfig`, and limits. They return decoded accounts with the RPC context. A missing required account is `FetchError::Missing(address)`. You own the transport and executor. See [RPC](../core/rpc.md).

```toml
[dependencies]
solana_kit_token_2022 = { git = "https://github.com/pina-rs/solana_kit", features = ["client"] }
```

### Errors and status

`errors::Token2022Error::from_code(u32)` returns `Known(KnownToken2022Error)` or `Unknown(code)`, and `label()` gives the pinned Rust label. `errors::classify_failure` reads the custom code only when the failing instruction index belongs to the program you expect.

The 116 public Token-2022 test bodies run on native Rust, Chrome, both WASI previews, and the LeptosKit interpreter. The receipt is [`docs/parity/token-2022-client.md`](https://github.com/pina-rs/solana_kit/tree/main/docs/parity/token-2022-client.md).

> [!NOTE]
> TLV framing follows the canonical program layout, not the pinned Dart generator, which leaves out TLV payload lengths. Byte comparisons against Dart cover whole instruction and base-account frames, and bare extension tags and payloads. Validator transitions and device journeys are still pending.
