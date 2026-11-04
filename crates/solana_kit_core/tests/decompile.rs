use solana_kit_core::{
    AccountLookupMeta, AccountMeta, AccountRole, Address, Hash, Instruction, InstructionAccount,
    InstructionWithAccounts, LookupRole,
    transaction::{
        DurableNonce, Error, HeapSize, Lifetime, LookupTable, Message, NonceAdvance, Transaction,
        V1Config, Version,
    },
};

const PAYER: u8 = 0;
const PROGRAM: u8 = 1;
const WRITABLE_ACCOUNT: u8 = 2;
const READONLY_ACCOUNT: u8 = 3;
const EXTRA_SIGNER: u8 = 4;
const BLOCKHASH: u8 = 9;
const LAST_VALID_BLOCK_HEIGHT: u64 = 77;

const FIRST_TABLE: u8 = 20;
const SECOND_TABLE: u8 = 21;
const FIRST_LOOKUP: u8 = 10;
const SECOND_LOOKUP: u8 = 11;
const READONLY_LOOKUP: u8 = 12;
const MERGED_LOOKUP: u8 = 13;
const WRITABLE_SECOND_LOOKUP: u8 = 14;

const MANUAL_TABLE: u8 = 5;
const MANUAL_SECOND_TABLE: u8 = 6;
const MANUAL_WRITABLE: u8 = 10;
const MANUAL_READONLY: u8 = 11;
const SHARED_ADDRESS: u8 = 8;
const WIRE_PAYER: u8 = 9;
const WIRE_BLOCKHASH: u8 = 7;
const V0_VERSION: u8 = 128;
const REQUIRED_SIGNATURES: u8 = 1;
const STATIC_ACCOUNT_COUNT: u8 = 2;
const INSTRUCTION_COUNT: u8 = 1;
const PROGRAM_INDEX: u8 = 1;
const FIRST_LOADED_INDEX: u8 = 2;
const SECOND_LOADED_INDEX: u8 = 3;

const NONCE_ACCOUNT: u8 = 4;
const NONCE_AUTHORITY: u8 = 5;
const NONCE_PAYER: u8 = 6;

const PRIORITY_FEE: u64 = u64::MAX;
const COMPUTE_LIMIT: u32 = 1_400_000;
const LOADED_DATA_LIMIT: u32 = 64;
const HEAP_BYTES: u32 = 65_536;

const SNAPSHOT_LIMIT: usize = 256;
const OVERSIZED_SNAPSHOT: usize = 257;
const UNUSED_TABLE: u8 = 21;
const UNUSED_ADDRESS: u8 = 99;
const FILLER_COUNT: usize = 254;

const SIGNER_SEED: u8 = 42;

fn address(byte: u8) -> Address {
    Address::from_bytes([byte; 32])
}

fn hash(byte: u8) -> Hash {
    Hash::from_bytes([byte; 32])
}

fn lifetime() -> Lifetime {
    Lifetime::Blockhash {
        blockhash: hash(BLOCKHASH),
        last_valid_block_height: LAST_VALID_BLOCK_HEIGHT,
    }
}

fn account(byte: u8, role: AccountRole) -> AccountMeta {
    AccountMeta {
        address: address(byte),
        role,
    }
}

fn instruction(accounts: Vec<AccountMeta>, data: Vec<u8>) -> Instruction {
    Instruction {
        program_address: address(PROGRAM),
        accounts,
        data,
    }
}

fn lookup(byte: u8, table: u8, index: u8, role: LookupRole) -> InstructionAccount {
    InstructionAccount::Lookup(AccountLookupMeta {
        address: address(byte),
        lookup_table_address: address(table),
        address_index: index,
        role,
    })
}

fn static_account(byte: u8, role: AccountRole) -> InstructionAccount {
    InstructionAccount::Account(account(byte, role))
}

fn table(byte: u8, addresses: Vec<Address>) -> LookupTable {
    LookupTable {
        address: address(byte),
        addresses,
    }
}

fn decompile_instruction(instruction: &InstructionWithAccounts<InstructionAccount>) -> Instruction {
    Instruction {
        program_address: instruction.program_address,
        accounts: instruction
            .accounts
            .iter()
            .map(|account| account.account_meta())
            .collect(),
        data: instruction.data.clone(),
    }
}

/// Builds the smallest v0 wire with one program instruction and caller lookups.
fn v0_wire(accounts: &[u8], lookups: &[(u8, &[u8], &[u8])]) -> Vec<u8> {
    let mut bytes = vec![V0_VERSION, REQUIRED_SIGNATURES, 0, 0, STATIC_ACCOUNT_COUNT];
    bytes.extend([WIRE_PAYER; 32]);
    bytes.extend([PROGRAM; 32]);
    bytes.extend([WIRE_BLOCKHASH; 32]);
    bytes.extend([INSTRUCTION_COUNT, PROGRAM_INDEX]);
    push_sequence(&mut bytes, accounts);
    push_sequence(&mut bytes, &[]);

    let count = u8::try_from(lookups.len()).unwrap();
    bytes.push(count);

    for (address, writable, readonly) in lookups {
        bytes.extend([*address; 32]);
        push_sequence(&mut bytes, writable);
        push_sequence(&mut bytes, readonly);
    }

    bytes
}

fn push_sequence(bytes: &mut Vec<u8>, values: &[u8]) {
    bytes.push(u8::try_from(values.len()).unwrap());
    bytes.extend(values);
}

#[test]
fn legacy_preserves_payer_roles_duplicates_and_data() {
    let instructions = vec![
        instruction(
            vec![
                account(WRITABLE_ACCOUNT, AccountRole::Writable),
                account(WRITABLE_ACCOUNT, AccountRole::Writable),
                account(READONLY_ACCOUNT, AccountRole::Readonly),
                account(EXTRA_SIGNER, AccountRole::WritableSigner),
            ],
            vec![7, 8, 9],
        ),
        instruction(Vec::new(), Vec::new()),
    ];
    let message = Message::compile(
        Version::Legacy,
        address(PAYER),
        lifetime(),
        &instructions,
        &[],
    )
    .unwrap();
    let decompiled = message.decompile(&[]).unwrap();

    assert_eq!(decompiled.version(), Version::Legacy);
    assert_eq!(decompiled.payer(), address(PAYER));
    assert_eq!(decompiled.lifetime(), lifetime());
    assert_eq!(decompiled.config(), None);

    let rebuilt = decompiled.instructions();

    assert_eq!(rebuilt.len(), 2);
    assert_eq!(rebuilt[0].program_address, address(PROGRAM));
    assert_eq!(rebuilt[0].data, [7, 8, 9]);
    assert_eq!(
        rebuilt[0].accounts,
        [
            static_account(WRITABLE_ACCOUNT, AccountRole::Writable),
            static_account(WRITABLE_ACCOUNT, AccountRole::Writable),
            static_account(READONLY_ACCOUNT, AccountRole::Readonly),
            static_account(EXTRA_SIGNER, AccountRole::WritableSigner),
        ]
    );
    assert!(rebuilt[1].accounts.is_empty());
    assert!(rebuilt[1].data.is_empty());
    assert_eq!(decompiled.compile(&[]).unwrap().bytes(), message.bytes());
}

#[test]
fn version_zero_rebuilds_exact_lookup_provenance_and_merged_privileges() {
    let tables = vec![
        table(
            FIRST_TABLE,
            vec![
                address(FIRST_LOOKUP),
                address(SECOND_LOOKUP),
                address(READONLY_LOOKUP),
            ],
        ),
        table(
            SECOND_TABLE,
            vec![address(MERGED_LOOKUP), address(WRITABLE_SECOND_LOOKUP)],
        ),
    ];
    let instructions = vec![
        instruction(
            vec![
                account(FIRST_LOOKUP, AccountRole::Writable),
                account(READONLY_LOOKUP, AccountRole::Readonly),
                account(MERGED_LOOKUP, AccountRole::Readonly),
                account(WRITABLE_SECOND_LOOKUP, AccountRole::Writable),
                account(FIRST_LOOKUP, AccountRole::Writable),
            ],
            vec![1, 2],
        ),
        instruction(vec![account(MERGED_LOOKUP, AccountRole::Writable)], vec![3]),
    ];
    let message = Message::compile(
        Version::V0,
        address(PAYER),
        lifetime(),
        &instructions,
        &tables,
    )
    .unwrap();
    let decompiled = message.decompile(&tables).unwrap();

    assert_eq!(decompiled.version(), Version::V0);
    assert_eq!(
        decompiled.instructions()[0].accounts,
        [
            lookup(FIRST_LOOKUP, FIRST_TABLE, 0, LookupRole::Writable),
            lookup(READONLY_LOOKUP, FIRST_TABLE, 2, LookupRole::Readonly),
            // The second instruction writes this account, so compilation merged it
            // globally to writable and the narrower first role is unrecoverable.
            lookup(MERGED_LOOKUP, SECOND_TABLE, 0, LookupRole::Writable),
            lookup(
                WRITABLE_SECOND_LOOKUP,
                SECOND_TABLE,
                1,
                LookupRole::Writable
            ),
            lookup(FIRST_LOOKUP, FIRST_TABLE, 0, LookupRole::Writable),
        ]
    );
    assert_eq!(
        decompiled.instructions()[1].accounts,
        [lookup(MERGED_LOOKUP, SECOND_TABLE, 0, LookupRole::Writable)]
    );
    assert_eq!(
        decompiled.compile(&tables).unwrap().bytes(),
        message.bytes()
    );
}

#[test]
fn decoded_wire_keeps_unresolved_lifetime_until_the_caller_supplies_it() {
    let tables = vec![table(FIRST_TABLE, vec![address(FIRST_LOOKUP)])];
    let message = Message::compile(
        Version::V0,
        address(PAYER),
        lifetime(),
        &[instruction(
            vec![account(FIRST_LOOKUP, AccountRole::Readonly)],
            vec![5],
        )],
        &tables,
    )
    .unwrap();
    let decoded = Message::decode(message.bytes()).unwrap();
    let decompiled = decoded.decompile(&tables).unwrap();

    assert_eq!(
        decompiled.lifetime(),
        Lifetime::Unresolved(hash(BLOCKHASH)),
        "received expiry height is absent from the wire"
    );
    assert_eq!(decompiled.compile(&tables), Err(Error::UnresolvedLifetime));
    assert_eq!(
        decompiled
            .with_lifetime(lifetime())
            .compile(&tables)
            .unwrap()
            .bytes(),
        message.bytes()
    );
}

#[test]
fn version_one_retains_inline_configuration_and_ignores_snapshots() {
    let config = V1Config::default()
        .with_priority_fee(PRIORITY_FEE)
        .with_compute_unit_limit(COMPUTE_LIMIT)
        .with_loaded_accounts_data_limit(LOADED_DATA_LIMIT)
        .with_heap_size(HeapSize::new(HEAP_BYTES).unwrap());
    let instructions = vec![instruction(
        vec![
            account(WRITABLE_ACCOUNT, AccountRole::Writable),
            account(READONLY_ACCOUNT, AccountRole::ReadonlySigner),
        ],
        vec![4, 5],
    )];
    let message = Message::compile_v1(address(PAYER), lifetime(), &instructions, config).unwrap();
    let decompiled = message.decompile(&[]).unwrap();

    assert_eq!(decompiled.version(), Version::V1);
    assert_eq!(decompiled.config(), Some(config));
    assert_eq!(
        decompiled.instructions()[0].accounts,
        [
            static_account(WRITABLE_ACCOUNT, AccountRole::Writable),
            static_account(READONLY_ACCOUNT, AccountRole::ReadonlySigner),
        ]
    );
    assert_eq!(
        decompiled
            .compile(&[table(FIRST_TABLE, vec![address(FIRST_LOOKUP)])])
            .unwrap()
            .bytes(),
        message.bytes(),
        "version one has no lookup tables, so snapshots are unused"
    );

    let decoded = Message::decode(message.bytes()).unwrap();
    let from_wire = decoded.decompile(&[]).unwrap();

    assert_eq!(from_wire.config(), Some(config));
    assert_eq!(
        from_wire.lifetime(),
        Lifetime::Unresolved(lifetime().token())
    );
    assert_eq!(
        from_wire
            .with_lifetime(lifetime())
            .compile(&[])
            .unwrap()
            .bytes(),
        message.bytes()
    );
}

#[test]
fn durable_nonce_metadata_is_preserved_and_never_inferred_from_shape() {
    let nonce = DurableNonce::new(
        hash(WIRE_BLOCKHASH),
        address(NONCE_ACCOUNT),
        address(NONCE_AUTHORITY),
    );
    let instructions = nonce.prepare_instructions(&[instruction(
        vec![account(WRITABLE_ACCOUNT, AccountRole::Writable)],
        vec![9],
    )]);
    let message = Message::compile(
        Version::Legacy,
        address(NONCE_PAYER),
        Lifetime::DurableNonce(nonce),
        &instructions,
        &[],
    )
    .unwrap();
    let decompiled = message.decompile(&[]).unwrap();

    assert_eq!(decompiled.lifetime(), Lifetime::DurableNonce(nonce));
    assert_eq!(decompiled.compile(&[]).unwrap().bytes(), message.bytes());

    let decoded = Message::decode(message.bytes()).unwrap();
    let from_wire = decoded.decompile(&[]).unwrap();

    assert_eq!(
        from_wire.lifetime(),
        Lifetime::Unresolved(hash(WIRE_BLOCKHASH))
    );
    assert_eq!(
        NonceAdvance::recognize(&decompile_instruction(&from_wire.instructions()[0])),
        Some(nonce.advance()),
        "shape recognition is evidence, not authentication"
    );
    assert_eq!(
        from_wire
            .with_lifetime(Lifetime::DurableNonce(nonce))
            .compile(&[])
            .unwrap()
            .bytes(),
        message.bytes()
    );
}

#[test]
fn manually_ordered_lookups_load_all_writable_addresses_before_readonly() {
    let bytes = v0_wire(
        &[FIRST_LOADED_INDEX, SECOND_LOADED_INDEX],
        &[(MANUAL_SECOND_TABLE, &[], &[0]), (MANUAL_TABLE, &[0], &[])],
    );
    let message = Message::decode(&bytes).unwrap();
    let tables = vec![
        table(MANUAL_TABLE, vec![address(MANUAL_WRITABLE)]),
        table(MANUAL_SECOND_TABLE, vec![address(MANUAL_READONLY)]),
    ];
    let decompiled = message.decompile(&tables).unwrap();

    assert_eq!(
        decompiled.instructions()[0].accounts,
        [
            lookup(MANUAL_WRITABLE, MANUAL_TABLE, 0, LookupRole::Writable),
            lookup(
                MANUAL_READONLY,
                MANUAL_SECOND_TABLE,
                0,
                LookupRole::Readonly
            ),
        ],
        "writable lookups precede readonly lookups regardless of descriptor order"
    );
}

#[test]
fn duplicate_addresses_keep_distinct_table_provenance() {
    let bytes = v0_wire(
        &[FIRST_LOADED_INDEX, SECOND_LOADED_INDEX],
        &[(MANUAL_TABLE, &[0], &[1])],
    );
    let message = Message::decode(&bytes).unwrap();
    let snapshot = table(
        MANUAL_TABLE,
        vec![address(SHARED_ADDRESS), address(SHARED_ADDRESS)],
    );
    let decompiled = message.decompile(&[snapshot]).unwrap();

    assert_eq!(
        decompiled.instructions()[0].accounts,
        [
            lookup(SHARED_ADDRESS, MANUAL_TABLE, 0, LookupRole::Writable),
            lookup(SHARED_ADDRESS, MANUAL_TABLE, 1, LookupRole::Readonly),
        ]
    );
    assert_eq!(
        decompiled.with_lifetime(lifetime()).compile(&[table(
            MANUAL_TABLE,
            vec![address(SHARED_ADDRESS), address(SHARED_ADDRESS)]
        )]),
        Err(Error::InvalidLookup),
        "one address cannot keep two explicit origins in the lowered message"
    );
}

#[test]
fn missing_out_of_range_conflicting_and_oversized_snapshots_reject() {
    let tables = vec![table(
        FIRST_TABLE,
        vec![address(FIRST_LOOKUP), address(SECOND_LOOKUP)],
    )];
    let message = Message::compile(
        Version::V0,
        address(PAYER),
        lifetime(),
        &[instruction(
            vec![account(SECOND_LOOKUP, AccountRole::Readonly)],
            vec![1],
        )],
        &tables,
    )
    .unwrap();

    assert_eq!(message.decompile(&[]), Err(Error::InvalidLookup));
    assert_eq!(
        message.decompile(&[table(FIRST_TABLE, vec![address(FIRST_LOOKUP)])]),
        Err(Error::InvalidLookup)
    );
    assert_eq!(
        message.decompile(&[tables[0].clone(), tables[0].clone()]),
        Err(Error::InvalidLookup)
    );

    let oversized = table(FIRST_TABLE, vec![address(FIRST_LOOKUP); OVERSIZED_SNAPSHOT]);
    assert_eq!(message.decompile(&[oversized]), Err(Error::InvalidLookup));

    let mut extra = vec![address(FIRST_LOOKUP), address(SECOND_LOOKUP)];
    extra.extend([address(UNUSED_ADDRESS); FILLER_COUNT]);
    let unused = table(UNUSED_TABLE, extra);
    assert_eq!(unused.addresses.len(), SNAPSHOT_LIMIT);
    assert_eq!(
        message
            .decompile(&[tables[0].clone(), unused])
            .unwrap()
            .instructions()[0]
            .accounts,
        [lookup(SECOND_LOOKUP, FIRST_TABLE, 1, LookupRole::Readonly)],
        "unreferenced snapshots are caller data, not message content"
    );
}

#[test]
fn legacy_without_lookups_ignores_supplied_snapshots() {
    let message = Message::compile(
        Version::Legacy,
        address(PAYER),
        lifetime(),
        &[instruction(
            vec![account(WRITABLE_ACCOUNT, AccountRole::Readonly)],
            vec![1],
        )],
        &[],
    )
    .unwrap();
    let unrelated = [
        table(FIRST_TABLE, vec![address(FIRST_LOOKUP)]),
        table(FIRST_TABLE, vec![address(SECOND_LOOKUP)]),
    ];

    assert_eq!(
        message.decompile(&unrelated).unwrap().compile(&[]).unwrap(),
        message
    );
}

#[cfg(feature = "crypto")]
#[test]
fn decompiling_leaves_original_signed_bytes_and_slots_untouched() {
    use solana_kit_core::crypto::Keypair;

    let keypair = Keypair::from_seed(&[SIGNER_SEED; 32]);
    let instructions = vec![instruction(
        vec![
            account(PAYER, AccountRole::Readonly),
            AccountMeta {
                address: keypair.address(),
                role: AccountRole::WritableSigner,
            },
        ],
        vec![1, 2, 3],
    )];
    let message = Message::compile(
        Version::Legacy,
        keypair.address(),
        lifetime(),
        &instructions,
        &[],
    )
    .unwrap();
    let signed = Transaction::new(message.clone()).sign(&keypair).unwrap();
    let before = signed.encode(true).unwrap();
    let decompiled = signed.message().decompile(&[]).unwrap();

    assert_eq!(signed.message().bytes(), message.bytes());
    assert_eq!(signed.encode(true).unwrap(), before);
    assert_eq!(decompiled.compile(&[]).unwrap(), message);
}
