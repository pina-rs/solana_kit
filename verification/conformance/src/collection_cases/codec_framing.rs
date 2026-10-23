use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

use solana_kit_core::codecs::{
    ByteSizePrefix, Bytes, Decoder, Encoder, Error, FixedSize, FrameConsumption, Limits, Number,
    Offset, OffsetScope, Padded, Reader, Resized, Reverse, Sentinel, Unit, Writer,
};

const SCRATCH: usize = 32;
const WIRE: usize = 64;
const WORD: u16 = 513;
const STEP: usize = 17;
const MAX_FIELD: usize = 5;
const MAX_PAYLOAD: usize = 6;
const NESTED_WIRE: usize = 18;
const MAX_FLOAT: f64 = 16.0;

const LIMITS: Limits = Limits::new(4096, 4096);

fn fixed(size: usize, decoded_size: Option<usize>) -> FixedSize<Bytes> {
    FixedSize {
        item: Bytes,
        size,
        decoded_size,
        allow_truncation: true,
        max_encoded_size: SCRATCH,
    }
}

pub(super) fn fixed_fields_truncate_extend_and_clear_dirty_padding() {
    for size in 0..=MAX_FIELD {
        for length in 0..=MAX_PAYLOAD {
            let input = vec![7; length];

            let codec = fixed(size, None);

            let bytes = codec.encode(input.as_slice(), SCRATCH).unwrap();

            let mut expected = vec![0; size];
            expected[..size.min(length)].fill(7);

            assert_eq!(bytes, expected);

            assert_eq!(codec.decode(&bytes, LIMITS).unwrap(), expected);
        }
    }

    let codec = fixed(4, Some(MAX_PAYLOAD));

    assert_eq!(
        codec.decode(&[1, 2, 3, 4], LIMITS).unwrap(),
        [1, 2, 3, 4, 0, 0]
    );

    assert_eq!(
        fixed(4, Some(2)).decode(&[1, 2, 3, 4], LIMITS).unwrap(),
        [1, 2]
    );

    let mut dirty = [9; 8];

    let mut writer = Writer::new(&mut dirty);
    writer.seek(2).unwrap();
    codec.write(&[1, 2][..], &mut writer).unwrap();

    assert_eq!(writer.position(), 6);

    assert_eq!(dirty, [9, 9, 1, 2, 0, 0, 9, 9]);
}

pub(super) fn fixed_failures_retain_documented_cursor_and_budget() {
    let mut codec = fixed(2, Some(8));
    codec.allow_truncation = false;

    let mut output = [9; 4];

    let mut writer = Writer::new(&mut output);

    assert_eq!(
        codec.write(&[1, 2, 3][..], &mut writer),
        Err(Error::InvalidValue)
    );

    assert_eq!(writer.position(), 0);

    assert_eq!(output, [9; 4]);

    assert_eq!(codec.encode(&[0; 33][..], WIRE), Err(Error::LimitExceeded));

    assert_eq!(codec.decode(&[1], LIMITS), Err(Error::Truncated));

    let mut reader = Reader::new(&[1, 2, 3], Limits::new(3, 7)).unwrap();

    assert_eq!(codec.read(&mut reader), Err(Error::LimitExceeded));

    assert_eq!(reader.position(), 2);
    reader.claim(7).unwrap();

    assert_eq!(reader.claim(1), Err(Error::LimitExceeded));
}

pub(super) fn selected_prefixes_use_encoded_bytes_and_explicit_consumption() {
    let exact = ByteSizePrefix {
        item: Number::<u16>::LE,
        prefix: Number::<u64>::BE,
        consumption: FrameConsumption::Exact,
    };

    assert_eq!(
        exact.encode(&WORD, 10).unwrap(),
        [0, 0, 0, 0, 0, 0, 0, 2, 1, 2]
    );

    assert_eq!(
        exact
            .decode(&[0, 0, 0, 0, 0, 0, 0, 2, 1, 2], LIMITS)
            .unwrap(),
        513
    );

    let skip = ByteSizePrefix {
        consumption: FrameConsumption::Skip,
        ..exact
    };

    let bytes = [99, 0, 0, 0, 0, 0, 0, 0, 3, 1, 2, 7, 88];

    assert_eq!(skip.decode_prefix(&bytes, 1, LIMITS).unwrap(), (WORD, 12));

    assert_eq!(
        exact.decode_prefix(&bytes, 1, LIMITS),
        Err(Error::TrailingBytes)
    );

    assert_eq!(skip.decode(&bytes[1..], LIMITS), Err(Error::TrailingBytes));

    assert_eq!(skip.decode(&[0; 7], LIMITS), Err(Error::Truncated));

    assert_eq!(skip.decode(&[255; 8], LIMITS), Err(Error::Overflow));

    let mut reader = Reader::new(&[0, 0, 0, 0, 0, 0, 0, 3, 1], LIMITS).unwrap();

    assert_eq!(skip.read(&mut reader), Err(Error::Truncated));

    assert_eq!(reader.position(), 8);
}

pub(super) fn nested_prefix_and_scratch_claims_share_one_budget() {
    let nested = ByteSizePrefix {
        item: ByteSizePrefix {
            item: Bytes,
            prefix: Number::<u64>::LE,
            consumption: FrameConsumption::Skip,
        },
        prefix: Number::<u64>::LE,
        consumption: FrameConsumption::Exact,
    };

    let wire = nested.encode(&[4, 5][..], NESTED_WIRE).unwrap();

    assert_eq!(
        nested.decode(&wire, Limits::new(NESTED_WIRE, 1)),
        Err(Error::LimitExceeded)
    );

    assert_eq!(
        nested.decode(&wire, Limits::new(NESTED_WIRE, 2)).unwrap(),
        [4, 5]
    );

    let reversed = Reverse {
        item: Bytes,
        size: 2,
    };

    assert_eq!(
        reversed.decode(&[1, 2], Limits::new(2, 3)),
        Err(Error::LimitExceeded)
    );

    assert_eq!(reversed.decode(&[1, 2], Limits::new(2, 4)).unwrap(), [2, 1]);
}

pub(super) fn resize_changes_extent_without_changing_write_or_read_movement() {
    let larger = Resized {
        item: Number::<u16>::LE,
        resize: |size: usize| size.checked_add(2).ok_or(Error::Overflow),
    };

    assert_eq!(larger.encoded_len(&WORD).unwrap(), 4);

    assert_eq!(larger.encode(&WORD, 4).unwrap(), [1, 2, 0, 0]);

    assert_eq!(
        larger.decode_prefix(&[1, 2, 9, 9], 0, LIMITS).unwrap(),
        (WORD, 2)
    );

    assert_eq!(
        larger.decode(&[1, 2, 0, 0], LIMITS),
        Err(Error::TrailingBytes)
    );

    assert_eq!(larger.encode(&WORD, 3), Err(Error::LimitExceeded));

    let smaller = Resized {
        item: Number::<u16>::LE,
        resize: |_| Ok(1),
    };

    assert_eq!(smaller.encode(&WORD, 4), Err(Error::Truncated));

    let invalid = Resized {
        item: Unit,
        resize: |_| Err(Error::InvalidValue),
    };

    assert_eq!(invalid.encoded_len(&()), Err(Error::InvalidValue));

    let overflow = Resized {
        item: Unit,
        resize: |_| usize::MAX.checked_add(1).ok_or(Error::Overflow),
    };

    assert_eq!(overflow.encoded_len(&()), Err(Error::Overflow));
}

pub(super) fn reverse_fixed_width_preserves_nonzero_boundaries() {
    let codec = Reverse {
        item: Number::<u32>::LE,
        size: 4,
    };

    assert_eq!(codec.encode(&0x0102_0304, 4).unwrap(), [1, 2, 3, 4]);

    assert_eq!(
        codec.decode_prefix(&[9, 1, 2, 3, 4, 8], 1, LIMITS).unwrap(),
        (0x0102_0304, 5)
    );

    assert_eq!(codec.decode(&[1, 2, 3], LIMITS), Err(Error::Truncated));

    assert_eq!(
        Reverse {
            item: Number::<u16>::LE,
            size: 1
        }
        .encode(&3, 2),
        Err(Error::InvalidValue)
    );

    assert_eq!(
        Reverse {
            item: Unit,
            size: 0
        }
        .decode(&[], LIMITS)
        .unwrap(),
        ()
    );
}

fn rewind(scope: OffsetScope<'_>) -> Result<usize, Error> {
    Ok(scope.original)
}
fn previous(scope: OffsetScope<'_>) -> Result<usize, Error> {
    Ok(scope.wrap(-1))
}

pub(super) fn offset_callbacks_see_whole_buffer_and_allow_backwards_movement() {
    let codec = Offset {
        item: Number::<u8>::LE,
        pre: previous,
        post: rewind,
    };

    assert_eq!(codec.decode_prefix(&[1, 2, 3], 1, LIMITS).unwrap(), (3, 1));

    assert_eq!(codec.encode(&7, 1).unwrap(), [7]);

    let mut bytes = [1, 2, 3];

    let mut writer = Writer::new(&mut bytes);
    writer.seek(1).unwrap();
    codec.write(&7, &mut writer).unwrap();

    assert_eq!(writer.position(), 1);

    assert_eq!(writer.bytes(), [1, 2, 7]);

    let scope = OffsetScope {
        bytes: &[],
        original: 0,
        start: 0,
        end: 0,
    };

    assert_eq!(scope.wrap(isize::MIN), 0);

    let scope = OffsetScope {
        bytes: &[0; 3],
        ..scope
    };

    assert_eq!(scope.wrap(-4), 2);

    assert_eq!(scope.wrap(3), 0);

    assert!(scope.wrap(isize::MIN) < 3);
}

pub(super) fn offset_failures_do_not_rollback_writes_or_refund_claims() {
    let invalid = Offset {
        item: Number::<u8>::LE,
        pre: |_: OffsetScope<'_>| Ok(2),
        post: rewind,
    };

    let mut reader = Reader::new(&[1], LIMITS).unwrap();

    assert_eq!(invalid.read(&mut reader), Err(Error::Truncated));

    assert_eq!(reader.position(), 0);

    let invalid = Offset {
        item: Bytes,
        pre: rewind,
        post: |_: OffsetScope<'_>| Err(Error::InvalidValue),
    };

    assert_eq!(invalid.read(&mut reader), Err(Error::InvalidValue));

    assert_eq!(reader.position(), 1);
    reader.seek(0).unwrap();

    assert_eq!(reader.claim(4096), Err(Error::LimitExceeded));

    let invalid = Offset {
        item: Number::<u8>::LE,
        pre: rewind,
        post: |_: OffsetScope<'_>| Ok(2),
    };

    let mut bytes = [9];

    let mut writer = Writer::new(&mut bytes);

    assert_eq!(invalid.write(&7, &mut writer), Err(Error::Truncated));

    assert_eq!(writer.position(), 1);

    assert_eq!(writer.bytes(), [7]);
}

pub(super) fn dart_sentinel_matches_empty_and_overlap_semantics() {
    let codec = Sentinel::dart(Bytes, &[1, 1], 8).unwrap();

    assert_eq!(codec.encode(&[1][..], 3).unwrap(), [1, 1, 1]);

    assert_eq!(
        codec.decode_prefix(&[9, 1, 1, 1, 8], 1, LIMITS).unwrap(),
        (vec![], 3)
    );

    assert_eq!(codec.encode(&[1, 1][..], 4), Err(Error::InvalidValue));

    let empty = Sentinel::dart(Unit, &[], 0).unwrap();

    assert_eq!(empty.encode(&(), 0).unwrap(), [] as [u8; 0]);

    assert_eq!(empty.decode(&[], LIMITS), Err(Error::InvalidDiscriminator));

    assert_eq!(empty.decode_prefix(&[9], 0, LIMITS).unwrap(), ((), 0));

    assert_eq!(
        Sentinel::dart(Bytes, &[], 2).unwrap().encode(&[1][..], 2),
        Err(Error::InvalidValue)
    );

    assert_eq!(
        Sentinel::dart(Unit, &[0], usize::MAX).unwrap_err(),
        Error::Overflow
    );

    let wide = [8; 257];

    assert_eq!(
        Sentinel::dart(Unit, &wide, 0)
            .unwrap()
            .decode(&wide, LIMITS)
            .unwrap(),
        ()
    );

    let skip = Sentinel::dart(Number::<u8>::LE, &[255], 4).unwrap();

    assert_eq!(skip.decode(&[1, 2, 255], LIMITS).unwrap(), 1);

    assert_eq!(
        skip.decode(&[1, 2], LIMITS),
        Err(Error::InvalidDiscriminator)
    );

    assert_eq!(skip.encode(&1, 1), Err(Error::LimitExceeded));
}

pub(super) fn existing_padding_and_exact_sentinel_contracts_remain() {
    let codec = Padded {
        left: 1,
        item: Number::<u8>::LE,
        right: 1,
    };

    assert_eq!(codec.decode(&[9, 7, 9], LIMITS).unwrap(), 7);

    let mut bytes = [9; 3];
    codec.write(&7, &mut Writer::new(&mut bytes)).unwrap();

    assert_eq!(bytes, [0, 7, 0]);

    assert!(Sentinel::new(Unit, &[], 0).is_err());

    let codec = Sentinel::new(Bytes, &[1, 1], 2).unwrap();

    assert_eq!(codec.encode(&[1][..], 4), Err(Error::InvalidValue));
}

extern crate alloc;
use crate::framing as portable;

pub(super) fn portable_framing_functions_execute_natively() {
    portable::transforms().unwrap();
    portable::boundaries().unwrap();
}

fn parsed_bytes(field: &str) -> Vec<u8> {
    if field == "-" {
        return vec![];
    }
    field.split(',').map(|byte| byte.parse().unwrap()).collect()
}

fn narrow_count(value: &u64) -> Result<u8, Error> {
    u8::try_from(*value).map_err(|_| Error::Overflow)
}

fn wide_count(value: u8) -> Result<u64, Error> {
    Ok(u64::from(value))
}

fn sentinel_row(fields: &[&str]) {
    let payload = parsed_bytes(fields[1]);

    let delimiter = parsed_bytes(fields[2]);

    let codec = Sentinel::dart(Bytes, &delimiter, SCRATCH).unwrap();

    let encoded = codec.encode(payload.as_slice(), WIRE);

    if fields[3] == "error" {
        assert_eq!(encoded, Err(Error::InvalidValue));
        return;
    }

    assert_eq!(encoded.unwrap(), parsed_bytes(fields[3]));
}

fn ordinary_row(fields: &[&str]) -> Vec<u8> {
    use solana_kit_core::codecs::Mapped;

    let width: usize = fields[1].parse().unwrap();

    let length: usize = fields[2].parse().unwrap();

    let input: Vec<_> = (0..length)
        .map(|index| u8::try_from(index * STEP + 3).unwrap())
        .collect();

    match fields[0] {
        "fixed" => fixed(width, None).encode(input.as_slice(), WIRE),
        "prefix" => ByteSizePrefix {
            item: Bytes,
            prefix: Mapped::new(Number::<u8>::LE, narrow_count, wide_count),
            consumption: FrameConsumption::Skip,
        }
        .encode(input.as_slice(), WIRE),
        "resize" => Resized {
            item: Bytes,
            resize: |size: usize| size.checked_add(width).ok_or(Error::Overflow),
        }
        .encode(input.as_slice(), WIRE),
        "reverse" => reverse_row(width),
        _ => panic!("unknown oracle family"),
    }
    .unwrap()
}

fn reverse_row(width: usize) -> Result<Vec<u8>, Error> {
    let value: Vec<_> = (1..=width)
        .map(|index| u8::try_from(index).unwrap())
        .collect();
    Reverse {
        item: Bytes,
        size: width,
    }
    .encode(value.as_slice(), WIRE)
}

pub(super) fn actual_dart_oracle_replays_all_rows() {
    for line in include_str!("../../../codec-framing/dart.tsv").lines() {
        let fields: Vec<_> = line.split('\t').collect();

        if fields[0] == "sentinel" {
            sentinel_row(&fields);
            continue;
        }

        assert_eq!(ordinary_row(&fields), parsed_bytes(fields[3]), "{line}");
    }
}

pub(super) fn oversized_scratch_and_extent_fail_without_panicking() {
    let huge = Resized {
        item: Unit,
        resize: |_| Ok(usize::MAX),
    };

    assert_eq!(huge.encode(&(), usize::MAX), Err(Error::AllocationFailed));

    let field = FixedSize {
        item: Unit,
        size: 0,
        decoded_size: Some(usize::MAX),
        allow_truncation: true,
        max_encoded_size: 0,
    };

    assert_eq!(
        field.decode(&[], Limits::new(0, usize::MAX)),
        Err(Error::AllocationFailed)
    );

    let over = Padded {
        left: usize::MAX,
        item: Unit,
        right: 1,
    };

    assert_eq!(over.encoded_len(&()), Err(Error::Overflow));
}

fn checked_float(value: f64) -> Result<u64, Error> {
    if !value.is_finite() || value < 0.0 || value.fract() != 0.0 || value > MAX_FLOAT {
        return Err(Error::InvalidValue);
    }

    value
        .to_string()
        .parse::<u64>()
        .map_err(|_| Error::InvalidValue)
}

pub(super) fn malformed_numeric_prefix_domains_are_validated_in_typed_adapters() {
    use solana_kit_core::codecs::Mapped;

    let prefix = Mapped::new(
        Number::<f64>::LE,
        |value: &u64| {
            value
                .to_string()
                .parse::<f64>()
                .map_err(|_| Error::InvalidValue)
        },
        checked_float,
    );

    let codec = ByteSizePrefix {
        item: Bytes,
        prefix,
        consumption: FrameConsumption::Skip,
    };

    for value in [-1.0, 0.5, f64::INFINITY, f64::NAN] {
        assert_eq!(
            codec.decode(&value.to_le_bytes(), LIMITS),
            Err(Error::InvalidValue)
        );
    }

    assert_eq!(
        codec.decode(&0.0f64.to_le_bytes(), LIMITS).unwrap(),
        [] as [u8; 0]
    );
}

pub(super) fn offset_contexts_observe_entry_exit_and_post_write_bytes() {
    let codec = Offset {
        item: Number::<u8>::LE,
        pre: |scope: OffsetScope<'_>| {
            assert_eq!((scope.original, scope.start, scope.end), (1, 1, 1));
            assert_eq!(scope.bytes.len(), 3);
            Ok(2)
        },
        post: |scope: OffsetScope<'_>| {
            assert_eq!((scope.original, scope.start, scope.end), (1, 2, 3));
            assert_eq!(scope.bytes, [9, 8, 7]);
            Ok(0)
        },
    };

    assert_eq!(codec.decode_prefix(&[9, 8, 7], 1, LIMITS).unwrap(), (7, 0));

    let mut storage = [9, 8, 0];
    let mut writer = Writer::new(&mut storage);
    writer.seek(1).unwrap();
    codec.write(&7, &mut writer).unwrap();

    assert_eq!(writer.position(), 0);
}

pub(super) fn seek_and_callback_failures_preserve_checked_positions() {
    let codec = Offset {
        item: Number::<u16>::LE,
        pre: |_: OffsetScope<'_>| Err(Error::InvalidValue),
        post: rewind,
    };
    let mut reader = Reader::new(&[1], LIMITS).unwrap();

    assert_eq!(codec.read(&mut reader), Err(Error::InvalidValue));
    assert_eq!(reader.position(), 0);
    reader.seek(1).unwrap();

    assert_eq!(reader.seek(2), Err(Error::Truncated));
    assert_eq!(reader.position(), 1);

    let codec = Offset {
        item: Number::<u16>::LE,
        pre: rewind,
        post: rewind,
    };

    assert_eq!(codec.read(&mut reader), Err(Error::Truncated));
    assert_eq!(reader.position(), 1);

    let mut bytes = [0];
    let mut writer = Writer::new(&mut bytes);
    writer.seek(1).unwrap();

    assert_eq!(writer.seek(2), Err(Error::Truncated));
    assert_eq!(writer.position(), 1);
}

pub(super) fn prefix_frames_keep_exact_write_contract_with_metadata_resize() {
    let codec = ByteSizePrefix {
        item: Resized {
            item: Number::<u8>::LE,
            resize: |_| Ok(2),
        },
        prefix: Number::<u64>::LE,
        consumption: FrameConsumption::Skip,
    };

    assert_eq!(codec.encode(&1, 10), Err(Error::InvalidValue));
}
