//! Failed reads retain their documented byte positions and allocation claims.

use solana_kit_core::codecs::{
    Count, Counted, Decoder, Error, FixedBytes, ItemSentinel, Limits, Number, Reader, Sequence,
    Termination, Unit,
};

#[test]
fn failed_item_consumes_a_claim_without_speculative_rollback() {
    let codec = ItemSentinel::new(FixedBytes::<2>, &[255], Termination::Required, 2).unwrap();
    let mut reader = Reader::new(&[1, 2, 3, 4, 255], Limits::new(8, 1)).unwrap();
    assert_eq!(codec.read(&mut reader), Err(Error::LimitExceeded));
    assert_eq!(reader.position(), 2);
    assert_eq!(reader.remaining(), [3, 4, 255]);
    assert_eq!(reader.claim(1), Err(Error::LimitExceeded));
    reader.seek(0).unwrap();
    assert_eq!(reader.claim(1), Err(Error::LimitExceeded));
}

#[test]
fn zero_width_counted_items_share_the_remaining_budget() {
    let codec = Counted {
        prefix: Number::<u32>::LE,
        item: Unit,
    };
    let mut reader = Reader::new(&[2, 0, 0, 0], Limits::new(4, 3)).unwrap();
    assert_eq!(codec.read(&mut reader).unwrap(), [(), ()]);
    assert_eq!(reader.position(), 4);
    reader.claim(1).unwrap();
    assert_eq!(reader.claim(1), Err(Error::LimitExceeded));
    let fixed = Sequence {
        item: Unit,
        count: Count::Fixed(4),
    };
    assert_eq!(
        fixed.decode(&[], Limits::new(0, 3)),
        Err(Error::LimitExceeded)
    );
}

#[test]
fn failed_frame_advances_outer_offset_and_retains_inner_claims() {
    let mut reader = Reader::new(&[1, 2, 9], Limits::new(3, 2)).unwrap();
    let result = reader.frame(2, |inner| {
        inner.claim(1)?;
        Number::<u8>::LE.read(inner)
    });
    assert_eq!(result, Err(Error::TrailingBytes));
    assert_eq!(reader.position(), 2);
    assert_eq!(reader.remaining(), [9]);
    reader.claim(1).unwrap();
    assert_eq!(reader.claim(1), Err(Error::LimitExceeded));
}
