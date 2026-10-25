use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Limits},
};
use solana_kit_loader::*;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

const LIMITS: Limits = Limits::new(4096, 4096);

/// Authority options keep fixed header allocation without accepting unknown tags.
#[cfg_attr(test, test)]
pub fn authority_header_literals_values_and_diagnostics_are_exact() {
    for authority in [None, Some(Address::from_bytes([73; 32]))] {
        let value = UpgradeableState::ProgramData {
            slot: u64::MAX,
            authority,
        };
        let mut bytes = vec![3, 0, 0, 0];
        bytes.extend_from_slice(&[255; 8]);
        bytes.push(u8::from(authority.is_some()));
        bytes.extend_from_slice(&[authority.map_or(0, |_| 73); 32]);
        assert_eq!(UpgradeableStateCodec.encode(&value, 4096).unwrap(), bytes);
        assert_eq!(UpgradeableStateCodec.decode(&bytes, LIMITS).unwrap(), value);

        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let mut cloned = DefaultHasher::new();
        value.clone().hash(&mut cloned);
        assert_eq!(hasher.finish(), cloned.finish());
        assert!(format!("{value}").contains("ProgramData"));
        bytes[12] = 2;
        assert!(UpgradeableStateCodec.decode(&bytes, LIMITS).is_err());
    }
}
