//! Lookup origin, public text collation and explicit numeric adaptation.

use alloc::{string::ToString, vec::Vec};
use core::{
    cmp::Ordering,
    hash::{Hash as HashTrait, Hasher},
};
use solana_kit_core::{
    AccountLookupMeta, AccountRole, Address, Hash, LookupRole, Signature,
    codecs::{Decoder, Encoder, Limits, Mapped, Number},
    compare_addresses,
};

pub fn lookup_metadata() {
    let address = Address::from_bytes([7; 32]);
    let table = Address::from_bytes([8; 32]);
    let lookup = AccountLookupMeta {
        address,
        lookup_table_address: table,
        address_index: 255,
        role: LookupRole::Writable,
    };
    assert_eq!(lookup.account_meta().address, address);
    assert_eq!(lookup.account_meta().role, AccountRole::Writable);
    assert!(!lookup.account_meta().role.is_signer());
    let readonly = AccountLookupMeta {
        role: LookupRole::Readonly,
        ..lookup
    };
    assert_eq!(readonly.account_meta().role, AccountRole::Readonly);
    assert_ne!(lookup, readonly);
    assert_ne!(
        lookup,
        AccountLookupMeta {
            address_index: 254,
            ..lookup
        }
    );
    assert_ne!(
        lookup,
        AccountLookupMeta {
            lookup_table_address: address,
            ..lookup
        }
    );
    assert_ne!(record_hash(&lookup), record_hash(&readonly));
}

#[derive(Default)]
struct TraceHasher(u64);

impl Hasher for TraceHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 = self.0.wrapping_mul(257).wrapping_add(u64::from(*byte) + 1);
        }
    }
}

fn record_hash(value: &impl HashTrait) -> u64 {
    let mut hasher = TraceHasher::default();
    value.hash(&mut hasher);
    hasher.finish()
}

pub fn text_collation_and_display() {
    let alphabet = "123456789aAbBcCdDeEfFgGhHijJkKLmMnNopPqQrRsStTuUvVwWxXyYzZ";
    let mut addresses: Vec<Address> = alphabet
        .chars()
        .map(|character| {
            alloc::format!("1111111111111111111111111111111{character}")
                .parse()
                .unwrap()
        })
        .collect();
    let expected = addresses.clone();
    addresses.reverse();
    addresses.sort_by(compare_addresses);
    assert_eq!(addresses, expected);
    assert_eq!(
        compare_addresses(&addresses[0], &addresses[0]),
        Ordering::Equal
    );
    let short = "5111111111111111111111111111111111111111111"
        .parse()
        .unwrap();
    let long = "51111111111111111111111111111111111111111111"
        .parse()
        .unwrap();
    assert_eq!(compare_addresses(&short, &long), Ordering::Less);
    let text = "11111111111111111111111111111111";
    assert_eq!(Address::from_bytes([0; 32]).to_string(), text);
    assert_eq!(Hash::from_bytes([0; 32]).to_string(), text);
    assert_eq!(Signature::from_bytes([0; 64]).to_string(), "1".repeat(64));
    assert_eq!(
        record_hash(&short),
        record_hash(&short.to_string().parse::<Address>().unwrap())
    );
}

pub fn floating_conversion() {
    let codec = Mapped::new(
        Number::<f32>::LE,
        |value: &f64| Ok(*value as f32),
        |value: f32| Ok(f64::from(value)),
    );
    let literals = [
        (f64::MAX, [0, 0, 128, 127]),
        (-f64::MAX, [0, 0, 128, 255]),
        (f64::MIN_POSITIVE, [0, 0, 0, 0]),
        (-0.0, [0, 0, 0, 128]),
        (16_777_217.0, [0, 0, 128, 75]),
    ];

    for (value, expected) in literals {
        assert_eq!(codec.encode(&value, 4).unwrap(), expected);
    }

    let negative_zero = codec.decode(&[0, 0, 0, 128], Limits::new(4, 0)).unwrap();
    assert_eq!(negative_zero.to_bits(), (-0.0f64).to_bits());
    assert!(
        codec
            .decode(&[0, 0, 192, 127], Limits::new(4, 0))
            .unwrap()
            .is_nan()
    );
    assert_eq!(codec.encode(&f64::NAN, 4).unwrap().len(), 4);
}
