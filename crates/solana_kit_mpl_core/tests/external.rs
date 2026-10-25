use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Limits},
};
use solana_kit_mpl_core::*;
const LIMITS: Limits = Limits::new(8192, 2048);
const DATA_SECTION_TAG: u8 = 5;
const AGENT_IDENTITY_TAG: u8 = 6;
const UPDATE_AGENT_IDENTITY_TAG: u8 = 5;
const FIRST_UNKNOWN_ADAPTER_TAG: u8 = 7;
const FIRST_UNKNOWN_UPDATE_TAG: u8 = 6;
fn check<T: core::fmt::Debug + Eq, C: Encoder<T> + Decoder<T>>(codec: C, value: T, bytes: Vec<u8>) {
    assert_eq!(codec.encode(&value, 8192).unwrap(), bytes);
    assert_eq!(codec.decode(&bytes, LIMITS).unwrap(), value);

    for offset in 0..bytes.len() {
        assert!(codec.decode(&bytes[..offset], LIMITS).is_err());
    }
}
fn lifecycle() -> Vec<LifecycleCheck> {
    vec![LifecycleCheck {
        event: HookableLifecycleEvent::Execute,
        result: ExternalCheckResult { flags: 0x0102_0304 },
    }]
}

#[cfg_attr(all(test, not(target_arch = "wasm32")), test)]
pub fn every_external_plugin_init_update_and_key_variant_has_a_literal_contract() {
    external_values();
    hook_initializers();
    other_initializers();
    external_updates();
    external_keys();
}

fn external_values() {
    let key = Address::from_bytes([7; 32]);
    let bytes = vec![7; 32];
    check(
        ExternalPluginAdapterCodec,
        ExternalPluginAdapter::LifecycleHook(LifecycleHook {
            hooked_program: key,
            extra_accounts: None,
            data_authority: None,
            schema: ExternalPluginAdapterSchema::Json,
        }),
        [vec![0], bytes.clone(), vec![0, 0, 1]].concat(),
    );
    check(
        ExternalPluginAdapterCodec,
        ExternalPluginAdapter::Oracle(Oracle {
            base_address: key,
            base_address_config: None,
            results_offset: ValidationResultsOffset::Custom(0x0102_0304_0506_0708),
        }),
        [vec![1], bytes.clone(), vec![0, 2, 8, 7, 6, 5, 4, 3, 2, 1]].concat(),
    );
    check(
        ExternalPluginAdapterCodec,
        ExternalPluginAdapter::AppData(AppData {
            data_authority: Authority::Owner,
            schema: ExternalPluginAdapterSchema::Json,
        }),
        vec![2, 1, 1],
    );
    data_and_linked_adapters(key, bytes);
}

fn data_and_linked_adapters(key: Address, bytes: Vec<u8>) {
    check(
        ExternalPluginAdapterCodec,
        ExternalPluginAdapter::LinkedLifecycleHook(LinkedLifecycleHook {
            hooked_program: key,
            extra_accounts: None,
            data_authority: None,
            schema: ExternalPluginAdapterSchema::MsgPack,
        }),
        [vec![3], bytes.clone(), vec![0, 0, 2]].concat(),
    );
    check(
        ExternalPluginAdapterCodec,
        ExternalPluginAdapter::LinkedAppData(LinkedAppData {
            data_authority: Authority::Owner,
            schema: ExternalPluginAdapterSchema::Binary,
        }),
        vec![4, 1, 0],
    );
    check(
        ExternalPluginAdapterCodec,
        ExternalPluginAdapter::DataSection(DataSection {
            parent_key: LinkedDataKey::LinkedLifecycleHook(key),
            schema: ExternalPluginAdapterSchema::Json,
        }),
        [vec![DATA_SECTION_TAG, 0], bytes.clone(), vec![1]].concat(),
    );

    check(
        ExternalPluginAdapterCodec,
        ExternalPluginAdapter::AgentIdentity(AgentIdentity { uri: "A".into() }),
        vec![6, 1, 0, 0, 0, b'A'],
    );
}

fn hook_initializers() {
    let key = Address::from_bytes([7; 32]);
    let bytes = vec![7; 32];
    for (tag, value) in [
        (
            0,
            ExternalPluginAdapterInitInfo::LifecycleHook(LifecycleHookInitInfo {
                hooked_program: key,
                init_plugin_authority: None,
                lifecycle_checks: lifecycle(),
                extra_accounts: None,
                data_authority: None,
                schema: None,
            }),
        ),
        (
            3,
            ExternalPluginAdapterInitInfo::LinkedLifecycleHook(LinkedLifecycleHookInitInfo {
                hooked_program: key,
                init_plugin_authority: None,
                lifecycle_checks: lifecycle(),
                extra_accounts: None,
                data_authority: None,
                schema: None,
            }),
        ),
    ] {
        check(
            ExternalPluginAdapterInitInfoCodec,
            value,
            [
                vec![tag],
                bytes.clone(),
                vec![0, 1, 0, 0, 0, 4, 4, 3, 2, 1, 0, 0, 0],
            ]
            .concat(),
        );
    }
}

fn other_initializers() {
    let key = Address::from_bytes([7; 32]);
    let bytes = vec![7; 32];
    check(
        ExternalPluginAdapterInitInfoCodec,
        ExternalPluginAdapterInitInfo::Oracle(OracleInitInfo {
            base_address: key,
            init_plugin_authority: None,
            lifecycle_checks: lifecycle(),
            base_address_config: None,
            results_offset: None,
        }),
        [
            vec![1],
            bytes.clone(),
            vec![0, 1, 0, 0, 0, 4, 4, 3, 2, 1, 0, 0],
        ]
        .concat(),
    );
    check(
        ExternalPluginAdapterInitInfoCodec,
        ExternalPluginAdapterInitInfo::AppData(AppDataInitInfo {
            data_authority: Authority::Owner,
            init_plugin_authority: None,
            schema: None,
        }),
        vec![2, 1, 0, 0],
    );
    check(
        ExternalPluginAdapterInitInfoCodec,
        ExternalPluginAdapterInitInfo::LinkedAppData(LinkedAppDataInitInfo {
            data_authority: Authority::Owner,
            init_plugin_authority: None,
            schema: None,
        }),
        vec![4, 1, 0, 0],
    );
    data_initializers();
}

fn data_initializers() {
    check(
        ExternalPluginAdapterInitInfoCodec,
        ExternalPluginAdapterInitInfo::DataSection(DataSectionInitInfo {
            parent_key: LinkedDataKey::LinkedAppData(Authority::Owner),
            schema: ExternalPluginAdapterSchema::Json,
        }),
        vec![5, 1, 1, 1],
    );
    check(
        ExternalPluginAdapterInitInfoCodec,
        ExternalPluginAdapterInitInfo::AgentIdentity(AgentIdentityInitInfo {
            uri: "A".into(),
            init_plugin_authority: None,
            lifecycle_checks: lifecycle(),
        }),
        vec![6, 1, 0, 0, 0, b'A', 0, 1, 0, 0, 0, 4, 4, 3, 2, 1],
    );
}

fn external_updates() {
    check(
        ExternalPluginAdapterUpdateInfoCodec,
        ExternalPluginAdapterUpdateInfo::LifecycleHook(LifecycleHookUpdateInfo {
            lifecycle_checks: None,
            extra_accounts: Some(vec![]),
            schema: Some(ExternalPluginAdapterSchema::Json),
        }),
        vec![0, 0, 1, 0, 0, 0, 0, 1, 1],
    );
    check(
        ExternalPluginAdapterUpdateInfoCodec,
        ExternalPluginAdapterUpdateInfo::Oracle(OracleUpdateInfo {
            lifecycle_checks: Some(lifecycle()),
            base_address_config: None,
            results_offset: Some(ValidationResultsOffset::Anchor),
        }),
        vec![1, 1, 1, 0, 0, 0, 4, 4, 3, 2, 1, 0, 1, 1],
    );
    check(
        ExternalPluginAdapterUpdateInfoCodec,
        ExternalPluginAdapterUpdateInfo::AppData(AppDataUpdateInfo {
            schema: Some(ExternalPluginAdapterSchema::Json),
        }),
        vec![2, 1, 1],
    );
    check(
        ExternalPluginAdapterUpdateInfoCodec,
        ExternalPluginAdapterUpdateInfo::LinkedLifecycleHook(LinkedLifecycleHookUpdateInfo {
            lifecycle_checks: None,
            extra_accounts: None,
            schema: None,
        }),
        vec![3, 0, 0, 0],
    );
    check(
        ExternalPluginAdapterUpdateInfoCodec,
        ExternalPluginAdapterUpdateInfo::LinkedAppData(LinkedAppDataUpdateInfo { schema: None }),
        vec![4, 0],
    );
    check(
        ExternalPluginAdapterUpdateInfoCodec,
        ExternalPluginAdapterUpdateInfo::AgentIdentity(AgentIdentityUpdateInfo {
            uri: Some("A".into()),
            lifecycle_checks: None,
        }),
        vec![UPDATE_AGENT_IDENTITY_TAG, 1, 1, 0, 0, 0, b'A', 0],
    );
}

fn external_keys() {
    let key = Address::from_bytes([7; 32]);
    let bytes = vec![7; 32];
    for (value, wire) in [
        (
            ExternalPluginAdapterKey::LifecycleHook(key),
            [vec![0], bytes.clone()].concat(),
        ),
        (
            ExternalPluginAdapterKey::Oracle(key),
            [vec![1], bytes.clone()].concat(),
        ),
        (
            ExternalPluginAdapterKey::AppData(Authority::Owner),
            vec![2, 1],
        ),
        (
            ExternalPluginAdapterKey::LinkedLifecycleHook(key),
            [vec![3], bytes.clone()].concat(),
        ),
        (
            ExternalPluginAdapterKey::LinkedAppData(Authority::Owner),
            vec![4, 1],
        ),
        (
            ExternalPluginAdapterKey::DataSection(LinkedDataKey::LinkedLifecycleHook(key)),
            [vec![DATA_SECTION_TAG, 0], bytes].concat(),
        ),
        (
            ExternalPluginAdapterKey::AgentIdentity,
            vec![AGENT_IDENTITY_TAG],
        ),
    ] {
        check(ExternalPluginAdapterKeyCodec, value, wire);
    }

    assert_eq!(
        ExternalPluginAdapterCodec.decode(&[FIRST_UNKNOWN_ADAPTER_TAG], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(
        ExternalPluginAdapterUpdateInfoCodec.decode(&[FIRST_UNKNOWN_UPDATE_TAG], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
}

#[cfg_attr(all(test, not(target_arch = "wasm32")), test)]
pub fn every_extra_account_variant_and_optional_plugin_list_has_literal_wire() {
    preconfigured_extra_accounts();
    custom_extra_accounts_and_plugin_list();
}

fn preconfigured_extra_accounts() {
    let flags = vec![1, 0];
    check(
        ExtraAccountCodec,
        ExtraAccount::PreconfiguredProgram(ExtraAccountPreconfiguredProgramPayload {
            is_signer: true,
            is_writable: false,
        }),
        [vec![0], flags.clone()].concat(),
    );
    check(
        ExtraAccountCodec,
        ExtraAccount::PreconfiguredCollection(ExtraAccountPreconfiguredCollectionPayload {
            is_signer: true,
            is_writable: false,
        }),
        [vec![1], flags.clone()].concat(),
    );
    check(
        ExtraAccountCodec,
        ExtraAccount::PreconfiguredOwner(ExtraAccountPreconfiguredOwnerPayload {
            is_signer: true,
            is_writable: false,
        }),
        [vec![2], flags.clone()].concat(),
    );
    check(
        ExtraAccountCodec,
        ExtraAccount::PreconfiguredRecipient(ExtraAccountPreconfiguredRecipientPayload {
            is_signer: true,
            is_writable: false,
        }),
        [vec![3], flags.clone()].concat(),
    );
    check(
        ExtraAccountCodec,
        ExtraAccount::PreconfiguredAsset(ExtraAccountPreconfiguredAssetPayload {
            is_signer: true,
            is_writable: false,
        }),
        [vec![4], flags.clone()].concat(),
    );
}

fn custom_extra_accounts_and_plugin_list() {
    const CUSTOM_SEED_BYTES: &[u8] = &[9, 8, 7];
    let flags = vec![1, 0];
    check(
        ExtraAccountCodec,
        ExtraAccount::CustomPda(ExtraAccountCustomPdaPayload {
            seeds: vec![Seed::Bytes(CUSTOM_SEED_BYTES.to_vec())],
            custom_program_id: Some(Address::from_bytes([7; 32])),
            is_signer: true,
            is_writable: false,
        }),
        [
            vec![5, 1, 0, 0, 0, 5, 3, 0, 0, 0, 9, 8, 7, 1],
            vec![7; 32],
            flags.clone(),
        ]
        .concat(),
    );
    check(
        ExtraAccountCodec,
        ExtraAccount::Address(ExtraAccountAddressPayload {
            address: Address::from_bytes([7; 32]),
            is_signer: true,
            is_writable: false,
        }),
        [vec![6], vec![7; 32], flags].concat(),
    );
    optional_plugin_lists();
}

fn optional_plugin_lists() {
    let value = CreateV2 {
        data_state: DataState::AccountState,
        name: "A".into(),
        uri: "U".into(),
        plugins: Some(vec![PluginAuthorityPair {
            plugin: Plugin::FreezeDelegate(FreezeDelegate { frozen: true }),
            authority: Some(Authority::Owner),
        }]),
        external_plugin_adapters: Some(vec![]),
    };
    check(
        CreateV2Codec,
        value,
        vec![
            20, 0, 1, 0, 0, 0, b'A', 1, 0, 0, 0, b'U', 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0,
        ],
    );
    assert_eq!(
        ExtraAccountCodec.decode(&[0, 2, 0], LIMITS),
        Err(Error::InvalidBoolean { value: 2 })
    );
    assert!(
        CreateV2Codec
            .decode(&[20, 0, 255, 255, 255, 255], LIMITS)
            .is_err()
    );
}

macro_rules! units {
        ($codec:ident,[$($value:expr),+ $(,)?])=>{{
            let values=[$($value),+];

            for (tag,value) in values.into_iter().enumerate() {
                check($codec,value,vec![u8::try_from(tag).unwrap()]);
            }

            assert_eq!($codec.decode(&[255],LIMITS),Err(Error::InvalidDiscriminator));
        }};
}

#[cfg_attr(all(test, not(target_arch = "wasm32")), test)]
pub fn all_unit_enum_values_are_literal_and_unknown_values_reject() {
    units!(
        DataStateCodec,
        [DataState::AccountState, DataState::LedgerState]
    );
    units!(
        ExternalPluginAdapterSchemaCodec,
        [
            ExternalPluginAdapterSchema::Binary,
            ExternalPluginAdapterSchema::Json,
            ExternalPluginAdapterSchema::MsgPack
        ]
    );
    units!(
        ExternalPluginAdapterTypeCodec,
        [
            ExternalPluginAdapterType::LifecycleHook,
            ExternalPluginAdapterType::Oracle,
            ExternalPluginAdapterType::AppData,
            ExternalPluginAdapterType::LinkedLifecycleHook,
            ExternalPluginAdapterType::LinkedAppData,
            ExternalPluginAdapterType::DataSection,
            ExternalPluginAdapterType::AgentIdentity
        ]
    );
    units!(
        ExternalValidationResultCodec,
        [
            ExternalValidationResult::Approved,
            ExternalValidationResult::Rejected,
            ExternalValidationResult::Pass
        ]
    );
    units!(
        HookableLifecycleEventCodec,
        [
            HookableLifecycleEvent::Create,
            HookableLifecycleEvent::Transfer,
            HookableLifecycleEvent::Burn,
            HookableLifecycleEvent::Update,
            HookableLifecycleEvent::Execute
        ]
    );
    units!(
        KeyCodec,
        [
            Key::Uninitialized,
            Key::AssetV1,
            Key::HashedAssetV1,
            Key::PluginHeaderV1,
            Key::PluginRegistryV1,
            Key::CollectionV1,
            Key::GroupV1
        ]
    );
    plugin_and_relationship_unit_tags();
}

fn plugin_and_relationship_unit_tags() {
    units!(
        PluginTypeCodec,
        [
            PluginType::Royalties,
            PluginType::FreezeDelegate,
            PluginType::BurnDelegate,
            PluginType::TransferDelegate,
            PluginType::UpdateDelegate,
            PluginType::PermanentFreezeDelegate,
            PluginType::Attributes,
            PluginType::PermanentTransferDelegate,
            PluginType::PermanentBurnDelegate,
            PluginType::Edition,
            PluginType::MasterEdition,
            PluginType::AddBlocker,
            PluginType::ImmutableMetadata,
            PluginType::VerifiedCreators,
            PluginType::Autograph,
            PluginType::BubblegumV2,
            PluginType::FreezeExecute,
            PluginType::PermanentFreezeExecute,
            PluginType::Groups
        ]
    );
    units!(
        RelationshipKindCodec,
        [
            RelationshipKind::Collection,
            RelationshipKind::ChildGroup,
            RelationshipKind::ParentGroup,
            RelationshipKind::Asset
        ]
    );
    units!(
        UpdateTypeCodec,
        [UpdateType::Mint, UpdateType::Add, UpdateType::Remove]
    );
    units!(
        ValidationResultCodec,
        [
            ValidationResult::Approved,
            ValidationResult::Rejected,
            ValidationResult::Pass,
            ValidationResult::ForceApproved
        ]
    );
}
