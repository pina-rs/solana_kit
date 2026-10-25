//! Failure paths target observable budgets, index aliases and immutable royalty resolution.
use super::fixtures::LIMITS;
use solana_kit_core::{AccountRole, Address, codecs::Limits};
use solana_kit_mpl_bubblegum::*;

pub fn proof_accounts() {
    const FIRST_INDEX_ALIAS: u32 = 8;
    let proof = AssetProof {
        tree: Address::from_bytes([1; 32]),
        root: [2; 32],
        leaf: [3; 32],
        index: 3,
        siblings: vec![[4; 32], [5; 32], [6; 32]],
    };
    let accounts = proof.remaining_accounts(3, 1, LIMITS).unwrap();
    assert_eq!(accounts.len(), 2);
    assert_eq!(accounts[0].address.as_bytes(), &[4; 32]);
    assert!(
        accounts
            .iter()
            .all(|account| account.role == AccountRole::Readonly)
    );
    assert!(proof.remaining_accounts(2, 0, LIMITS).is_err());
    assert!(proof.remaining_accounts(3, 4, LIMITS).is_err());

    assert!(proof.remaining_accounts(3, 0, Limits::new(95, 3)).is_err());

    assert!(proof.remaining_accounts(3, 0, Limits::new(96, 2)).is_err());
    let mut alias = proof;
    alias.index = FIRST_INDEX_ALIAS;
    assert!(alias.remaining_accounts(3, 0, LIMITS).is_err());
    assert!(can_transfer(false, false));
    assert!(!can_transfer(true, false));
    assert!(!can_transfer(false, true));
    assert!(!can_transfer(true, true));
}

pub fn metadata_constraints() {
    let creator = Creator {
        address: Address::from_bytes([1; 32]),
        verified: true,
        share: 100,
    };
    let creators = core::slice::from_ref(&creator);
    let defaults = MetadataArgs::nft("N".into(), "U".into(), vec![creator.clone()]);
    assert_eq!(defaults.symbol, "");
    assert_eq!(defaults.seller_fee_basis_points, 0);
    assert!(!defaults.primary_sale_happened);
    assert!(defaults.is_mutable);
    assert_eq!(defaults.edition_nonce, None);
    assert_eq!(defaults.token_standard, Some(TokenStandard::NonFungible));

    assert_eq!(
        defaults.token_program_version,
        TokenProgramVersion::Original
    );
    assert_eq!(defaults.collection, None);
    assert_eq!(defaults.uses, None);
    assert!(validate_metadata("N", "S", "U", 500, creators).is_ok());
    assert!(validate_metadata(&"é".repeat(17), "S", "U", 500, creators).is_err());
    assert!(validate_metadata("N", "S", "U", 10_001, creators).is_err());
    assert!(validate_metadata("N", "S", "U", u16::MAX, &[]).is_err());
    assert!(validate_metadata_v2("N", "S", "U", u16::MAX, &[]).is_ok());

    assert!(validate_metadata("N", "S", "U", u16::MAX, creators).is_err());
    assert!(validate_metadata("N", "S", "U", 500, &[creator.clone(), creator]).is_err());
}

pub fn raw_royalties() {
    use solana_kit_core::codecs::Decoder;
    let original = MetadataArgsCodec
        .decode(&super::fixtures::metadata(false), LIMITS)
        .unwrap();
    let raw = RoyaltyRawFields {
        seller_fee_basis_points: Some(u16::MAX),
        creators: Some(vec![]),
    };
    let leaf = to_leaf_metadata(&original, &raw);
    assert_eq!(leaf.seller_fee_basis_points, u16::MAX);
    assert!(leaf.creators.is_empty());
    assert_eq!(original.seller_fee_basis_points, 0x1234);
    assert_eq!(original.creators.len(), 1);
    let converted = metadata_to_v2(&original, &raw);

    assert_eq!(converted.collection, Some(Address::from_bytes([8; 32])));
    assert!(converted.creators.is_empty());
    assert_eq!(
        to_leaf_metadata(&original, &RoyaltyRawFields::default()),
        original
    );
}

pub fn leaf_authorities() {
    let address = |byte| Address::from_bytes([byte; 32]);
    let accounts = LeafAccounts {
        program: PROGRAM,
        tree_authority: address(1),
        owner: address(2),
        delegate: address(3),
        tree: address(4),
        programs: CompressionPrograms::default(),
    };
    let burn = Burn {
        root: [5; 32],
        data_hash: [6; 32],
        creator_hash: [7; 32],
        nonce: u64::MAX,
        index: 9,
    };

    for signer in [LeafAuthority::Owner, LeafAuthority::Delegate] {
        let instruction = accounts.burn(&burn, signer, &[[8; 32]], LIMITS).unwrap();
        assert_eq!(instruction.accounts[0].address, address(1));
        assert_ne!(
            instruction.accounts[0].address,
            instruction.accounts[3].address
        );
        assert_eq!(Burn::parse(&instruction, LIMITS).unwrap(), burn);
        let signed = if signer == LeafAuthority::Owner { 1 } else { 2 };
        assert_eq!(
            instruction.accounts[signed].role,
            AccountRole::ReadonlySigner
        );
        assert_eq!(instruction.accounts.last().unwrap().address, address(8));
    }

    transfer_and_delegate(&accounts, &burn);
}

fn transfer_and_delegate(accounts: &LeafAccounts, burn: &Burn) {
    const NEW_DELEGATE_BYTE: u8 = 11;
    let address = |byte| Address::from_bytes([byte; 32]);
    let transfer = Transfer {
        root: burn.root,
        data_hash: burn.data_hash,
        creator_hash: burn.creator_hash,
        nonce: burn.nonce,
        index: burn.index,
    };
    let instruction = accounts
        .transfer(&transfer, address(10), LeafAuthority::Owner, &[], LIMITS)
        .unwrap();
    assert_eq!(instruction.accounts[3].address, address(10));
    assert_eq!(Transfer::parse(&instruction, LIMITS).unwrap(), transfer);
    let delegate = Delegate {
        root: burn.root,
        data_hash: burn.data_hash,
        creator_hash: burn.creator_hash,
        nonce: burn.nonce,
        index: burn.index,
    };
    let instruction = accounts
        .delegate(&delegate, address(NEW_DELEGATE_BYTE), &[], LIMITS)
        .unwrap();

    assert_eq!(instruction.accounts[2].address, address(3));
    assert_eq!(instruction.accounts[3].address, address(NEW_DELEGATE_BYTE));

    assert_eq!(Delegate::parse(&instruction, LIMITS).unwrap(), delegate);
}

#[test]
fn proof_budget_and_privileges() {
    proof_accounts();
}
#[test]
fn checked_metadata_boundaries() {
    metadata_constraints();
}
#[test]
fn leaf_royalties_preserve_display() {
    raw_royalties();
}
#[test]
fn leaf_helpers_select_actual_authorities() {
    leaf_authorities();
}
