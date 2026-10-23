//! Declared minimum widths guard against nonprogress and underconsumption.

use solana_kit_core::codecs::{
    Decoder, Encoder, Error, ItemSentinel, Limits, Number, Reader, Termination, Unit,
};

#[test]
fn false_minimum_is_rejected_after_the_actual_decoder_progress() {
    let codec = ItemSentinel::new(Number::<u8>::LE, &[255], Termination::Required, 2).unwrap();
    let mut reader = Reader::new(&[1, 2, 255], Limits::new(3, 2)).unwrap();
    assert_eq!(codec.read(&mut reader), Err(Error::InvalidValue));
    assert_eq!(reader.position(), 1);
    assert_eq!(reader.remaining(), [2, 255]);
    reader.claim(1).unwrap();
    assert_eq!(reader.claim(1), Err(Error::LimitExceeded));
    assert_eq!(codec.encode(&[1][..], 8), Err(Error::InvalidValue));
}

#[test]
fn nonprogress_and_configuration_failures_are_bounded() {
    let codec = ItemSentinel::new(Unit, &[255], Termination::Required, 1).unwrap();
    assert_eq!(
        codec.decode(&[1], Limits::new(1, 1)),
        Err(Error::InvalidValue)
    );
    assert!(ItemSentinel::new(Unit, &[255], Termination::Optional, 0).is_err());
    assert!(ItemSentinel::new(Unit, &[], Termination::Required, 1).is_err());
    assert!(ItemSentinel::new(Unit, &[1, 2], Termination::Omitted, 1).is_err());
}
