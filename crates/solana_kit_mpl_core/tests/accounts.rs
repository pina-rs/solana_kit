use solana_kit_core::{
    Address,
    accounts::{Account, AccountError},
    codecs::{Decoder, Encoder, Error, Limits},
};
use solana_kit_mpl_core::*;
const LIMITS: Limits = Limits::new(8192, 2048);

fn check<T: core::fmt::Debug + Eq, C: Encoder<T> + Decoder<T>>(codec: C, value: T, bytes: &[u8]) {
    assert_eq!(codec.encode(&value, 8192).unwrap(), bytes);
    assert_eq!(codec.decode(bytes, LIMITS).unwrap(), value);

    for offset in 0..bytes.len() {
        assert!(codec.decode(&bytes[..offset], LIMITS).is_err());
    }

    let mut wrong = bytes.to_vec();
    wrong[0] = 255;
    assert_eq!(
        codec.decode(&wrong, LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    let mut tail = bytes.to_vec();
    const TRAILING_BYTE: u8 = 9;
    tail.push(TRAILING_BYTE);
    assert_eq!(codec.decode(&tail, LIMITS), Err(Error::TrailingBytes));
}

#[cfg_attr(all(test, not(target_arch = "wasm32")), test)]
pub fn every_account_has_independent_nonzero_wire_and_strict_header() {
    let owner = Address::from_bytes([7; 32]);
    let asset = [
        vec![1],
        vec![7; 32],
        vec![
            0, 1, 0, 0, 0, b'A', 1, 0, 0, 0, b'U', 1, 8, 7, 6, 5, 4, 3, 2, 1,
        ],
    ]
    .concat();
    check(
        AssetV1Codec,
        AssetV1 {
            owner,
            update_authority: UpdateAuthority::None,
            name: "A".into(),
            uri: "U".into(),
            seq: Some(0x0102_0304_0506_0708),
        },
        &asset,
    );
    check(
        HashedAssetV1Codec,
        HashedAssetV1 { hash: [7; 32] },
        &[vec![2], vec![7; 32]].concat(),
    );
    check(
        PluginHeaderV1Codec,
        PluginHeaderV1 {
            plugin_registry_offset: 0x0102_0304_0506_0708,
        },
        &[3, 8, 7, 6, 5, 4, 3, 2, 1],
    );
    check(
        PluginRegistryV1Codec,
        PluginRegistryV1 {
            registry: vec![RegistryRecord {
                plugin_type: PluginType::FreezeDelegate,
                authority: Authority::Owner,
                offset: 0x0102_0304_0506_0708,
            }],
            external_registry: vec![],
        },
        &[4, 1, 0, 0, 0, 1, 1, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0],
    );
    collection_and_group_accounts(owner);
}

fn collection_and_group_accounts(owner: Address) {
    const COLLECTION_KEY: u8 = 5;
    const GROUP_KEY: u8 = 6;
    let collection = [
        vec![COLLECTION_KEY],
        vec![7; 32],
        vec![1, 0, 0, 0, b'A', 1, 0, 0, 0, b'U', 4, 3, 2, 1, 8, 7, 6, 5],
    ]
    .concat();
    check(
        CollectionV1Codec,
        CollectionV1 {
            update_authority: owner,
            name: "A".into(),
            uri: "U".into(),
            num_minted: 0x0102_0304,
            current_size: 0x0506_0708,
        },
        &collection,
    );

    let group = [
        vec![GROUP_KEY],
        vec![7; 32],
        vec![1, 0, 0, 0, b'A', 1, 0, 0, 0, b'U', 1, 0, 0, 0],
        vec![8; 32],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]
    .concat();
    check(
        GroupV1Codec,
        GroupV1 {
            update_authority: owner,
            name: "A".into(),
            uri: "U".into(),
            collections: vec![Address::from_bytes([8; 32])],
            groups: vec![],
            parent_groups: vec![],
            assets: vec![],
        },
        &group,
    );
}

fn snapshot(data: Vec<u8>) -> Account<Vec<u8>> {
    Account {
        address: Address::from_bytes([11; 32]),
        owner: PROGRAM,
        lamports: 123,
        executable: false,
        space: u64::try_from(data.len()).unwrap(),
        rent_epoch: Some(42),
        data,
    }
}

#[cfg_attr(all(test, not(target_arch = "wasm32")), test)]
pub fn fetched_snapshots_preserve_extensions_and_reject_owner_or_allocation_drift() {
    let fixed = snapshot([vec![2], vec![7; 32]].concat());
    assert!(decode_hashed_asset_v1(fixed, Limits::new(8192, 0)).is_ok());
    let data = [vec![2], vec![7; 32], vec![255, 9, 8]].concat();
    let account = snapshot(data.clone());
    let decoded = decode_hashed_asset_v1(account, LIMITS).unwrap();
    assert_eq!(decoded.data.base.hash, [7; 32]);
    assert_eq!(decoded.data.extension_bytes, [255, 9, 8]);
    assert_eq!(decoded.rent_epoch, Some(42));

    assert_eq!(decoded.lamports, 123);
    let mut foreign = snapshot(data.clone());
    foreign.owner = Address::from_bytes([9; 32]);
    assert!(matches!(
        decode_hashed_asset_v1(foreign, LIMITS),
        Err(AccountError::OwnerMismatch { .. })
    ));
    let mut wrong = snapshot(data);
    wrong.space += 1;
    assert!(matches!(
        decode_hashed_asset_v1(wrong, LIMITS),
        Err(AccountError::DataSizeMismatch(_))
    ));
}

fn registry_asset() -> Vec<u8> {
    [
        vec![1],
        vec![7; 32],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![3, 54, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1],
        vec![4, 1, 0, 0, 0, 1, 1, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]
    .concat()
}

#[cfg_attr(all(test, not(target_arch = "wasm32")), test)]
pub fn registry_bounds_tags_and_overlaps_are_checked_before_returning_views() {
    const INVALID_EARLY_OFFSET: u64 = 42;
    const BEFORE_PAYLOAD: u64 = 51;
    const INSIDE_REGISTRY_HEADER: u64 = 55;
    let bytes = registry_asset();
    let parsed = inspect_asset_plugins(&bytes, LIMITS).unwrap();
    assert_eq!(parsed.plugins.len(), 1);
    assert_eq!(parsed.plugins[0].range, 52..54);
    assert_eq!(
        parsed.plugins[0].value,
        Plugin::FreezeDelegate(FreezeDelegate { frozen: true })
    );

    for bad_offset in [
        0u64,
        INVALID_EARLY_OFFSET,
        BEFORE_PAYLOAD,
        INSIDE_REGISTRY_HEADER,
        u64::MAX,
    ] {
        let mut changed = bytes.clone();
        changed[44..52].copy_from_slice(&bad_offset.to_le_bytes());
        assert!(
            inspect_asset_plugins(&changed, LIMITS).is_err(),
            "invalid registry offset {bad_offset} was accepted"
        );
    }

    let mut wrong_tag = bytes.clone();
    wrong_tag[52] = 255;
    assert!(inspect_asset_plugins(&wrong_tag, LIMITS).is_err());
    let mut mismatched = bytes.clone();
    mismatched[59] = 18;
    assert!(inspect_asset_plugins(&mismatched, LIMITS).is_err());
    assert!(inspect_asset_plugins(&bytes, Limits::new(8192, 0)).is_err());
    let mut duplicate = bytes.clone();

    duplicate[55..59].copy_from_slice(&2u32.to_le_bytes());
    let descriptor = bytes[59..69].to_vec();
    duplicate.splice(69..69, descriptor);
    assert!(inspect_asset_plugins(&duplicate, LIMITS).is_err());
    let overlap = [
        vec![1],
        vec![7; 32],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![3, 60, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 244, 1, 0, 0, 0, 0, 0],
        vec![
            4, 2, 0, 0, 0, 0, 1, 52, 0, 0, 0, 0, 0, 0, 0, 1, 1, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    ]
    .concat();
    assert!(inspect_asset_plugins(&overlap, LIMITS).is_err());
}

#[cfg_attr(all(test, not(target_arch = "wasm32")), test)]
pub fn external_registry_data_pairs_and_disjoint_payload_ranges_are_required() {
    let bytes = [
        vec![1],
        vec![7; 32],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![3, 61, 0, 0, 0, 0, 0, 0, 0],
        vec![6, 1, 0, 0, 0, b'A', 9, 8, 7],
        vec![
            4, 0, 0, 0, 0, 1, 0, 0, 0, 6, 1, 0, 52, 0, 0, 0, 0, 0, 0, 0, 1, 58, 0, 0, 0, 0, 0, 0,
            0, 1, 3, 0, 0, 0, 0, 0, 0, 0,
        ],
    ]
    .concat();
    let registry = inspect_asset_plugins(&bytes, LIMITS).unwrap();
    assert_eq!(registry.external_plugins[0].range, 52..58);
    assert_eq!(registry.external_data, [Some(58..61)]);
    assert_eq!(
        registry.external_plugins[0].value,
        ExternalPluginAdapter::AgentIdentity(AgentIdentity { uri: "A".into() })
    );
    let mut overlap = bytes.clone();
    overlap[82..90].copy_from_slice(&54u64.to_le_bytes());
    assert!(inspect_asset_plugins(&overlap, LIMITS).is_err());

    let mut overflow = bytes.clone();
    overflow[91..99].copy_from_slice(&u64::MAX.to_le_bytes());
    assert!(inspect_asset_plugins(&overflow, LIMITS).is_err());
    let mut truncated = bytes;
    truncated.pop();
    assert!(inspect_asset_plugins(&truncated, LIMITS).is_err());
}
