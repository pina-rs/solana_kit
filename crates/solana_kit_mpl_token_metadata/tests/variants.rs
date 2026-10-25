//! Variant discriminants and significant nested branches use independent literal wire data.
use solana_kit_core::codecs::{Decoder, Encoder, Error, Limits};
use solana_kit_mpl_token_metadata::*;

const LIMITS: Limits = Limits::new(8192, 4096);
const NUMBER: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

fn check<T: core::fmt::Debug>(codec: impl Encoder<T> + Decoder<T>, bytes: &[u8], variant: &str) {
    let value = codec.decode(bytes, LIMITS).unwrap();
    let display = format!("{value:?}");
    assert!(
        display == variant || display.starts_with(&format!("{variant}(")),
        "{display}"
    );
    assert_eq!(codec.encode(&value, 8192).unwrap(), bytes);

    for end in 0..bytes.len() {
        assert!(codec.decode(&bytes[..end], LIMITS).is_err());
    }

    let mut changed = bytes.to_vec();
    changed[0] = 255;
    assert!(codec.decode(&changed, LIMITS).is_err());
    changed = bytes.to_vec();
    changed.push(0);
    assert!(codec.decode(&changed, LIMITS).is_err());
}

fn finite<T: core::fmt::Debug>(codec: impl Encoder<T> + Decoder<T>, names: &str) {
    for (index, name) in names.split(',').enumerate() {
        check(&codec, &[u8::try_from(index).unwrap()], name);
    }
}

#[cfg_attr(test, test)]
pub fn finite_tags() {
    assert!(identify_instruction(&[]).is_err());
    assert!(identify_instruction(&[58]).is_err());
    assert!(identify_instruction(&[255]).is_err());
    assert!(
        MplTokenMetadataInstructionDataCodec
            .decode(&[255], LIMITS)
            .is_err()
    );
    finite(
        AuthorityTypeCodec,
        "None,Metadata,Holder,MetadataDelegate,TokenDelegate",
    );
    finite(HolderDelegateRoleCodec, "PrintDelegate");
    finite(
        KeyCodec,
        concat!(
            "Uninitialized,EditionV1,MasterEditionV1,ReservationListV1,",
            "MetadataV1,ReservationListV2,MasterEditionV2,EditionMarker,UseAuthorityRecord,",
            "CollectionAuthorityRecord,TokenOwnedEscrow,TokenRecord,MetadataDelegate,",
            "EditionMarkerV2,HolderDelegate"
        ),
    );
    finite(
        MetadataDelegateRoleCodec,
        concat!(
            "AuthorityItem,Collection,Use,Data,",
            "ProgrammableConfig,DataItem,CollectionItem,ProgrammableConfigItem"
        ),
    );

    finite(MigrationTypeCodec, "CollectionV1,ProgrammableV1");
    finite(
        PayloadKeyCodec,
        concat!(
            "Amount,Authority,AuthoritySeeds,Delegate,DelegateSeeds,",
            "Destination,DestinationSeeds,Holder,Source,SourceSeeds"
        ),
    );
    finite(
        RevokeArgsCodec,
        concat!(
            "CollectionV1,SaleV1,TransferV1,DataV1,UtilityV1,StakingV1,",
            "StandardV1,LockedTransferV1,ProgrammableConfigV1,MigrationV1,AuthorityItemV1,",
            "DataItemV1,CollectionItemV1,ProgrammableConfigItemV1,PrintDelegateV1"
        ),
    );
    finite(
        TokenDelegateRoleCodec,
        "Sale,Transfer,Utility,Staking,Standard,LockedTransfer,Migration",
    );
    finite(
        TokenStandardCodec,
        concat!(
            "NonFungible,FungibleAsset,Fungible,NonFungibleEdition,",
            "ProgrammableNonFungible,ProgrammableNonFungibleEdition"
        ),
    );
    finite(TokenStateCodec, "Unlocked,Locked,Listed");
    finite(UseMethodCodec, "Burn,Multiple,Single");
    finite(VerificationArgsCodec, "CreatorV1,CollectionV1");
}

#[cfg_attr(test, test)]
pub fn toggles_and_print_variants() {
    check(
        CollectionDetailsCodec,
        &[vec![0], NUMBER.to_vec()].concat(),
        "V1",
    );
    check(
        CollectionDetailsCodec,
        &[vec![1], vec![17; 8]].concat(),
        "V2",
    );
    check(PrintArgsCodec, &[vec![0], NUMBER.to_vec()].concat(), "V1");
    check(PrintArgsCodec, &[vec![1], NUMBER.to_vec()].concat(), "V2");
    check(PrintSupplyCodec, &[0], "Zero");
    check(
        PrintSupplyCodec,
        &[vec![1], NUMBER.to_vec()].concat(),
        "Limited",
    );
    check(PrintSupplyCodec, &[2], "Unlimited");
    check(EscrowAuthorityCodec, &[0], "TokenOwner");

    check(
        EscrowAuthorityCodec,
        &[vec![1], vec![17; 32]].concat(),
        "Creator",
    );
    check(ProgrammableConfigCodec, &[0, 0], "V1");
    check(
        ProgrammableConfigCodec,
        &[vec![0, 1], vec![17; 32]].concat(),
        "V1",
    );
    toggle_variants();
}

fn toggle_variants() {
    for (tag, name) in [(0, "None"), (1, "Clear")] {
        check(CollectionToggleCodec, &[tag], name);
        check(CollectionDetailsToggleCodec, &[tag], name);
        check(UsesToggleCodec, &[tag], name);
        check(RuleSetToggleCodec, &[tag], name);
    }

    check(
        CollectionToggleCodec,
        &[vec![2, 1], vec![17; 32]].concat(),
        "Set",
    );
    check(
        CollectionDetailsToggleCodec,
        &[vec![2, 0], NUMBER.to_vec()].concat(),
        "Set",
    );
    check(
        UsesToggleCodec,
        &[vec![2, 2], NUMBER.to_vec(), NUMBER.to_vec()].concat(),
        "Set",
    );
    check(RuleSetToggleCodec, &[vec![2], vec![17; 32]].concat(), "Set");
}

#[cfg_attr(test, test)]
/// Optional authorization and lock-address fields differ by literal delegate role.
pub fn every_delegate_variant() {
    const LOCKED_TRANSFER: usize = 7;
    const STANDARD: usize = 6;
    let names = [
        "CollectionV1",
        "SaleV1",
        "TransferV1",
        "DataV1",
        "UtilityV1",
        "StakingV1",
        "StandardV1",
        "LockedTransferV1",
        "ProgrammableConfigV1",
        "AuthorityItemV1",
        "DataItemV1",
        "CollectionItemV1",
        "ProgrammableConfigItemV1",
        "PrintDelegateV1",
    ];

    for (index, name) in names.iter().enumerate() {
        let mut bytes = vec![u8::try_from(index).unwrap()];

        if [1, 2, 4, 5, 6, 7].contains(&index) {
            bytes.extend(NUMBER);
        }

        if index == LOCKED_TRANSFER {
            bytes.extend([17; 32]);
        }

        if index != STANDARD {
            bytes.push(0);
        }

        check(DelegateArgsCodec, &bytes, name);
    }
}

#[cfg_attr(test, test)]
pub fn every_update_variant() {
    const LEGACY_FIELDS: usize = 9;
    const AUTHORITY_ITEM_FIELDS: usize = 5;
    const PROGRAMMABLE_CONFIG_TAG: u8 = 5;
    const DATA_ITEM_TAG: u8 = 6;
    const COLLECTION_ITEM_TAG: u8 = 7;
    const PROGRAMMABLE_CONFIG_ITEM_TAG: u8 = 8;
    for (tag, count, name) in [
        (0, LEGACY_FIELDS, "V1"),
        (1, 10, "AsUpdateAuthorityV2"),
        (2, AUTHORITY_ITEM_FIELDS, "AsAuthorityItemDelegateV2"),
        (3, 2, "AsCollectionDelegateV2"),
        (4, 2, "AsDataDelegateV2"),
        (PROGRAMMABLE_CONFIG_TAG, 2, "AsProgrammableConfigDelegateV2"),
        (DATA_ITEM_TAG, 2, "AsDataItemDelegateV2"),
        (COLLECTION_ITEM_TAG, 2, "AsCollectionItemDelegateV2"),
        (
            PROGRAMMABLE_CONFIG_ITEM_TAG,
            2,
            "AsProgrammableConfigItemDelegateV2",
        ),
    ] {
        check(UpdateArgsCodec, &[vec![tag], vec![0; count]].concat(), name);
    }
}

#[cfg_attr(test, test)]
pub fn nested_seeds_proofs_and_authorization() {
    const BYTE_BUDGET: usize = 8192;
    let seed_wire = vec![3, 0, 0, 0, 3, 0, 0, 0, 1, 2, 3, 0, 0, 0, 0, 1, 0, 0, 0, 4];
    let seeds = SeedsVec {
        seeds: vec![vec![1, 2, 3], vec![], vec![4]],
    };
    assert_eq!(
        SeedsVecCodec.encode(&seeds, BYTE_BUDGET).unwrap(),
        seed_wire
    );
    assert_eq!(SeedsVecCodec.decode(&seed_wire, LIMITS).unwrap(), seeds);
    assert_eq!(
        SeedsVecCodec
            .encode(&SeedsVec { seeds: vec![] }, 8192)
            .unwrap(),
        [0; 4]
    );
    let payload = Payload {
        map: vec![("x".into(), PayloadType::Seeds(seeds))],
    };
    let payload_wire = [vec![1, 0, 0, 0, 1, 0, 0, 0, b'x', 1], seed_wire].concat();
    assert_eq!(PayloadCodec.encode(&payload, 8192).unwrap(), payload_wire);

    assert_eq!(
        AuthorizationDataCodec
            .decode(&payload_wire, LIMITS)
            .unwrap()
            .payload,
        payload
    );
    let transfer = [vec![0], NUMBER.to_vec(), vec![1], payload_wire].concat();
    let value = TransferArgsCodec.decode(&transfer, LIMITS).unwrap();
    assert_eq!(TransferArgsCodec.encode(&value, 8192).unwrap(), transfer);
    payload_variant_and_seed_failures();
}

fn payload_variant_and_seed_failures() {
    check(
        PayloadTypeCodec,
        &[vec![0], vec![17; 32]].concat(),
        "Pubkey",
    );
    check(PayloadTypeCodec, &[1, 0, 0, 0, 0], "Seeds");
    check(
        PayloadTypeCodec,
        &[vec![2, 1, 0, 0, 0], vec![17; 32]].concat(),
        "MerkleProof",
    );
    check(
        PayloadTypeCodec,
        &[vec![3], NUMBER.to_vec()].concat(),
        "Number",
    );

    let overflowing_seed_length = [1, 0, 0, 0, 255, 255, 255, 255];
    assert!(SeedsVecCodec.decode(&[255; 4], LIMITS).is_err());
    assert!(
        SeedsVecCodec
            .decode(&overflowing_seed_length, LIMITS)
            .is_err()
    );
    assert!(
        SeedsVecCodec
            .decode(&[1, 0, 0, 0, 0, 0, 0, 0], Limits::new(8192, 0))
            .is_err()
    );
}

#[cfg_attr(test, test)]
pub fn creator_options_utf8_boolean_and_budget_failures() {
    let prefix = vec![1, 0, 0, 0, b'N', 1, 0, 0, 0, b'S', 1, 0, 0, 0, b'U', 1, 2];

    for (suffix, creators) in [(vec![0], None), (vec![1, 0, 0, 0, 0], Some(Vec::new()))] {
        let bytes = [prefix.clone(), suffix].concat();
        let value = DataCodec.decode(&bytes, LIMITS).unwrap();
        assert_eq!(value.name, "N");
        assert_eq!(value.seller_fee_basis_points, 513);
        assert_eq!(value.creators, creators);
        assert_eq!(DataCodec.encode(&value, 8192).unwrap(), bytes);
    }

    let mut bad_utf8 = [prefix, vec![0]].concat();
    let mut bad_option = bad_utf8.clone();
    *bad_option.last_mut().unwrap() = 2;
    assert!(DataCodec.decode(&bad_option, LIMITS).is_err());
    bad_utf8[4] = 255;
    assert_eq!(DataCodec.decode(&bad_utf8, LIMITS), Err(Error::InvalidUtf8));
    let bad_bool = [vec![17; 32], vec![2, 100]].concat();
    assert!(CreatorCodec.decode(&bad_bool, LIMITS).is_err());
    assert!(DataCodec.decode(&[255; 4], LIMITS).is_err());

    let empty = Payload { map: vec![] };
    assert_eq!(PayloadCodec.encode(&empty, 4).unwrap(), [0; 4]);
    assert!(PayloadCodec.encode(&empty, 3).is_err());
}
