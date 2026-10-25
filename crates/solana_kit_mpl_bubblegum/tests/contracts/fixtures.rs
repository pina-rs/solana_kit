//! Literal nonzero field bytes are independent of the product encoder.
use solana_kit_core::codecs::Limits;

pub const LIMITS: Limits = Limits::new(8192, 2048);
pub const NONCE: u64 = 0x0102_0304_0506_0708;
pub const INDEX: u32 = 0x0a0b_0c0d;

pub fn metadata(v2: bool) -> Vec<u8> {
    let mut bytes = vec![
        1, 0, 0, 0, b'N', 1, 0, 0, 0, b'S', 1, 0, 0, 0, b'U', 0x34, 0x12, 1, 0,
    ];

    if !v2 {
        bytes.extend([1, 7]);
    }

    bytes.extend([1, 0]);

    if !v2 {
        bytes.push(1);
        bytes.push(1);
        bytes.extend([8; 32]);
        bytes.extend([1, 2]);
        bytes.extend(3u64.to_le_bytes());
        bytes.extend(4u64.to_le_bytes());
        bytes.push(1);
    }

    bytes.extend([1, 0, 0, 0]);
    bytes.extend([9; 32]);
    bytes.extend([1, 100]);

    if v2 {
        bytes.push(1);
        bytes.extend([8; 32]);
    }

    bytes
}

pub fn proof() -> Vec<u8> {
    let mut bytes = vec![1; 32];
    bytes.extend([2; 32]);
    bytes.extend([3; 32]);
    bytes.extend(NONCE.to_le_bytes());
    bytes.extend(INDEX.to_le_bytes());
    bytes
}

fn v2(collection: bool) -> Vec<u8> {
    let mut bytes = vec![1; 32];
    bytes.extend([2; 32]);
    bytes.extend([3; 32]);

    if collection {
        bytes.push(1);
        bytes.extend([4; 32]);
    }

    bytes.push(1);
    bytes.extend([5; 32]);
    bytes.extend([1, 0x81]);
    bytes.extend(NONCE.to_le_bytes());
    bytes.extend(INDEX.to_le_bytes());
    bytes
}

fn metadata_proof() -> Vec<u8> {
    let mut bytes = vec![1; 32];
    bytes.push(1);
    bytes.extend([5; 32]);
    bytes.extend([1, 0x81]);
    bytes.extend(NONCE.to_le_bytes());
    bytes.extend(INDEX.to_le_bytes());
    bytes
}

pub fn payload(profile: &str) -> Vec<u8> {
    match profile {
        "empty" => vec![],
        "root" => vec![1; 32],
        "state" => vec![1],
        "create" => vec![14, 0, 0, 0, 64, 0, 0, 0, 1, 1],
        "proof" => proof(),
        "v2" => v2(false),
        "v2collection" => v2(true),
        "metadata" => metadata(false),
        "proofmetadata" => [proof(), metadata(false)].concat(),
        "collection" => [proof(), metadata(false), vec![6; 32]].concat(),
        "metadata2proof" => [metadata_proof(), metadata(true)].concat(),
        "update1" => [
            vec![1; 32],
            NONCE.to_le_bytes().to_vec(),
            INDEX.to_le_bytes().to_vec(),
            metadata(false),
            vec![0; 7],
        ]
        .concat(),
        "update2" => [metadata_proof(), metadata(true), vec![0; 7]].concat(),
        "mint2" => [metadata(true), vec![1, 3, 0, 0, 0, 0xa1, 0xb2, 0xc3, 1, 2]].concat(),
        "assetupdate" => [v2(false), vec![1, 3, 0, 0, 0, 0xa1, 0xb2, 0xc3, 1, 2]].concat(),
        _ => panic!("unrecognized independent fixture profile"),
    }
}
