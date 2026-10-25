use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Error, Limits},
};
use solana_kit_mpl_core::*;

const LIMITS: Limits = Limits::new(8192, 2048);
const A: &[u8] = &[1, 0, 0, 0, b'A'];
const U: &[u8] = &[1, 0, 0, 0, b'U'];
const BYTE_BUDGET: usize = 8192;
const COLLECT_TAG: u8 = 19;
const CREATE_GROUP_PARTS: &[&[u8]] = &[&[39], A, U, &[1, 0, 0, 0, 3], &[7; 32]];

const CUSTOM_PROGRAM_BYTE: u8 = 91;
const DUPLICATE_REMAINING_BYTE: u8 = 81;
const DISTINCT_REMAINING_BYTE: u8 = 82;
const REMAINING_ACCOUNT_TAGS: &[u8] = &[19, 31, 33, 34, 35, 36, 37, 38, 39];

fn literal(parts: &[&[u8]]) -> Vec<u8> {
    parts.concat()
}
fn plugin() -> Plugin {
    Plugin::FreezeDelegate(FreezeDelegate { frozen: true })
}
fn proof() -> CompressionProof {
    CompressionProof {
        owner: Address::from_bytes([7; 32]),
        update_authority: UpdateAuthority::None,
        name: "A".into(),
        uri: "U".into(),
        seq: 0x0102_0304_0506_0708,
        plugins: vec![],
    }
}
fn proof_wire(tag: u8) -> Vec<u8> {
    literal(&[
        &[tag],
        &[7; 32],
        &[0],
        A,
        U,
        &[8, 7, 6, 5, 4, 3, 2, 1],
        &[0, 0, 0, 0],
    ])
}
fn init() -> ExternalPluginAdapterInitInfo {
    ExternalPluginAdapterInitInfo::AgentIdentity(AgentIdentityInitInfo {
        uri: "A".into(),
        init_plugin_authority: None,
        lifecycle_checks: vec![LifecycleCheck {
            event: HookableLifecycleEvent::Execute,
            result: ExternalCheckResult { flags: 0x0102_0304 },
        }],
    })
}

fn external_init_wire(tag: u8) -> Vec<u8> {
    const AGENT_IDENTITY_TAG: u8 = 6;
    const INITIALIZER_TAIL: &[u8] = &[0, 1, 0, 0, 0, 4, 4, 3, 2, 1];
    let header = [tag, AGENT_IDENTITY_TAG];
    literal(&[&header, A, INITIALIZER_TAIL])
}
fn update() -> ExternalPluginAdapterUpdateInfo {
    ExternalPluginAdapterUpdateInfo::AgentIdentity(AgentIdentityUpdateInfo {
        uri: Some("A".into()),
        lifecycle_checks: None,
    })
}
fn authority() -> Option<UpdateAuthority> {
    Some(UpdateAuthority::Collection(Address::from_bytes([8; 32])))
}
fn update_wire(tag: u8) -> Vec<u8> {
    literal(&[&[tag, 1], A, &[0, 1, 2], &[8; 32]])
}
fn metas(flags: &str) -> Vec<AccountMeta> {
    flags
        .split(',')
        .enumerate()
        .map(|(index, flags)| AccountMeta {
            address: Address::from_bytes([u8::try_from(index + 1).unwrap(); 32]),
            role: AccountRole::from_privileges(flags.contains('s'), flags.contains('w')),
        })
        .collect()
}

type Build<T> = fn(&T, Address, &[AccountMeta], Limits) -> Result<Instruction, Error>;
type Parse<T> = fn(Address, &Instruction, Limits) -> Result<T, Error>;

fn check<T: core::fmt::Debug + Eq, C: Encoder<T> + Decoder<T>>(
    codec: C,
    value: T,
    bytes: Vec<u8>,
    flags: &str,
    build: Build<T>,
    parse: Parse<T>,
) {
    let accounts = metas(flags);

    check_codec(&codec, &value, &bytes);
    check_instruction(&value, &bytes, &accounts, build, parse);
    check_account_privileges(&value, flags, &accounts, build, parse);
    check_custom_program(&value, flags, &accounts, build, parse);
    check_remaining_accounts(&value, &bytes, &accounts, build, parse);
}

/// Checks the independent wire fixture, round trip, truncation and byte budget.
fn check_codec<T: core::fmt::Debug + Eq, C: Encoder<T> + Decoder<T>>(
    codec: &C,
    value: &T,
    bytes: &[u8],
) {
    assert_eq!(codec.encode(value, BYTE_BUDGET).unwrap(), bytes);
    assert_eq!(codec.decode(bytes, LIMITS).unwrap(), *value);

    for offset in 0..bytes.len() {
        assert!(codec.decode(&bytes[..offset], LIMITS).is_err());
    }

    let mut trailing = bytes.to_vec();
    trailing.push(0);
    assert_eq!(codec.decode(&trailing, LIMITS), Err(Error::TrailingBytes));
    assert!(codec.encode(value, bytes.len() - 1).is_err());
}

/// Checks the complete instruction and rejects missing accounts and exhausted limits.
fn check_instruction<T: core::fmt::Debug + Eq>(
    value: &T,
    bytes: &[u8],
    accounts: &[AccountMeta],
    build: Build<T>,
    parse: Parse<T>,
) {
    let instruction = build(value, PROGRAM, accounts, LIMITS).unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(instruction.accounts, accounts);
    assert_eq!(parse(PROGRAM, &instruction, LIMITS).unwrap(), *value);
    assert!(build(value, PROGRAM, &accounts[..accounts.len() - 1], LIMITS).is_err());

    assert_eq!(
        MplCoreInstructionDataCodec
            .encode(
                &parse_instruction(&instruction, LIMITS).unwrap(),
                BYTE_BUDGET
            )
            .unwrap(),
        bytes
    );
    assert!(build(value, PROGRAM, accounts, Limits::new(BYTE_BUDGET, 0)).is_err());
}

/// Checks each privilege independently and each supported optional account sentinel.
fn check_account_privileges<T: core::fmt::Debug + Eq>(
    value: &T,
    flags: &str,
    accounts: &[AccountMeta],
    build: Build<T>,
    parse: Parse<T>,
) {
    for (index, flags) in flags.split(',').enumerate() {
        check_flipped_privileges(value, accounts, index, build);

        if flags.ends_with('?') {
            let mut absent = accounts.to_vec();
            absent[index] = AccountMeta {
                address: PROGRAM,
                role: AccountRole::Readonly,
            };
            let absent = build(value, PROGRAM, &absent, LIMITS).unwrap();
            assert_eq!(parse(PROGRAM, &absent, LIMITS).unwrap(), *value);
        }
    }
}

/// Rejects a signer flip and a writable flip without changing the other privilege.
fn check_flipped_privileges<T>(value: &T, accounts: &[AccountMeta], index: usize, build: Build<T>) {
    for flip_signer in [false, true] {
        let mut changed = accounts.to_vec();
        let role = changed[index].role;
        changed[index].role = AccountRole::from_privileges(
            role.is_signer() ^ flip_signer,
            role.is_writable() ^ !flip_signer,
        );
        assert!(build(value, PROGRAM, &changed, LIMITS).is_err());
    }
}

/// Checks alternate program identity and its optional account sentinels.
fn check_custom_program<T: core::fmt::Debug + Eq>(
    value: &T,
    flags: &str,
    accounts: &[AccountMeta],
    build: Build<T>,
    parse: Parse<T>,
) {
    let custom = Address::from_bytes([CUSTOM_PROGRAM_BYTE; 32]);
    let explicit = build(value, custom, accounts, LIMITS).unwrap();
    assert_eq!(parse(custom, &explicit, LIMITS).unwrap(), *value);
    assert!(parse(PROGRAM, &explicit, LIMITS).is_err());
    let mut absent = accounts.to_vec();

    for (index, flags) in flags.split(',').enumerate() {
        if flags.ends_with('?') {
            absent[index] = AccountMeta {
                address: custom,
                role: AccountRole::Readonly,
            };
        }
    }

    let explicit = build(value, custom, &absent, LIMITS).unwrap();
    assert_eq!(parse(custom, &explicit, LIMITS).unwrap(), *value);
}

/// Keeps remaining account order and duplicate privileges only for supporting instructions.
fn check_remaining_accounts<T: core::fmt::Debug + Eq>(
    value: &T,
    bytes: &[u8],
    accounts: &[AccountMeta],
    build: Build<T>,
    parse: Parse<T>,
) {
    let mut remaining = accounts.to_vec();
    remaining.extend([
        AccountMeta {
            address: Address::from_bytes([DUPLICATE_REMAINING_BYTE; 32]),
            role: AccountRole::Readonly,
        },
        AccountMeta {
            address: Address::from_bytes([DUPLICATE_REMAINING_BYTE; 32]),
            role: AccountRole::WritableSigner,
        },
        AccountMeta {
            address: Address::from_bytes([DISTINCT_REMAINING_BYTE; 32]),
            role: AccountRole::ReadonlySigner,
        },
    ]);

    if REMAINING_ACCOUNT_TAGS.contains(&bytes[0]) {
        let instruction = build(value, PROGRAM, &remaining, LIMITS).unwrap();
        assert_eq!(instruction.accounts, remaining);
        assert_eq!(parse(PROGRAM, &instruction, LIMITS).unwrap(), *value);
    } else {
        assert!(build(value, PROGRAM, &remaining, LIMITS).is_err());
    }
}

macro_rules! case {
    ($name:ident, $ty:ident, $codec:ident, $value:expr, $bytes:expr, $flags:expr) => {
        #[cfg_attr(all(test, not(target_arch = "wasm32")), test)]
        pub fn $name() {
            check(
                $codec,
                $value,
                $bytes,
                $flags,
                $ty::instruction_with_program,
                $ty::parse_with_program,
            );
        }
    };
}

case!(
    create_v1,
    CreateV1,
    CreateV1Codec,
    CreateV1 {
        data_state: DataState::LedgerState,
        name: "A".into(),
        uri: "U".into(),
        plugins: None
    },
    literal(&[&[0, 1], A, U, &[0]]),
    "ws,w?,s?,ws,r?,r?,r,r?"
);
case!(
    create_collection_v1,
    CreateCollectionV1,
    CreateCollectionV1Codec,
    CreateCollectionV1 {
        name: "A".into(),
        uri: "U".into(),
        plugins: None
    },
    literal(&[&[1], A, U, &[0]]),
    "ws,r?,ws,r"
);
case!(
    add_plugin_v1,
    AddPluginV1,
    AddPluginV1Codec,
    AddPluginV1 {
        plugin: plugin(),
        init_authority: None
    },
    vec![2, 1, 1, 0],
    "w,w?,ws,s?,r,r?"
);
case!(
    add_collection_plugin_v1,
    AddCollectionPluginV1,
    AddCollectionPluginV1Codec,
    AddCollectionPluginV1 {
        plugin: plugin(),
        init_authority: None
    },
    vec![3, 1, 1, 0],
    "w,ws,s?,r,r?"
);
case!(
    remove_plugin_v1,
    RemovePluginV1,
    RemovePluginV1Codec,
    RemovePluginV1 {
        plugin_type: PluginType::Groups
    },
    vec![4, 18],
    "w,w?,ws,s?,r,r?"
);
case!(
    remove_collection_plugin_v1,
    RemoveCollectionPluginV1,
    RemoveCollectionPluginV1Codec,
    RemoveCollectionPluginV1 {
        plugin_type: PluginType::Groups
    },
    vec![5, 18],
    "w,ws,s?,r,r?"
);
case!(
    update_plugin_v1,
    UpdatePluginV1,
    UpdatePluginV1Codec,
    UpdatePluginV1 { plugin: plugin() },
    vec![6, 1, 1],
    "w,w?,ws,s?,r,r?"
);
case!(
    update_collection_plugin_v1,
    UpdateCollectionPluginV1,
    UpdateCollectionPluginV1Codec,
    UpdateCollectionPluginV1 { plugin: plugin() },
    vec![7, 1, 1],
    "w,ws,s?,r,r?"
);

case!(
    approve_plugin_authority_v1,
    ApprovePluginAuthorityV1,
    ApprovePluginAuthorityV1Codec,
    ApprovePluginAuthorityV1 {
        plugin_type: PluginType::Groups,
        new_authority: Authority::Owner
    },
    vec![8, 18, 1],
    "w,w?,ws,s?,r,r?"
);
case!(
    approve_collection_plugin_authority_v1,
    ApproveCollectionPluginAuthorityV1,
    ApproveCollectionPluginAuthorityV1Codec,
    ApproveCollectionPluginAuthorityV1 {
        plugin_type: PluginType::Groups,
        new_authority: Authority::Owner
    },
    vec![9, 18, 1],
    "w,ws,s?,r,r?"
);
case!(
    revoke_plugin_authority_v1,
    RevokePluginAuthorityV1,
    RevokePluginAuthorityV1Codec,
    RevokePluginAuthorityV1 {
        plugin_type: PluginType::Groups
    },
    vec![10, 18],
    "w,w?,ws,s?,r,r?"
);
case!(
    revoke_collection_plugin_authority_v1,
    RevokeCollectionPluginAuthorityV1,
    RevokeCollectionPluginAuthorityV1Codec,
    RevokeCollectionPluginAuthorityV1 {
        plugin_type: PluginType::Groups
    },
    vec![11, 18],
    "w,ws,s?,r,r?"
);
case!(
    burn_v1,
    BurnV1,
    BurnV1Codec,
    BurnV1 {
        compression_proof: None
    },
    vec![12, 0],
    "w,w?,ws,s?,r?,r?"
);
case!(
    burn_collection_v1,
    BurnCollectionV1,
    BurnCollectionV1Codec,
    BurnCollectionV1 {
        compression_proof: None
    },
    vec![13, 0],
    "w,ws,ws?,r?"
);
case!(
    transfer_v1,
    TransferV1,
    TransferV1Codec,
    TransferV1 {
        compression_proof: None
    },
    vec![14, 0],
    "w,r?,ws,s?,r,r?,r?"
);
case!(
    update_v1,
    UpdateV1,
    UpdateV1Codec,
    UpdateV1 {
        new_name: Some("A".into()),
        new_uri: None,
        new_update_authority: authority()
    },
    update_wire(15),
    "w,r?,ws,s?,r,r?"
);

case!(
    update_collection_v1,
    UpdateCollectionV1,
    UpdateCollectionV1Codec,
    UpdateCollectionV1 {
        new_name: Some("A".into()),
        new_uri: None
    },
    literal(&[&[16, 1], A, &[0]]),
    "w,ws,s?,r?,r,r?"
);
case!(
    compress_v1,
    CompressV1,
    CompressV1Codec,
    CompressV1 {},
    vec![17],
    "w,r?,ws,s?,r,r?"
);
case!(
    decompress_v1,
    DecompressV1,
    DecompressV1Codec,
    DecompressV1 {
        compression_proof: proof()
    },
    proof_wire(18),
    "w,r?,ws,s?,r,r?"
);
case!(
    collect,
    Collect,
    CollectCodec,
    Collect {},
    vec![COLLECT_TAG],
    // Both recipient balances may be credited.
    "w,w"
);
case!(
    create_v2,
    CreateV2,
    CreateV2Codec,
    CreateV2 {
        data_state: DataState::LedgerState,
        name: "A".into(),
        uri: "U".into(),
        plugins: None,
        external_plugin_adapters: None
    },
    literal(&[&[20, 1], A, U, &[0, 0]]),
    "ws,w?,s?,ws,r?,r?,r,r?"
);
case!(
    create_collection_v2,
    CreateCollectionV2,
    CreateCollectionV2Codec,
    CreateCollectionV2 {
        name: "A".into(),
        uri: "U".into(),
        plugins: None,
        external_plugin_adapters: None
    },
    literal(&[&[21], A, U, &[0, 0]]),
    "ws,r?,ws,r"
);
case!(
    add_external_v1,
    AddExternalPluginAdapterV1,
    AddExternalPluginAdapterV1Codec,
    AddExternalPluginAdapterV1 { init_info: init() },
    external_init_wire(22),
    "w,w?,ws,s?,r,r?"
);
case!(
    add_collection_external_v1,
    AddCollectionExternalPluginAdapterV1,
    AddCollectionExternalPluginAdapterV1Codec,
    AddCollectionExternalPluginAdapterV1 { init_info: init() },
    external_init_wire(23),
    "w,ws,s?,r,r?"
);

case!(
    remove_external_v1,
    RemoveExternalPluginAdapterV1,
    RemoveExternalPluginAdapterV1Codec,
    RemoveExternalPluginAdapterV1 {
        key: ExternalPluginAdapterKey::AgentIdentity
    },
    vec![24, 6],
    "w,w?,ws,s?,r,r?"
);
case!(
    remove_collection_external_v1,
    RemoveCollectionExternalPluginAdapterV1,
    RemoveCollectionExternalPluginAdapterV1Codec,
    RemoveCollectionExternalPluginAdapterV1 {
        key: ExternalPluginAdapterKey::AgentIdentity
    },
    vec![25, 6],
    "w,ws,s?,r,r?"
);
case!(
    update_external_v1,
    UpdateExternalPluginAdapterV1,
    UpdateExternalPluginAdapterV1Codec,
    UpdateExternalPluginAdapterV1 {
        key: ExternalPluginAdapterKey::AgentIdentity,
        update_info: update()
    },
    literal(&[&[26, 6, 5, 1], A, &[0]]),
    "w,w?,ws,s?,r,r?"
);
case!(
    update_collection_external_v1,
    UpdateCollectionExternalPluginAdapterV1,
    UpdateCollectionExternalPluginAdapterV1Codec,
    UpdateCollectionExternalPluginAdapterV1 {
        key: ExternalPluginAdapterKey::AgentIdentity,
        update_info: update()
    },
    literal(&[&[27, 6, 5, 1], A, &[0]]),
    "w,ws,s?,r,r?"
);
case!(
    write_external_v1,
    WriteExternalPluginAdapterDataV1,
    WriteExternalPluginAdapterDataV1Codec,
    WriteExternalPluginAdapterDataV1 {
        key: ExternalPluginAdapterKey::AgentIdentity,
        data: Some(vec![5, 6, 7])
    },
    vec![28, 6, 1, 3, 0, 0, 0, 5, 6, 7],
    "w,w?,ws,s?,r?,r,r?"
);
case!(
    write_collection_external_v1,
    WriteCollectionExternalPluginAdapterDataV1,
    WriteCollectionExternalPluginAdapterDataV1Codec,
    WriteCollectionExternalPluginAdapterDataV1 {
        key: ExternalPluginAdapterKey::AgentIdentity,
        data: Some(vec![5, 6, 7])
    },
    vec![29, 6, 1, 3, 0, 0, 0, 5, 6, 7],
    "w,ws,s?,r?,r,r?"
);
case!(
    update_v2,
    UpdateV2,
    UpdateV2Codec,
    UpdateV2 {
        new_name: Some("A".into()),
        new_uri: None,
        new_update_authority: authority()
    },
    update_wire(30),
    "w,w?,ws,s?,w?,r,r?"
);
case!(
    execute_v1,
    ExecuteV1,
    ExecuteV1Codec,
    ExecuteV1 {
        instruction_data: vec![5, 6, 7]
    },
    vec![31, 3, 0, 0, 0, 5, 6, 7],
    "w,w?,r,ws,s?,r,r"
);

case!(
    update_collection_info_v1,
    UpdateCollectionInfoV1,
    UpdateCollectionInfoV1Codec,
    UpdateCollectionInfoV1 {
        update_type: UpdateType::Remove,
        amount: 0x0102_0304
    },
    vec![32, 2, 4, 3, 2, 1],
    "w,s"
);
case!(
    add_collections_v1,
    AddCollectionsToGroupV1,
    AddCollectionsToGroupV1Codec,
    AddCollectionsToGroupV1 {},
    vec![33],
    "w,ws,s?,r"
);
case!(
    remove_collections_v1,
    RemoveCollectionsFromGroupV1,
    RemoveCollectionsFromGroupV1Codec,
    RemoveCollectionsFromGroupV1 {
        collections: vec![Address::from_bytes([7; 32])]
    },
    literal(&[&[34, 1, 0, 0, 0], &[7; 32]]),
    "w,ws,s?,r"
);
case!(
    add_assets_v1,
    AddAssetsToGroupV1,
    AddAssetsToGroupV1Codec,
    AddAssetsToGroupV1 {},
    vec![35],
    "w,ws,s?,r"
);
case!(
    remove_assets_v1,
    RemoveAssetsFromGroupV1,
    RemoveAssetsFromGroupV1Codec,
    RemoveAssetsFromGroupV1 {
        assets: vec![Address::from_bytes([7; 32])]
    },
    literal(&[&[36, 1, 0, 0, 0], &[7; 32]]),
    "w,ws,s?,r"
);
case!(
    add_groups_v1,
    AddGroupsToGroupV1,
    AddGroupsToGroupV1Codec,
    AddGroupsToGroupV1 {
        groups: vec![Address::from_bytes([7; 32])]
    },
    literal(&[&[37, 1, 0, 0, 0], &[7; 32]]),
    "w,ws,s?,r"
);
case!(
    remove_groups_v1,
    RemoveGroupsFromGroupV1,
    RemoveGroupsFromGroupV1Codec,
    RemoveGroupsFromGroupV1 {
        groups: vec![Address::from_bytes([7; 32])]
    },
    literal(&[&[38, 1, 0, 0, 0], &[7; 32]]),
    "w,ws,s?,r"
);
case!(
    create_group_v1,
    CreateGroupV1,
    CreateGroupV1Codec,
    CreateGroupV1 {
        name: "A".into(),
        uri: "U".into(),
        relationships: vec![RelationshipEntry {
            kind: RelationshipKind::Asset,
            key: Address::from_bytes([7; 32])
        }]
    },
    literal(CREATE_GROUP_PARTS),
    "ws,s?,ws,r"
);

case!(
    close_group_v1,
    CloseGroupV1,
    CloseGroupV1Codec,
    CloseGroupV1 {},
    vec![40],
    "w,ws,s?"
);
case!(
    update_group_v1,
    UpdateGroupV1,
    UpdateGroupV1Codec,
    UpdateGroupV1 {
        new_name: Some("A".into()),
        new_uri: None
    },
    literal(&[&[41, 1], A, &[0]]),
    "w,ws,s?,r?,r"
);

#[cfg_attr(all(test, not(target_arch = "wasm32")), test)]
pub fn remaining_privileges_unknown_tags_and_optional_payloads() {
    let mut accounts = metas("w,w");
    accounts.extend(metas("r,ws"));
    let instruction = Collect {}.instruction(&accounts, LIMITS).unwrap();
    assert_eq!(Collect::parse(&instruction, LIMITS).unwrap(), Collect {});
    assert_eq!(&instruction.accounts[2..], &accounts[2..]);
    assert_eq!(
        identify_instruction(&[255]),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(identify_instruction(&[]), Err(Error::Truncated));
    assert_eq!(
        BurnV1Codec.decode(&[12, 2], LIMITS),
        Err(Error::InvalidDiscriminator)
    );

    let value = BurnV1 {
        compression_proof: Some(proof()),
    };
    let bytes = literal(&[&[12, 1], &proof_wire(18)[1..]]);
    assert_eq!(BurnV1Codec.encode(&value, 8192).unwrap(), bytes);
    assert_eq!(BurnV1Codec.decode(&bytes, LIMITS).unwrap(), value);
    assert_eq!(
        UpdateV1Codec.decode(&[15, 1, 1, 0, 0, 0, 255, 0, 0], LIMITS),
        Err(Error::InvalidUtf8)
    );
}
