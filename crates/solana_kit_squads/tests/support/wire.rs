//! Nonzero independent literal framing, shared by instruction and account golden cases.
pub fn hex(text: &str) -> Vec<u8> {
    const HEX_RADIX: u32 = 16;
    text.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(core::str::from_utf8(pair).unwrap(), HEX_RADIX).unwrap())
        .collect()
}

pub fn spending() -> Vec<u8> {
    [
        vec![7; 32],
        vec![2],
        vec![8; 32],
        0x0102_0304_0506_0708u64.to_le_bytes().to_vec(),
        vec![1, 1, 0, 0, 0],
        vec![9; 32],
        vec![1, 0, 0, 0],
        vec![10; 32],
    ]
    .concat()
}

pub fn actions() -> Vec<u8> {
    const ADD_SPENDING_LIMIT_TAG: u8 = 5;
    [
        vec![7, 0, 0, 0, 0],
        vec![8; 32],
        vec![7, 1],
        vec![9; 32],
        vec![2, 1, 0, 3, 17, 0, 0, 0, 4],
        spending(),
        vec![ADD_SPENDING_LIMIT_TAG],
        vec![11; 32],
        vec![6, 1],
        vec![12; 32],
    ]
    .concat()
}
