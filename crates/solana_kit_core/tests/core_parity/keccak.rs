//! Original Keccak literals from the pinned reference, distinct from SHA3 padding.

use solana_kit_core::keccak::keccak256;

fn literal(text: &str) -> [u8; 32] {
    let mut output = [0; 32];

    for (index, pair) in text.as_bytes().as_chunks::<2>().0.iter().enumerate() {
        let text = core::str::from_utf8(pair).unwrap();
        output[index] = u8::from_str_radix(text, 16).unwrap();
    }

    output
}

pub fn keccak_literals() {
    let empty = literal("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470");
    let abc = literal("4e03657aea45a94fc7d47ba826c8d667c0d1e6e33a64a036ec44f58fa12d6c45");
    let zero = literal("bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a");
    let fox = literal("4d741b6f1eb29cb2a9b9911c82f56fa8d73b04959d3d9d222895df6c0b28aa15");

    assert_eq!(keccak256(&[]).to_bytes(), empty);
    assert_eq!(keccak256(&[b"abc"]).to_bytes(), abc);
    assert_eq!(keccak256(&[b"a", b"", b"bc"]).to_bytes(), abc);
    assert_eq!(keccak256(&[&[0]]).to_bytes(), zero);
    assert_eq!(
        keccak256(&[b"The quick brown fox jumps over the lazy dog"]).to_bytes(),
        fox
    );
    let sha3 = literal("a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a");
    assert_ne!(keccak256(&[]).to_bytes(), sha3);
}
