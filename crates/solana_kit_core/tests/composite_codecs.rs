use solana_kit_core::codecs::{
    BitArray, Bytes, Checked, Choice, Count, Counted, Decoder, Dependent, Either, Encoder, Error,
    Limits, Literal, Mapped, Number, OwnedSequence, Padded, PairError, Paired, Reader, Sentinel,
    Sequence, Tuple, Unit, Writer,
};

const MAX_WIRE: usize = 4096;
const LIMITS: Limits = Limits::new(MAX_WIRE, MAX_WIRE);
const SELECTED: u8 = 42;
const U16_MAX: u16 = u16::MAX;

fn narrow(value: &u32) -> Result<u8, Error> {
    u8::try_from(*value).map_err(|_| Error::Overflow)
}

fn widen(value: u8) -> Result<u32, Error> {
    Ok(u32::from(value))
}

fn bytes(field: &str) -> Vec<u8> {
    if field == "-" {
        return vec![];
    }

    field.split(',').map(|byte| byte.parse().unwrap()).collect()
}

fn golden_bits(fields: &[&str]) {
    const SOURCE_BITS: usize = 27;
    const BITS_PER_BYTE: usize = 8;
    let codec = BitArray {
        bytes: fields[1].parse().unwrap(),
        backward: fields[2] == "1",
    };
    let value: u8 = fields[3].parse().unwrap();
    let bits: Vec<_> = (0..SOURCE_BITS)
        .map(|index| value & (1 << (index % BITS_PER_BYTE)) != 0)
        .collect();
    let wire = bytes(fields[4]);
    let text = if fields[5] == "-" { "" } else { fields[5] };
    let expected: Vec<_> = text.bytes().map(|bit| bit == b'1').collect();
    assert_eq!(codec.encode(&bits, MAX_WIRE).unwrap(), wire);

    assert_eq!(codec.decode(&wire, LIMITS).unwrap(), expected);
}

fn golden_count(fields: &[&str], number: usize) {
    const COUNT_CYCLE: usize = 9;
    let prefix = Mapped::new(Number::<u8>::LE, narrow, widen);
    let counted = Counted {
        prefix,
        item: Number::<u8>::LE,
    };
    let values: Vec<_> = (0..number % COUNT_CYCLE)
        .map(|index| (number + index) as u8)
        .collect();
    let wire = bytes(fields[2]);
    assert_eq!(counted.encode(&values, MAX_WIRE).unwrap(), wire);
    assert_eq!(counted.decode(&wire, LIMITS).unwrap(), values);
}

fn golden_literal(fields: &[&str], number: usize) {
    const VARIANTS: &[&str] = &["a", "b", "c", "a"];
    let index = Mapped::new(
        Number::<u16>::LE,
        |value: &u32| u16::try_from(*value).map_err(|_| Error::Overflow),
        |value: u16| Ok(u32::from(value)),
    );
    let literal = Literal {
        variants: VARIANTS,
        index,
    };
    let value = VARIANTS[number % 3];
    let wire = bytes(fields[2]);
    assert_eq!(literal.encode(&value, MAX_WIRE).unwrap(), wire);
    assert_eq!(literal.decode(&wire, LIMITS).unwrap(), value);
}

fn golden_mapping(fields: &[&str], number: usize) {
    let mapped = Mapped::new(
        Number::<u16>::LE,
        |text: &String| text.parse::<u16>().map_err(|_| Error::InvalidValue),
        |number: u16| Ok(number.to_string()),
    );
    let wire = bytes(fields[2]);
    assert_eq!(mapped.encode(&number.to_string(), MAX_WIRE).unwrap(), wire);
    assert_eq!(mapped.decode(&wire, LIMITS).unwrap(), number.to_string());
}

fn golden_row(fields: &[&str]) {
    let number: usize = fields[1].parse().unwrap();
    let padded = Padded {
        left: 2,
        item: Number::<u16>::LE,
        right: 3,
    };
    let sentinel = Sentinel::new(Number::<u16>::LE, &[254, 255], 2).unwrap();

    match fields[0] {
        "bits" => golden_bits(fields),
        "pad" => assert_eq!(
            padded.encode(&(number as u16), MAX_WIRE).unwrap(),
            bytes(fields[2])
        ),
        "map" => golden_mapping(fields, number),
        "sentinel" => assert_eq!(
            sentinel.encode(&(number as u16), MAX_WIRE).unwrap(),
            bytes(fields[2])
        ),
        "count" => golden_count(fields, number),
        "literal" => golden_literal(fields, number),
        _ => panic!("unknown golden family"),
    }
}

#[test]
fn actual_dart_composition_goldens() {
    const GOLDEN_COUNT: usize = 3840;
    let mut rows = 0;

    for line in include_str!("fixtures/composite-dart.tsv").lines() {
        golden_row(&line.split('\t').collect::<Vec<_>>());
        rows += 1;
    }

    assert_eq!(rows, GOLDEN_COUNT);
}

#[test]
fn mappings_and_pairing_preserve_failures() {
    let mapped = Mapped::new(Number::<u8>::LE, narrow, widen);
    assert_eq!(mapped.encode(&256, 10), Err(Error::Overflow));
    assert_eq!(mapped.decode(&[255], LIMITS), Ok(255));
    assert_eq!(mapped.encode(&7, 4), Ok(vec![7]));
    assert_eq!(
        Number::<u32>::LE.decode(&[1], LIMITS),
        Err(Error::Truncated)
    );
    assert!(matches!(
        Paired::try_new(mapped, Number::<u32>::LE),
        Err(PairError::FixedSizeMismatch {
            encoder: 1,
            decoder: 4
        })
    ));
    let paired = Paired::try_new(mapped, Number::<u8>::LE).unwrap();
    assert_eq!(paired.encode(&7u32, 4), Ok(vec![7]));
    assert_eq!(paired.decode_prefix(&[99, 7, 88], 1, LIMITS), Ok((7u8, 2)));
    assert_eq!(paired.decode(&[7, 88], LIMITS), Err(Error::TrailingBytes));
}

fn reject_nine(value: &u8) -> Result<(), Error> {
    if *value == 9 {
        return Err(Error::InvalidValue);
    }

    Ok(())
}

#[test]
fn observers_preserve_failure_cursor_state() {
    let calls = core::cell::Cell::new(0);
    let checked = Checked {
        item: Number::<u8>::LE,
        before_write: |value: &u8| {
            calls.set(calls.get() + 1);

            reject_nine(value)
        },
        after_read: reject_nine,
    };
    assert_eq!(checked.encoded_len(&9), Ok(1));
    assert_eq!(calls.get(), 0);
    let mut output = [99; 3];
    let mut writer = Writer::new(&mut output);
    writer.put(&[88]).unwrap();
    assert_eq!(checked.write(&9, &mut writer), Err(Error::InvalidValue));
    assert_eq!(writer.position(), 1);
    assert_eq!(output, [88, 99, 99]);

    let mut reader = Reader::new(&[9, 4], LIMITS).unwrap();
    assert_eq!(checked.read(&mut reader), Err(Error::InvalidValue));
    assert_eq!(reader.position(), 1);
    assert_eq!(calls.get(), 1);
}

#[test]
fn bits_literals_unit_and_tagged_choices_reject_bad_boundaries() {
    let codec = BitArray {
        bytes: 2,
        backward: true,
    };
    assert_eq!(codec.encode(&[true], 2), Ok(vec![0, 1]));
    assert_eq!(
        codec.decode(&[0, 1], Limits::new(2, 15)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(codec.decode(&[1], LIMITS), Err(Error::Truncated));
    assert_eq!(
        BitArray {
            bytes: usize::MAX,
            backward: false
        }
        .encoded_len(&[]),
        Err(Error::Overflow)
    );
    let literal = Literal {
        variants: &[SELECTED, 8, SELECTED],
        index: Number::<u32>::LE,
    };
    assert_eq!(literal.encode(&SELECTED, 4), Ok(vec![0; 4]));
    assert_eq!(literal.encode(&1, 4), Err(Error::InvalidDiscriminator));

    assert_eq!(
        literal.decode(&[3, 0, 0, 0], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    let choice = Choice(Unit, Number::<u16>::LE);
    assert_eq!(choice.encode(&Either::First(()), 3), Ok(vec![0]));
    assert_eq!(choice.decode(&[1, 2, 3], LIMITS), Ok(Either::Second(770)));
    assert_eq!(
        choice.decode(&[2], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(choice.decode(&[1, 2], LIMITS), Err(Error::Truncated));
    assert_eq!(Unit.decode(&[1], LIMITS), Err(Error::TrailingBytes));
    let remainder = Sequence {
        item: Unit,
        count: Count::Remainder,
    };

    assert_eq!(remainder.decode(&[1], LIMITS), Err(Error::InvalidValue));
}

#[test]
fn nested_counts_and_dependent_records_share_allocation_claims() {
    let counted = Counted {
        prefix: Number::<u32>::LE,
        item: Unit,
    };
    assert_eq!(
        counted.decode(&[255, 255, 255, 255], Limits::new(4, 3)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        counted.decode(&[2, 0, 0, 0], Limits::new(4, 2)),
        Ok(vec![(), ()])
    );
}

#[test]
fn dependent_records_share_allocation_claims() {
    let record = Dependent(Number::<u8>::LE, |count: &u8| {
        if *count > 3 {
            return Err(Error::InvalidValue);
        }

        Ok(OwnedSequence(Sequence {
            item: Number::<u16>::LE,
            count: Count::Fixed(usize::from(*count)),
        }))
    });
    let wire = [2, 1, 0, 255, 255];
    assert_eq!(
        record.decode(&wire, Limits::new(5, 2)),
        Ok((2, vec![1, U16_MAX]))
    );
    assert_eq!(
        record.decode(&wire, Limits::new(5, 1)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(record.encode(&(2, vec![1, U16_MAX]), 5), Ok(wire.to_vec()));
    assert_eq!(record.encode(&(1, vec![1, 2]), 5), Err(Error::InvalidValue));
    assert_eq!(record.decode(&[4], LIMITS), Err(Error::InvalidValue));
    let nested = Tuple(
        Sequence {
            item: Unit,
            count: Count::Fixed(1),
        },
        record,
    );
    assert_eq!(
        nested.decode(&wire, Limits::new(5, 2)),
        Err(Error::LimitExceeded)
    );

    assert_eq!(
        nested.decode(&wire, Limits::new(5, 3)),
        Ok((vec![()], (2, vec![1, U16_MAX])))
    );
}

#[test]
fn delimiters_padding_offsets_and_exact_frames() {
    let codec = Sentinel::new(Bytes, &[255, 254], 3).unwrap();
    assert_eq!(codec.encode(&[1, 2][..], 5), Ok(vec![1, 2, 255, 254]));
    assert_eq!(
        codec.decode_prefix(&[99, 1, 2, 255, 254, 88], 1, LIMITS),
        Ok((vec![1, 2], 5))
    );
    assert_eq!(
        codec.decode(&[1, 2, 255, 254, 88], LIMITS),
        Err(Error::TrailingBytes)
    );
    assert_eq!(
        codec.decode(&[1, 2, 255], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(
        codec.decode(&[1, 2, 3, 4, 255, 254], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(codec.encode(&[255, 254][..], 5), Err(Error::InvalidValue));
    assert_eq!(
        codec.encode(&[1, 2, 3, 4][..], 6),
        Err(Error::LimitExceeded)
    );

    assert_eq!(
        codec.decode(&[1, 2, 255, 254], Limits::new(4, 1)),
        Err(Error::LimitExceeded)
    );
}

#[test]
fn exact_sentinel_frames_and_overlap_reject() {
    let exact = Sentinel::new(Number::<u8>::LE, &[255], 2).unwrap();
    assert_eq!(
        exact.decode(&[1, 2, 255], LIMITS),
        Err(Error::TrailingBytes)
    );
    let overlap = Sentinel::new(Bytes, &[1, 1], 3).unwrap();
    assert_eq!(overlap.encode(&[1][..], 3), Err(Error::InvalidValue));
    assert!(Sentinel::new(Bytes, &[], 3).is_err());
    assert!(Sentinel::new(Bytes, &[1], usize::MAX).is_err());
}

#[test]
fn padding_skips_input_and_zeroes_written_offsets() {
    let padded = Padded {
        left: 1,
        item: Number::<u8>::LE,
        right: 2,
    };
    assert_eq!(padded.decode(&[7, SELECTED, 8, 9], LIMITS), Ok(SELECTED));
    assert_eq!(
        padded.decode(&[7, SELECTED, 8], LIMITS),
        Err(Error::Truncated)
    );
    let mut output = [99; 6];
    let mut writer = Writer::new(&mut output);
    writer.put(&[88]).unwrap();
    padded.write(&SELECTED, &mut writer).unwrap();
    assert_eq!(writer.position(), 5);

    assert_eq!(output, [88, 0, SELECTED, 0, 0, 99]);
}

#[path = "../../../verification/conformance/src/composites.rs"]
mod portable;

#[test]
fn isolated_portable_composite_cases() {
    portable::composition().unwrap();
    portable::bits().unwrap();
    portable::malformed().unwrap();
}

#[test]
fn sentinel_roundtrip_matches_first_delimiter_across_all_small_binary_inputs() {
    const MAX_SENTINEL: usize = 3;
    const INPUT_PAIRS: usize = 434;
    let mut checked = 0;

    for sentinel_width in 1..=MAX_SENTINEL {
        for sentinel_pattern in 0..1usize << sentinel_width {
            let sentinel = binary_pattern(sentinel_pattern, sentinel_width);
            checked += probe_payloads(&sentinel);
        }
    }

    assert_eq!(checked, INPUT_PAIRS);
}

fn probe_payloads(sentinel: &[u8]) -> usize {
    const MAX_PAYLOAD: usize = 4;
    let codec = Sentinel::new(Bytes, sentinel, MAX_PAYLOAD).unwrap();
    let mut checked = 0;

    for width in 0..=MAX_PAYLOAD {
        for pattern in 0..1usize << width {
            sentinel_probe(&codec, sentinel, &binary_pattern(pattern, width));
            checked += 1;
        }
    }

    checked
}

fn binary_pattern(pattern: usize, width: usize) -> Vec<u8> {
    (0..width)
        .map(|bit| if pattern & (1 << bit) == 0 { 1 } else { 2 })
        .collect()
}

fn sentinel_probe(codec: &Sentinel<'_, Bytes>, sentinel: &[u8], payload: &[u8]) {
    let mut combined = payload.to_vec();
    combined.extend_from_slice(sentinel);
    let first = combined
        .windows(sentinel.len())
        .position(|window| window == sentinel)
        .unwrap();
    let encoded = codec.encode(payload, MAX_WIRE);

    if first != payload.len() {
        assert_eq!(encoded, Err(Error::InvalidValue));

        return;
    }

    assert_eq!(encoded.unwrap(), combined);
    assert_eq!(codec.decode(&combined, LIMITS).unwrap(), payload);
}
