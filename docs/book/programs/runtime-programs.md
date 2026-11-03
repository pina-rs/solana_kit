# Stake, Loader, Config, and lookup tables

These four crates cover native runtime programs. `solana_kit_stake` handles staking, including activation and withdrawal math. `solana_kit_loader` handles program deployment with loader v3 (upgradeable) and loader v4. `solana_kit_config` handles the Config program. `solana_kit_address_lookup_table` handles lookup tables for v0 transactions. All four are `no_std + alloc`.

Their builders differ slightly from the System and Token clients. Stake and Loader builders are methods on the `…Accounts` struct, and they take the program address, the instruction data, and a slice of extra accounts explicitly. Their parsers also take the expected program address.

## Stake

### Plans for common flows

`create_stake_account_plan` combines System `CreateAccount` (with `STAKE_STATE_ACCOUNT_SIZE`, 200 bytes) and Stake `Initialize` into one indivisible instruction plan. `delegate_stake_plan` builds `DelegateStake` with the canonical Clock, StakeHistory, and `STAKE_CONFIG` accounts. Both return a core `InstructionPlan`. See [Build a transaction](../getting-started/build-a-transaction.md) for how plans are executed.

```rust
use solana_kit_core::{Address, codecs::Limits};
use solana_kit_stake::{Authorized, create_stake_account_plan, delegate_stake_plan};

fn stake_plans(
    payer: Address,
    stake: Address,
    authority: Address,
    vote: Address,
) -> Result<(), Box<dyn std::error::Error>> {
    let limits = Limits::new(1232, 64);
    let create = create_stake_account_plan(
        payer,
        stake,
        2_282_880,
        Authorized { staker: authority, withdrawer: authority },
        None, // no lockup: zero timestamp and epoch, System Program custodian
        limits,
    )?;
    let delegate = delegate_stake_plan(stake, vote, authority, limits)?;
    let _ = (create, delegate);
    Ok(())
}
```

### Single instructions

All 17 Stake instructions have a data type and an `…Accounts` struct: `Initialize`, `Authorize`, `DelegateStake`, `Split`, `Withdraw`, `Deactivate`, `SetLockup`, `Merge`, `AuthorizeWithSeed`, `InitializeChecked`, `AuthorizeChecked`, `AuthorizeCheckedWithSeed`, `SetLockupChecked`, `GetMinimumDelegation`, `DeactivateDelinquent`, `MoveStake`, and `MoveLamports`. Sysvar accounts are explicit fields, so pass the core `sysvars` constants.

```rust
use solana_kit_core::{
    Address,
    codecs::{Error, Limits},
    sysvars::{CLOCK_ADDRESS, STAKE_HISTORY_ADDRESS},
};
use solana_kit_stake::{PROGRAM, StakeInstructionData, Withdraw, WithdrawAccounts, parse_instruction};

fn withdraw(stake: Address, recipient: Address, authority: Address) -> Result<(), Error> {
    let limits = Limits::new(1232, 64);
    let instruction = WithdrawAccounts {
        stake,
        recipient,
        clock_sysvar: CLOCK_ADDRESS,
        stake_history: STAKE_HISTORY_ADDRESS,
        withdraw_authority: authority,
        lockup_authority: None,
    }
    .instruction(PROGRAM, &Withdraw { args: 1_000_000 }, &[], limits)?;

    match parse_instruction(&instruction, PROGRAM, limits)? {
        StakeInstructionData::Withdraw(data) => assert_eq!(data.args, 1_000_000),
        other => panic!("unexpected Stake instruction: {other:?}"),
    }
    Ok(())
}
```

Each data type also has `parse_instruction(&instruction, program, limits)` and `parse_with_metadata`. The second returns a `ParsedInstruction` that borrows the original instruction, so its accounts stay available. Stake parsers check the program, the data, and the account-count budget. Unlike the System and Token parsers, they do not check each account's role.

### Accounts, activation, and withdrawals

`decode_account` decodes a fetched Stake account into `Account<StakeStateAccount>` and checks the owner and allocation. Bytes after the encoded state are reserved space and are allowed.

`Delegation::activation_status_v2(epoch, &history, new_rate_activation_epoch)` computes effective, activating, and deactivating lamports from newest-first stake history, using the program's integer V2 rate rules. `assess_withdrawal(&account, signers, context)` returns `WithdrawalLimits`, which holds the largest partial withdrawal that keeps the rent reserve and, for an inactive account, the full close amount. `WithdrawalLimits::permits(lamports)` checks a requested amount against both.

<Callout kind="warning" title="Supply consistent observations">

These calculations use whatever you pass in: the account, the `Clock`, the stake history, the rent reserve, and the rate epoch. All of them must come from the same bank and match the deployed program. A matching signer identity does not prove a signature. A result does not validate the destination or guarantee that the transaction will succeed.

</Callout>

`StakeError::from_code` maps the program's custom errors and keeps unknown codes. The activation math was compared against the published `solana-stake-interface` 5.1.1 in native tests. That crate is never an SDK dependency.

## Loader

`solana_kit_loader` has two modules. `v3` is the upgradeable BPF loader: `InitializeBuffer`, `Write`, `DeployWithMaxDataLen`, `Upgrade`, `SetAuthority`, `Close`, `ExtendProgram`, and `SetAuthorityChecked`. `v4` is loader v4: `Write`, `Truncate`, `Deploy`, `Retract`, `TransferAuthority`, and `Finalize`. Each module has its own `PROGRAM` constant and `…Accounts` builders.

Deploying means writing the program bytes into a buffer in chunks and then running one final action. The plan helpers build that sequence for you. The writes come first, in increasing offset order, and the final instruction runs only after every write:

- `v3::DeployWithMaxDataLenAccounts::plan(bytes, max_data_len, chunk_size, limits)`
- `v3::UpgradeAccounts::plan(bytes, chunk_size, limits)`
- `BufferWriter { buffer, authority }.plan(bytes, chunk_size, final_instruction, limits)` for any final action

```rust
use solana_kit_core::{Address, codecs::{Error, Limits}};
use solana_kit_loader::{DEFAULT_WRITE_CHUNK_SIZE, v3};

fn upgrade_plan(
    program_bytes: &[u8],
    accounts: v3::UpgradeAccounts,
) -> Result<(), Error> {
    let plan = accounts.plan(program_bytes, DEFAULT_WRITE_CHUNK_SIZE, Limits::new(1232, 4096))?;
    let _ = plan;
    Ok(())
}
```

`max_items` must cover every write instruction plus the final one. The plans do not create the buffer account or send anything, and the buffer must already exist.

To read program accounts, use `decode_upgradeable_account`. It decodes a fetched v3 account header into `UpgradeableState` (`Uninitialized`, `Buffer`, `Program`, or `ProgramData`). The executable bytes after the header are left out of the typed value. For loader v4, `ProgramStateAccountCodec` decodes the 48-byte header into `slot`, `authority_address_or_next_version`, and `LoaderV4Status`.

## Config

The Config program has no instruction discriminator. Store data and account data share one layout: a compact key count, then each key's 32-byte address and signer flag, then opaque bytes. `ConfigCodec` encodes and decodes it as `ConfigData { keys, data }`.

```rust
use solana_kit_core::{Address, codecs::{Error, Limits}};
use solana_kit_config::{ConfigData, ConfigKey, PROGRAM, parse_store, store_config};

fn store(config: Address, signer: Address) -> Result<(), Error> {
    let limits = Limits::new(1232, 1232);
    let value = ConfigData {
        keys: vec![ConfigKey { address: signer, is_signer: true }],
        data: br#"{"name":"validator"}"#.to_vec(),
    };

    let instruction = store_config(PROGRAM, config, false, &value, limits)?;
    assert_eq!(instruction.accounts.len(), 2);
    assert_eq!(parse_store(&instruction, PROGRAM, limits)?, value);
    Ok(())
}
```

`store_config` derives the signer accounts from the keys marked `is_signer`, as the Dart helper does. Use `store` with a `StoreAccounts` struct to choose the signers yourself. `parse_store` checks the program, a writable Config account, and the signer accounts. It cannot know who is actually authorized, because that depends on the account's previous state. `decode_account` decodes a fetched Config account.

## Address Lookup Table

`Operation` is an enum with all five instructions: `Create`, `Freeze`, `Extend`, `Deactivate`, and `Close`. `Operation::build(program, limits)` returns the `Instruction`. With the `pda` feature, `find_lookup_table(authority, recent_slot, program)` derives the table address and bump.

```rust
use solana_kit_core::{Address, codecs::Limits};
use solana_kit_address_lookup_table::{Funding, InstructionData, Operation, PROGRAM, find_lookup_table, parse_instruction};

fn create_and_extend(authority: Address, payer: Address, recent_slot: u64) -> Result<(), Box<dyn std::error::Error>> {
    let limits = Limits::new(1232, 64);
    let table = find_lookup_table(authority, recent_slot, PROGRAM)?;

    let create = Operation::Create {
        table: table.address(),
        authority,
        funding: Funding { payer, system_program: Address::from_bytes([0; 32]) },
        recent_slot,
        bump: table.bump(),
    }
    .build(PROGRAM, limits)?;

    let extend = Operation::Extend {
        table: table.address(),
        authority,
        funding: None,
        addresses: vec![Address::from_bytes([9; 32])],
    }
    .build(PROGRAM, limits)?;

    assert!(matches!(parse_instruction(&create, PROGRAM, limits)?, InstructionData::Create { .. }));
    assert!(matches!(parse_instruction(&extend, PROGRAM, limits)?, InstructionData::Extend(_)));
    Ok(())
}
```

`Extend` takes either the full `Funding` pair or `None`, as the pinned Rust constructor does. Parsers allow stronger privileges than required and keep duplicate positions.

`decode_account` decodes a fetched table into `AddressLookupTable`. It rejects uninitialized tables and tables with more than `MAX_ADDRESSES` (256) entries.

- `active_addresses(current_slot, &recent_slots)` returns the addresses usable at a given bank slot. Entries added in the current slot are not yet usable. A deactivated table stays usable only while its deactivation slot is in the SlotHashes history you pass in.
- `into_lookup_table(address)` converts the table into the core `transaction::LookupTable` used when compiling v0 messages. See [Messages and transactions](../core/transactions.md).

`AddressLookupTableError` maps the program's errors.

## Status

The original public test bodies of all four crates, 88 in total, run on native Rust, Chrome, both WASI previews, and the LeptosKit interpreter. Still pending:

- Validator lifecycle tests and real network fetching.
- Publishing.

The pinned sources and known source differences are in each crate's README. For example, the lookup table header puts padding after an absent authority as the Rust program does, which differs from the Dart codec for nonzero padding.
