use alloc::{vec, vec::Vec};
use solana_kit_core::{
    codecs::{
        Bool, Count, Decoder, Encoder, Error, Limits, Number, OrderedMapCodec, OrderedSetCodec,
        OwnedSequence, Reader, Sequence, Tuple, Unit,
    },
    collections::{OrderedMap, OrderedSet},
};

const MAX_BYTES: usize = 64;
const PREFIXED_U16_BYTES: usize = 6;
const NESTED_ALLOCATION_CLAIMS: usize = 6;

const LIMITS: Limits = Limits::new(MAX_BYTES, MAX_BYTES);

/// Duplicate reduction changes only the value, never bytes consumed or claims.
pub fn duplicate_claims() {
    let map = OrderedMapCodec(Sequence {
        item: Tuple(Number::<u8>::LE, Number::<u8>::LE),
        count: Count::U32,
    });
    let wire = [3, 0, 0, 0, 2, 20, 1, 10, 2, 99];
    let canonical = [2, 0, 0, 0, 2, 99, 1, 10];
    let mut reader = Reader::new(&wire, Limits::new(MAX_BYTES, 3)).unwrap();
    let value = map.read(&mut reader).unwrap();
    assert_eq!(value.as_slice(), &[(2, 99), (1, 10)]);
    assert_eq!(reader.position(), wire.len());
    assert_eq!(reader.claim(1), Err(Error::LimitExceeded));
    assert_eq!(map.encode(&value, MAX_BYTES).unwrap(), canonical);

    let mut denied = Reader::new(&wire, Limits::new(MAX_BYTES, 2)).unwrap();
    assert_eq!(map.read(&mut denied), Err(Error::LimitExceeded));
    assert_eq!(denied.position(), 4);

    set_duplicate_claims();
    nonzero_offset();
}

fn set_duplicate_claims() {
    let set = OrderedSetCodec(Sequence {
        item: Number::<u8>::LE,
        count: Count::U32,
    });
    let wire = [3, 0, 0, 0, 2, 1, 2];
    let mut reader = Reader::new(&wire, Limits::new(MAX_BYTES, 3)).unwrap();
    let value = set.read(&mut reader).unwrap();
    assert_eq!(value.as_slice(), &[2, 1]);
    assert_eq!(reader.position(), wire.len());
    assert_eq!(reader.claim(1), Err(Error::LimitExceeded));
    assert_eq!(set.encode(&value, MAX_BYTES).unwrap(), [2, 0, 0, 0, 2, 1]);
}

fn nonzero_offset() {
    let map = OrderedMapCodec(Sequence {
        item: Tuple(Number::<u8>::LE, Number::<u8>::LE),
        count: Count::U32,
    });
    let set = OrderedSetCodec(Sequence {
        item: Number::<u8>::LE,
        count: Count::U32,
    });
    let map_input = [77, 2, 0, 0, 0, 2, 20, 2, 99, 88];
    let set_input = [77, 2, 0, 0, 0, 2, 2, 88];
    let (value, next) = map.decode_prefix(&map_input, 1, LIMITS).unwrap();
    assert_eq!(value.as_slice(), &[(2, 99)]);
    assert_eq!(next, 9);
    let (value, next) = set.decode_prefix(&set_input, 1, LIMITS).unwrap();
    assert_eq!(value.as_slice(), &[2]);
    assert_eq!(next, 7);
}

/// Malformed values and nested payloads retain existing shared-budget failures.
pub fn malformed_and_nested() {
    let map = OrderedMapCodec(Sequence {
        item: Tuple(Number::<u8>::LE, Bool),
        count: Count::U32,
    });
    let set = OrderedSetCodec(Sequence {
        item: Number::<u16>::LE,
        count: Count::U32,
    });
    assert_eq!(
        map.decode(&[1, 0, 0, 0, 7, 2], LIMITS),
        Err(Error::InvalidBoolean { value: 2 })
    );
    assert_eq!(map.decode(&[1, 0, 0, 0, 7], LIMITS), Err(Error::Truncated));
    assert_eq!(set.decode(&[1, 0, 0, 0, 7], LIMITS), Err(Error::Truncated));
    assert_eq!(
        set.decode(&[0, 0, 0, 0, 7], LIMITS),
        Err(Error::TrailingBytes)
    );
    assert_eq!(
        set.decode(&[0; 4], Limits::new(3, MAX_BYTES)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        set.encode(&OrderedSet::from_values(vec![1u16]), PREFIXED_U16_BYTES - 1),
        Err(Error::LimitExceeded)
    );

    nested_claims();
    zero_width();
}

fn nested_claims() {
    let map = OrderedMapCodec(Sequence {
        item: Tuple(
            Number::<u8>::LE,
            OwnedSequence(Sequence {
                item: Number::<u8>::LE,
                count: Count::U32,
            }),
        ),
        count: Count::U32,
    });
    let bytes = [2, 0, 0, 0, 1, 2, 0, 0, 0, 7, 8, 1, 2, 0, 0, 0, 3, 4];
    let value = map
        .decode(&bytes, Limits::new(MAX_BYTES, NESTED_ALLOCATION_CLAIMS))
        .unwrap();
    assert_eq!(value.as_slice(), &[(1, vec![3, 4])]);

    let mut reader =
        Reader::new(&bytes, Limits::new(MAX_BYTES, NESTED_ALLOCATION_CLAIMS - 1)).unwrap();
    assert_eq!(map.read(&mut reader), Err(Error::LimitExceeded));
    assert_eq!(reader.position(), 16);

    assert_eq!(reader.claim(1), Ok(()));
    assert_eq!(reader.claim(1), Err(Error::LimitExceeded));
}

fn zero_width() {
    let map = OrderedMapCodec(Sequence {
        item: Tuple(Unit, Unit),
        count: Count::Fixed(3),
    });
    let set = OrderedSetCodec(Sequence {
        item: Unit,
        count: Count::Fixed(3),
    });
    assert_eq!(
        map.decode(&[], Limits::new(0, 3)).unwrap().as_slice(),
        &[((), ())]
    );
    assert_eq!(
        set.decode(&[], Limits::new(0, 3)).unwrap().as_slice(),
        &[()]
    );
    assert_eq!(
        map.decode(&[], Limits::new(0, 2)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        set.decode(&[], Limits::new(0, 2)),
        Err(Error::LimitExceeded)
    );

    assert_eq!(
        map.encode(&OrderedMap::from_entries(vec![((), ())]), 0),
        Err(Error::InvalidValue)
    );
    assert_eq!(
        set.encode(&OrderedSet::from_values(vec![()]), 0),
        Err(Error::InvalidValue)
    );

    let remainder = OrderedSetCodec(Sequence {
        item: Unit,
        count: Count::Remainder,
    });

    assert_eq!(
        remainder.encode(&OrderedSet::from_values(vec![()]), 0),
        Err(Error::InvalidValue)
    );
    assert_eq!(remainder.decode(&[1], LIMITS), Err(Error::InvalidValue));
}

/// Exhausts small wire orders independently of the collection constructors.
pub fn small_sequences() {
    const ALPHABET: usize = 3;
    const MAX_ITEMS: u32 = 4;

    for length in 0..=MAX_ITEMS {
        for mut word in 0..ALPHABET.pow(length) {
            let values: Vec<_> = (0..length)
                .map(|_| {
                    let value = u8::try_from(word % ALPHABET).unwrap();
                    word /= ALPHABET;
                    value
                })
                .collect();
            check_small(&values);
        }
    }
}

fn check_small(values: &[u8]) {
    let first: Vec<_> = values
        .iter()
        .enumerate()
        .filter_map(|(index, &value)| {
            (values.iter().position(|item| *item == value) == Some(index)).then_some(value)
        })
        .collect();
    let set_sequence = Sequence {
        item: Number::<u8>::LE,
        count: Count::U32,
    };
    let set = OrderedSetCodec(set_sequence);
    let raw = set_sequence.encode(values, MAX_BYTES).unwrap();
    let decoded = set.decode(&raw, LIMITS).unwrap();
    assert_eq!(decoded.as_slice(), first.as_slice());
    assert_eq!(
        set.encode(&decoded, MAX_BYTES),
        set_sequence.encode(first.as_slice(), MAX_BYTES)
    );

    check_small_map(values, &first);
}

fn check_small_map(values: &[u8], first: &[u8]) {
    let pairs: Vec<_> = values
        .iter()
        .enumerate()
        .map(|(index, &key)| (key, u8::try_from(index).unwrap()))
        .collect();
    let expected: Vec<_> = first
        .iter()
        .map(|&key| {
            let last = values.iter().rposition(|item| *item == key).unwrap();
            (key, u8::try_from(last).unwrap())
        })
        .collect();
    let sequence = Sequence {
        item: Tuple(Number::<u8>::LE, Number::<u8>::LE),
        count: Count::U32,
    };
    let map = OrderedMapCodec(sequence);
    let raw = sequence.encode(pairs.as_slice(), MAX_BYTES).unwrap();
    let decoded = map.decode(&raw, LIMITS).unwrap();

    assert_eq!(decoded.as_slice(), expected.as_slice());
    assert_eq!(
        map.encode(&decoded, MAX_BYTES),
        sequence.encode(expected.as_slice(), MAX_BYTES)
    );
    assert_eq!(decoded, OrderedMap::from_entries(expected));
}
