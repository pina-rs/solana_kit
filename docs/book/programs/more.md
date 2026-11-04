# More program clients

This page covers four more clients. `solana_kit_squads` handles Squads V4 multisigs. `solana_kit_sns` provides offline Solana Name Service helpers. `solana_kit_subscriptions` handles the recurring-payments Subscriptions program, and `solana_kit_attestation_service` handles the Solana Attestation Service. All four are `no_std + alloc` and use Kit-owned addresses, instructions, and account snapshots. None of them does any RPC.

## Squads V4

The crate models 36 instructions, nine accounts, eight PDA recipes, and 45 errors (`SquadsMultisigError`). Each instruction value has `instruction`, `instruction_with_program`, `parse`, and `parse_with_program`, and takes an ordered `AccountMeta` slice. To leave out an optional account, pass the program address with a readonly role. Builders never add a signer. Execution instructions keep any extra accounts you pass, with their privileges as given.

```rust
use solana_kit_core::{AccountMeta, AccountRole, Address, codecs::{Error, Limits}};
use solana_kit_squads::{PROGRAM, ProgramConfigSetAuthority};

fn set_authority(config: Address, authority: Address, new_authority: Address) -> Result<(), Error> {
    let value = ProgramConfigSetAuthority { new_authority };
    let accounts = [
        AccountMeta { address: config, role: AccountRole::Writable },
        AccountMeta { address: authority, role: AccountRole::ReadonlySigner },
    ];
    let limits = Limits::new(4096, 256);

    let instruction = value.instruction(&accounts, limits)?;
    assert_eq!(instruction.program_address, PROGRAM);
    assert_eq!(ProgramConfigSetAuthority::parse(&instruction, limits)?, value);
    Ok(())
}
```

Account decoders validate the full eight-byte discriminator and payload, and keep the core account metadata:

- `decode_multisig`, `decode_program_config`, `decode_proposal`, `decode_spending_limit`
- `decode_vault_transaction`, `decode_config_transaction`, `decode_batch`, `decode_vault_batch_transaction`, `decode_transaction_buffer`

`identify_account` reads only the header. `validate_vault_message` checks a stored vault message's header partitions, indexes, and size bounds. It does not check lookup tables or bank state.

Enable `pda` for `find_program_config`, `find_multisig`, `find_vault`, `find_transaction`, `find_proposal`, `find_batch_transaction`, `find_ephemeral_signer`, and `find_spending_limit`. Each has a `_with_program` variant. Vault and signer indexes are `u8`. Transaction and batch indexes are little-endian `u64`.

```rust
use solana_kit_core::{Address, crypto::PdaError};
use solana_kit_squads::{find_multisig, find_vault};

fn first_vault(create_key: Address) -> Result<Address, PdaError> {
    let multisig = find_multisig(create_key)?.address();
    Ok(find_vault(multisig, 0)?.address())
}
```

## SNS

`solana_kit_sns` covers the offline part of the Solana Name Service. It derives domain, subdomain, record, and reverse-lookup addresses, and it parses registry and record data. It has no instruction builders and no RPC. You fetch accounts yourself and pass in the bytes. The crate always enables core's `crypto` feature, because every derivation needs it.

Domain names are given without the `.sol` suffix. Passing `.sol` or `.sns` is an error.

```rust
use solana_kit_sns::{DomainError, SnsRecord, find_domain_key, find_record_v2_address, find_reverse_address_for_domain};

fn addresses() -> Result<(), DomainError> {
    let domain = find_domain_key("bonfida", None)?;
    assert_eq!(domain.address.to_string(), "Crf8hzfthWGbGbLTVCiqRqV5MVnbpHB1L9KQMd6gsinb");

    let sub = find_domain_key("dex.bonfida", None)?;
    assert_eq!(sub.parent_address, Some(domain.address));

    let _url_record = find_record_v2_address("bonfida", SnsRecord::Url)?;
    let _reverse = find_reverse_address_for_domain("bonfida")?;
    Ok(())
}
```

For data:

- `NameRegistryState::decode(bytes)` reads the 96-byte header (`parent_name`, `owner`, `registry_class`) and keeps the rest as `data`.
- `decode_name_value` and `decode_reverse_value` read the stored strings.
- `SnsRecordV2::deserialize(account)` gives bounded slices for `staleness_id`, `roa_id`, and `content`.
- `decode_record_content` and `encode_record_content` convert a record's content for its `SnsRecord` kind. All 28 record labels are covered.

<Callout kind="caution" title="Validation IDs are just bytes">

A V2 record's staleness and right-of-association IDs are returned as raw bytes. Having them does not prove ownership or association. Apply your own validation policy.

</Callout>

## Subscriptions

This is the recurring-payments Subscriptions program. It is not the same thing as RPC subscriptions, which are covered in [Subscriptions](../core/subscriptions.md). The crate covers all 18 instructions, the account and event codecs, and 78 errors (`SubscriptionsError`). The instructions are:

- Authorities: `InitSubscriptionAuthority`, `CloseSubscriptionAuthority`, `RevokeSubscriptionAuthority`
- Delegations: `CreateFixedDelegation`, `CreateRecurringDelegation`, `RevokeDelegation`, `TransferFixed`, `TransferRecurring`, `RevokeAbandonedDelegation`
- Plans: `CreatePlan`, `UpdatePlan`, `DeletePlan`
- Subscriptions: `Subscribe`, `TransferSubscription`, `CancelSubscription`, `CancelSubscriptionNow`, `ResumeSubscription`, `RevokeAbandonedSubscription`

Without features, each instruction value takes an `AccountMeta` slice, as Squads does. With `pda`, you also get the six PDA recipes (`find_subscription_authority`, `find_fixed_delegation`, `find_recurring_delegation`, `find_plan`, `find_subscription_delegation`, `find_event_authority`) and a typed `…Accounts` struct for every instruction. In those structs, `None` selects the declared default (a PDA or a fixed program address), and any address you pass overrides it.

```rust
use solana_kit_core::{Address, codecs::Limits};
use solana_kit_subscriptions::{
    DefaultError, InitSubscriptionAuthority, InitSubscriptionAuthorityAccounts, find_subscription_authority,
};

fn init_authority(owner: Address, mint: Address, owner_ata: Address, token_program: Address) -> Result<(), DefaultError> {
    let accounts = InitSubscriptionAuthorityAccounts {
        owner,
        subscription_authority: None, // derived from owner and mint
        token_mint: mint,
        user_ata: owner_ata,
        system_program: None,         // fixed System Program address
        token_program,
        payer: None,                  // optional trailing account, left out
    };
    let instruction = accounts.instruction(&InitSubscriptionAuthority {}, Limits::new(1232, 32))?;
    assert_eq!(instruction.accounts[1].address, find_subscription_authority(owner, mint)?.address());
    Ok(())
}
```

`instruction_with_program` derives every PDA against the program you pass. Decoders such as `decode_plan`, `decode_fixed_delegation`, `decode_recurring_delegation`, `decode_subscription_authority`, and `decode_subscription_delegation` check the account discriminator. For a fetched `MaybeEncodedAccount`, call `require()` first to turn a missing account into an error. The schedule helpers `is_expired`, `validate_fixed_transfer`, `RecurringAllowance::transfer`, `period_seconds`, `validate_plan_end`, and `check_plan_terms` take explicit timestamps. They never read a clock.

Fixed UTF-8 fields are exactly 128 bytes and keep their NUL padding when decoded.

## Attestation Service

The crate models the 12 instructions, three accounts (credential, schema, attestation), the close event, 26 schema data types, and 12 errors (`AttestationServiceError`). Builders take ordered `AccountMeta` slices and enforce the full role policy. With `pda`, you get `find_credential`, `find_schema`, `find_attestation`, `find_schema_mint`, `find_attestation_mint`, `find_event_authority`, and `find_sas_authority`.

```rust
use solana_kit_core::{Address, crypto::PdaError};
use solana_kit_attestation_service::{find_credential, find_schema};

fn schema_address(authority: Address) -> Result<Address, PdaError> {
    let credential = find_credential(authority, "issuer")?.address();
    Ok(find_schema(credential, "counter", 1)?.address())
}
```

Attestation `data` stays as opaque bytes by default. To encode against a schema, build an `AttestationDataCodec::from_schema(&schema, limits)`. It checks that the values you pass have exactly the schema's field names and types. `FieldValue` keeps exact integer widths.

```rust
use solana_kit_attestation_service::{AttestationDataCodec, AttestationField, FieldValue, Schema};
use solana_kit_core::{Address, codecs::{Encoder, Error, Limits}};

fn encode_count() -> Result<(), Error> {
    let schema = Schema {
        credential: Address::from_bytes([7; 32]),
        name: b"counter".to_vec(),
        description: Vec::new(),
        layout: vec![0],
        field_names: vec![5, 0, 0, 0, b'c', b'o', b'u', b'n', b't'],
        is_paused: false,
        version: 1,
    };
    let codec = AttestationDataCodec::from_schema(&schema, Limits::new(1024, 64))?;
    let values = [AttestationField { name: "count".into(), value: FieldValue::U8(9) }];
    assert_eq!(codec.encode(&values, 1024)?, [9]);
    Ok(())
}
```

`decode_credential`, `decode_schema`, and `decode_attestation` decode fetched accounts and keep the core metadata. There is no separate revoke instruction: closing the attestation is how you revoke it. When attestation string fields are displayed, malformed UTF-8 is shown as `0x` hex text, which cannot be converted back to the original bytes. Keep the opaque bytes if you need a lossless copy.

## Status

The Squads, Subscriptions, and Attestation registries (83 original behavior bodies) run on native Rust, Chrome, both WASI previews, and the LeptosKit interpreter. The SNS receipt records native execution only. Its browser, WASI, and interpreter runs still need fresh evidence.

For all four crates, these are still pending:

- Validator transitions and installed-app journeys.
- Full per-declaration parity review.

Publishing is disabled. See [`docs/verification/remaining-program-clients.md`](https://github.com/pina-rs/solana_kit/tree/main/docs/verification/remaining-program-clients.md) and [`docs/verification/sns-client.md`](https://github.com/pina-rs/solana_kit/tree/main/docs/verification/sns-client.md).
