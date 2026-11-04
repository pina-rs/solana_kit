# Accounts and sysvars

An account snapshot is an owned copy of one account at one point in time: its address, owner, lamports, executable flag, allocation size, rent epoch and data. `solana_kit_core::accounts` defines snapshots and decodes their data with any [codec](codecs.md). `solana_kit_core::sysvars` provides typed layouts for the sysvar accounts. `solana_kit_client::accounts` fetches snapshots over RPC. Decoding never performs I/O, and fetching never guesses a layout.

## Snapshots

`Account<T>` is generic over its data. `EncodedAccount` is `Account<Vec<u8>>`. A decoded snapshot keeps every metadata field and swaps the data for your type.

| Field        | Type          | Meaning                                                       |
| ------------ | ------------- | ------------------------------------------------------------- |
| `address`    | `Address`     | The address you asked for                                     |
| `owner`      | `Address`     | The program that owns the account                             |
| `lamports`   | `u64`         | Balance                                                       |
| `executable` | `bool`        | Whether the account holds a program                           |
| `space`      | `u64`         | The reported data allocation, independent of the decoded form |
| `rent_epoch` | `Option<u64>` | Present only when the source reported it                      |
| `data`       | `T`           | Raw bytes or a decoded value                                  |

A fetch can find nothing at an address. That case is a value, not an error: `MaybeAccount<T>` is either `Present(Account<T>)` or `Missing(Address)`, and `MaybeEncodedAccount` is `MaybeAccount<Vec<u8>>`. Call `exists()` to check, `address()` to read the address in either state, and `require()` to turn `Missing` into `AccountError::Missing(address)`.

## Decoding account data

`EncodedAccount::decode(&decoder, expected_owner, limits)` decodes the data only after it checks two things:

1. The owner equals `expected_owner`. Otherwise it returns `AccountError::OwnerMismatch { address, expected, actual }`.
2. The data length equals `space`. Otherwise it returns `AccountError::DataSizeMismatch(address)`.

The data must then decode exactly, with no trailing bytes. A codec failure comes back as `AccountError::Decode { address, source }` with the original `codecs::Error` attached.

```rust
use solana_kit_core::{
    Address,
    accounts::{Account, AccountError, MaybeAccount},
    codecs::{Limits, Number},
    sysvars::OWNER_ADDRESS,
};

fn example() -> Result<(), AccountError> {
    let address = Address::from_bytes([3; 32]);
    let account = Account {
        address,
        owner: OWNER_ADDRESS,
        lamports: 42,
        executable: false,
        space: 8,
        rent_epoch: None,
        data: vec![255; 8],
    };

    let decoded = account.decode(&Number::<u64>::LE, OWNER_ADDRESS, Limits::new(8, 0))?;
    assert_eq!(decoded.data, u64::MAX);
    assert_eq!(decoded.lamports, 42);

    let missing = MaybeAccount::<Vec<u8>>::Missing(address);
    assert!(!missing.exists());
    assert_eq!(missing.require(), Err(AccountError::Missing(address)));
    Ok(())
}
```

`MaybeAccount<Vec<u8>>::decode` has the same checks. It returns `Missing` unchanged and never calls the decoder for it.

The client crate also has looser helpers. `solana_kit_client::accounts::decode_account` and `decode_maybe_account` borrow the snapshot, keep it intact on failure, and leave the owner check to you. `assert_accounts_exist` returns every missing address in input order.

## Fetching accounts over RPC

`Client<T>` adds four fetch helpers. Each returns `Contextual<...>`, which pairs the value with the node's evaluation `context` (slot and optional API version).

| Method                                          | Returns                                                                   |
| ----------------------------------------------- | ------------------------------------------------------------------------- |
| `fetch_encoded_account(address, config)`        | `Contextual<MaybeEncodedAccount>` with full base64 data                   |
| `fetch_encoded_accounts(addresses, config)`     | One entry per requested address, in order, duplicates included            |
| `fetch_json_parsed_account(address, config)`    | `Contextual<MaybeAccount<AccountData>>`, parsed JSON or a binary fallback |
| `fetch_json_parsed_accounts(addresses, config)` | The positional parsed form                                                |

`ReadConfig` carries the optional `commitment` and `min_context_slot`. For transports and errors, see [RPC](rpc.md). For a guided walkthrough, see [Fetch an account](../getting-started/fetch-an-account.md).

## Sysvars

`SysvarCodec::<T>::NEW` is a stateless codec that implements both `Encoder<T>` and `Decoder<T>` for each supported sysvar type. All fields use the pinned little-endian widths.

| Type                                              | Address constant             | Size                               |
| ------------------------------------------------- | ---------------------------- | ---------------------------------- |
| `Clock`                                           | `CLOCK_ADDRESS`              | `CLOCK_SIZE` (40 bytes)            |
| `Rent`                                            | `RENT_ADDRESS`               | `RENT_SIZE` (17 bytes)             |
| `EpochSchedule`                                   | `EPOCH_SCHEDULE_ADDRESS`     | `EPOCH_SCHEDULE_SIZE` (33 bytes)   |
| `EpochRewards`                                    | `EPOCH_REWARDS_ADDRESS`      | `EPOCH_REWARDS_SIZE` (81 bytes)    |
| `LastRestartSlot`                                 | `LAST_RESTART_SLOT_ADDRESS`  | `LAST_RESTART_SLOT_SIZE` (8 bytes) |
| `SlotHashes` (`Vec<SlotHashEntry>`)               | `SLOT_HASHES_ADDRESS`        | u64 count, then entries            |
| `StakeHistory` (`Vec<StakeHistoryEntry>`)         | `STAKE_HISTORY_ADDRESS`      | u64 count, then entries            |
| `RecentBlockhashes` (`Vec<RecentBlockhashEntry>`) | `RECENT_BLOCKHASHES_ADDRESS` | u64 count, then entries            |
| `SlotHistory`                                     | `SLOT_HISTORY_ADDRESS`       | `SLOT_HISTORY_SIZE`                |

Every sysvar account is owned by `OWNER_ADDRESS` (`Sysvar1111111111111111111111111111111111111`). The module also exports `FEES_ADDRESS`, `REWARDS_ADDRESS` and `INSTRUCTIONS_ADDRESS`. Those three have no codec.

To fetch a sysvar, combine the account fetch with the core decoder. This keeps the node context and the full account metadata:

```rust
use solana_kit_client::{Client, Transport, methods::{Contextual, ReadConfig}};
use solana_kit_core::{
    accounts::Account,
    codecs::Limits,
    sysvars::{CLOCK_ADDRESS, Clock, OWNER_ADDRESS, SysvarCodec},
};

async fn fetch_clock<T: Transport>(
    client: &Client<T>,
) -> Result<Contextual<Account<Clock>>, Box<dyn std::error::Error>> {
    let Contextual { context, value } = client
        .fetch_encoded_account(CLOCK_ADDRESS, ReadConfig::default())
        .await?;
    let value = value
        .require()?
        .decode(&SysvarCodec::<Clock>::NEW, OWNER_ADDRESS, Limits::new(40, 0))?;
    Ok(Contextual { context, value })
}
```

For a variable-length sysvar, give the decoder an element budget large enough for the entries you accept. For example, use `SysvarCodec::<SlotHashes>::NEW` with `SLOT_HASHES_ADDRESS` and a nonzero `max_items`.

> [!NOTE]
> `RecentBlockhashes` is kept for reading historical snapshots. For fees on new transactions, use RPC fee estimation. `Rent` is a wire codec: it keeps every float bit pattern and burn byte, but it does not check that the values are sensible.

## Status

The [account and sysvar receipt](https://github.com/pina-rs/solana_kit/tree/main/docs/verification/accounts-sysvars.md) maps 151 reference declarations. It lists the 78 unresolved ones in a separate file. The snapshot types, sysvar layouts and fetch helpers on this page are implemented. No package-wide parity is claimed.
