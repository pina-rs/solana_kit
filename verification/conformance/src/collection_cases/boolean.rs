//! Strict numeric booleans and explicitly ambiguous untagged absence patterns.

use solana_kit_core::codecs::{
    Absence, Bool, Boolean, Decoder, Encoder, Error, Limits, Nullable, Number,
};

const LIMITS: Limits = Limits::new(16, 8);
const WORD_BIT: u32 = 65_536;
const CAPACITY: usize = 8;
const TRAILING: &[u8] = &[9];

pub(super) fn custom_boolean_storage_rejects_every_other_word() {
    let codec = Boolean(Number::<u32>::BE);
    assert_eq!(codec.encode(&false, 4).unwrap(), [0, 0, 0, 0]);
    assert_eq!(codec.encode(&true, 4).unwrap(), [0, 0, 0, 1]);

    for value in [2, 255, WORD_BIT, u32::MAX] {
        assert_eq!(
            codec.decode(&value.to_be_bytes(), LIMITS),
            Err(Error::InvalidDiscriminator)
        );
    }

    for size in 0..4 {
        assert_eq!(codec.decode(&[0; 4][..size], LIMITS), Err(Error::Truncated));
    }
}

pub(super) fn untagged_absent_patterns_reserve_present_wire_values() {
    for (absence, value) in [
        (Absence::Zeroes(2), 0),
        (Absence::Constant(&[170, 187]), 48042),
    ] {
        let codec = Nullable {
            item: Number::<u16>::LE,
            prefix: None::<Bool>,
            absence,
        };
        let bytes = codec.encode(&Some(value), CAPACITY).unwrap();
        assert_eq!(codec.decode(&bytes, LIMITS).unwrap(), None);
        assert_eq!(
            codec
                .decode_prefix(&[bytes.as_slice(), TRAILING].concat(), 0, LIMITS)
                .unwrap(),
            (None, 2)
        );
        assert_eq!(
            codec.decode(&[bytes.as_slice(), TRAILING].concat(), LIMITS),
            Err(Error::TrailingBytes)
        );
        assert_eq!(codec.decode(&bytes[..1], LIMITS), Err(Error::Truncated));
    }
}
