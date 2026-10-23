//! Checked count domains distinguish empty input, partial prefixes and overflow.

use solana_kit_core::codecs::{
    Counted, Decoder, Encoder, Error, Limits, Mapped, Number, OptionalCount, Reader, Unit,
};

const WIDTH: usize = 8;

pub(super) fn wide_count_mapping_checks_before_zero_width_allocation() {
    let mapped = Mapped::new(
        Number::<u64>::BE,
        |value: &u32| Ok(u64::from(*value)),
        |value| u32::try_from(value).map_err(|_| Error::Overflow),
    );
    let codec = Counted {
        prefix: OptionalCount(mapped),
        item: Unit,
    };
    assert_eq!(codec.decode(&[], Limits::new(8, 0)).unwrap(), [(); 0]);
    assert_eq!(
        codec.encode(&[(), (), ()][..], WIDTH).unwrap(),
        [0, 0, 0, 0, 0, 0, 0, 3]
    );

    for size in 1..8 {
        let partial = [0; 8];
        let mut reader = Reader::new(&partial[..size], Limits::new(8, 0)).unwrap();
        assert_eq!(codec.read(&mut reader), Err(Error::Truncated));
        assert_eq!(reader.position(), 0);
    }

    let mut reader = Reader::new(&[0, 0, 0, 1, 0, 0, 0, 0], Limits::new(8, 1)).unwrap();
    assert_eq!(codec.read(&mut reader), Err(Error::Overflow));
    assert_eq!(reader.position(), WIDTH);
    reader.claim(1).unwrap();
    let maximum = [0, 0, 0, 0, 255, 255, 255, 255];
    assert_eq!(
        codec.decode(&maximum, Limits::new(8, 3)),
        Err(Error::LimitExceeded)
    );
}
