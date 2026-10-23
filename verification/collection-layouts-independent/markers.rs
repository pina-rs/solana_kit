//! Independent multi-byte collection boundaries and ambiguous marker values.

use solana_kit_core::codecs::{
    Decoder, Encoder, Error, FixedBytes, ItemSentinel, Limits, Termination,
};

const LIMITS: Limits = Limits::new(32, 8);
const END_OFFSET: usize = 7;

#[test]
fn multibyte_markers_match_only_complete_item_boundaries() {
    for policy in [
        Termination::Required,
        Termination::Optional,
        Termination::Omitted,
    ] {
        let codec = ItemSentinel::new(FixedBytes::<2>, &[254, 253], policy, 2).unwrap();
        let payload = [[1, 254], [253, 2]];
        let expected = if policy == Termination::Omitted {
            vec![1, 254, 253, 2]
        } else {
            vec![1, 254, 253, 2, 254, 253]
        };

        assert_eq!(codec.encode(&payload[..], 32).unwrap(), expected);
        assert_eq!(
            codec.decode(&[1, 254, 253, 2, 254, 253], LIMITS).unwrap(),
            payload
        );
        assert_eq!(
            codec.decode_prefix(&framed_payload(), 1, LIMITS).unwrap(),
            (payload.to_vec(), END_OFFSET)
        );
        assert_eq!(
            codec.decode(&[254, 253], LIMITS).unwrap(),
            Vec::<[u8; 2]>::new()
        );
        assert_eq!(codec.decode(&[254], LIMITS), Err(Error::Truncated));
        assert_eq!(codec.decode(&[1, 2, 254], LIMITS), Err(Error::Truncated));
        assert_eq!(
            codec.decode(&[], LIMITS).is_ok(),
            policy != Termination::Required
        );
        assert_eq!(
            codec.decode(&[1, 2], LIMITS).is_ok(),
            policy != Termination::Required
        );
    }
}

/// Keep the complete literal frame separate from call argument structure.
fn framed_payload() -> [u8; 8] {
    [9, 1, 254, 253, 2, 254, 253, 7]
}

#[test]
fn marker_valued_item_is_the_documented_reserved_representation() {
    let codec = ItemSentinel::new(FixedBytes::<2>, &[254, 253], Termination::Optional, 2).unwrap();
    let bytes = codec.encode(&[[254, 253]][..], 8).unwrap();
    assert_eq!(bytes, [254, 253, 254, 253]);
    assert_eq!(codec.decode(&bytes, LIMITS), Err(Error::TrailingBytes));
    assert_eq!(
        codec.decode_prefix(&bytes, 0, LIMITS).unwrap(),
        (Vec::<[u8; 2]>::new(), 2)
    );
}
