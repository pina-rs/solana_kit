//! Nullable absence policies in dirty storage and at nonzero offsets.

use solana_kit_core::codecs::{
    Absence, Bool, Decoder, Encoder, Error, FixedBytes, Limits, Nullable, Number, OptionCodec,
    Reader, Writer,
};

/// Cross every presence policy with absent/present values to protect dirty writes.
#[test]
fn dirty_writes_preserve_surroundings_for_every_presence_policy() {
    for prefix in [Some(Bool), None] {
        for (absence, raw) in [
            (Absence::Omit, vec![]),
            (Absence::Zeroes(2), vec![0, 0]),
            (Absence::Constant(&[170, 187]), vec![170, 187]),
        ] {
            let codec = Nullable {
                item: FixedBytes::<2>,
                prefix,
                absence,
            };

            for value in [None, Some([1, 2])] {
                check_dirty(&codec, value, &raw);
            }
        }
    }
}

/// Check literal payload and unchanged surroundings for one policy/value pair.
fn check_dirty(codec: &Nullable<'_, FixedBytes<2>, Bool>, value: Option<[u8; 2]>, raw: &[u8]) {
    let mut expected = Vec::new();
    let mut dirty = [238; 12];

    if codec.prefix.is_some() {
        expected.push(u8::from(value.is_some()));
    }

    expected.extend(value.map_or_else(|| raw.to_vec(), |value| value.to_vec()));
    let mut writer = Writer::new(&mut dirty);
    writer.seek(2).unwrap();
    codec.write(&value, &mut writer).unwrap();
    assert_eq!(writer.position(), 2 + expected.len());
    assert_eq!(&dirty[2..2 + expected.len()], expected);
    assert_eq!(&dirty[..2], [238, 238]);
    assert!(dirty[2 + expected.len()..].iter().all(|byte| *byte == 238));

    assert_eq!(
        codec
            .decode_prefix(&dirty[..2 + expected.len()], 2, Limits::new(12, 8))
            .unwrap(),
        (value, 2 + expected.len())
    );
}

#[test]
fn zero_present_frames_fail_exactly_and_absent_constants_are_validated() {
    let codec = Nullable {
        item: Number::<u8>::LE,
        prefix: Some(Bool),
        absence: Absence::Zeroes(2),
    };
    let mut reader = Reader::new(&[1, 3, 4, 9], Limits::new(4, 8)).unwrap();
    assert_eq!(codec.read(&mut reader), Err(Error::TrailingBytes));
    assert_eq!(reader.position(), 3);
    assert_eq!(reader.remaining(), [9]);
    assert_eq!(codec.decode(&[0, 3, 4], Limits::new(3, 0)).unwrap(), None);
    assert_eq!(
        OptionCodec::c_option(Number::<u8>::LE, 2).decode(&[1, 0, 0, 0, 3, 4], Limits::new(6, 0)),
        Err(Error::TrailingBytes)
    );

    let constant = Nullable {
        item: Number::<u8>::LE,
        prefix: Some(Bool),
        absence: Absence::Constant(&[170, 187]),
    };
    assert_eq!(
        constant.decode(&[0, 170], Limits::new(3, 0)),
        Err(Error::Truncated)
    );
    assert_eq!(
        constant.decode(&[0, 170, 188], Limits::new(3, 0)),
        Err(Error::InvalidDiscriminator)
    );
}
