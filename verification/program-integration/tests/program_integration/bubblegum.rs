//! Real Bubblegum + SPL Account Compression: tree, mint, transfer and burn leaves.

use crate::util;
use solana_kit_core::{Address, crypto::Keypair};
use solana_kit_mpl_bubblegum::{
    Burn, CompressionPrograms, CreateTree, Creator, LeafAccounts, LeafAuthority, MetadataArgs,
    MintAccounts, PROGRAM, TokenProgramVersion, TokenStandard, Transfer, TreeAccounts,
    decode_tree_config, find_leaf_asset_id, find_tree_authority, hash_metadata_creators,
    hash_metadata_data,
};
use solana_kit_program_integration::{
    CODEC_LIMITS, Error, IntegrationEnv, expect_custom_error,
    fixtures::{ACCOUNT_COMPRESSION, MPL_BUBBLEGUM, NOOP},
};
use solana_kit_system::{CreateAccount, CreateAccountAccounts};

const MAX_DEPTH: u32 = 14;
const MAX_BUFFER: u32 = 64;
const SELLER_FEE_BASIS_POINTS: u16 = 500;
const NAME: &str = "Test NFT";
const SYMBOL: &str = "TST";
const URI: &str = "https://example.invalid/nft.json";
const FOREIGN_DELEGATE_ERROR: u32 = 6_001;
const TREE_HEADER: usize = 56;
const TREE_COUNTERS: usize = 24;
const ACTIVE_INDEX_OFFSET: usize = TREE_HEADER + 8;
const ROOT_BYTES: usize = 32;
const PROOF_NODE_BYTES: usize = 32;
const LEAF_INDEX_BYTES: usize = 8;

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    let payer = env.payer_address();
    let tree = create_tree(env, payer).await?;
    let leaf_owner = env.new_key().await?;
    let (metadata, state) = mint_leaf(env, tree.address(), payer, &leaf_owner).await?;
    let next_owner = env.new_key().await?;
    let transferred = transfer_leaf(
        env,
        tree.address(),
        &metadata,
        &state,
        &leaf_owner,
        &next_owner,
    )
    .await?;

    expect_foreign_delegate_rejection(env, tree.address(), next_owner.address()).await?;
    let unchanged = tree_state(env, tree.address()).await?;
    assert_eq!(unchanged.root, transferred.root);
    assert_eq!(unchanged.index, transferred.index);

    burn_leaf(env, tree.address(), &metadata, &transferred, &next_owner).await
}

async fn create_tree(env: &mut IntegrationEnv, payer: Address) -> Result<Keypair, Error> {
    let tree = env.new_key().await?;
    let space = TreeAccounts::allocation_size(MAX_DEPTH, MAX_BUFFER, 0)? as u64;
    let create_account = CreateAccount {
        lamports: env.rent(space).await?,
        space,
        program_address: ACCOUNT_COMPRESSION.address(),
    }
    .instruction(
        CreateAccountAccounts {
            payer,
            new_account: tree.address(),
        },
        CODEC_LIMITS,
    )?;
    env.submit(&[create_account], &[env.payer(), &tree]).await?;

    let create_tree = TreeAccounts::canonical(tree.address(), payer).create_v1(
        &CreateTree {
            max_depth: MAX_DEPTH,
            max_buffer_size: MAX_BUFFER,
            public: Some(true),
        },
        CODEC_LIMITS,
    )?;
    let signature = env.submit(&[create_tree], &[env.payer()]).await?;

    let authority = find_tree_authority(tree.address())?.address();
    let account = env.account(authority).await?.expect("tree config exists");
    let config = decode_tree_config(account, CODEC_LIMITS)?;
    assert_eq!(config.data.tree_creator, payer);
    assert_eq!(config.data.tree_delegate, payer);
    assert_eq!(config.data.num_minted, 0);
    assert_eq!(config.data.total_mint_capacity, 1_u64 << MAX_DEPTH);
    println!(
        "bubblegum: createTree {signature} decoded capacity {} minted {}",
        config.data.total_mint_capacity, config.data.num_minted
    );

    Ok(tree)
}

async fn mint_leaf(
    env: &IntegrationEnv,
    tree: Address,
    payer: Address,
    leaf_owner: &Keypair,
) -> Result<(MetadataArgs, TreeState), Error> {
    let metadata = metadata(payer);
    let mint = MintAccounts {
        program: PROGRAM,
        tree,
        payer,
        tree_delegate: Some(payer),
        owner: leaf_owner.address(),
        delegate: Some(leaf_owner.address()),
        programs: CompressionPrograms::default(),
    }
    .mint_v1(metadata.clone(), &[], CODEC_LIMITS)?;
    let signature = env.submit(&[mint], &[env.payer()]).await?;

    let authority = find_tree_authority(tree)?.address();
    let account = env.account(authority).await?.expect("tree config exists");
    let config = decode_tree_config(account, CODEC_LIMITS)?;
    assert_eq!(config.data.num_minted, 1);

    let state = tree_state(env, tree).await?;
    let asset = find_leaf_asset_id(tree, u64::from(state.index))?.address();
    println!(
        "bubblegum: mintV1 {signature} created compressed leaf asset {asset} index {} root {}; tree minted {}",
        state.index,
        hex(&state.root),
        config.data.num_minted
    );

    Ok((metadata, state))
}

async fn transfer_leaf(
    env: &IntegrationEnv,
    tree: Address,
    metadata: &MetadataArgs,
    state: &TreeState,
    leaf_owner: &Keypair,
    next_owner: &Keypair,
) -> Result<TreeState, Error> {
    let transfer = LeafAccounts::derive(tree, leaf_owner.address(), Some(leaf_owner.address()))?
        .transfer(
            &Transfer {
                root: state.root,
                data_hash: hash_metadata_data(metadata, CODEC_LIMITS)?,
                creator_hash: hash_metadata_creators(&metadata.creators),
                nonce: u64::from(state.index),
                index: state.index,
            },
            next_owner.address(),
            LeafAuthority::Owner,
            &[],
            CODEC_LIMITS,
        )?;
    let signature = env.submit(&[transfer], &[env.payer(), leaf_owner]).await?;

    let transferred = tree_state(env, tree).await?;
    assert_ne!(transferred.root, state.root);
    assert_eq!(transferred.index, state.index);
    println!(
        "bubblegum: transfer {signature} moved leaf {} to {}; new root {}",
        transferred.index,
        next_owner.address(),
        hex(&transferred.root)
    );

    Ok(transferred)
}

async fn burn_leaf(
    env: &IntegrationEnv,
    tree: Address,
    metadata: &MetadataArgs,
    state: &TreeState,
    owner: &Keypair,
) -> Result<(), Error> {
    let burn = LeafAccounts::derive(tree, owner.address(), Some(owner.address()))?.burn(
        &Burn {
            root: state.root,
            data_hash: hash_metadata_data(metadata, CODEC_LIMITS)?,
            creator_hash: hash_metadata_creators(&metadata.creators),
            nonce: u64::from(state.index),
            index: state.index,
        },
        LeafAuthority::Owner,
        &[],
        CODEC_LIMITS,
    )?;
    let signature = env.submit(&[burn], &[env.payer(), owner]).await?;
    let burned = tree_state(env, tree).await?;
    assert_ne!(burned.root, state.root, "burn must change the Merkle root");
    assert_eq!(burned.index, state.index);
    println!("bubblegum: burn {signature} removed leaf {}", state.index);

    Ok(())
}

fn metadata(payer: Address) -> MetadataArgs {
    MetadataArgs {
        name: NAME.to_owned(),
        symbol: SYMBOL.to_owned(),
        uri: URI.to_owned(),
        seller_fee_basis_points: SELLER_FEE_BASIS_POINTS,
        primary_sale_happened: false,
        is_mutable: true,
        edition_nonce: None,
        token_standard: Some(TokenStandard::NonFungible),
        collection: None,
        uses: None,
        token_program_version: TokenProgramVersion::Original,
        creators: vec![Creator {
            address: payer,
            verified: true,
            share: 100,
        }],
    }
}

struct TreeState {
    root: [u8; 32],
    index: u32,
}

async fn tree_state(env: &IntegrationEnv, tree: Address) -> Result<TreeState, Error> {
    let account = env.account(tree).await?.expect("Merkle tree exists");
    parse_tree(&account.data)
}

fn parse_tree(bytes: &[u8]) -> Result<TreeState, Error> {
    let log_size = ROOT_BYTES + PROOF_NODE_BYTES * MAX_DEPTH as usize + LEAF_INDEX_BYTES;
    let active = u64::from_le_bytes(
        bytes
            .get(ACTIVE_INDEX_OFFSET..ACTIVE_INDEX_OFFSET + 8)
            .ok_or(Error::Client("tree has no active change log"))?
            .try_into()
            .map_err(|_| Error::Client("truncated active change log"))?,
    );
    let offset = TREE_HEADER
        + TREE_COUNTERS
        + usize::try_from(active).map_err(|_| Error::Client("active change log is too large"))?
            * log_size;
    let log = bytes
        .get(offset..offset + log_size)
        .ok_or(Error::Client("change log is truncated"))?;
    let root = log[..ROOT_BYTES]
        .try_into()
        .map_err(|_| Error::Client("truncated root"))?;
    let index_offset = ROOT_BYTES + PROOF_NODE_BYTES * MAX_DEPTH as usize;
    let index = u32::from_le_bytes(
        log.get(index_offset..index_offset + 4)
            .ok_or(Error::Client("truncated leaf index"))?
            .try_into()
            .map_err(|_| Error::Client("truncated leaf index"))?,
    );

    Ok(TreeState { root, index })
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

async fn expect_foreign_delegate_rejection(
    env: &mut IntegrationEnv,
    tree: Address,
    owner: Address,
) -> Result<(), Error> {
    let intruder = env.new_key().await?;
    let state = tree_state(env, tree).await?;
    let metadata = metadata(env.payer_address());
    let transfer = LeafAccounts {
        program: PROGRAM,
        tree_authority: find_tree_authority(tree)?.address(),
        owner,
        delegate: intruder.address(),
        tree,
        programs: CompressionPrograms::default(),
    }
    .transfer(
        &Transfer {
            root: state.root,
            data_hash: hash_metadata_data(&metadata, CODEC_LIMITS)?,
            creator_hash: hash_metadata_creators(&metadata.creators),
            nonce: u64::from(state.index),
            index: state.index,
        },
        env.payer_address(),
        LeafAuthority::Delegate,
        &[],
        CODEC_LIMITS,
    )?;
    let error = env
        .submit(&[transfer], &[env.payer(), &intruder])
        .await
        .expect_err("a foreign delegate cannot transfer the leaf");

    expect_custom_error(&error, 0, FOREIGN_DELEGATE_ERROR);
    println!(
        "bubblegum: foreign delegate rejected with 6001 ({:?}) and the tree stayed unchanged",
        solana_kit_mpl_bubblegum::MplBubblegumError::from_code(FOREIGN_DELEGATE_ERROR)
    );

    Ok(())
}

#[tokio::test]
async fn creates_tree_mints_transfers_and_burns_a_compressed_leaf() {
    let mut env = util::start_with(&[MPL_BUBBLEGUM, ACCOUNT_COMPRESSION, NOOP]).await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
