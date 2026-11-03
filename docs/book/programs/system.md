# System

`solana_kit_system` covers the System Program. You get all 14 instructions as typed data plus account builders, strict parsers, durable-nonce account snapshots, and the program's custom errors. It is `no_std + alloc` and depends only on `solana_kit_core`.

## Instructions

Each instruction is a data struct (`TransferSol { amount }`) with a matching `…Accounts` struct that names every account position. `instruction(accounts, limits)` always targets `SYSTEM_PROGRAM` and gives each account the role the program expects.

| Tag | Data type                   | Accounts type                       |
| --: | --------------------------- | ----------------------------------- |
|   0 | `CreateAccount`             | `CreateAccountAccounts`             |
|   1 | `Assign`                    | `AssignAccounts`                    |
|   2 | `TransferSol`               | `TransferSolAccounts`               |
|   3 | `CreateAccountWithSeed`     | `CreateAccountWithSeedAccounts`     |
|   4 | `AdvanceNonceAccount`       | `AdvanceNonceAccountAccounts`       |
|   5 | `WithdrawNonceAccount`      | `WithdrawNonceAccountAccounts`      |
|   6 | `InitializeNonceAccount`    | `InitializeNonceAccountAccounts`    |
|   7 | `AuthorizeNonceAccount`     | `AuthorizeNonceAccountAccounts`     |
|   8 | `Allocate`                  | `AllocateAccounts`                  |
|   9 | `AllocateWithSeed`          | `AllocateWithSeedAccounts`          |
|  10 | `AssignWithSeed`            | `AssignWithSeedAccounts`            |
|  11 | `TransferSolWithSeed`       | `TransferSolWithSeedAccounts`       |
|  12 | `UpgradeNonceAccount`       | `UpgradeNonceAccountAccounts`       |
|  13 | `CreateAccountAllowPrefund` | `CreateAccountAllowPrefundAccounts` |

```rust
use solana_kit_core::{Address, codecs::Limits};
use solana_kit_system::{ClientError, CreateAccount, CreateAccountAccounts};

fn create_account(payer: Address, new_account: Address, owner: Address) -> Result<(), ClientError> {
    let instruction = CreateAccount {
        lamports: 1_000_000,
        space: 165,
        program_address: owner,
    }
    .instruction(CreateAccountAccounts { payer, new_account }, Limits::new(1232, 32))?;

    // Both the payer and the new account must sign.
    assert!(instruction.accounts.iter().all(|meta| meta.role.is_signer()));
    Ok(())
}
```

The builders check the rules the client can check locally and return a `ClientError` when one is broken:

- `ClientError::SeedTooLong` if a seed's UTF-8 encoding is longer than 32 bytes. The limit counts bytes, not characters.
- `ClientError::BaseMismatch` if the `base` in a seeded instruction's data differs from the signing base account you supplied.
- `ClientError::MissingPayer` if `CreateAccountAllowPrefund` has nonzero `lamports` but no `payer`.

Nonce instructions add the `RECENT_BLOCKHASHES` and `RENT` sysvars for you. Optional accounts, such as `base_account: Option<Address>` on `CreateAccountWithSeedAccounts`, stay explicit. If you pass the same address in two positions, both positions stay in the instruction.

<Callout kind="note" title="Derived addresses are your job">

Seeded builders do not hash the seed to check the destination address. Derive it yourself with core's optional cryptography (see [Addresses and keys](../getting-started/addresses-and-keys.md)), then pass it in. The validator checks it when the transaction runs.

</Callout>

## Parsing

`TransferSol::parse(&instruction, limits)` returns the data and the typed accounts. `parse_instruction` dispatches on the four-byte tag and returns a `SystemInstructionData` enum. Both reject:

- a program other than `SYSTEM_PROGRAM` (`ClientError::ProgramMismatch`)
- the wrong number of accounts (`ClientError::AccountCount`)
- an account with the wrong privileges (`ClientError::AccountRole(index)`)
- a wrong fixed sysvar address (`ClientError::AccountAddress(index)`)
- an unknown tag, truncated data, trailing bytes, or a blown budget (`ClientError::Codec(_)`)

```rust
use solana_kit_core::{Address, codecs::Limits};
use solana_kit_system::{ClientError, TransferSol, TransferSolAccounts, identify_instruction};

fn parse_transfer() -> Result<(), ClientError> {
    let limits = Limits::new(1232, 32);
    let destination = Address::from_bytes([2; 32]);
    let instruction = TransferSol { amount: 500 }.instruction(
        TransferSolAccounts { source: Address::from_bytes([1; 32]), destination },
        limits,
    )?;

    assert_eq!(identify_instruction(&instruction.data)?, 2);
    let (data, accounts) = TransferSol::parse(&instruction, limits)?;
    assert_eq!(data.amount, 500);
    assert_eq!(accounts.destination, destination);

    let mut tampered = instruction.clone();
    tampered.data.push(0);
    assert!(TransferSol::parse(&tampered, limits).is_err());
    Ok(())
}
```

`identify_instruction` reads only the tag. Use `parse_instruction` or a type's `parse` before you trust anything else.

> [!NOTE]
> Parsers expect each account's role as the instruction declared it, before message compilation. A compiled transaction can raise an account's privileges because another instruction uses it as a signer or writable. Rebuild the original instruction roles before you run a strict parser over a decompiled message.

## Nonce accounts

`NonceCodec` decodes the exact 80-byte allocated nonce account. `decode_nonce` takes a fetched `EncodedAccount`, checks that the System Program owns it and that its size is right, and keeps all account metadata.

```rust
use solana_kit_core::{accounts::{AccountError, EncodedAccount}, codecs::Limits};
use solana_kit_system::decode_nonce;

fn read_nonce(account: EncodedAccount) -> Result<(), AccountError> {
    let snapshot = decode_nonce(account, Limits::new(1232, 32))?;

    match snapshot.data.durable_data() {
        Some(nonce) => {
            println!("authority {}", nonce.authority);
            println!("fee {} lamports per signature", nonce.lamports_per_signature);
        }
        None => println!("legacy or uninitialized nonce"),
    }
    Ok(())
}
```

The model is stricter than the flat IDL:

- `NonceVersion::Legacy` and `NonceVersion::Current` stay separate. Decoding never upgrades a legacy hash.
- `NonceState::Uninitialized([u8; 72])` keeps the unused storage bytes as they are. It has no authority or hash.
- `NonceState::Initialized(NonceData)` holds `authority`, the stored `blockhash`, and `lamports_per_signature` as a full `u64`.
- `Nonce::durable_data()` returns data only for an initialized `Current` nonce, the only kind you should use to sign a durable transaction.

For building and confirming durable-nonce transactions, see [Messages and transactions](../core/transactions.md).

## Errors

`SystemError::from_code(u32)` maps all nine pinned custom codes (0 to 8), from `AccountAlreadyInUse` to `NonceUnexpectedBlockhashValue`. Any other code becomes `SystemError::Unknown(code)`. `code()` returns the original number, and `message()` returns the pinned description for known codes.

```rust
use solana_kit_system::SystemError;

let error = SystemError::from_code(6);
assert_eq!(error, SystemError::NonceNoRecentBlockhashes);
assert!(error.message().is_some());
assert_eq!(SystemError::from_code(42), SystemError::Unknown(42));
```

## Status

All 14 builders were compared byte for byte, including every ordered account and its signer and writable flags, against the official `solana-system-interface` 3.3.0 constructors. Nonce layouts were compared against `solana-nonce` 3.2.1. The client contract cases run on native Rust, Chrome, both WASI previews, and the LeptosKit interpreter.

Still pending:

- Validator execution and full durable-nonce transaction journeys.
- Standalone enum codecs and fixed default constructors, which are marked partial in the [System client receipt](https://github.com/pina-rs/solana_kit/tree/main/docs/parity/system-client.tsv).

The crate does no RPC, signing, or seed derivation. Details are in [`docs/verification/system-client.md`](https://github.com/pina-rs/solana_kit/tree/main/docs/verification/system-client.md).
