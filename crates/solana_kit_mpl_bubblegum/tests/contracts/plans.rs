//! Actual public helper construction retains exact defaults and complete account lists.
#[cfg(feature = "pda")]
use super::fixtures::LIMITS;
#[cfg(feature = "pda")]
use solana_kit_core::{AccountMeta, AccountRole, Address, codecs::Decoder};
#[cfg(feature = "pda")]
use solana_kit_mpl_bubblegum::*;

#[cfg(feature = "pda")]
const HEADER_BYTES: usize = 8;

#[cfg(feature = "pda")]
fn mint_accounts() -> MintAccounts {
    MintAccounts {
        program: PROGRAM,
        tree: Address::from_bytes([3; 32]),
        payer: Address::from_bytes([2; 32]),
        tree_delegate: None,
        owner: Address::from_bytes([4; 32]),
        delegate: None,
        programs: CompressionPrograms::default(),
    }
}

#[cfg(feature = "pda")]
pub fn mint_defaults() {
    let accounts = mint_accounts();
    let metadata = MetadataArgsCodec
        .decode(&super::fixtures::metadata(false), LIMITS)
        .unwrap();
    let tail = [AccountMeta {
        address: Address::from_bytes([99; 32]),
        role: AccountRole::ReadonlySigner,
    }];
    let instruction = accounts.mint_v1(metadata.clone(), &tail, LIMITS).unwrap();
    assert_eq!(
        &instruction.data[..HEADER_BYTES],
        &[145, 98, 192, 118, 184, 147, 118, 104]
    );
    assert_eq!(
        instruction.accounts[0].address,
        find_tree_authority(accounts.tree).unwrap().address()
    );
    assert_ne!(instruction.accounts[0].address, accounts.tree);
    assert_eq!(instruction.accounts[1].address, accounts.owner);

    assert_eq!(instruction.accounts[2].address, accounts.owner);
    assert_eq!(instruction.accounts[5].address, accounts.payer);
    assert_eq!(instruction.accounts[9], tail[0]);
    assert_eq!(
        MintV1::parse(&instruction, LIMITS).unwrap().message,
        metadata
    );

    collection_mint_defaults(accounts, metadata, &tail);
}

#[cfg(feature = "pda")]
fn collection_mint_defaults(accounts: MintAccounts, metadata: MetadataArgs, tail: &[AccountMeta]) {
    let collection = CollectionMintAccounts {
        mint: accounts,
        collection_authority: Address::from_bytes([7; 32]),
        authority_record: None,
        collection_mint: Address::from_bytes([8; 32]),
        collection_metadata: Address::from_bytes([9; 32]),
        edition: Address::from_bytes([10; 32]),
        bubblegum_signer: Address::from_bytes([11; 32]),
        token_metadata_program: Address::from_bytes([12; 32]),
    };
    let instruction = collection
        .mint_to_collection_v1(metadata.clone(), tail, LIMITS)
        .unwrap();
    assert_eq!(instruction.accounts.len(), 17);
    assert_eq!(instruction.accounts[6].role, AccountRole::ReadonlySigner);
    assert_eq!(instruction.accounts[7].address, PROGRAM);
    assert_eq!(
        MintToCollectionV1::parse(&instruction, LIMITS)
            .unwrap()
            .metadata_args,
        metadata
    );
}

#[cfg(feature = "pda")]
pub fn mint_v2_options() {
    let mint = mint_accounts();
    let metadata_args = MetadataArgsV2Codec
        .decode(&super::fixtures::metadata(true), LIMITS)
        .unwrap();
    let data = MintV2 {
        metadata_args,
        asset_data: Some(vec![1, 2, 3]),
        asset_data_schema: Some(AssetDataSchema::MsgPack),
    };
    let accounts = CoreMintAccounts {
        mint,
        collection_authority: None,
        collection: Some(Address::from_bytes([8; 32])),
        cpi_signer: Some(find_mpl_core_cpi_signer().unwrap().address()),
        core_program: Address::from_bytes([13; 32]),
    };
    let instruction = accounts.mint_v2(data.clone(), &[], LIMITS).unwrap();
    assert_eq!(instruction.accounts.len(), 13);
    assert_eq!(instruction.accounts[2].address, PROGRAM);
    assert_eq!(instruction.accounts[2].role, AccountRole::Readonly);

    assert_eq!(instruction.accounts[3].address, PROGRAM);
    assert_eq!(instruction.accounts[5].address, PROGRAM);
    assert_eq!(
        &instruction.data[..HEADER_BYTES],
        &[120, 121, 23, 146, 173, 110, 199, 205]
    );
    assert_eq!(MintV2::parse(&instruction, LIMITS).unwrap(), data);
    let mut missing = accounts;
    missing.cpi_signer = None;
    assert!(missing.mint_v2(data.clone(), &[], LIMITS).is_err());
    missing = accounts;

    missing.collection = None;
    assert!(missing.mint_v2(data, &[], LIMITS).is_err());
}

#[cfg(feature = "tree")]
pub fn tree_defaults() {
    let accounts =
        TreeAccounts::canonical(Address::from_bytes([1; 32]), Address::from_bytes([2; 32]));
    let legacy = CreateTree {
        max_depth: 14,
        max_buffer_size: 64,
        public: Some(false),
    };
    let instruction = accounts.create_v1(&legacy, LIMITS).unwrap();
    assert_eq!(
        instruction.accounts[0].address,
        find_tree_authority(accounts.tree).unwrap().address()
    );
    assert_eq!(instruction.accounts[3].address, accounts.payer);
    assert_eq!(instruction.accounts[3].role, AccountRole::ReadonlySigner);
    assert_eq!(CreateTree::parse(&instruction, LIMITS).unwrap(), legacy);

    let modern = CreateTreeV2 {
        max_depth: 14,
        max_buffer_size: 64,
        public: None,
    };
    let instruction = accounts.create_v2(&modern, LIMITS).unwrap();
    assert_eq!(instruction.accounts[3].address, PROGRAM);
    assert_eq!(instruction.accounts[3].role, AccountRole::Readonly);
    assert_eq!(CreateTreeV2::parse(&instruction, LIMITS).unwrap(), modern);
    assert!(TreeAccounts::allocation_size(99, 64, 0).is_err());
    assert!(TreeAccounts::allocation_size(14, 64, 15).is_err());
}

#[cfg(feature = "pda")]
#[test]
fn canonical_v1_and_collection_mint_helpers() {
    mint_defaults();
}
#[cfg(feature = "pda")]
#[test]
fn complete_v2_mint_options() {
    mint_v2_options();
}
#[cfg(feature = "tree")]
#[test]
fn checked_tree_initialization_defaults() {
    tree_defaults();
}
