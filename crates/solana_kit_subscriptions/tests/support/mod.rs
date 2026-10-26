//! Nonzero independent primary wire values reused by instruction/account/event matrices.
use solana_kit_core::{AccountMeta, AccountRole, Address, codecs::Limits};
use solana_kit_subscriptions::*;

pub const NUMBER: u64 = 0x0807060504030201;
pub const SIGNED: i64 = 0x0807060504030201;
pub const WORD: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
pub const ADDRESS: Address = Address::from_bytes([77; 32]);
pub const LIMITS: Limits = Limits::new(8192, 8192);
pub const BUMP: u8 = 201;
pub const TEXT_WIDTH: usize = 128;
pub const ADDRESS_COUNT: usize = 4;

pub fn words(count: usize) -> Vec<u8> {
    WORD.repeat(count)
}

pub fn roles(codes: &str) -> Vec<AccountMeta> {
    codes
        .bytes()
        .map(|code| AccountMeta {
            address: ADDRESS,
            role: match code {
                b'r' => AccountRole::Readonly,
                b'w' => AccountRole::Writable,
                b's' => AccountRole::ReadonlySigner,
                b'S' => AccountRole::WritableSigner,
                _ => panic!("unknown independent privilege fixture"),
            },
        })
        .collect()
}

pub fn terms() -> PlanTerms {
    PlanTerms {
        amount: NUMBER,
        period_hours: NUMBER,
        created_at: SIGNED,
    }
}

pub fn plan_data() -> PlanData {
    PlanData {
        plan_id: NUMBER,
        mint: ADDRESS,
        terms: terms(),
        end_ts: SIGNED,
        destinations: vec![ADDRESS; ADDRESS_COUNT],
        pullers: vec![ADDRESS; ADDRESS_COUNT],
        metadata_uri: "x".repeat(TEXT_WIDTH),
    }
}

pub fn plan_bytes() -> Vec<u8> {
    let mut bytes = words(1);
    bytes.extend([77; 32]);
    bytes.extend(words(4));
    bytes.extend([77; 32].repeat(ADDRESS_COUNT * 2));
    bytes.extend([b'x'; TEXT_WIDTH]);
    bytes
}

pub fn transfer_data() -> TransferData {
    TransferData {
        amount: NUMBER,
        delegator: ADDRESS,
        mint: ADDRESS,
    }
}

pub fn transfer_bytes() -> Vec<u8> {
    let mut bytes = words(1);
    bytes.extend([77; 64]);
    bytes
}

pub fn header(discriminator: u8) -> Header {
    Header {
        discriminator,
        version: 1,
        bump: BUMP,
        delegator: ADDRESS,
        delegatee: ADDRESS,
        payer: ADDRESS,
        init_id: SIGNED,
    }
}

pub fn header_bytes(discriminator: u8) -> Vec<u8> {
    let mut bytes = vec![discriminator, 1, BUMP];
    bytes.extend([77; 96]);
    bytes.extend(WORD);
    bytes
}

pub fn check<T: PartialEq + core::fmt::Debug + core::fmt::Display + core::hash::Hash>(
    codec: impl solana_kit_core::codecs::Encoder<T> + solana_kit_core::codecs::Decoder<T>,
    value: &T,
    bytes: &[u8],
) {
    assert_eq!(codec.encode(value, LIMITS.max_bytes).unwrap(), bytes);
    let decoded = codec.decode(bytes, LIMITS).unwrap();
    assert_eq!(&decoded, value);
    assert_eq!(format!("{value}"), format!("{value:?}"));
    assert_eq!(hash(&decoded), hash(value));

    for end in 0..bytes.len() {
        assert!(codec.decode(&bytes[..end], LIMITS).is_err());
    }

    let mut trailing = bytes.to_vec();
    trailing.push(0);
    assert!(codec.decode(&trailing, LIMITS).is_err());
}

pub fn hash(value: &impl core::hash::Hash) -> u64 {
    let mut state = std::collections::hash_map::DefaultHasher::new();
    core::hash::Hash::hash(value, &mut state);
    core::hash::Hasher::finish(&state)
}
