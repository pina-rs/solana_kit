//! Literal digest domains and sparse-tree proof behaviors.
#[cfg(feature = "hashing")]
use super::fixtures::LIMITS;
#[cfg(feature = "hashing")]
use solana_kit_mpl_bubblegum::*;

#[cfg(feature = "hashing")]
pub fn hash_domains() {
    assert_eq!(
        bubblegum_hash(&[]),
        [
            0xc5, 0xd2, 0x46, 0x01, 0x86, 0xf7, 0x23, 0x3c, 0x92, 0x7e, 0x7d, 0xb2, 0xdc, 0xc7,
            0x03, 0xc0, 0xe5, 0x00, 0xb6, 0x53, 0xca, 0x82, 0x27, 0x3b, 0x7b, 0xfa, 0xd8, 0x04,
            0x5d, 0x85, 0xa4, 0x70
        ]
    );
    assert_eq!(hash_asset_data(None), hash_asset_data(Some(&[])));
    assert_eq!(
        bubblegum_hash(&[b"abc", b"def"]),
        bubblegum_hash(&[b"abcdef"])
    );
    let address = solana_kit_core::Address::from_bytes([1; 32]);
    let leaf = LeafHashInput {
        asset_id: address,
        owner: address,
        delegate: None,
        nonce: u64::MAX,
        data_hash: [2; 32],
        creator_hash: [7; 32],
    };
    let mut explicit = leaf;
    explicit.delegate = Some(address);
    assert_eq!(hash_leaf_v1(&leaf), hash_leaf_v1(&explicit));

    let modern = LeafHashV2 {
        leaf,
        collection_hash: [3; 32],
        asset_data_hash: [4; 32],
        flags: 0x81,
    };
    assert_ne!(hash_leaf_v1(&leaf), hash_leaf_v2(&modern));
    let mut changed = modern;
    changed.flags ^= 0x80;
    assert_ne!(hash_leaf_v2(&changed), hash_leaf_v2(&modern));
    leaf_byte_oracles(&leaf, &modern);
}

#[cfg(feature = "hashing")]
fn leaf_byte_oracles(leaf: &LeafHashInput, modern: &LeafHashV2) {
    use sha3::{Digest, Keccak256};
    let mut literal = vec![1];
    literal.extend([1; 32]);
    literal.extend([1; 32]);
    literal.extend([1; 32]);
    literal.extend(u64::MAX.to_le_bytes());
    literal.extend([2; 32]);
    literal.extend([7; 32]);
    let expected: [u8; 32] = Keccak256::digest(&literal).into();

    assert_eq!(hash_leaf_v1(leaf), expected);
    literal[0] = 2;
    literal.extend([3; 32]);
    literal.extend([4; 32]);
    literal.push(0x81);
    let expected: [u8; 32] = Keccak256::digest(&literal).into();
    assert_eq!(hash_leaf_v2(modern), expected);
}

#[cfg(feature = "hashing")]
pub fn sparse_tree() {
    const MAX_TREE_DEPTH: u8 = 32;
    const LAST_EMPTY_LEAF: u32 = 15;
    let leaves = [[1; 32], [2; 32], [3; 32]];
    let tree = MerkleTree::new(&leaves, MAX_TREE_DEPTH, LIMITS).unwrap();

    for (index, leaf) in leaves.iter().enumerate() {
        let proof = tree.proof(u32::try_from(index).unwrap(), LIMITS).unwrap();
        assert!(verify_proof(
            tree.root(),
            *leaf,
            u32::try_from(index).unwrap(),
            &proof
        ));
        let mut corrupt = proof;
        corrupt[0][0] ^= 1;
        assert!(!verify_proof(
            tree.root(),
            *leaf,
            u32::try_from(index).unwrap(),
            &corrupt
        ));
    }

    let empty = MerkleTree::new(&[], 4, LIMITS).unwrap();
    assert_eq!(empty.root(), compute_empty_node(4).unwrap());
    assert!(verify_proof(
        empty.root(),
        [0; 32],
        LAST_EMPTY_LEAF,
        &empty.proof(LAST_EMPTY_LEAF, LIMITS).unwrap()
    ));
    assert!(!verify_proof([0; 32], [0; 32], 2, &[[0; 32]]));
    assert!(MerkleTree::new(&leaves, 1, LIMITS).is_err());
    assert!(
        MerkleTree::new(
            &leaves,
            MAX_TREE_DEPTH,
            solana_kit_core::codecs::Limits::new(32, 1)
        )
        .is_err()
    );
    assert!(compute_empty_node(33).is_err());
}

#[cfg(feature = "hashing")]
#[test]
fn exact_keccak_and_leaf_domains() {
    hash_domains();
}
#[cfg(feature = "hashing")]
#[test]
fn bounded_sparse_merkle_paths() {
    sparse_tree();
}
