use solana_kit_core::{
    Address,
    codecs::{Decoder, Limits},
};
use solana_kit_stake::*;
use std::{
    collections::hash_map::DefaultHasher,
    fmt::Display,
    hash::{Hash, Hasher},
};

const LIMITS: Limits = Limits::new(4096, 4096);
const STATE_SAMPLE_BYTES: usize = 197;

fn hash(value: &impl Hash) -> u64 {
    let mut state = DefaultHasher::new();
    value.hash(&mut state);
    state.finish()
}

fn value<T: Hash + Clone + Eq + Display>(item: T) {
    assert_eq!(hash(&item), hash(&item.clone()));
    assert!(!item.to_string().is_empty());
}

/// All source data families support consistent value hashing and Rust diagnostics.
#[cfg_attr(test, test)]
pub fn source_value_traits_retain_payload_and_variant_identity() {
    primitive_values();
    parameter_values();
    state_values();
}

fn primitive_values() {
    let address = Address::from_bytes([7; 32]);
    value(Epoch { value: u64::MAX });
    value(UnixTimestamp { value: i64::MIN });
    value(StakeFlags { bits: u8::MAX });
    value(Authorized {
        staker: address,
        withdrawer: address,
    });
    assert_ne!(hash(&Epoch { value: 0 }), hash(&Epoch { value: 1 }));
}

fn parameter_values() {
    let address = Address::from_bytes([7; 32]);
    value(AuthorizeWithSeed {
        new_authorized_pubkey: address,
        stake_authorize: StakeAuthorize::Withdrawer,
        authority_seed: "é".into(),
        authority_owner: address,
    });
    value(AuthorizeCheckedWithSeed {
        stake_authorize: StakeAuthorize::Staker,
        authority_seed: "é".into(),
        authority_owner: address,
    });
    value(SetLockup {
        unix_timestamp: None,
        epoch: Some(Epoch { value: 9 }),
        custodian: Some(address),
    });
    value(SetLockupChecked {
        unix_timestamp: Some(UnixTimestamp { value: -1 }),
        epoch: None,
    });
}

fn state_values() {
    for tag in 0_u8..4 {
        let mut bytes = vec![0; STATE_SAMPLE_BYTES];
        bytes[0] = tag;
        let state = StakeStateV2Codec
            .decode_prefix(&bytes, 0, LIMITS)
            .unwrap()
            .0;
        value(StakeStateAccount {
            state: state.clone(),
        });
        value(state);
        let old = StakeStateCodec.decode_prefix(&bytes, 0, LIMITS).unwrap().0;
        value(old);
    }

    assert_ne!(
        hash(&StakeAuthorize::Staker),
        hash(&StakeAuthorize::Withdrawer)
    );
    assert_ne!(
        hash(&StakeStateV2::Uninitialized),
        hash(&StakeStateV2::RewardsPool)
    );
}
