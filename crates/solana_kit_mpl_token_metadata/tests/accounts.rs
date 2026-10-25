//! Complete nonzero account layouts and explicit fetched-allocation policies.
use solana_kit_core::{
    Address,
    accounts::{Account, AccountError, EncodedAccount},
    codecs::{Decoder, Encoder, Limits},
};
use solana_kit_mpl_token_metadata::*;

const LIMITS: Limits = Limits::new(8192, 4096);
const N: u64 = 0x0807_0605_0403_0201;
const NUMBER: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

fn address(value: u8) -> Address {
    Address::from_bytes([value; 32])
}
fn raw(parts: &[&[u8]]) -> Vec<u8> {
    parts.concat()
}

fn snapshot(bytes: Vec<u8>) -> EncodedAccount {
    Account {
        address: address(99),
        owner: PROGRAM,
        lamports: 73,
        executable: false,
        space: u64::try_from(bytes.len()).unwrap(),
        rent_epoch: Some(17),
        data: bytes,
    }
}

type Fetch<T> =
    fn(EncodedAccount, AllocationPolicy, Limits) -> Result<Account<Allocated<T>>, AccountError>;
type Explicit<T> = fn(
    EncodedAccount,
    Address,
    AllocationPolicy,
    Limits,
) -> Result<Account<Allocated<T>>, AccountError>;

fn verify<T: Clone + Eq + core::fmt::Debug>(
    codec: impl Decoder<T> + Encoder<T>,
    expected: T,
    bytes: Vec<u8>,
    fetch: Fetch<T>,
    explicit: Explicit<T>,
) {
    const CUSTOM_OWNER_BYTE: u8 = 97;
    const WRONG_OWNER_BYTE: u8 = 98;
    assert_eq!(codec.decode(&bytes, LIMITS).unwrap(), expected);
    assert_eq!(codec.encode(&expected, 8192).unwrap(), bytes);
    let account = fetch(snapshot(bytes.clone()), AllocationPolicy::Exact, LIMITS).unwrap();
    assert_eq!(account.data.value, expected);
    assert!(account.data.trailing.is_empty());
    assert_eq!(account.address, address(99));
    assert_eq!(account.lamports, 73);
    assert_eq!(account.rent_epoch, Some(17));

    let mut custom = snapshot(bytes.clone());
    custom.owner = address(CUSTOM_OWNER_BYTE);
    let custom = explicit(
        custom,
        address(CUSTOM_OWNER_BYTE),
        AllocationPolicy::Exact,
        LIMITS,
    )
    .unwrap();
    assert_eq!(custom.owner, address(CUSTOM_OWNER_BYTE));
    assert_eq!(custom.data.value, expected);

    for end in 0..bytes.len() {
        assert!(codec.decode(&bytes[..end], LIMITS).is_err());
    }

    let mut changed = bytes.clone();
    changed[0] ^= 0x80;
    assert!(codec.decode(&changed, LIMITS).is_err());
    changed = bytes.clone();
    changed.extend([0, 0]);
    assert!(codec.decode(&changed, LIMITS).is_err());
    assert!(fetch(snapshot(changed.clone()), AllocationPolicy::Exact, LIMITS).is_err());
    let padded = fetch(
        snapshot(changed.clone()),
        AllocationPolicy::ZeroPadding,
        LIMITS,
    )
    .unwrap();

    assert_eq!(padded.data.value, expected);
    assert_eq!(padded.data.trailing, [0, 0]);
    changed[bytes.len()] = 9;
    assert!(
        fetch(
            snapshot(changed.clone()),
            AllocationPolicy::ZeroPadding,
            LIMITS
        )
        .is_err()
    );
    let retained = fetch(snapshot(changed), AllocationPolicy::PreserveTail, LIMITS).unwrap();
    assert_eq!(retained.data.trailing, [9, 0]);
    let mut bad_owner = snapshot(bytes.clone());
    bad_owner.owner = address(WRONG_OWNER_BYTE);

    assert!(matches!(
        fetch(bad_owner, AllocationPolicy::Exact, LIMITS),
        Err(AccountError::OwnerMismatch { .. })
    ));
    let mut bad_space = snapshot(bytes);
    bad_space.space += 1;
    assert!(matches!(
        fetch(bad_space, AllocationPolicy::Exact, LIMITS),
        Err(AccountError::DataSizeMismatch(_))
    ));
}

macro_rules! account {
    ($function:ident, $codec:ident, $fetch:ident, $explicit:ident,
        $expected:expr, $bytes:expr) => {
        #[cfg_attr(test, test)]
        pub fn $function() {
            verify($codec, $expected, $bytes, $fetch, $explicit);
        }
    };
}

account!(
    edition,
    EditionCodec,
    decode_edition,
    decode_edition_with_program,
    Edition {
        parent: address(17),
        edition: N
    },
    raw(&[&[1], &[17; 32], &NUMBER])
);
account!(
    master_edition_v1,
    MasterEditionV1Codec,
    decode_master_edition_v1,
    decode_master_edition_v1_with_program,
    MasterEditionV1 {
        supply: N,
        max_supply: Some(N),
        printing_mint: address(17),
        one_time_printing_authorization_mint: address(34)
    },
    raw(&[&[2], &NUMBER, &[1], &NUMBER, &[17; 32], &[34; 32]])
);
account!(
    reservation_list_v1,
    ReservationListV1Codec,
    decode_reservation_list_v1,
    decode_reservation_list_v1_with_program,
    ReservationListV1 {
        master_edition: address(17),
        supply_snapshot: Some(N),
        reservations: vec![ReservationV1 {
            address: address(34),
            spots_remaining: 7,
            total_spots: 9
        }]
    },
    raw(&[
        &[3],
        &[17; 32],
        &[1],
        &NUMBER,
        &[1, 0, 0, 0],
        &[34; 32],
        &[7, 9]
    ])
);
account!(
    reservation_list_v2,
    ReservationListV2Codec,
    decode_reservation_list_v2,
    decode_reservation_list_v2_with_program,
    ReservationListV2 {
        master_edition: address(17),
        supply_snapshot: None,
        reservations: vec![Reservation {
            address: address(34),
            spots_remaining: N,
            total_spots: N
        }],
        total_reservation_spots: N,
        current_reservation_spots: N
    },
    raw(&[
        &[5],
        &[17; 32],
        &[0, 1, 0, 0, 0],
        &[34; 32],
        &NUMBER,
        &NUMBER,
        &NUMBER,
        &NUMBER
    ])
);
account!(
    master_edition_v2,
    MasterEditionV2Codec,
    decode_master_edition_v2,
    decode_master_edition_v2_with_program,
    MasterEditionV2 {
        supply: N,
        max_supply: None
    },
    raw(&[&[6], &NUMBER, &[0]])
);
account!(
    edition_marker,
    EditionMarkerCodec,
    decode_edition_marker,
    decode_edition_marker_with_program,
    EditionMarker { ledger: [17; 31] },
    raw(&[&[7], &[17; 31]])
);
account!(
    use_authority_record,
    UseAuthorityRecordCodec,
    decode_use_authority_record,
    decode_use_authority_record_with_program,
    UseAuthorityRecord {
        allowed_uses: N,
        bump: 251
    },
    raw(&[&[8], &NUMBER, &[251]])
);
account!(
    collection_authority_record,
    CollectionAuthorityRecordCodec,
    decode_collection_authority_record,
    decode_collection_authority_record_with_program,
    CollectionAuthorityRecord {
        bump: 249,
        new_update_authority: Some(address(17))
    },
    raw(&[&[9, 249, 1], &[17; 32]])
);

account!(
    token_owned_escrow,
    TokenOwnedEscrowCodec,
    decode_token_owned_escrow,
    decode_token_owned_escrow_with_program,
    TokenOwnedEscrow {
        base_token: address(17),
        authority: EscrowAuthority::Creator(address(34)),
        bump: 247
    },
    raw(&[&[10], &[17; 32], &[1], &[34; 32], &[247]])
);
account!(
    token_record,
    TokenRecordCodec,
    decode_token_record,
    decode_token_record_with_program,
    TokenRecord {
        bump: 245,
        state: TokenState::Listed,
        rule_set_revision: Some(N),
        delegate: Some(address(17)),
        delegate_role: Some(TokenDelegateRole::LockedTransfer),
        locked_transfer: Some(address(34))
    },
    raw(&[
        &[11, 245, 2, 1],
        &NUMBER,
        &[1],
        &[17; 32],
        &[1, 5, 1],
        &[34; 32]
    ])
);
account!(
    metadata_delegate_record,
    MetadataDelegateRecordCodec,
    decode_metadata_delegate_record,
    decode_metadata_delegate_record_with_program,
    MetadataDelegateRecord {
        bump: 243,
        mint: address(17),
        delegate: address(34),
        update_authority: address(51)
    },
    raw(&[&[12, 243], &[17; 32], &[34; 32], &[51; 32]])
);
account!(
    edition_marker_v2,
    EditionMarkerV2Codec,
    decode_edition_marker_v2,
    decode_edition_marker_v2_with_program,
    EditionMarkerV2 {
        ledger: vec![129, 2, 4]
    },
    vec![13, 3, 0, 0, 0, 129, 2, 4]
);
account!(
    holder_delegate_record,
    HolderDelegateRecordCodec,
    decode_holder_delegate_record,
    decode_holder_delegate_record_with_program,
    HolderDelegateRecord {
        bump: 241,
        mint: address(17),
        delegate: address(34),
        update_authority: address(51)
    },
    raw(&[&[14, 241], &[17; 32], &[34; 32], &[51; 32]])
);

#[cfg_attr(test, test)]
pub fn metadata() {
    let data = Data {
        name: "N".into(),
        symbol: "S".into(),
        uri: "U".into(),
        seller_fee_basis_points: 513,
        creators: Some(vec![Creator {
            address: address(85),
            verified: true,
            share: 100,
        }]),
    };
    let expected = Metadata {
        update_authority: address(17),
        mint: address(34),
        data,
        primary_sale_happened: true,
        is_mutable: false,
        edition_nonce: Some(239),
        token_standard: Some(TokenStandard::ProgrammableNonFungible),
        collection: Some(Collection {
            verified: true,
            key: address(51),
        }),
        uses: Some(Uses {
            use_method: UseMethod::Single,
            remaining: N,
            total: N,
        }),
        collection_details: Some(CollectionDetails::V2(CollectionDetailsV2Payload {
            padding: [9; 8],
        })),
        programmable_config: Some(ProgrammableConfig::V1(ProgrammableConfigV1Payload {
            rule_set: Some(address(68)),
        })),
    };
    let bytes = raw(&[
        &[4],
        &[17; 32],
        &[34; 32],
        &[
            1, 0, 0, 0, b'N', 1, 0, 0, 0, b'S', 1, 0, 0, 0, b'U', 1, 2, 1, 1, 0, 0, 0,
        ],
        &[85; 32],
        &[1, 100, 1, 0, 1, 239, 1, 4, 1, 1],
        &[51; 32],
        &[1, 2],
        &NUMBER,
        &NUMBER,
        &[1, 1],
        &[9; 8],
        &[1, 0, 1],
        &[68; 32],
    ]);
    verify(
        MetadataCodec,
        expected,
        bytes,
        decode_metadata,
        decode_metadata_with_program,
    );
}

#[cfg_attr(test, test)]
pub fn every_allocated_account_key() {
    let expected = [
        AccountKind::Edition,
        AccountKind::MasterEditionV1,
        AccountKind::ReservationListV1,
        AccountKind::Metadata,
        AccountKind::ReservationListV2,
        AccountKind::MasterEditionV2,
        AccountKind::EditionMarker,
        AccountKind::UseAuthorityRecord,
        AccountKind::CollectionAuthorityRecord,
        AccountKind::TokenOwnedEscrow,
        AccountKind::TokenRecord,
        AccountKind::MetadataDelegateRecord,
        AccountKind::EditionMarkerV2,
        AccountKind::HolderDelegateRecord,
    ];

    for (index, kind) in expected.into_iter().enumerate() {
        assert_eq!(
            identify_account(&[u8::try_from(index + 1).unwrap()]).unwrap(),
            kind
        );
    }

    assert!(identify_account(&[]).is_err());
    assert!(identify_account(&[0]).is_err());
    assert!(identify_account(&[255]).is_err());
}
