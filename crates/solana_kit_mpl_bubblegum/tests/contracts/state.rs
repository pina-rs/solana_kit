//! Independent account bytes cover both leaf branches and all explicit enum values.
use super::fixtures::{LIMITS, NONCE};
use solana_kit_core::{
    Address,
    accounts::Account,
    codecs::{Decoder, Encoder, Limits},
};
use solana_kit_mpl_bubblegum::*;

fn tree_bytes() -> Vec<u8> {
    let mut bytes = vec![122, 245, 175, 248, 171, 34, 0, 207];
    bytes.extend([1; 32]);
    bytes.extend([2; 32]);
    bytes.extend(0x1234u64.to_le_bytes());
    bytes.extend(0x5678u64.to_le_bytes());
    bytes.extend([1, 0, 1]);
    bytes
}

fn snapshot(data: Vec<u8>) -> Account<Vec<u8>> {
    Account {
        address: Address::from_bytes([33; 32]),
        owner: PROGRAM,
        lamports: 1,
        executable: false,
        space: u64::try_from(data.len()).unwrap(),
        rent_epoch: None,
        data,
    }
}

pub fn tree_account() {
    let bytes = tree_bytes();
    assert_eq!(bytes.len(), TREE_CONFIG_SERIALIZED_SIZE);
    let value = TreeConfigCodec.decode(&bytes, LIMITS).unwrap();
    assert_eq!(value.tree_creator, Address::from_bytes([1; 32]));
    assert_eq!(value.tree_delegate, Address::from_bytes([2; 32]));
    assert_eq!(value.total_mint_capacity, 0x1234);
    assert_eq!(value.num_minted, 0x5678);
    assert_eq!(value.version, Version::V2);

    assert_eq!(TreeConfigCodec.encode(&value, 1024).unwrap(), bytes);
    assert_eq!(
        decode_tree_config(snapshot(bytes.clone()), LIMITS)
            .unwrap()
            .data,
        value
    );
    let mut allocated = bytes.clone();
    allocated.extend([0; 5]);
    assert_eq!(allocated.len(), TREE_CONFIG_ACCOUNT_SIZE);
    assert!(decode_tree_config(snapshot(allocated.clone()), LIMITS).is_ok());
    allocated[95] = 1;
    assert!(decode_tree_config(snapshot(allocated), LIMITS).is_err());

    for index in [88, 89, 90] {
        let mut changed = bytes.clone();
        changed[index] = 2;
        assert!(TreeConfigCodec.decode(&changed, LIMITS).is_err());
    }

    let mut wrong_owner = snapshot(bytes.clone());
    wrong_owner.owner = Address::from_bytes([0; 32]);
    assert!(decode_tree_config(wrong_owner, LIMITS).is_err());
    let mut wrong_size = snapshot(bytes);
    wrong_size.space += 1;
    assert!(decode_tree_config(wrong_size, LIMITS).is_err());
}

fn voucher_bytes(modern: bool) -> Vec<u8> {
    let mut bytes = vec![191, 204, 149, 234, 213, 165, 13, 65, u8::from(modern)];

    for byte in [1, 2, 3] {
        bytes.extend([byte; 32]);
    }

    bytes.extend(NONCE.to_le_bytes());
    bytes.extend([4; 32]);
    bytes.extend([5; 32]);

    if modern {
        bytes.extend([6; 32]);
        bytes.extend([7; 32]);
        bytes.push(0x81);
    }

    bytes.extend(0x1234u32.to_le_bytes());
    bytes.extend([8; 32]);
    bytes
}

/// Both schema branches must preserve exact lengths, fields and every truncation failure.
pub fn voucher_accounts() {
    for modern in [false, true] {
        let bytes = voucher_bytes(modern);

        assert_eq!(bytes.len(), if modern { 278 } else { 213 });
        let value = VoucherCodec.decode(&bytes, LIMITS).unwrap();
        assert_eq!(value.index, 0x1234);
        assert_eq!(value.merkle_tree, Address::from_bytes([8; 32]));

        match &value.leaf_schema {
            LeafSchema::V1(leaf) => {
                assert!(!modern);
                assert_eq!(leaf.nonce, NONCE);
            }
            LeafSchema::V2(leaf) => {
                assert!(modern);
                assert_eq!(leaf.collection_hash, [6; 32]);
                assert_eq!(leaf.asset_data_hash, [7; 32]);
                assert_eq!(leaf.flags, 0x81);
            }
        }

        assert_eq!(VoucherCodec.encode(&value, 1024).unwrap(), bytes);
        assert_eq!(
            decode_voucher(snapshot(bytes.clone()), LIMITS)
                .unwrap()
                .data,
            value
        );

        for end in 0..bytes.len() {
            assert!(VoucherCodec.decode(&bytes[..end], LIMITS).is_err());
        }

        let mut changed = bytes;
        changed[8] = 2;
        assert!(VoucherCodec.decode(&changed, LIMITS).is_err());
    }
}

pub fn metadata_branches() {
    const BYTE_BUDGET: usize = 8192;
    let full = super::fixtures::metadata(false);
    let value = MetadataArgsCodec.decode(&full, LIMITS).unwrap();
    assert_eq!(value.name, "N");
    assert_eq!(value.creators[0].share, 100);
    assert_eq!(value.token_standard, Some(TokenStandard::NonFungible));
    assert_eq!(MetadataArgsCodec.encode(&value, 1024).unwrap(), full);
    let mut absent = value.clone();
    absent.edition_nonce = None;

    absent.token_standard = None;
    absent.collection = None;
    absent.uses = None;
    absent.creators.clear();
    let encoded = MetadataArgsCodec.encode(&absent, 1024).unwrap();
    assert_eq!(MetadataArgsCodec.decode(&encoded, LIMITS).unwrap(), absent);
    assert!(
        MetadataArgsCodec
            .decode(&full, Limits::new(BYTE_BUDGET, 0))
            .is_err()
    );
    let mut invalid_utf8 = full.clone();

    invalid_utf8[4] = 0xff;
    assert!(MetadataArgsCodec.decode(&invalid_utf8, LIMITS).is_err());
    let mut invalid_option = full;
    invalid_option[19] = 2;
    assert!(MetadataArgsCodec.decode(&invalid_option, LIMITS).is_err());

    for bytes in [vec![0; 7], vec![1, 1, 0, 0, 0, b'X', 0, 0, 0, 0, 0, 0]] {
        let value = UpdateArgsCodec.decode(&bytes, LIMITS).unwrap();
        assert_eq!(UpdateArgsCodec.encode(&value, 1024).unwrap(), bytes);
    }
}

pub fn program_errors() {
    const FIRST_KNOWN_CODE: u32 = 6000;
    const LAST_KNOWN_CODE: u32 = 6059;
    let mut count = 0;

    for row in include_str!("../fixtures/errors.tsv").lines().skip(1) {
        let (code, message) = row.split_once('\t').unwrap();
        let code = code.parse::<u32>().unwrap();
        let value = MplBubblegumError::from_code(code);
        assert_eq!(value.code(), code);
        assert_eq!(value.message(), Some(message));
        count += 1;
    }

    assert_eq!(count, 60);
    assert_eq!(
        MplBubblegumError::from_code(FIRST_KNOWN_CODE),
        MplBubblegumError::AssetOwnerMismatch
    );
    assert_eq!(
        MplBubblegumError::from_code(LAST_KNOWN_CODE),
        MplBubblegumError::CannotRemoveFromCollectionWithInheritedSellerFee
    );
    assert_eq!(MplBubblegumError::from_code(6060).message(), None);
    assert_eq!(MplBubblegumError::from_code(u32::MAX).code(), u32::MAX);
}

pub fn leaf_events() {
    for modern in [false, true] {
        let voucher = voucher_bytes(modern);
        let mut event = vec![1, u8::from(modern)];
        event.extend_from_slice(&voucher[8..voucher.len() - 36]);
        event.extend([9; 32]);
        let value = decode_leaf_schema_event(&event, LIMITS).unwrap();
        assert_eq!(value.leaf_hash, [9; 32]);
        assert_eq!(LeafSchemaEventCodec.encode(&value, 1024).unwrap(), event);

        let mut bad = event.clone();
        bad[0] = 0;
        assert!(decode_leaf_schema_event(&bad, LIMITS).is_err());
        bad = event.clone();
        bad[1] ^= 1;
        assert!(decode_leaf_schema_event(&bad, LIMITS).is_err());

        for end in 0..event.len() {
            assert!(decode_leaf_schema_event(&event[..end], LIMITS).is_err());
        }
    }
}

#[test]
fn tree_account_layout_and_owner() {
    tree_account();
}
#[test]
fn both_complete_voucher_branches() {
    voucher_accounts();
}
#[test]
fn metadata_option_and_utf8_boundaries() {
    metadata_branches();
}
#[test]
fn all_program_error_codes() {
    program_errors();
}
#[test]
fn complete_leaf_events_match_their_schema_version() {
    leaf_events();
}
