use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Limits},
};
use solana_kit_mpl_core::*;
const LIMITS: Limits = Limits::new(8192, 2048);
const ATTRIBUTES_TAG: u8 = 6;
const PERMANENT_TRANSFER_TAG: u8 = 7;
const PERMANENT_BURN_TAG: u8 = 8;
const ADD_BLOCKER_TAG: u8 = 11;
const IMMUTABLE_METADATA_TAG: u8 = 12;
const VERIFIED_CREATORS_TAG: u8 = 13;
const AUTOGRAPH_TAG: u8 = 14;
const BUBBLEGUM_V2_TAG: u8 = 15;

const GROUPS_TAG: u8 = 18;
const FIRST_UNKNOWN_PLUGIN_TAG: u8 = 19;
fn check<T: core::fmt::Debug + Eq, C: Encoder<T> + Decoder<T>>(codec: C, value: T, bytes: Vec<u8>) {
    assert_eq!(codec.encode(&value, 8192).unwrap(), bytes);
    assert_eq!(codec.decode(&bytes, LIMITS).unwrap(), value);
}

fn base_plugin_cases() -> Vec<(Plugin, Vec<u8>)> {
    let address = Address::from_bytes([7; 32]);
    let count = vec![1, 0, 0, 0];
    let key = vec![7; 32];
    vec![
        (
            Plugin::Royalties(Royalties {
                basis_points: 500,
                creators: vec![Creator {
                    address,
                    percentage: 100,
                }],
                rule_set: RuleSet::None,
            }),
            [vec![0, 244, 1], count.clone(), key.clone(), vec![100, 0]].concat(),
        ),
        (
            Plugin::FreezeDelegate(FreezeDelegate { frozen: true }),
            vec![1, 1],
        ),
        (Plugin::BurnDelegate(BurnDelegate {}), vec![2]),
        (Plugin::TransferDelegate(TransferDelegate {}), vec![3]),
        (
            Plugin::UpdateDelegate(UpdateDelegate {
                additional_delegates: vec![address],
            }),
            [vec![4], count.clone(), key.clone()].concat(),
        ),
        (
            Plugin::PermanentFreezeDelegate(PermanentFreezeDelegate { frozen: true }),
            vec![5, 1],
        ),
    ]
}

fn attribute_and_delegate_cases() -> Vec<(Plugin, Vec<u8>)> {
    let name_wire = vec![1, 0, 0, 0, b'A'];
    let uri_wire = vec![1, 0, 0, 0, b'U'];
    let count = vec![1, 0, 0, 0];
    vec![
        (
            Plugin::Attributes(Attributes {
                attribute_list: vec![Attribute {
                    key: "A".into(),
                    value: "U".into(),
                }],
            }),
            [vec![ATTRIBUTES_TAG], count.clone(), name_wire, uri_wire].concat(),
        ),
        (
            Plugin::PermanentTransferDelegate(PermanentTransferDelegate {}),
            vec![PERMANENT_TRANSFER_TAG],
        ),
        (
            Plugin::PermanentBurnDelegate(PermanentBurnDelegate {}),
            vec![PERMANENT_BURN_TAG],
        ),
        (
            Plugin::Edition(Edition {
                number: 0x0102_0304,
            }),
            vec![9, 4, 3, 2, 1],
        ),
    ]
}

fn additional_plugin_cases() -> Vec<(Plugin, Vec<u8>)> {
    let address = Address::from_bytes([7; 32]);
    let name_wire = vec![1, 0, 0, 0, b'A'];
    let count = vec![1, 0, 0, 0];
    let key = vec![7; 32];
    vec![
        (
            Plugin::MasterEdition(MasterEdition {
                max_supply: Some(0x0102_0304),
                name: Some("A".into()),
                uri: None,
            }),
            [vec![10, 1, 4, 3, 2, 1, 1], name_wire.clone(), vec![0]].concat(),
        ),
        (Plugin::AddBlocker(AddBlocker {}), vec![ADD_BLOCKER_TAG]),
        (
            Plugin::ImmutableMetadata(ImmutableMetadata {}),
            vec![IMMUTABLE_METADATA_TAG],
        ),
        (
            Plugin::VerifiedCreators(VerifiedCreators {
                signatures: vec![VerifiedCreatorsSignature {
                    address,
                    verified: true,
                }],
            }),
            [
                vec![VERIFIED_CREATORS_TAG],
                count.clone(),
                key.clone(),
                vec![1],
            ]
            .concat(),
        ),
        (
            Plugin::Autograph(Autograph {
                signatures: vec![AutographSignature {
                    address,
                    message: "A".into(),
                }],
            }),
            [vec![AUTOGRAPH_TAG], count.clone(), key.clone(), name_wire].concat(),
        ),
    ]
}

fn execution_and_group_cases() -> Vec<(Plugin, Vec<u8>)> {
    let address = Address::from_bytes([7; 32]);
    let count = vec![1, 0, 0, 0];
    let key = vec![7; 32];
    vec![
        (Plugin::BubblegumV2(BubblegumV2 {}), vec![BUBBLEGUM_V2_TAG]),
        (
            Plugin::FreezeExecute(FreezeExecute { frozen: true }),
            vec![16, 1],
        ),
        (
            Plugin::PermanentFreezeExecute(PermanentFreezeExecute { frozen: true }),
            vec![17, 1],
        ),
        (
            Plugin::Groups(Groups {
                groups: vec![address],
            }),
            [vec![GROUPS_TAG], count, key].concat(),
        ),
    ]
}

#[cfg_attr(all(test, not(target_arch = "wasm32")), test)]
pub fn every_plugin_variant_has_an_independent_payload_vector() {
    let values = base_plugin_cases()
        .into_iter()
        .chain(attribute_and_delegate_cases())
        .chain(additional_plugin_cases())
        .chain(execution_and_group_cases());

    for (value, bytes) in values {
        check(PluginCodec, value, bytes);
    }

    assert_eq!(
        PluginCodec.decode(&[FIRST_UNKNOWN_PLUGIN_TAG], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(
        PluginCodec.decode(&[1, 2], LIMITS),
        Err(Error::InvalidBoolean { value: 2 })
    );
    check(
        HashablePluginSchemaCodec,
        HashablePluginSchema {
            index: 0x0102_0304_0506_0708,
            authority: Authority::Owner,
            plugin: Plugin::BurnDelegate(BurnDelegate {}),
        },
        vec![8, 7, 6, 5, 4, 3, 2, 1, 1, 2],
    );
    let hashes = HashedAssetSchema {
        asset_hash: [7; 32],
        plugin_hashes: vec![[8; 32]],
    };
    check(
        HashedAssetSchemaCodec,
        hashes,
        [vec![7; 32], vec![1, 0, 0, 0], vec![8; 32]].concat(),
    );
    check(DataSectionUpdateInfoCodec, DataSectionUpdateInfo {}, vec![]);
}

#[cfg_attr(all(test, not(target_arch = "wasm32")), test)]
pub fn authority_rule_seed_lifecycle_and_schema_variants_keep_their_literal_tags() {
    let address = Address::from_bytes([7; 32]);
    check(AuthorityCodec, Authority::None, vec![0]);
    check(AuthorityCodec, Authority::Owner, vec![1]);
    check(AuthorityCodec, Authority::UpdateAuthority, vec![2]);
    check(
        AuthorityCodec,
        Authority::Address(AuthorityAddressPayload { address }),
        [vec![3], vec![7; 32]].concat(),
    );
    check(
        UpdateAuthorityCodec,
        UpdateAuthority::Address(address),
        [vec![1], vec![7; 32]].concat(),
    );
    check(
        UpdateAuthorityCodec,
        UpdateAuthority::Collection(address),
        [vec![2], vec![7; 32]].concat(),
    );
    check(
        RuleSetCodec,
        RuleSet::ProgramAllowList(vec![address]),
        [vec![1, 1, 0, 0, 0], vec![7; 32]].concat(),
    );

    check(
        RuleSetCodec,
        RuleSet::ProgramDenyList(vec![address]),
        [vec![2, 1, 0, 0, 0], vec![7; 32]].concat(),
    );
    seed_and_lifecycle_variants(address);
}

fn seed_and_lifecycle_variants(address: Address) {
    const FIRST_UNKNOWN_SEED_TAG: u8 = 6;
    check(SeedCodec, Seed::Collection, vec![0]);
    check(SeedCodec, Seed::Owner, vec![1]);
    check(SeedCodec, Seed::Recipient, vec![2]);
    check(SeedCodec, Seed::Asset, vec![3]);
    check(
        SeedCodec,
        Seed::Address(address),
        [vec![4], vec![7; 32]].concat(),
    );
    check(
        SeedCodec,
        Seed::Bytes(vec![9, 8, 7]),
        vec![5, 3, 0, 0, 0, 9, 8, 7],
    );
    validation_variants();
    assert_eq!(
        SeedCodec.decode(&[FIRST_UNKNOWN_SEED_TAG], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(
        ValidationResultsOffsetCodec.decode(&[3], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
}

fn validation_variants() {
    check(
        ValidationResultsOffsetCodec,
        ValidationResultsOffset::NoOffset,
        vec![0],
    );

    check(
        ValidationResultsOffsetCodec,
        ValidationResultsOffset::Anchor,
        vec![1],
    );
    check(
        ValidationResultsOffsetCodec,
        ValidationResultsOffset::Custom(0x0102_0304_0506_0708),
        vec![2, 8, 7, 6, 5, 4, 3, 2, 1],
    );
    check(
        OracleValidationCodec,
        OracleValidation::Uninitialized,
        vec![0],
    );
    check(
        OracleValidationCodec,
        OracleValidation::V1(OracleValidationV1Payload {
            create: ExternalValidationResult::Approved,
            transfer: ExternalValidationResult::Rejected,
            burn: ExternalValidationResult::Pass,
            update: ExternalValidationResult::Approved,
        }),
        vec![1, 0, 1, 2, 0],
    );
}
