# Typed parsed accounts

This slice adds an opt-in typed projection of the node's `jsonParsed` account
payloads to `solana_kit_client`. It covers every account family declared by the
pinned Dart `solana_kit_rpc_parsed_types` package at revision
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`, not just the token slice suggested by
the phase audit.

The projection never replaces the node payload. `AccountData::Parsed` keeps its
`program`, exact `parsed` `RawValue` and `space`; `ParsedAccount::decode_typed`
borrows that value and returns an owned typed family. Encoded and compressed
representations (`AccountData::Bytes`, `AccountData::Zstd`) are untouched, and no
binary fallback is invented from parsed JSON.

## Source

| Source path (pinned revision)                                                       | Rust owner                                   |
| ----------------------------------------------------------------------------------- | -------------------------------------------- |
| `packages/solana_kit_rpc_parsed_types/lib/src/rpc_parsed_type.dart`                 | `parsed::{ParsedAccountData, UnknownParsed}` |
| `packages/solana_kit_rpc_parsed_types/lib/src/token_accounts.dart`                  | `parsed::token`                              |
| `packages/solana_kit_rpc_parsed_types/lib/src/stake_accounts.dart`                  | `parsed::stake`                              |
| `packages/solana_kit_rpc_parsed_types/lib/src/vote_accounts.dart`                   | `parsed::vote`                               |
| `packages/solana_kit_rpc_parsed_types/lib/src/nonce_accounts.dart`                  | `parsed::nonce`                              |
| `packages/solana_kit_rpc_parsed_types/lib/src/address_lookup_table_accounts.dart`   | `parsed::address_lookup_table`               |
| `packages/solana_kit_rpc_parsed_types/lib/src/bpf_upgradeable_loader_accounts.dart` | `parsed::loader`                             |
| `packages/solana_kit_rpc_parsed_types/lib/src/config_accounts.dart`                 | `parsed::config`                             |
| `packages/solana_kit_rpc_parsed_types/lib/src/sysvar_accounts.dart`                 | `parsed::sysvar`                             |

The node-facing field names and example values were checked against the local
Agave `solana-account-decoder 4.3.2` sources, especially
`parse_account_data.rs` (parser names and `{ "type", "info" }` envelope),
`parse_token.rs`, `parse_stake.rs`, `parse_vote.rs`, `parse_nonce.rs`,
`parse_address_lookup_table.rs`, `parse_bpf_loader.rs`, `parse_config.rs` and
`parse_sysvar.rs`. The local validator was not executed.

A node parser name is data, not owner authentication. `spl-token` and
`spl-token-2022` select the token family, but this module never asserts that the
account's `owner` is the token program; callers keep that policy.

## Declaration mapping

All 65 top-level declarations in the package map to a Rust item. Constructors,
`operator==`, `hashCode` and `toString` members are not separate Rust items:
construction is struct literals, equality is `PartialEq` (opaque JSON members
compare by their exact text), and formatting is `Debug`. The 358 capability rows
are therefore covered at family level rather than as 358 Rust declarations.

| Dart declaration                                                   | Rust item                                               |
| ------------------------------------------------------------------ | ------------------------------------------------------- |
| `RpcParsedType`, `RpcParsedInfo`                                   | `ParsedAccountData` variants and family enums           |
| `JsonParsedTokenProgramAccount`                                    | `TokenProgramAccount`                                   |
| `JsonParsedTokenAccountVariant`                                    | `TokenProgramAccount::Account`                          |
| `JsonParsedTokenAccount`                                           | `TokenAccountInfo`                                      |
| `TokenAccountState`                                                | `TokenAccountState`                                     |
| `JsonParsedMintAccount`, `JsonParsedMintInfo`                      | `TokenProgramAccount::Mint`, `MintInfo`                 |
| `JsonParsedMultisigAccount`, `JsonParsedMultisigInfo`              | `TokenProgramAccount::Multisig`, `MultisigInfo`         |
| `JsonParsedStakeProgramAccount`                                    | `StakeProgramAccount`                                   |
| `JsonParsedDelegatedStake`                                         | `StakeProgramAccount::Delegated`                        |
| `JsonParsedInitializedStake`                                       | `StakeProgramAccount::Initialized`                      |
| `JsonParsedStakeAccountInfo`                                       | `StakeAccountInfo`                                      |
| `JsonParsedStakeMeta`                                              | `StakeMeta`                                             |
| `JsonParsedStakeAuthorized`                                        | `StakeAuthorized`                                       |
| `JsonParsedStakeLockup`                                            | `StakeLockup`                                           |
| `JsonParsedStakeData`                                              | `StakeData`                                             |
| `JsonParsedStakeDelegation`                                        | `StakeDelegation`                                       |
| `JsonParsedVoteAccount`                                            | `ParsedAccountData::Vote(VoteInfo)`                     |
| `JsonParsedVoteInfo`                                               | `VoteInfo`                                              |
| `JsonParsedAuthorizedVoter`                                        | `AuthorizedVoter`                                       |
| `JsonParsedEpochCredit`                                            | `EpochCredit`                                           |
| `JsonParsedLastTimestamp`                                          | `LastTimestamp`                                         |
| `JsonParsedPriorVoter`                                             | `PriorVoter`                                            |
| `JsonParsedVote`                                                   | `Vote`                                                  |
| `JsonParsedNonceAccount`                                           | `ParsedAccountData::Nonce(NonceInfo)`                   |
| `JsonParsedNonceInfo`                                              | `NonceInfo`                                             |
| `JsonParsedNonceFeeCalculator`                                     | `FeeCalculator`                                         |
| `JsonParsedAddressLookupTableAccount`                              | `ParsedAccountData::AddressLookupTable`                 |
| `JsonParsedAddressLookupTableInfo`                                 | `AddressLookupTableInfo`                                |
| `JsonParsedBpfUpgradeableLoaderProgramAccount`                     | `BpfUpgradeableLoaderProgramAccount`                    |
| `JsonParsedBpfProgram`                                             | `BpfUpgradeableLoaderProgramAccount::Program`           |
| `JsonParsedBpfProgramInfo`                                         | `BpfProgramInfo`                                        |
| `JsonParsedBpfProgramData`                                         | `BpfUpgradeableLoaderProgramAccount::ProgramData`       |
| `JsonParsedBpfProgramDataInfo`                                     | `BpfProgramDataInfo`                                    |
| `JsonParsedConfigProgramAccount`                                   | `ConfigProgramAccount`                                  |
| `JsonParsedStakeConfig`                                            | `ConfigProgramAccount::StakeConfig`                     |
| `JsonParsedStakeConfigInfo`                                        | `StakeConfigInfo`                                       |
| `JsonParsedValidatorInfo`                                          | `ConfigProgramAccount::ValidatorInfo`                   |
| `JsonParsedValidatorInfoData`                                      | `ValidatorInfoData`                                     |
| `JsonParsedValidatorInfoKey`                                       | `ValidatorInfoKey`                                      |
| `JsonParsedSysvarAccount`                                          | `SysvarAccount`                                         |
| `JsonParsedClockSysvar`, `JsonParsedClockInfo`                     | `SysvarAccount::Clock`, `ClockInfo`                     |
| `JsonParsedEpochScheduleSysvar`, `JsonParsedEpochScheduleInfo`     | `SysvarAccount::EpochSchedule`, `EpochScheduleInfo`     |
| `JsonParsedFeesSysvar`, `JsonParsedFeesInfo`                       | `SysvarAccount::Fees`, `FeesInfo`                       |
| `JsonParsedFeeCalculator`                                          | `FeeCalculator`                                         |
| `JsonParsedRecentBlockhashesSysvar`                                | `SysvarAccount::RecentBlockhashes`                      |
| `JsonParsedRecentBlockhashEntry`                                   | `RecentBlockhashEntry`                                  |
| `JsonParsedRentSysvar`, `JsonParsedRentInfo`                       | `SysvarAccount::Rent`, `RentInfo`                       |
| `JsonParsedSlotHashesSysvar`                                       | `SysvarAccount::SlotHashes`                             |
| `JsonParsedSlotHashEntry`                                          | `SlotHashEntry`                                         |
| `JsonParsedSlotHistorySysvar`, `JsonParsedSlotHistoryInfo`         | `SysvarAccount::SlotHistory`, `SlotHistoryInfo`         |
| `JsonParsedStakeHistorySysvar`                                     | `SysvarAccount::StakeHistory`                           |
| `JsonParsedStakeHistoryEntry`                                      | `StakeHistoryEntry`                                     |
| `JsonParsedStakeHistoryData`                                       | `StakeHistoryData`                                      |
| `JsonParsedLastRestartSlotSysvar`, `JsonParsedLastRestartSlotInfo` | `SysvarAccount::LastRestartSlot`, `LastRestartSlotInfo` |
| `JsonParsedEpochRewardsSysvar`, `JsonParsedEpochRewardsInfo`       | `SysvarAccount::EpochRewards`, `EpochRewardsInfo`       |

## Families and variant discrimination

| Node `program`           | Declared node `type` values                                                                                                                   |
| ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------- |
| `spl-token`              | `account`, `mint`, `multisig`                                                                                                                 |
| `spl-token-2022`         | `account`, `mint`, `multisig` (extensions included)                                                                                           |
| `stake`                  | `delegated`, `initialized`                                                                                                                    |
| `vote`                   | `vote`                                                                                                                                        |
| `nonce`                  | `initialized`                                                                                                                                 |
| `address-lookup-table`   | `lookupTable`                                                                                                                                 |
| `bpf-upgradeable-loader` | `program`, `programData`                                                                                                                      |
| `config`                 | `validatorInfo`, legacy `stakeConfig`                                                                                                         |
| `sysvar`                 | `clock`, `epochRewards`, `epochSchedule`, `fees`, `lastRestartSlot`, `recentBlockhashes`, `rent`, `slotHashes`, `slotHistory`, `stakeHistory` |

`vote`, `nonce` and `address-lookup-table` expose the info payload directly,
matching the pinned `RpcParsedInfo` typedefs; their wire discriminator is still
validated.

## Typing policy and differences

- **Exact integers.** Decimal-text amounts are parsed as `u64` with an
  exact-value check; no field is parsed through `f64`. Slot, epoch, credits,
  commission and basis-point fields use `u64`/`u32`/`u16`/`u8` exactly as the
  wire reports them. `UnixTimestamp` members use `i64`. A decimal string outside
  `u64`, a JSON number where the source declares text, or a fractional number
  where the source declares an integer is rejected.
- **Stringified integers are `u64`.** The reference `StringifiedBigInt` is
  arbitrary precision. Every node field that carries it is an on-chain
  `u64`-width value (amounts, supply, stake, credits, slots, reward totals), so
  the Rust projection keeps the exact value and rejects out-of-range text rather
  than importing a big-integer dependency. This is a deliberate width contract,
  not a lossy conversion.
- **Token display amount.** The reference `TokenAmount` has `amount`,
  `decimals` and `uiAmountString` only, so the node's approximate `uiAmount`
  number is not parsed at all. Token-2022 `extensions` remain their exact JSON
  array because the source declares `List<Object?>`.
- **Opaque config data.** `ValidatorInfoData.configData` is the exact
  `RawValue`, including `null`, matching `Object?`.
- **Legacy shapes remain.** `stakeConfig`, `StakeDelegation.warmupCooldownRate`
  and the pre-4.1 rent members (`burnPercent`, `exemptionThreshold`,
  `lamportsPerByteYear`) are optional because Agave 4.1.0 and later no longer
  emit them but older validators still do.
- **`epochRewards` mismatch.** The pinned model requires
  `distributionCompleteBlockHeight`. The local Agave 4.3.2 decoder reports a
  different `EpochRewards` shape without that member
  (`distributionStartingBlockHeight`, `numPartitions`, `parentBlockhash`,
  `totalPoints`, `active`). Rust therefore represents the source member with
  `Presence<u64>`: absent stays `Missing`, an explicit `null` is rejected, and a
  present value is `Present`. Unmodeled Agave members are not dropped from the
  original raw payload.
- **Unknown values stay explicit.** A program outside the table becomes
  `ParsedAccountData::UnknownProgram`. A declared program with an undeclared
  discriminator becomes `UnknownType`; this includes the node's `uninitialized`
  variants for nonce, lookup table and loader, the loader `buffer` variant and
  the `rewards` sysvar, none of which the pinned model declares. Both variants
  carry the parser name, the string discriminator when present, and the
  complete original `parsed` value. Unknown discriminators need not carry the
  known variants' `info` member; known variants still require valid info.
- **Changed Rust names.** `JsonParsed*` prefixes are dropped inside
  `solana_kit_client::parsed`, and the two source fee-calculator classes share
  one `FeeCalculator`, because their shapes are identical.

## Admission and errors

`ParsedAccount::decode_typed(&self, limits)` and `parsed::decode_parts(program,
parsed, limits)` apply, in order:

1. the parser-name byte bound and existing payload byte/nesting check (each
   input is bounded by `Limits::bytes` before retaining it);
2. the existing duplicate-object-key rejection, including inside unknown
   extension members;
3. the typed family projection.

Failures reuse existing error variants: `Error::BodyLimit`, `Error::DepthLimit`
and `Error::Schema`. Errors never echo the payload. `RawValue` guarantees JSON
syntax before this boundary, so "malformed" here means a valid JSON value with
the wrong shape, discriminator, address, integer width or required member.

## Feature and dependency contract

`parsed-accounts` is opt-in and adds no dependency:

```toml
[features]
parsed-accounts = []
```

The default dependency graph is unchanged. The existing
`fetch_json_parsed_account`, `fetch_json_parsed_accounts`,
`parse_json_rpc_account` and `assert_account_decoded` helpers keep their exact
behaviour; projection is an additional method on `ParsedAccount`.

## Evidence

Executed locally on this worktree with `CARGO_INCREMENTAL=0`,
`CARGO_BUILD_JOBS=4`, `--locked --offline`:

- `cargo +1.99.1 test -p solana_kit_client --all-features` — complete client
  suite, including the four new `parsed_accounts*` targets.
- `cargo +1.99.1 clippy -p solana_kit_client --all-targets --all-features --
  -D warnings`.
- `RUSTDOCFLAGS="-D warnings" cargo +1.99.1 doc -p solana_kit_client
  --all-features --no-deps`.
- `cargo +1.89.0 test -p solana_kit_client --all-features --test parsed_accounts
  --test parsed_accounts_sysvar --test parsed_accounts_unknown --test
  parsed_accounts_invalid`.
- `cargo +1.99.1 check -p solana_kit_client --all-features` for
  `wasm32-unknown-unknown`, `wasm32-wasip1` and `wasm32-wasip2`.
- `cargo +1.99.1 fmt -p solana_kit_client --check`, `dprint check`,
  `monochange check`.

The literal fixtures are independently authored for this slice: they use the
pinned model's field names and its published example addresses and values, plus
full-width `u64`/`i64` boundaries. They exercise every supported variant,
required-versus-optional members, explicit nulls, unknown programs, unknown
discriminators, unknown extensions, invalid addresses, integer overflow,
duplicate keys, nesting and byte budgets, and the untouched binary fallback.

## Limitations

- No node, validator or SurfPool execution was performed for this slice, so the
  literals are source-grounded but not live-RPC capture evidence.
- WebAssembly targets were compiled, not executed.
- The `epochRewards` difference above is a source-versus-Agave semantic
  decision, resolved in favour of keeping the source member representable and
  the raw payload intact.
- This document claims the parsed-type family coverage above only. It does not
  claim whole-SDK or phase completion, and it changes no canonical capability
  status, ID, hash, checker or threshold.
