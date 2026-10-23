use solana_kit_core::codecs::{
    Absence, Bool, Boolean, Count, Counted, Decoder, Encoder, Error, FixedBytes, ItemSentinel,
    Limits, Mapped, Nullable, Number, OptionCodec, OptionalCount, Reader, Sentinel, Sequence,
    Termination, Unit, Writer,
};

const LIMITS: Limits = Limits::new(128, 32);
const PAYLOAD: u16 = 513;
const CAPACITY: usize = 64;
const ABSENT: u8 = 170;

#[test]
fn existing_byte_delimiters_and_option_defaults_are_preserved() {
    let item = FixedBytes::<2>;
    let original = Sentinel::dart(
        Sequence {
            item,
            count: Count::Remainder,
        },
        &[255],
        8,
    )
    .unwrap();
    assert_eq!(
        original.decode(&[1, 255, 255], LIMITS),
        Err(Error::Truncated)
    );
    assert_eq!(
        OptionCodec::u8(Number::<u8>::LE).decode(&[0, 170], LIMITS),
        Err(Error::TrailingBytes)
    );
}

#[test]
fn boundary_markers_preserve_existing_byte_delimiters() {
    let item = FixedBytes::<2>;

    for policy in [
        Termination::Required,
        Termination::Optional,
        Termination::Omitted,
    ] {
        let codec = ItemSentinel::new(item, &[255], policy, 2).unwrap();
        let expected = if policy == Termination::Omitted {
            vec![1, 255]
        } else {
            vec![1, 255, 255]
        };

        assert_eq!(codec.encode(&[[1, 255]][..], 8).unwrap(), expected);
        assert_eq!(codec.decode(&[1, 255, 255], LIMITS).unwrap(), [[1, 255]]);
        assert_eq!(
            codec.decode_prefix(&[1, 255, 255, 9], 0, LIMITS).unwrap(),
            (vec![[1, 255]], 3)
        );
        assert_eq!(
            codec.decode(&[1, 255, 255, 9], LIMITS),
            Err(Error::TrailingBytes)
        );
        assert_eq!(codec.decode(&[1], LIMITS), Err(Error::Truncated));
        assert_eq!(
            codec.decode(&[1, 255], LIMITS).is_ok(),
            policy != Termination::Required
        );
        assert_eq!(
            codec.decode(&[], LIMITS).is_ok(),
            policy != Termination::Required
        );
        assert_eq!(codec.decode(&[255], LIMITS).unwrap(), Vec::<[u8; 2]>::new());
    }
}

#[test]
fn marker_limits_reject_nonprogress_and_truncated_items() {
    assert!(ItemSentinel::new(Unit, &[255], Termination::Required, 0).is_err());
    assert!(ItemSentinel::new(Bool, &[], Termination::Required, 1).is_err());
    assert!(ItemSentinel::new(Bool, &[255, 255], Termination::Optional, 1).is_err());
    let bad = ItemSentinel::new(Unit, &[255], Termination::Required, 1).unwrap();
    assert_eq!(bad.decode(&[1], LIMITS), Err(Error::InvalidValue));
    assert_eq!(bad.encode(&[()][..], 8), Err(Error::InvalidValue));
    let codec = ItemSentinel::new(FixedBytes::<2>, &[255], Termination::Required, 2).unwrap();
    assert_eq!(
        codec.decode(&[1, 2, 255], Limits::new(8, 0)),
        Err(Error::LimitExceeded)
    );

    assert_eq!(
        codec.decode(&[1, 2, 255], Limits::new(2, 3)),
        Err(Error::LimitExceeded)
    );
    let nested = Sequence {
        item: codec,
        count: Count::Fixed(1),
    };
    assert_eq!(
        nested.decode(&[1, 2, 255], Limits::new(8, 1)),
        Err(Error::LimitExceeded)
    );
}

#[test]
fn configured_absence_consumes_and_validates_literal_patterns() {
    for prefix in [Some(Bool), None] {
        for absence in [
            Absence::Omit,
            Absence::Zeroes(2),
            Absence::Constant(&[170, 187]),
        ] {
            check_absence(prefix, absence);
        }
    }
}

fn check_absence(prefix: Option<Bool>, absence: Absence<'_>) {
    let codec = Nullable {
        item: Number::<u16>::LE,
        prefix,
        absence,
    };

    for value in [None, Some(PAYLOAD)] {
        let bytes = codec.encode(&value, CAPACITY).unwrap();
        assert_eq!(codec.decode(&bytes, LIMITS).unwrap(), value);
        let mut dirty = [255; 16];
        let mut writer = Writer::new(&mut dirty);
        codec.write(&value, &mut writer).unwrap();
        assert_eq!(writer.position(), bytes.len());

        assert_eq!(&dirty[..bytes.len()], bytes);
        assert!(dirty[bytes.len()..].iter().all(|byte| *byte == 255));
    }
}

#[test]
fn literal_absence_rejects_missing_and_invalid_bytes() {
    let constant = Nullable {
        item: Number::<u8>::LE,
        prefix: Some(Bool),
        absence: Absence::Constant(&[ABSENT]),
    };
    assert_eq!(constant.decode(&[0, 170], LIMITS).unwrap(), None);
    assert_eq!(
        constant.decode(&[0, 171], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(constant.decode(&[0], LIMITS), Err(Error::Truncated));
    assert_eq!(constant.decode(&[1], LIMITS), Err(Error::Truncated));
    assert_eq!(
        constant.decode(&[2, 170], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(constant.decode(&[], LIMITS), Err(Error::Truncated));
    assert_eq!(
        constant.decode_prefix(&[9, 0, 170, 7], 1, LIMITS).unwrap(),
        (None, 3)
    );
}

#[test]
fn zero_absence_and_custom_boolean_storage_follow_source() {
    let codec = Nullable {
        item: Number::<u16>::LE,
        prefix: Some(Boolean(Number::<u32>::BE)),
        absence: Absence::Zeroes(2),
    };
    assert_eq!(codec.encode(&Some(513), 16).unwrap(), [0, 0, 0, 1, 1, 2]);
    assert_eq!(codec.decode(&[0, 0, 0, 0, 255, 255], LIMITS).unwrap(), None);
    assert_eq!(
        codec.decode(&[0, 0, 0, 0, 0], LIMITS),
        Err(Error::Truncated)
    );
    assert_eq!(
        codec.decode(&[0, 0, 0, 2, 0, 0], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    let raw = Nullable {
        item: Number::<u16>::LE,
        prefix: None::<Bool>,
        absence: Absence::Zeroes(2),
    };
    assert_eq!(raw.decode(&[0, 0], LIMITS).unwrap(), None);
    assert_eq!(raw.decode(&[1, 0], LIMITS).unwrap(), Some(1));

    assert_eq!(raw.decode(&[0], LIMITS), Err(Error::Truncated));
    let bad = Nullable {
        item: Number::<u8>::LE,
        prefix: Some(Bool),
        absence: Absence::Zeroes(2),
    };
    assert_eq!(bad.encode(&Some(1), 8), Err(Error::InvalidValue));
}

#[test]
fn validated_wide_count_and_exhaustion_policy_share_claims() {
    let prefix = Mapped::new(
        Number::<u64>::BE,
        |value: &u32| Ok(u64::from(*value)),
        |value| u32::try_from(value).map_err(|_| Error::Overflow),
    );
    let codec = Counted {
        prefix: OptionalCount(prefix),
        item: Unit,
    };
    assert_eq!(
        codec.encode(&[(), ()][..], CAPACITY).unwrap(),
        [0, 0, 0, 0, 0, 0, 0, 2]
    );
    assert_eq!(codec.decode(&[], LIMITS).unwrap(), [(); 0]);
    assert_eq!(codec.decode(&[0], LIMITS), Err(Error::Truncated));
    assert_eq!(codec.decode(&[255; 8], LIMITS), Err(Error::Overflow));
    assert_eq!(
        codec.decode(&[0, 0, 0, 0, 255, 255, 255, 255], LIMITS),
        Err(Error::LimitExceeded)
    );
    let bytes = [0, 0, 0, 0, 0, 0, 0, 2];

    let mut reader = Reader::new(&bytes, Limits::new(8, 1)).unwrap();

    assert_eq!(codec.read(&mut reader), Err(Error::LimitExceeded));
    assert_eq!(reader.position(), 8);
}

#[test]
fn fresh_dart_literals_match_both_optional_value_families() {
    let corpus = include_str!("fixtures/collection-layouts-dart.tsv");
    assert_eq!(corpus.lines().count(), 35);

    for line in corpus.lines() {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 4);
        let expected = parse_bytes(fields[3]);

        check_oracle_row(&fields, &expected);
    }
}

fn check_oracle_row(fields: &[&str], expected: &[u8]) {
    match fields[0] {
        "array" => check_array(fields, expected),
        "nullable" | "option" => check_optional(fields, expected),
        "boolean" => {
            let value = fields[2] == "1";
            let codec = Boolean(Number::<u32>::LE);
            assert_eq!(codec.encode(&value, CAPACITY).unwrap(), expected);
            assert_eq!(codec.decode(expected, LIMITS).unwrap(), value);
        }
        _ => panic!("unknown family"),
    }
}

fn check_array(fields: &[&str], expected: &[u8]) {
    let policy = match fields[1] {
        "0" => Termination::Required,
        "1" => Termination::Optional,
        "2" => Termination::Omitted,
        _ => panic!("unknown termination"),
    };

    let values: Vec<u16> = fields[2]
        .split(',')
        .filter(|value| !value.is_empty())
        .map(|value| value.parse().unwrap())
        .collect();
    let codec = ItemSentinel::new(Number::<u16>::LE, &[255], policy, 2).unwrap();
    assert_eq!(codec.encode(values.as_slice(), CAPACITY).unwrap(), expected);
    assert_eq!(codec.decode(expected, LIMITS).unwrap(), values);
}

fn check_optional(fields: &[&str], expected: &[u8]) {
    let tagged = fields[1].starts_with('1');

    let absence = match fields[1].as_bytes()[2] {
        b'0' => Absence::Omit,
        b'1' => Absence::Zeroes(2),
        b'2' => Absence::Constant(&[170, 187]),
        _ => panic!("unknown absence"),
    };
    let value = if fields[2] == "none" {
        None
    } else {
        Some(fields[2].parse::<u16>().unwrap())
    };

    let codec = Nullable {
        item: Number::<u16>::LE,
        prefix: tagged.then_some(Bool),
        absence,
    };
    assert_eq!(codec.encode(&value, CAPACITY).unwrap(), expected);
    assert_eq!(codec.decode(expected, LIMITS).unwrap(), value);
}

fn parse_bytes(text: &str) -> Vec<u8> {
    if text == "-" {
        return Vec::new();
    }

    text.split(',')
        .map(|value| value.parse().unwrap())
        .collect()
}
