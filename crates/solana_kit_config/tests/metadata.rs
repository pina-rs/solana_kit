use solana_kit_config::*;
use solana_kit_core::{
    Address,
    accounts::Account,
    codecs::{Encoder, Limits},
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

const LIMITS: Limits = Limits::new(4096, 4096);
const ADDRESS_BYTE: u8 = 17;
const OPAQUE_DATA: [u8; 3] = [0, 255, 13];

/// Config's data tail and full account metadata survive caller-owned fetching.
#[cfg_attr(test, test)]
pub fn fetched_config_value_equality_hash_and_diagnostics() {
    let address = Address::from_bytes([ADDRESS_BYTE; 32]);
    let value = ConfigData {
        keys: vec![
            ConfigKey {
                address,
                is_signer: true
            };
            2
        ],
        data: OPAQUE_DATA.to_vec(),
    };
    let data = ConfigCodec.encode(&value, 4096).unwrap();
    let account = Account {
        address,
        owner: PROGRAM,
        lamports: u64::MAX,
        executable: false,
        space: data.len() as u64,
        rent_epoch: None,
        data,
    };
    let decoded = decode_account(account.clone(), LIMITS).unwrap();

    assert_eq!(decoded.data, value);
    assert_eq!(decoded.address, address);
    assert_eq!(decoded.owner, PROGRAM);
    assert_eq!(decoded.lamports, u64::MAX);
    assert_eq!(decoded.space, account.space);
    assert!(!decoded.executable);
    assert_eq!(decoded.rent_epoch, None);

    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let mut clone = DefaultHasher::new();
    value.clone().hash(&mut clone);
    assert_eq!(hasher.finish(), clone.finish());
    assert!(format!("{value}").contains("ConfigData"));
}
